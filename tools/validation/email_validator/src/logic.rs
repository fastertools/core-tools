use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailValidatorInput {
    /// Email address to validate
    pub email: String,
    /// Whether to check DNS records (not implemented in basic version)
    pub check_dns: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailParts {
    /// Local part (before @)
    pub local: String,
    /// Domain part (after @)
    pub domain: String,
    /// Top-level domain
    pub tld: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn validate_email(input: EmailValidatorInput) -> Result<EmailValidatorResult, String> {
    let email = input.email.trim();

    // Initialize checks
    let mut checks = ValidationChecks {
        has_single_at: false,
        valid_local: false,
        valid_domain: false,
        no_consecutive_dots: false,
        no_edge_dots: false,
        valid_characters: false,
        reasonable_length: false,
    };

    // Check length
    checks.reasonable_length = email.len() >= 3 && email.len() <= 320;
    if !checks.reasonable_length {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some("Email length must be between 3 and 320 characters".to_string()),
            parts: None,
            checks,
        });
    }

    // Check for exactly one @ symbol
    let at_count = email.matches('@').count();
    checks.has_single_at = at_count == 1;
    if !checks.has_single_at {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some(if at_count == 0 {
                "Email must contain @ symbol".to_string()
            } else {
                "Email must contain exactly one @ symbol".to_string()
            }),
            parts: None,
            checks,
        });
    }

    // Split into local and domain parts
    let parts: Vec<&str> = email.split('@').collect();
    let local = parts[0];
    let domain = parts[1];

    // Check for consecutive dots
    checks.no_consecutive_dots = !email.contains("..");
    if !checks.no_consecutive_dots {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some("Email cannot contain consecutive dots".to_string()),
            parts: None,
            checks,
        });
    }

    // Check for leading/trailing dots
    checks.no_edge_dots = !local.starts_with('.')
        && !local.ends_with('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.');
    if !checks.no_edge_dots {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some("Email parts cannot start or end with dots".to_string()),
            parts: None,
            checks,
        });
    }

    // Validate local part
    checks.valid_local = validate_local_part(local);
    if !checks.valid_local {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some("Invalid local part (before @)".to_string()),
            parts: None,
            checks,
        });
    }

    // Validate domain part
    checks.valid_domain = validate_domain_part(domain);
    if !checks.valid_domain {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some("Invalid domain part (after @)".to_string()),
            parts: None,
            checks,
        });
    }

    // Check valid characters
    checks.valid_characters = validate_characters(email);
    if !checks.valid_characters {
        return Ok(EmailValidatorResult {
            is_valid: false,
            error: Some("Email contains invalid characters".to_string()),
            parts: None,
            checks,
        });
    }

    // Extract TLD
    let tld = domain.rfind('.').map(|pos| domain[pos + 1..].to_string());

    Ok(EmailValidatorResult {
        is_valid: true,
        error: None,
        parts: Some(EmailParts {
            local: local.to_string(),
            domain: domain.to_string(),
            tld,
        }),
        checks,
    })
}

fn validate_local_part(local: &str) -> bool {
    // Check length
    if local.is_empty() || local.len() > 64 {
        return false;
    }

    // Check for valid characters
    for ch in local.chars() {
        if !ch.is_alphanumeric() && !"-._+".contains(ch) {
            return false;
        }
    }

    true
}

fn validate_domain_part(domain: &str) -> bool {
    // Check length
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    // Must contain at least one dot
    if !domain.contains('.') {
        return false;
    }

    // Split into labels
    let labels: Vec<&str> = domain.split('.').collect();

    // Check each label
    for label in &labels {
        if label.is_empty() || label.len() > 63 {
            return false;
        }

        // Label cannot start or end with hyphen
        if label.starts_with('-') || label.ends_with('-') {
            return false;
        }

        // Check for valid characters
        for ch in label.chars() {
            if !ch.is_alphanumeric() && ch != '-' {
                return false;
            }
        }
    }

    // TLD should be at least 2 characters
    if let Some(tld) = labels.last() {
        if tld.len() < 2 {
            return false;
        }
        // TLD should not be all numeric
        if tld.chars().all(|c| c.is_numeric()) {
            return false;
        }
    }

    true
}

