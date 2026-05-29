use crate::error::MonitorError;

#[derive(Debug, Clone, Default)]
pub struct ModuleMetrics {
    pub name: String,
    pub requests: u64,
    pub failures: u64,
    pub latency_ms: u64,
}

#[derive(Debug, Default)]
pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<(), MonitorError> {
        Ok(())
    }

    pub async fn get_metrics(&self) -> Result<serde_json::Value, MonitorError> {
        Ok(serde_json::json!({
            "status": "ok",
            "modules": []
        }))
    }

    pub async fn get_module_metrics(&self, module: &str) -> Result<ModuleMetrics, MonitorError> {
        Ok(ModuleMetrics {
            name: module.to_string(),
            ..ModuleMetrics::default()
        })
    }
}
