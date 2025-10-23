//! # Blockchain Runtime
//!
//! Blockchain-agnostic runtime abstraction for dynamic analysis, testing, and simulation.
//!
//! ## Features
//!
//! - **Blockchain-Agnostic**: Works with any blockchain (Ethereum, Solana, etc.)
//! - **Dynamic Analysis**: Execute code in runtime environments
//! - **Testing**: Spin up test networks for contract testing
//! - **Simulation**: Simulate transactions and monitor state changes
//! - **Metrics Collection**: Track gas, compute units, state changes
//! - **Event Monitoring**: Capture events and logs
//! - **Async-First**: Non-blocking runtime operations
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use blockchain_runtime::{BlockchainRuntime, RuntimeConfig, NetworkMode};
//! use std::collections::HashMap;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Configure runtime
//! let config = RuntimeConfig {
//!     timeout_seconds: 300,
//!     memory_limit_mb: 1024,
//!     network_mode: NetworkMode::Local,
//!     enable_monitoring: true,
//!     blockchain_config: HashMap::new(),
//! };
//!
//! // Create runtime environment
//! // let runtime: Box<dyn BlockchainRuntime> = get_ethereum_runtime();
//! // let env = runtime.create_environment(config).await?;
//!
//! // Execute code
//! // let result = runtime.execute(env, code_path, inputs).await?;
//! // println!("Execution result: {:?}", result.success);
//! # Ok(())
//! # }
//! ```

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// Optional tracing
#[cfg(feature = "tracing")]
use tracing::info;

#[cfg(not(feature = "tracing"))]
macro_rules! info {
    ($($arg:tt)*) => {};
}

/// Main blockchain runtime trait
#[async_trait]
pub trait BlockchainRuntime: Send + Sync {
    /// Get the blockchain identifier
    fn blockchain_id(&self) -> &str;

    /// Create a runtime environment
    async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment>;

    /// Execute code in the runtime
    async fn execute(
        &self,
        env: &RuntimeEnvironment,
        code_path: &Path,
        inputs: &ExecutionInputs,
    ) -> Result<ExecutionResult>;

    /// Deploy a contract to the runtime
    async fn deploy_contract(
        &self,
        env: &RuntimeEnvironment,
        bytecode: &[u8],
        constructor_args: &[u8],
    ) -> Result<String>;

    /// Call a contract function
    async fn call_function(
        &self,
        env: &RuntimeEnvironment,
        contract_address: &str,
        function: &str,
        args: &[u8],
    ) -> Result<Vec<u8>>;

    /// Get runtime metrics
    fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition>;

    /// Monitor runtime events
    async fn monitor(
        &self,
        env: &RuntimeEnvironment,
        execution_id: &str,
    ) -> Result<Vec<RuntimeEvent>>;

    /// Destroy the environment
    async fn destroy(&self, env: RuntimeEnvironment) -> Result<()>;

    /// Check if runtime is available
    async fn is_available(&self) -> bool;

    /// Get runtime capabilities
    fn capabilities(&self) -> RuntimeCapabilities;
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub timeout_seconds: u64,
    pub memory_limit_mb: u64,
    pub network_mode: NetworkMode,
    pub enable_monitoring: bool,
    pub blockchain_config: HashMap<String, serde_json::Value>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 300,
            memory_limit_mb: 1024,
            network_mode: NetworkMode::Local,
            enable_monitoring: true,
            blockchain_config: HashMap::new(),
        }
    }
}

/// Network mode for runtime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkMode {
    Local,
    Testnet,
    MainnetFork,
}

/// Runtime environment instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEnvironment {
    pub environment_id: String,
    pub blockchain_id: String,
    pub runtime_type: RuntimeType,
    pub endpoint_url: String,
    pub state: EnvironmentState,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuntimeType {
    Docker,
    LocalProcess,
    CloudInstance,
    InMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentState {
    Creating,
    Ready,
    Running,
    Stopped,
    Error,
}

/// Execution inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionInputs {
    pub target_function: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub context: ExecutionContext,
}

/// Execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub sender: Option<String>,
    pub block_number: Option<u64>,
    pub timestamp: Option<u64>,
    pub extra: HashMap<String, serde_json::Value>,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub execution_id: String,
    pub success: bool,
    pub return_value: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub state_changes: Vec<StateChange>,
    pub events: Vec<RuntimeEvent>,
    pub execution_time_ms: u64,
}

/// State change during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub change_type: StateChangeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateChangeType {
    Created,
    Updated,
    Deleted,
}

/// Runtime event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: u64,
    pub data: HashMap<String, serde_json::Value>,
}

/// Runtime metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMetricDefinition {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricType {
    Gas,
    ComputeUnits,
    StorageBytes,
    Time,
    Custom(String),
}

/// Runtime capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeCapabilities {
    pub supports_contract_deployment: bool,
    pub supports_function_calls: bool,
    pub supports_state_inspection: bool,
    pub supports_event_monitoring: bool,
    pub supports_gas_estimation: bool,
    pub supports_time_travel: bool,
    pub max_execution_time_seconds: u64,
}

impl Default for RuntimeCapabilities {
    fn default() -> Self {
        Self {
            supports_contract_deployment: true,
            supports_function_calls: true,
            supports_state_inspection: true,
            supports_event_monitoring: true,
            supports_gas_estimation: false,
            supports_time_travel: false,
            max_execution_time_seconds: 300,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_config_default() {
        let config = RuntimeConfig::default();
        assert_eq!(config.timeout_seconds, 300);
        assert_eq!(config.memory_limit_mb, 1024);
        assert_eq!(config.network_mode, NetworkMode::Local);
        assert!(config.enable_monitoring);
    }

    #[test]
    fn test_runtime_environment_creation() {
        let env = RuntimeEnvironment {
            environment_id: "test-env".to_string(),
            blockchain_id: "ethereum".to_string(),
            runtime_type: RuntimeType::Docker,
            endpoint_url: "http://localhost:8545".to_string(),
            state: EnvironmentState::Ready,
            metadata: HashMap::new(),
        };

        assert_eq!(env.environment_id, "test-env");
        assert_eq!(env.state, EnvironmentState::Ready);
    }

    #[test]
    fn test_execution_result() {
        let result = ExecutionResult {
            execution_id: "exec-1".to_string(),
            success: true,
            return_value: Some(serde_json::json!({"value": 42})),
            error: None,
            metrics: HashMap::new(),
            state_changes: vec![],
            events: vec![],
            execution_time_ms: 150,
        };

        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.execution_time_ms, 150);
    }

    #[test]
    fn test_state_change() {
        let change = StateChange {
            key: "balance".to_string(),
            old_value: Some(serde_json::json!(100)),
            new_value: serde_json::json!(200),
            change_type: StateChangeType::Updated,
        };

        assert_eq!(change.change_type, StateChangeType::Updated);
    }

    #[test]
    fn test_runtime_capabilities() {
        let caps = RuntimeCapabilities::default();
        assert!(caps.supports_contract_deployment);
        assert!(caps.supports_function_calls);
        assert_eq!(caps.max_execution_time_seconds, 300);
    }

    #[test]
    fn test_network_modes() {
        assert_ne!(NetworkMode::Local, NetworkMode::Testnet);
        assert_ne!(NetworkMode::Testnet, NetworkMode::MainnetFork);
    }

    #[test]
    fn test_runtime_types() {
        assert_ne!(RuntimeType::Docker, RuntimeType::LocalProcess);
        assert_ne!(RuntimeType::LocalProcess, RuntimeType::InMemory);
    }

    #[test]
    fn test_environment_states() {
        assert_ne!(EnvironmentState::Creating, EnvironmentState::Ready);
        assert_ne!(EnvironmentState::Ready, EnvironmentState::Running);
    }

    #[test]
    fn test_metric_types() {
        assert_eq!(MetricType::Gas, MetricType::Gas);
        assert_ne!(MetricType::Gas, MetricType::ComputeUnits);
    }

    #[test]
    fn test_serialization() {
        let config = RuntimeConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: RuntimeConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.timeout_seconds, config.timeout_seconds);
        assert_eq!(deserialized.network_mode, config.network_mode);
    }
}

