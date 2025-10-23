//! Security configuration and validation for blockchain runtime

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::*;
use crate::types::{SecurityViolation, SecurityViolationType, SecuritySeverity};

/// Security configuration for runtime execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable execution sandboxing
    pub sandbox_enabled: bool,
    /// Enable reentrancy protection
    pub reentrancy_protection: bool,
    /// Enable integer overflow detection
    pub overflow_detection: bool,
    /// Enable access control verification
    pub access_control_verification: bool,
    /// Maximum call depth to prevent stack overflow
    pub max_call_depth: u32,
    /// Maximum number of external calls per transaction
    pub max_external_calls: u32,
    /// Enable gas limit enforcement
    pub gas_limit_enforcement: bool,
    /// Maximum gas limit
    pub max_gas_limit: u64,
    /// Enable memory limit enforcement
    pub memory_limit_enforcement: bool,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sandbox_enabled: true,
            reentrancy_protection: true,
            overflow_detection: true,
            access_control_verification: true,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            max_external_calls: DEFAULT_MAX_EXTERNAL_CALLS,
            gas_limit_enforcement: true,
            max_gas_limit: DEFAULT_MAX_GAS_LIMIT,
            memory_limit_enforcement: true,
            max_memory_bytes: DEFAULT_MAX_MEMORY_BYTES,
        }
    }
}

impl SecurityConfig {
    /// Create a new security configuration with custom values
    pub fn new(
        sandbox_enabled: bool,
        reentrancy_protection: bool,
        overflow_detection: bool,
        access_control_verification: bool,
    ) -> Self {
        Self {
            sandbox_enabled,
            reentrancy_protection,
            overflow_detection,
            access_control_verification,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            max_external_calls: DEFAULT_MAX_EXTERNAL_CALLS,
            gas_limit_enforcement: true,
            max_gas_limit: DEFAULT_MAX_GAS_LIMIT,
            memory_limit_enforcement: true,
            max_memory_bytes: DEFAULT_MAX_MEMORY_BYTES,
        }
    }

    /// Create a permissive security configuration
    pub fn permissive() -> Self {
        Self {
            sandbox_enabled: false,
            reentrancy_protection: false,
            overflow_detection: false,
            access_control_verification: false,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            max_external_calls: DEFAULT_MAX_EXTERNAL_CALLS,
            gas_limit_enforcement: false,
            max_gas_limit: u64::MAX,
            memory_limit_enforcement: false,
            max_memory_bytes: u64::MAX,
        }
    }

