use clap::clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug, ValueEnum)]
pub enum LogLevel {
    EMERGENCY,
    ALERT,
    CRITICAL,
    ERROR,
    WARNING,
    NOTIFICATION,
    INFO,
    DEBUG,
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EMERGENCY" => Ok(LogLevel::EMERGENCY),
            "ALERT" => Ok(LogLevel::ALERT),
            "CRITICAL" => Ok(LogLevel::CRITICAL),
            "ERROR" => Ok(LogLevel::ERROR),
            "WARNING" => Ok(LogLevel::WARNING),
            "NOTIFICATION" => Ok(LogLevel::NOTIFICATION),
            "INFO" => Ok(LogLevel::INFO),
            "DEBUG" => Ok(LogLevel::DEBUG),
            _ => Err(format!("{} is not a valid LogLevel", s)),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::EMERGENCY => write!(f, "EMERGENCY"),
            LogLevel::ALERT => write!(f, "ALERT"),
            LogLevel::CRITICAL => write!(f, "CRITICAL"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::NOTIFICATION => write!(f, "NOTIFICATION"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
        }
    }
}
