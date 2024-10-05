use clap::Parser;
use std::path::PathBuf;

/// Command line options
#[derive(Debug, Parser)]
pub struct App {
    /// Change to the given working directory
    #[clap(long)]
    pub cwd: Option<PathBuf>,
    /// The host to run on
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    /// The port to listen on
    #[clap(long, default_value_t = 3000)]
    pub port: u16,
    /// Enables debug mode, setting log level to trace
    #[clap(long)]
    pub debug: bool,
}