    /// Create a strict security configuration
    pub fn strict() -> Self {
        Self {
            sandbox_enabled: true,
            reentrancy_protection: true,
            overflow_detection: true,
            access_control_verification: true,
            max_call_depth: 100, // More restrictive
            max_external_calls: 10, // More restrictive
            gas_limit_enforcement: true,
            max_gas_limit: 1_000_000, // More restrictive
            memory_limit_enforcement: true,
            max_memory_bytes: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Security validator for runtime operations
pub struct SecurityValidator {
    config: SecurityConfig,
}

impl SecurityValidator {
    /// Create a new security validator
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }

    /// Validate call depth against security limits
    pub fn validate_call_depth(&self, call_depth: u32) -> Result<(), SecurityViolation> {
        if call_depth > self.config.max_call_depth {
            Err(self.create_violation(
                SecurityViolationType::CallDepthExceeded,
                format!("Call depth {} exceeds maximum {}", call_depth, self.config.max_call_depth),
                SecuritySeverity::High,
            ))
        } else {
            Ok(())
        }
    }

    /// Validate external call count against security limits
    pub fn validate_external_calls(&self, call_count: u32) -> Result<(), SecurityViolation> {
        if call_count > self.config.max_external_calls {
            Err(self.create_violation(
                SecurityViolationType::ExternalCallLimitExceeded,
                format!("External call count {} exceeds maximum {}", call_count, self.config.max_external_calls),
                SecuritySeverity::High,
            ))
        } else {
            Ok(())
        }
    }

    /// Validate gas usage against security limits
    pub fn validate_gas_usage(&self, gas_used: u64) -> Result<(), SecurityViolation> {
        if self.config.gas_limit_enforcement && gas_used > self.config.max_gas_limit {
            Err(self.create_violation(
                SecurityViolationType::GasLimitExceeded,
                format!("Gas usage {} exceeds maximum {}", gas_used, self.config.max_gas_limit),
                SecuritySeverity::High,
            ))
        } else {
            Ok(())
        }
    }

    /// Validate memory usage against security limits
    pub fn validate_memory_usage(&self, memory_used: u64) -> Result<(), SecurityViolation> {
        if self.config.memory_limit_enforcement && memory_used > self.config.max_memory_bytes {
            Err(self.create_violation(
                SecurityViolationType::MemoryLimitExceeded,
                format!("Memory usage {} exceeds maximum {}", memory_used, self.config.max_memory_bytes),
                SecuritySeverity::High,
            ))
        } else {
            Ok(())
        }
    }

    /// Check for reentrancy attacks
    pub fn check_reentrancy(&self, function_name: &str, caller: &str, call_stack: &[String]) -> Result<bool, SecurityViolation> {
        if !self.config.reentrancy_protection {
            return Ok(false);
        }

        // Check if the same function is being called recursively
        let recursive_calls = call_stack.iter().filter(|&f| f == function_name).count();
        if recursive_calls > 1 {
            Err(self.create_violation(
                SecurityViolationType::ReentrancyAttack,
                format!("Potential reentrancy attack in function {} called by {}", function_name, caller),
                SecuritySeverity::Critical,
            ))
        } else {
            Ok(false)
        }
    }

    /// Detect integer overflow
    pub fn detect_overflow(&self, operation: &str, operands: &[i64]) -> Result<bool, SecurityViolation> {
        if !self.config.overflow_detection {
            return Ok(false);
        }

        // Simple overflow detection for basic operations
        match operation {
            "add" | "+" => {
                if operands.len() >= 2 {
                    let result = operands[0].checked_add(operands[1]);
                    if result.is_none() {
                        return Err(self.create_violation(
                            SecurityViolationType::IntegerOverflow,
                            format!("Integer overflow detected in addition: {} + {}", operands[0], operands[1]),
                            SecuritySeverity::High,
                        ));
                    }
                }
            }
            "multiply" | "*" => {
                if operands.len() >= 2 {
                    let result = operands[0].checked_mul(operands[1]);
                    if result.is_none() {
                        return Err(self.create_violation(
                            SecurityViolationType::IntegerOverflow,
                            format!("Integer overflow detected in multiplication: {} * {}", operands[0], operands[1]),
                            SecuritySeverity::High,
                        ));
                    }
                }
            }
            _ => {}
        }

        Ok(false)
    }

    /// Verify access control
    pub fn verify_access_control(&self, function_name: &str, caller: &str, required_role: Option<&str>) -> Result<bool, SecurityViolation> {
        if !self.config.access_control_verification {
            return Ok(true);
        }

        // Simple access control check - in a real implementation, this would be more sophisticated
        if let Some(role) = required_role {
            if role == "admin" && !caller.ends_with("admin") {
                return Err(self.create_violation(
                    SecurityViolationType::AccessControlViolation,
                    format!("Access denied: {} does not have {} role for function {}", caller, role, function_name),
                    SecuritySeverity::Medium,
                ));
            }
        }

        Ok(true)
    }

    /// Create a security violation
    fn create_violation(&self, violation_type: SecurityViolationType, description: String, severity: SecuritySeverity) -> SecurityViolation {
        SecurityViolation {
            violation_type,
            description,
            severity,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            context: std::collections::HashMap::new(),
        }
    }
}

/// Security context manager for tracking security state
pub struct SecurityContext {
    validator: SecurityValidator,
    violations: Vec<SecurityViolation>,
}

impl SecurityContext {
    /// Create a new security context
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            validator: SecurityValidator::new(config),
            violations: Vec::new(),
        }
    }

    /// Get the security validator
    pub fn validator(&self) -> &SecurityValidator {
        &self.validator
    }

    /// Add a security violation
    pub fn add_violation(&mut self, violation: SecurityViolation) {
        self.violations.push(violation);
    }

    /// Get all security violations
    pub fn violations(&self) -> &[SecurityViolation] {
        &self.violations
    }

    /// Clear all violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }

    /// Check if there are any critical violations
    pub fn has_critical_violations(&self) -> bool {
        self.violations.iter().any(|v| matches!(v.severity, SecuritySeverity::Critical))
    }

    /// Get violation count by severity
    pub fn violation_count_by_severity(&self) -> (usize, usize, usize, usize) {
        let mut critical = 0;
        let mut high = 0;
        let mut medium = 0;
        let mut low = 0;

        for violation in &self.violations {
            match violation.severity {
                SecuritySeverity::Critical => critical += 1,
                SecuritySeverity::High => high += 1,
                SecuritySeverity::Medium => medium += 1,
                SecuritySeverity::Low => low += 1,
            }
        }

        (critical, high, medium, low)
    }
}
