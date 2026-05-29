use crate::error::MonitorError;

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub component: String,
    pub healthy: bool,
}

#[derive(Debug, Default)]
pub struct HealthChecker;

impl HealthChecker {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<(), MonitorError> {
        Ok(())
    }

    pub async fn get_health_status(&self) -> Result<serde_json::Value, MonitorError> {
        Ok(serde_json::json!({
            "status": "ok",
            "components": []
        }))
    }

    pub async fn check_component(&self, component: &str) -> Result<HealthStatus, MonitorError> {
        Ok(HealthStatus {
            component: component.to_string(),
            healthy: true,
        })
    }
}
