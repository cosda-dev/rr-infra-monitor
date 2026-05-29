use crate::error::MonitorError;

#[derive(Debug, Default)]
pub struct SystemObserver;

impl SystemObserver {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<(), MonitorError> {
        Ok(())
    }

    pub async fn get_observations(&self) -> Result<serde_json::Value, MonitorError> {
        Ok(serde_json::json!({
            "events": []
        }))
    }
}
