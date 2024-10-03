use derive_more::From;
use std::collections::HashSet;
use std::io::{stderr, stdout};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing::Subscriber;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{fmt, registry, Layer};
use tracing_subscriber::filter::filter_fn;

#[derive(Debug)]
pub struct Stdout(pub LevelFilter);
#[derive(Debug)]
pub struct Stderr(pub LevelFilter);
#[derive(Debug)]
pub struct File(pub LevelFilter, pub PathBuf);

impl File {
    /// Creates a new file target, generic over the path
    pub fn new(level: LevelFilter, path: impl AsRef<Path>) -> Self {
        Self(level, path.as_ref().to_path_buf())
    }
}

#[derive(Debug, From)]
pub enum Target {
    Stdout(Stdout),
    Stderr(Stderr),
    File(File),
}

impl Target {
    fn layer<S>(
        self,
        options: &LoggingOptions,
    ) -> Result<Box<dyn Layer<S> + Send + Sync + 'static>, InitializeLoggingError>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let layer = fmt::layer()
            .event_format(
                options
                    .format
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| fmt::format()),
            )
            .with_thread_ids(options.with_thread_ids)
            .with_thread_names(options.with_thread_names)
            .with_file(options.with_files)
            .with_line_number(options.with_lines);
        #[cfg(test)]
        let layer = layer.with_test_writer();

        let l = match self {
            Target::Stdout(Stdout(filter)) => layer.with_writer(stdout).with_filter(filter).boxed(),
            Target::Stderr(Stderr(filter)) => layer.with_writer(stderr).with_filter(filter).boxed(),
            Target::File(File(filter, path)) => {
                let file = std::fs::File::create(path)?;
                layer.with_writer(file).with_filter(filter).boxed()
            }
        };
        Ok(l)
    }
}

#[derive(Debug)]
struct Targets {
    targets: Vec<Target>,
}

impl Targets {
    fn new<I: IntoIterator<Item=Target>>(iter: I) -> Result<Self, InitializeLoggingError> {
        let mut out = false;
        let mut err = false;
        let mut paths = HashSet::new();

        let mut ret = Targets {
            targets: Vec::new(),
        };

        for target in iter {
            match target {
                Target::Stdout(_) if !out => {
                    out = true;
                    ret.targets.push(target);
                }
                Target::Stderr(_) if !err => {
                    err = true;
                    ret.targets.push(target);
                }
                Target::File(File(_, ref path)) if !paths.contains(path) => {
                    paths.insert(path.clone());
                    ret.targets.push(target);
                }
                _ => return Err(InitializeLoggingError::CannotReEmitToSameTarget(target)),
            }
        }

        Ok(ret)
    }

    fn into_layer<S: Subscriber + for<'a> LookupSpan<'a>>(mut self, options: &LoggingOptions) -> Result<impl Layer<S> + 'static, InitializeLoggingError> {
        let mut layers = vec![];

        let targets = self.targets.drain(..).collect::<Vec<_>>();
        for x in targets {
            let layer = x.layer(options)?;
            layers.push(layer);
        }

        Ok(layers)
    }

    fn into_subscriber(
        mut self,
        options: &LoggingOptions,
    ) -> Result<impl Subscriber, InitializeLoggingError> {
        let layers = self.into_layer(&options)?;
        Ok(registry().with(layers))
    }
}

/// Logging options to be used for initializing tracing for a binary
#[derive(Debug)]
pub struct LoggingOptions {
    pub default_level: LevelFilter,
    format: Option<Format>,
    targets: Vec<Target>,
    with_thread_ids: bool,
    with_thread_names: bool,
    with_files: bool,
    with_lines: bool,
}

impl LoggingOptions {
    /// Creates a new logging options
    pub const fn new() -> Self {
        Self {
            default_level: LevelFilter::TRACE,
            format: None,
            targets: vec![],
            with_thread_ids: false,
            with_thread_names: false,
            with_files: false,
            with_lines: false,
        }
    }

    pub fn level(mut self, level: LevelFilter) -> Self {
        self.default_level = level;
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn target<T>(mut self, target: T) -> Self
    where
        Target: From<T>,
    {
        self.targets.push(Target::from(target));
        self
    }

    pub fn thread_ids(mut self, with_thread_ids: bool) -> Self {
        self.with_thread_ids = with_thread_ids;
        self
    }
    pub fn thread_names(mut self, with_thread_names: bool) -> Self {
        self.with_thread_names = with_thread_names;
        self
    }
    pub fn files(mut self, with_files: bool) -> Self {
        self.with_files = with_files;
        self
    }
    pub fn lines(mut self, with_lines: bool) -> Self {
        self.with_lines = with_lines;
        self
    }

    /// Gets the number of targets
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    pub fn into_subscriber(mut self) -> Result<impl Subscriber, InitializeLoggingError> {
        let targets = Targets::new(self.targets.drain(..))?;

        Ok(self.default_level.with_subscriber(
            targets.into_subscriber(&self)?
        ))
    }

    pub fn into_layer<S : Subscriber + for<'w> LookupSpan<'w>>(mut self) -> Result<impl Layer<S>, InitializeLoggingError> {
        let targets = Targets::new(self.targets.drain(..))?;

        let targets_layer = targets.into_layer::<S>(&self)?.with_filter(self.default_level);

        Ok(targets_layer)
    }
}

/// Attempts to initialize logging with the given options
pub fn try_init_logging(logging_options: LoggingOptions) -> Result<(), InitializeLoggingError> {
    set_global_default(logging_options.into_subscriber()?)?;
    Ok(())
}

/// Initializes logging, panicking if something went wrong
pub fn init_logging(logging_options: LoggingOptions) {
    try_init_logging(logging_options).expect("failed to initialize logging")
}

/// An error occurred while trying to initialize logging
#[derive(Debug, Error)]
pub enum InitializeLoggingError {
    #[error("Can not emit to {0:?} multiple times")]
    CannotReEmitToSameTarget(Target),
    #[error(transparent)]
    SetGlobalDefault(#[from] SetGlobalDefaultError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("No targets have been set")]
    NoTargets,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_targets() {
        let _ = Targets::new([Stdout(LevelFilter::OFF).into()]).expect("failed to create targets");

        let _ = Targets::new([
            Stdout(LevelFilter::OFF).into(),
            Stdout(LevelFilter::OFF).into(),
        ])
            .expect_err("can not repeat");
    }
}
