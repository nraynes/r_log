use clap::clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl LogLevel {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "EMERGENCY" => Some(LogLevel::EMERGENCY),
            "ALERT" => Some(LogLevel::ALERT),
            "CRITICAL" => Some(LogLevel::CRITICAL),
            "ERROR" => Some(LogLevel::ERROR),
            "WARNING" => Some(LogLevel::WARNING),
            "NOTIFICATION" => Some(LogLevel::NOTIFICATION),
            "INFO" => Some(LogLevel::INFO),
            "DEBUG" => Some(LogLevel::DEBUG),
            _ => None,
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
