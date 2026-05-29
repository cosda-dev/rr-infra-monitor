#[derive(Debug, Clone)]
pub enum MonitorError {
    Metrics(String),
    Health(String),
    Observer(String),
}

impl core::fmt::Display for MonitorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Metrics(msg) => write!(f, "metrics error: {msg}"),
            Self::Health(msg) => write!(f, "health error: {msg}"),
            Self::Observer(msg) => write!(f, "observer error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MonitorError {}
