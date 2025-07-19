use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlValidatorInput {
    /// URL to validate
    pub url: String,
    /// Whether to require HTTPS
    pub require_https: Option<bool>,
    /// Allowed schemes (if specified, only these are valid)
    pub allowed_schemes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn validate_url(input: UrlValidatorInput) -> Result<UrlValidatorResult, String> {
    let url_str = input.url.trim();

    // Initialize checks
    let mut checks = ValidationChecks {
        valid_syntax: false,
        has_scheme: false,
        has_host: false,
        scheme_allowed: true,
        https_required_met: true,
        no_credentials: true,
        valid_port: true,
    };

    // Try to parse the URL
    let parsed_url = match Url::parse(url_str) {
        Ok(url) => url,
        Err(e) => {
            return Ok(UrlValidatorResult {
                is_valid: false,
                error: Some(format!("Invalid URL syntax: {}", e)),
                components: None,
                checks,
            });
        }
    };

    checks.valid_syntax = true;

    // Check scheme
    let scheme = parsed_url.scheme();
    checks.has_scheme = !scheme.is_empty();

    // Check allowed schemes
    if let Some(allowed) = &input.allowed_schemes {
        checks.scheme_allowed = allowed.iter().any(|s| s.eq_ignore_ascii_case(scheme));
        if !checks.scheme_allowed {
            return Ok(UrlValidatorResult {
                is_valid: false,
                error: Some(format!("Scheme '{}' is not allowed", scheme)),
                components: None,
                checks,
            });
        }
    }

    // Check HTTPS requirement
    if input.require_https.unwrap_or(false) && scheme != "https" {
        checks.https_required_met = false;
        return Ok(UrlValidatorResult {
            is_valid: false,
            error: Some("HTTPS is required but URL uses different scheme".to_string()),
            components: None,
            checks,
        });
    }

    // Check host
    let host = parsed_url.host_str().map(|s| s.to_string());
    checks.has_host = host.is_some();

    // For most schemes, host is required
    if (scheme == "http" || scheme == "https" || scheme == "ftp") && host.is_none() {
        return Ok(UrlValidatorResult {
            is_valid: false,
            error: Some("URL must have a host/domain".to_string()),
            components: None,
            checks,
        });
    }

    // Check for credentials
    let has_credentials = !parsed_url.username().is_empty() || parsed_url.password().is_some();
    checks.no_credentials = !has_credentials;

    // Check port validity
    if let Some(port) = parsed_url.port() {
        checks.valid_port = port > 0;
    }

    // Build components
    let components = UrlComponents {
        scheme: scheme.to_string(),
        host,
        port: parsed_url.port(),
        path: parsed_url.path().to_string(),
        query: parsed_url.query().map(|s| s.to_string()),
        fragment: parsed_url.fragment().map(|s| s.to_string()),
        username: if parsed_url.username().is_empty() {
            None
        } else {
            Some(parsed_url.username().to_string())
        },
        has_password: parsed_url.password().is_some(),
    };

    Ok(UrlValidatorResult {
        is_valid: true,
        error: None,
        components: Some(components),
        checks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        let valid_urls = vec![
            "https://www.example.com",
            "http://example.com/path/to/page",
            "https://example.com:8080",
            "https://example.com/search?q=rust&lang=en",
            "https://example.com/page#section",
            "ftp://files.example.com",
            "file:///home/user/document.txt",
            "mailto:user@example.com",
        ];

        for url in valid_urls {
            let input = UrlValidatorInput {
                url: url.to_string(),
                require_https: None,
                allowed_schemes: None,
            };
            let result = validate_url(input).unwrap();
            assert!(result.is_valid, "URL '{}' should be valid", url);
            assert!(result.components.is_some());
        }
    }

    #[test]
    fn test_invalid_urls() {
        let test_cases = vec![
            ("not a url", "Invalid URL syntax"),
            ("http://", "empty host"),
            ("https://", "empty host"),
            ("//example.com", "Invalid URL syntax"),
        ];

        for (url, expected_error) in test_cases {
            let input = UrlValidatorInput {
                url: url.to_string(),
                require_https: None,
                allowed_schemes: None,
            };
            let result = validate_url(input).unwrap();
            assert!(!result.is_valid, "URL '{}' should be invalid", url);
            assert!(result.error.is_some());
            assert!(
                result.error.unwrap().contains(expected_error),
                "URL '{}' should have error containing '{}'",
                url,
                expected_error
            );
        }
    }

    #[test]
    fn test_https_requirement() {
        let input = UrlValidatorInput {
            url: "http://example.com".to_string(),
            require_https: Some(true),
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(!result.is_valid);
        assert!(result.error.unwrap().contains("HTTPS is required"));
        assert!(!result.checks.https_required_met);
    }

    #[test]
    fn test_allowed_schemes() {
        let input = UrlValidatorInput {
            url: "ftp://files.example.com".to_string(),
            require_https: None,
            allowed_schemes: Some(vec!["http".to_string(), "https".to_string()]),
        };
        let result = validate_url(input).unwrap();
        assert!(!result.is_valid);
        assert!(
            result
                .error
                .unwrap()
                .contains("Scheme 'ftp' is not allowed")
        );
        assert!(!result.checks.scheme_allowed);
    }

    #[test]
    fn test_url_components() {
        let input = UrlValidatorInput {
            url: "https://user@example.com:8080/path?query=value#fragment".to_string(),
            require_https: None,
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);

        let components = result.components.unwrap();
        assert_eq!(components.scheme, "https");
        assert_eq!(components.host, Some("example.com".to_string()));
        assert_eq!(components.port, Some(8080));
        assert_eq!(components.path, "/path");
        assert_eq!(components.query, Some("query=value".to_string()));
        assert_eq!(components.fragment, Some("fragment".to_string()));
        assert_eq!(components.username, Some("user".to_string()));
        assert!(!components.has_password);
    }

    #[test]
    fn test_password_detection() {
        let input = UrlValidatorInput {
            url: "https://user:pass@example.com".to_string(),
            require_https: None,
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);

        let components = result.components.unwrap();
        assert_eq!(components.username, Some("user".to_string()));
        assert!(components.has_password);
        assert!(!result.checks.no_credentials);
    }

    #[test]
    fn test_localhost_url() {
        let input = UrlValidatorInput {
            url: "http://localhost:3000/api".to_string(),
            require_https: None,
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(
            result.components.unwrap().host,
            Some("localhost".to_string())
        );
    }

    #[test]
    fn test_ip_address_url() {
        let input = UrlValidatorInput {
            url: "http://192.168.1.1:8080".to_string(),
            require_https: None,
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(
            result.components.unwrap().host,
            Some("192.168.1.1".to_string())
        );
    }

    #[test]
    fn test_data_url() {
        let input = UrlValidatorInput {
            url: "data:text/plain;base64,SGVsbG8gV29ybGQ=".to_string(),
            require_https: None,
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.components.unwrap().scheme, "data");
    }

    #[test]
    fn test_validation_checks() {
        let input = UrlValidatorInput {
            url: "https://example.com".to_string(),
            require_https: Some(true),
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();

        assert!(result.checks.valid_syntax);
        assert!(result.checks.has_scheme);
        assert!(result.checks.has_host);
        assert!(result.checks.scheme_allowed);
        assert!(result.checks.https_required_met);
        assert!(result.checks.no_credentials);
        assert!(result.checks.valid_port);
    }

    #[test]
    fn test_trimmed_url() {
        let input = UrlValidatorInput {
            url: "  https://example.com  ".to_string(),
            require_https: None,
            allowed_schemes: None,
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_case_insensitive_scheme() {
        let input = UrlValidatorInput {
            url: "HTTPS://example.com".to_string(),
            require_https: Some(true),
            allowed_schemes: Some(vec!["https".to_string()]),
        };
        let result = validate_url(input).unwrap();
        assert!(result.is_valid);
    }
}
