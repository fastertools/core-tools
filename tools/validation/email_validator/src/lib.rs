use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

use ftl_sdk::ToolResponse;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{
    EmailParts as LogicParts, EmailValidatorInput as LogicInput,
    EmailValidatorResult as LogicOutput, ValidationChecks as LogicChecks,
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmailValidatorInput {
    /// Email address to validate
    pub email: String,
    /// Whether to check DNS records (not implemented in basic version)
    pub check_dns: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmailValidatorResult {
    /// Whether the email is valid
    pub is_valid: bool,
    /// Reason for invalidity (if applicable)
    pub error: Option<String>,
    /// Email parts breakdown
    pub parts: Option<EmailParts>,
    /// Validation checks performed
    pub checks: ValidationChecks,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmailParts {
    /// Local part (before @)
    pub local: String,
    /// Domain part (after @)
    pub domain: String,
    /// Top-level domain
    pub tld: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidationChecks {
    /// Has exactly one @ symbol
    pub has_single_at: bool,
    /// Local part is valid
    pub valid_local: bool,
    /// Domain part is valid
    pub valid_domain: bool,
    /// No consecutive dots
    pub no_consecutive_dots: bool,
    /// No leading/trailing dots
    pub no_edge_dots: bool,
    /// Valid characters only
    pub valid_characters: bool,
    /// Reasonable length
    pub reasonable_length: bool,
}

#[cfg_attr(not(test), tool)]
pub fn email_validator(input: EmailValidatorInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        email: input.email,
        check_dns: input.check_dns,
    };

    // Call logic implementation
    let result = match logic::validate_email(logic_input) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error validating email: {e}")),
    };

    // Convert back to wrapper types
    let email_result = EmailValidatorResult {
        is_valid: result.is_valid,
        error: result.error,
        parts: result.parts.map(|p| EmailParts {
            local: p.local,
            domain: p.domain,
            tld: p.tld,
        }),
        checks: ValidationChecks {
            has_single_at: result.checks.has_single_at,
            valid_local: result.checks.valid_local,
            valid_domain: result.checks.valid_domain,
            no_consecutive_dots: result.checks.no_consecutive_dots,
            no_edge_dots: result.checks.no_edge_dots,
            valid_characters: result.checks.valid_characters,
            reasonable_length: result.checks.reasonable_length,
        },
    };

    ToolResponse::text(
        serde_json::to_string(&email_result)
            .unwrap_or_else(|_| "Error serializing result".to_string()),
    )
}
