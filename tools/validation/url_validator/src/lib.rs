use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{UrlValidatorInput as LogicInput, UrlValidatorResult as LogicOutput, UrlComponents as LogicComponents, ValidationChecks as LogicChecks};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlValidatorInput {
    /// URL to validate
    pub url: String,
    /// Whether to require HTTPS
    pub require_https: Option<bool>,
    /// Allowed schemes (if specified, only these are valid)
    pub allowed_schemes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlValidatorResult {
    /// Whether the URL is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// Parsed URL components
    pub components: Option<UrlComponents>,
    /// Validation checks performed
    pub checks: ValidationChecks,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlComponents {
    /// URL scheme (http, https, ftp, etc.)
    pub scheme: String,
    /// Host/domain
    pub host: Option<String>,
    /// Port number
    pub port: Option<u16>,
    /// Path component
    pub path: String,
    /// Query string
    pub query: Option<String>,
    /// Fragment/anchor
    pub fragment: Option<String>,
    /// Username (if present)
    pub username: Option<String>,
    /// Whether a password is present
    pub has_password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidationChecks {
    /// Valid URL syntax
    pub valid_syntax: bool,
    /// Has a scheme
    pub has_scheme: bool,
    /// Has a host
    pub has_host: bool,
    /// Scheme is allowed
    pub scheme_allowed: bool,
    /// Uses HTTPS (if required)
    pub https_required_met: bool,
    /// No credentials in URL
    pub no_credentials: bool,
    /// Valid port number
    pub valid_port: bool,
}

#[cfg_attr(not(test), tool)]
pub fn url_validator(input: UrlValidatorInput) -> Result<UrlValidatorResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        url: input.url,
        require_https: input.require_https,
        allowed_schemes: input.allowed_schemes,
    };
    
    // Call logic implementation
    let result = logic::validate_url(logic_input)?;
    
    // Convert back to wrapper types
    Ok(UrlValidatorResult {
        is_valid: result.is_valid,
        error: result.error,
        components: result.components.map(|c| UrlComponents {
            scheme: c.scheme,
            host: c.host,
            port: c.port,
            path: c.path,
            query: c.query,
            fragment: c.fragment,
            username: c.username,
            has_password: c.has_password,
        }),
        checks: ValidationChecks {
            valid_syntax: result.checks.valid_syntax,
            has_scheme: result.checks.has_scheme,
            has_host: result.checks.has_host,
            scheme_allowed: result.checks.scheme_allowed,
            https_required_met: result.checks.https_required_met,
            no_credentials: result.checks.no_credentials,
            valid_port: result.checks.valid_port,
        },
    })
}