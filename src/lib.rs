//! Real-time Monitoring and Observability System
//! 
//! This module provides comprehensive monitoring capabilities for the infra ecosystem,
//! including performance metrics, health checking, and observability features.

pub mod metrics;
pub mod health;
pub mod observer;
pub mod error;

pub use error::MonitorError;

/// Main monitoring engine for the infra ecosystem
pub struct MonitoringEngine {
    metrics_collector: metrics::MetricsCollector,
    health_checker: health::HealthChecker,
    observer: observer::SystemObserver,
}

impl MonitoringEngine {
    /// Create a new monitoring engine with default configurations
    pub fn new() -> Result<Self, MonitorError> {
        Ok(Self {
            metrics_collector: metrics::MetricsCollector::new(),
            health_checker: health::HealthChecker::new(),
            observer: observer::SystemObserver::new(),
        })
    }

    /// Start monitoring all infra components
    pub async fn start_monitoring(&self) -> Result<(), MonitorError> {
        // Start metrics collection
        self.metrics_collector.start().await?;
        
        // Start health checking
        self.health_checker.start().await?;
        
        // Start system observation
        self.observer.start().await?;
        
        Ok(())
    }

    /// Get comprehensive system status
    pub async fn get_system_status(&self) -> Result<serde_json::Value, MonitorError> {
        let metrics = self.metrics_collector.get_metrics().await?;
        let health = self.health_checker.get_health_status().await?;
        let observations = self.observer.get_observations().await?;
        
        Ok(serde_json::json!({
            "metrics": metrics,
            "health": health,
            "observations": observations,
            "timestamp": hacetime::tick()
        }))
    }

    /// Check specific component health
    pub async fn check_component_health(
        &self,
        component: &str,
    ) -> Result<health::HealthStatus, MonitorError> {
        self.health_checker.check_component(component).await
    }

    /// Get performance metrics for a specific module
    pub async fn get_module_metrics(
        &self,
        module: &str,
    ) -> Result<metrics::ModuleMetrics, MonitorError> {
        self.metrics_collector.get_module_metrics(module).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_engine_creation() {
        let engine = MonitoringEngine::new().unwrap();
        assert!(true); // Engine created successfully
    }
}
