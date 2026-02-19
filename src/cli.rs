use std::fmt;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about = "Flock device agent")]
pub struct Cli {
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    #[arg(long, default_value = "/etc/flockd/config.toml")]
    pub config: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let cli = Cli::parse_from(["flockd"]);
        assert_eq!(cli.log_level, LogLevel::Info);
        assert_eq!(cli.config, PathBuf::from("/etc/flockd/config.toml"));
    }

    #[test]
    fn custom_log_level() {
        let cli = Cli::parse_from(["flockd", "--log-level", "debug"]);
        assert_eq!(cli.log_level, LogLevel::Debug);
    }

    #[test]
    fn custom_log_level_warn() {
        let cli = Cli::parse_from(["flockd", "--log-level", "warn"]);
        assert_eq!(cli.log_level, LogLevel::Warn);
    }

    #[test]
    fn custom_log_level_error() {
        let cli = Cli::parse_from(["flockd", "--log-level", "error"]);
        assert_eq!(cli.log_level, LogLevel::Error);
    }

    #[test]
    fn custom_config_path() {
        let cli = Cli::parse_from(["flockd", "--config", "/tmp/test.toml"]);
        assert_eq!(cli.config, PathBuf::from("/tmp/test.toml"));
    }

    #[test]
    fn all_flags() {
        let cli = Cli::parse_from([
            "flockd",
            "--log-level",
            "error",
            "--config",
            "/opt/flockd.toml",
        ]);
        assert_eq!(cli.log_level, LogLevel::Error);
        assert_eq!(cli.config, PathBuf::from("/opt/flockd.toml"));
    }

    #[test]
    fn invalid_log_level_errors() {
        let result = Cli::try_parse_from(["flockd", "--log-level", "trace"]);
        assert!(result.is_err());
    }

    #[test]
    fn version_flag() {
        let result = Cli::try_parse_from(["flockd", "--version"]);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    }

    #[test]
    fn log_level_display() {
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Error.to_string(), "error");
    }
}
