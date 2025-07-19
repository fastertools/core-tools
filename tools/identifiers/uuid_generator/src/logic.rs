use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidGeneratorInput {
    /// Number of UUIDs to generate (default: 1, max: 100)
    pub count: Option<u32>,
    /// Format for the UUIDs (default: "hyphenated")
    /// Options: "hyphenated", "simple", "urn", "braced"
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidGeneratorOutput {
    /// Generated UUID(s)
    pub uuids: Vec<String>,
    /// Version of UUID generated
    pub version: String,
    /// Format used
    pub format: String,
}

pub fn generate_uuids(input: UuidGeneratorInput) -> Result<UuidGeneratorOutput, String> {
    // Validate and set defaults
    let count = input.count.unwrap_or(1);
    if count == 0 {
        return Err("Count must be at least 1".to_string());
    }
    if count > 100 {
        return Err("Count cannot exceed 100".to_string());
    }

    let format = input.format.unwrap_or_else(|| "hyphenated".to_string());

    // Validate format
    if !["hyphenated", "simple", "urn", "braced"].contains(&format.as_str()) {
        return Err(format!(
            "Invalid format '{}'. Valid formats are: hyphenated, simple, urn, braced",
            format
        ));
    }

    // Generate UUIDs
    let mut uuids = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let uuid = Uuid::new_v4();

        let formatted = match format.as_str() {
            "hyphenated" => uuid.to_string(),
            "simple" => uuid.as_simple().to_string(),
            "urn" => uuid.as_urn().to_string(),
            "braced" => uuid.as_braced().to_string(),
            _ => unreachable!(), // We validated format above
        };

        uuids.push(formatted);
    }

    Ok(UuidGeneratorOutput {
        uuids,
        version: "4".to_string(),
        format,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_uuid_default() {
        let input = UuidGeneratorInput {
            count: None,
            format: None,
        };

        let result = generate_uuids(input).unwrap();
        assert_eq!(result.uuids.len(), 1);
        assert_eq!(result.version, "4");
        assert_eq!(result.format, "hyphenated");

        // Check hyphenated format (8-4-4-4-12)
        let uuid = &result.uuids[0];
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn test_multiple_uuids() {
        let input = UuidGeneratorInput {
            count: Some(5),
            format: None,
        };

        let result = generate_uuids(input).unwrap();
        assert_eq!(result.uuids.len(), 5);

        // Check all UUIDs are unique
        let unique_count = result
            .uuids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 5);
    }

    #[test]
    fn test_simple_format() {
        let input = UuidGeneratorInput {
            count: Some(1),
            format: Some("simple".to_string()),
        };

        let result = generate_uuids(input).unwrap();
        assert_eq!(result.format, "simple");

        // Simple format has no hyphens
        let uuid = &result.uuids[0];
        assert_eq!(uuid.len(), 32);
        assert!(!uuid.contains('-'));
    }

    #[test]
    fn test_urn_format() {
        let input = UuidGeneratorInput {
            count: Some(1),
            format: Some("urn".to_string()),
        };

        let result = generate_uuids(input).unwrap();
        assert_eq!(result.format, "urn");

        // URN format starts with "urn:uuid:"
        let uuid = &result.uuids[0];
        assert!(uuid.starts_with("urn:uuid:"));
        assert_eq!(uuid.len(), 45); // "urn:uuid:" (9) + UUID (36)
    }

    #[test]
    fn test_braced_format() {
        let input = UuidGeneratorInput {
            count: Some(1),
            format: Some("braced".to_string()),
        };

        let result = generate_uuids(input).unwrap();
        assert_eq!(result.format, "braced");

        // Braced format has curly braces
        let uuid = &result.uuids[0];
        assert!(uuid.starts_with('{'));
        assert!(uuid.ends_with('}'));
        assert_eq!(uuid.len(), 38); // UUID (36) + braces (2)
    }

    #[test]
    fn test_zero_count_error() {
        let input = UuidGeneratorInput {
            count: Some(0),
            format: None,
        };

        let result = generate_uuids(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Count must be at least 1");
    }

    #[test]
    fn test_exceeds_max_count_error() {
        let input = UuidGeneratorInput {
            count: Some(101),
            format: None,
        };

        let result = generate_uuids(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Count cannot exceed 100");
    }

    #[test]
    fn test_invalid_format_error() {
        let input = UuidGeneratorInput {
            count: Some(1),
            format: Some("invalid".to_string()),
        };

        let result = generate_uuids(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid format"));
    }

    #[test]
    fn test_uuid_v4_characteristics() {
        let input = UuidGeneratorInput {
            count: Some(10),
            format: Some("hyphenated".to_string()),
        };

        let result = generate_uuids(input).unwrap();

        for uuid_str in result.uuids {
            // Parse back to verify it's a valid UUID
            let uuid = Uuid::parse_str(&uuid_str).expect("Should be valid UUID");

            // Verify it's version 4
            assert_eq!(uuid.get_version(), Some(uuid::Version::Random));
        }
    }
}