fn validate_characters(email: &str) -> bool {
    for ch in email.chars() {
        if !ch.is_alphanumeric() && !"@.-_+".contains(ch) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        let valid_emails = vec![
            "test@example.com",
            "user.name@example.com",
            "user+tag@example.co.uk",
            "test_user@subdomain.example.com",
            "123@example.com",
            "a@example.com",
        ];

        for email in valid_emails {
            let input = EmailValidatorInput {
                email: email.to_string(),
                check_dns: None,
            };
            let result = validate_email(input).unwrap();
            assert!(result.is_valid, "Email '{email}' should be valid");
        }
    }

    #[test]
    fn test_invalid_emails() {
        let test_cases = vec![
            ("", "Email length must be between 3 and 320 characters"),
            ("test", "Email must contain @ symbol"),
            (
                "test@@example.com",
                "Email must contain exactly one @ symbol",
            ),
            (
                "test..user@example.com",
                "Email cannot contain consecutive dots",
            ),
            (
                ".test@example.com",
                "Email parts cannot start or end with dots",
            ),
            (
                "test.@example.com",
                "Email parts cannot start or end with dots",
            ),
            (
                "test@.example.com",
                "Email parts cannot start or end with dots",
            ),
            ("test@example", "Invalid domain part (after @)"),
            ("test@", "Invalid domain part (after @)"),
            ("@example.com", "Invalid local part (before @)"),
            ("test user@example.com", "Invalid local part (before @)"),
            ("test@example..com", "Email cannot contain consecutive dots"),
        ];

        for (email, expected_error) in test_cases {
            let input = EmailValidatorInput {
                email: email.to_string(),
                check_dns: None,
            };
            let result = validate_email(input).unwrap();
            assert!(!result.is_valid, "Email '{email}' should be invalid");
            assert!(result.error.is_some());
            let actual_error = result.error.unwrap();
            assert!(
                actual_error.contains(expected_error),
                "Email '{email}' should have error containing '{expected_error}', but got '{actual_error}'"
            );
        }
    }

    #[test]
    fn test_email_parts() {
        let input = EmailValidatorInput {
            email: "user@example.com".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(result.is_valid);

        let parts = result.parts.unwrap();
        assert_eq!(parts.local, "user");
        assert_eq!(parts.domain, "example.com");
        assert_eq!(parts.tld, Some("com".to_string()));
    }

    #[test]
    fn test_validation_checks() {
        let input = EmailValidatorInput {
            email: "valid@example.com".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();

        assert!(result.checks.has_single_at);
        assert!(result.checks.valid_local);
        assert!(result.checks.valid_domain);
        assert!(result.checks.no_consecutive_dots);
        assert!(result.checks.no_edge_dots);
        assert!(result.checks.valid_characters);
        assert!(result.checks.reasonable_length);
    }

    #[test]
    fn test_long_email() {
        let local = "a".repeat(64);
        let domain = "example.com";
        let email = format!("{local}@{domain}");

        let input = EmailValidatorInput {
            email,
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_too_long_local() {
        let local = "a".repeat(65);
        let domain = "example.com";
        let email = format!("{local}@{domain}");

        let input = EmailValidatorInput {
            email,
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_numeric_tld() {
        let input = EmailValidatorInput {
            email: "test@example.123".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_hyphen_in_domain() {
        let input = EmailValidatorInput {
            email: "test@ex-ample.com".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_leading_hyphen_domain() {
        let input = EmailValidatorInput {
            email: "test@-example.com".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_multiple_subdomains() {
        let input = EmailValidatorInput {
            email: "test@mail.subdomain.example.com".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.parts.unwrap().tld, Some("com".to_string()));
    }

    #[test]
    fn test_trimmed_email() {
        let input = EmailValidatorInput {
            email: "  test@example.com  ".to_string(),
            check_dns: None,
        };
        let result = validate_email(input).unwrap();
        assert!(result.is_valid);
    }
}
