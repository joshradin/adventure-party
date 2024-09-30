use derive_more::From;
use std::collections::HashSet;
use std::io::{stderr, stdout};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing::Subscriber;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::{fmt, Layer, Registry};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;

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
    fn layer<S>(self, fmt: &Format) -> Result<Box<dyn Layer<S> + Send + Sync + 'static>, InitializeLoggingError>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let layer = fmt::layer()
            .event_format(fmt.clone())
            .with_thread_ids(true)
            ;
        #[cfg(test)]
        let layer = layer.with_test_writer();

        let l = match self {
            Target::Stdout(Stdout(filter)) => {
                layer
                    .with_writer(
                        stdout
                    )
                    .with_filter(filter)
                    .boxed()
            }
            Target::Stderr(Stderr(filter)) => {
                layer
                    .with_writer(
                        stderr
                    )
                    .with_filter(filter)
                    .boxed()
            }
            Target::File(File(filter, path)) => {
                let file = std::fs::File::create(path)?;
                layer
                    .with_writer(
                        file
                    )
                    .with_filter(filter)
                    .boxed()
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

        let mut ret = Targets { targets: Vec::new() };

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
                _ => {
                    return Err(InitializeLoggingError::CannotReEmitToSameTarget(target))
                }
            }
        }

        Ok(ret)
    }

    fn into_subscriber(self, format: Format) -> Result<impl Subscriber, InitializeLoggingError> {
        let mut layers = vec![];

        for x in self.targets {
            let layer = x.layer(&format)?;
            layers.push(layer);
        }
        let registry: Registry = Registry::default();
        Ok(registry.with(layers))
    }
}

/// Logging options to be used for initializing tracing for a binary
#[derive(Debug)]
pub struct LoggingOptions {
    pub level: LevelFilter,
    format: Option<Format>,
    targets: Vec<Target>,
}

impl LoggingOptions {
    /// Creates a new logging options
    pub const fn new() -> Self {
        Self {
            level: LevelFilter::OFF,
            format: None,
            targets: vec![],
        }
    }

    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn target<T>(mut self, target: T) -> Self
        where Target: From<T>
    {
        self.targets.push(Target::from(target));
        self
    }

    /// Gets the number of targets
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    pub fn into_subscriber(self) -> Result<impl Subscriber, InitializeLoggingError> {
        let targets = Targets::new(self.targets)?;
        let format = self.format.unwrap_or_else(|| fmt::format());

        let subscriber = targets.into_subscriber(format)?;
        Ok(subscriber)
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
    NoTargets
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_targets() {
        let _ = Targets::new([
            Stdout(LevelFilter::OFF).into()
        ]).expect("failed to create targets");

        let _ = Targets::new([
            Stdout(LevelFilter::OFF).into(),
            Stdout(LevelFilter::OFF).into(),
        ]).expect_err("can not repeat");
    }
}