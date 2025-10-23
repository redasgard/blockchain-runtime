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
//! - **Security**: Built-in security validation and monitoring
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use blockchain_runtime::{BlockchainRuntime, RuntimeConfig, NetworkMode, DefaultBlockchainRuntime};
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Configure runtime
//! let config = RuntimeConfig::default();
//!
//! // Create runtime
//! let runtime = DefaultBlockchainRuntime::new("ethereum".to_string());
//! let env = runtime.create_environment(config).await?;
//!
//! // Execute code
//! // let result = runtime.execute(&env, code_path, inputs).await?;
//! // println!("Execution result: {:?}", result.success);
//! # Ok(())
//! # }
//! ```

// Re-export main types and traits
pub use config::*;
pub use runtime::*;
pub use security::*;
pub use types::*;

// Module declarations
mod config;
mod constants;
mod runtime;
mod security;
mod types;

// Optional tracing
#[cfg(feature = "tracing")]
use tracing::info;

#[cfg(not(feature = "tracing"))]
macro_rules! info {
    ($($arg:tt)*) => {};
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

    #[test]
    fn test_security_config_default() {
        let security_config = SecurityConfig::default();
        assert!(security_config.sandbox_enabled);
        assert!(security_config.reentrancy_protection);
        assert!(security_config.overflow_detection);
        assert!(security_config.access_control_verification);
        assert_eq!(security_config.max_call_depth, 1024);
        assert_eq!(security_config.max_external_calls, 100);
        assert!(security_config.gas_limit_enforcement);
        assert_eq!(security_config.max_gas_limit, 10_000_000);
        assert!(security_config.memory_limit_enforcement);
        assert_eq!(security_config.max_memory_bytes, 100 * 1024 * 1024);
    }

    #[test]
    fn test_security_violation_creation() {
        let violation = SecurityViolation {
            violation_type: SecurityViolationType::ReentrancyAttack,
            description: "Reentrancy attack detected".to_string(),
            severity: SecuritySeverity::Critical,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            context: HashMap::new(),
        };

        assert_eq!(violation.violation_type, SecurityViolationType::ReentrancyAttack);
        assert_eq!(violation.severity, SecuritySeverity::Critical);
    }

    #[test]
    fn test_secure_execution_context() {
        let mut context = SecureExecutionContext::default();
        context.call_depth = 5;
        context.external_call_count = 10;
        context.gas_used = 1000;
        context.memory_used = 1024;
        context.call_stack.push("function1".to_string());
        context.call_stack.push("function2".to_string());

        assert_eq!(context.call_depth, 5);
        assert_eq!(context.external_call_count, 10);
        assert_eq!(context.gas_used, 1000);
        assert_eq!(context.memory_used, 1024);
        assert_eq!(context.call_stack.len(), 2);
    }

    #[test]
    fn test_access_control_check() {
        let check = AccessControlCheck {
            function_name: "withdraw".to_string(),
            caller: "0x123".to_string(),
            required_role: Some("owner".to_string()),
            has_permission: true,
            check_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        assert_eq!(check.function_name, "withdraw");
        assert_eq!(check.caller, "0x123");
        assert_eq!(check.required_role, Some("owner".to_string()));
        assert!(check.has_permission);
    }

    #[test]
    fn test_execution_result_with_security() {
        let security_context = SecureExecutionContext::default();
        let security_violations = vec![SecurityViolation {
            violation_type: SecurityViolationType::IntegerOverflow,
            description: "Integer overflow detected".to_string(),
            severity: SecuritySeverity::High,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            context: HashMap::new(),
        }];

        let result = ExecutionResult {
            execution_id: "exec-1".to_string(),
            success: false,
            return_value: None,
            error: Some("Security violation detected".to_string()),
            metrics: HashMap::new(),
            state_changes: vec![],
            events: vec![],
            execution_time_ms: 150,
            security_context,
            security_violations,
        };

        assert!(!result.success);
        assert!(result.error.is_some());
        assert_eq!(result.security_violations.len(), 1);
        assert_eq!(result.security_violations[0].violation_type, SecurityViolationType::IntegerOverflow);
    }

    #[test]
    fn test_runtime_config_with_security() {
        let config = RuntimeConfig::default();
        assert!(config.security_config.sandbox_enabled);
        assert!(config.security_config.reentrancy_protection);
        assert!(config.security_config.overflow_detection);
        assert!(config.security_config.access_control_verification);
    }

    #[test]
    fn test_security_violation_types() {
        assert_ne!(SecurityViolationType::ReentrancyAttack, SecurityViolationType::IntegerOverflow);
        assert_ne!(SecurityViolationType::AccessControlViolation, SecurityViolationType::ResourceLimitExceeded);
        assert_eq!(SecurityViolationType::GasLimitExceeded, SecurityViolationType::GasLimitExceeded);
    }

    #[test]
    fn test_security_severity_levels() {
        assert_ne!(SecuritySeverity::Low, SecuritySeverity::Medium);
        assert_ne!(SecuritySeverity::Medium, SecuritySeverity::High);
        assert_ne!(SecuritySeverity::High, SecuritySeverity::Critical);
        assert_eq!(SecuritySeverity::Critical, SecuritySeverity::Critical);
    }

    #[test]
    fn test_security_serialization() {
        let security_config = SecurityConfig::default();
        let json = serde_json::to_string(&security_config).unwrap();
        let deserialized: SecurityConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.sandbox_enabled, security_config.sandbox_enabled);
        assert_eq!(deserialized.reentrancy_protection, security_config.reentrancy_protection);
        assert_eq!(deserialized.overflow_detection, security_config.overflow_detection);
    }
}

