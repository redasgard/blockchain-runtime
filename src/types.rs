//! Core types for blockchain runtime

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::*;

/// Network mode for runtime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkMode {
    Local,
    Testnet,
    MainnetFork,
}

/// Runtime type for environment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuntimeType {
    Docker,
    LocalProcess,
    CloudInstance,
    InMemory,
}

/// Environment state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentState {
    Creating,
    Ready,
    Running,
    Stopped,
    Error,
}

/// Metric type for runtime monitoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricType {
    Gas,
    ComputeUnits,
    StorageBytes,
    Time,
    Custom(String),
}

/// State change type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateChangeType {
    Created,
    Updated,
    Deleted,
}

/// Security violation type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityViolationType {
    ReentrancyAttack,
    IntegerOverflow,
    AccessControlViolation,
    ResourceLimitExceeded,
    SandboxViolation,
    CallDepthExceeded,
    ExternalCallLimitExceeded,
    GasLimitExceeded,
    MemoryLimitExceeded,
}

/// Security severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Runtime metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMetricDefinition {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub metric_type: MetricType,
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
            max_execution_time_seconds: DEFAULT_MAX_EXECUTION_TIME_SECONDS,
        }
    }
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

/// State change during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub change_type: StateChangeType,
}

/// Runtime event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: u64,
    pub data: HashMap<String, serde_json::Value>,
}

/// Access control check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlCheck {
    pub function_name: String,
    pub caller: String,
    pub required_role: Option<String>,
    pub has_permission: bool,
    pub check_timestamp: u64,
}

/// Security violation detected during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityViolation {
    pub violation_type: SecurityViolationType,
    pub description: String,
    pub severity: SecuritySeverity,
    pub timestamp: u64,
    pub context: HashMap<String, serde_json::Value>,
}

/// Execution context with security tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureExecutionContext {
    pub call_depth: u32,
    pub external_call_count: u32,
    pub gas_used: u64,
    pub memory_used: u64,
    pub call_stack: Vec<String>,
    pub access_control_checks: Vec<AccessControlCheck>,
    pub security_violations: Vec<SecurityViolation>,
}

impl Default for SecureExecutionContext {
    fn default() -> Self {
        Self {
            call_depth: 0,
            external_call_count: 0,
            gas_used: 0,
            memory_used: 0,
            call_stack: Vec::new(),
            access_control_checks: Vec::new(),
            security_violations: Vec::new(),
        }
    }
}

/// Execution result with security information
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
    /// Security context
    pub security_context: SecureExecutionContext,
    /// Security violations detected
    pub security_violations: Vec<SecurityViolation>,
}

impl ExecutionResult {
    /// Create a new execution result
    pub fn new(execution_id: String, success: bool) -> Self {
        Self {
            execution_id,
            success,
            return_value: None,
            error: None,
            metrics: HashMap::new(),
            state_changes: Vec::new(),
            events: Vec::new(),
            execution_time_ms: 0,
            security_context: SecureExecutionContext::default(),
            security_violations: Vec::new(),
        }
    }

    /// Add a security violation to the result
    pub fn add_security_violation(&mut self, violation: SecurityViolation) {
        self.security_violations.push(violation);
    }

    /// Check if execution has security violations
    pub fn has_security_violations(&self) -> bool {
        !self.security_violations.is_empty()
    }

    /// Get the highest severity level of violations
    pub fn get_highest_severity(&self) -> Option<SecuritySeverity> {
        self.security_violations
            .iter()
            .map(|v| &v.severity)
            .max_by(|a, b| {
                let severity_order = [SecuritySeverity::Low, SecuritySeverity::Medium, SecuritySeverity::High, SecuritySeverity::Critical];
                let a_order = severity_order.iter().position(|x| x == *a).unwrap_or(0);
                let b_order = severity_order.iter().position(|x| x == *b).unwrap_or(0);
                a_order.cmp(&b_order)
            })
            .cloned()
    }
}
