use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};
use md5::Md5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashGeneratorInput {
    /// Text to hash
    pub text: String,
    /// Hash algorithm (md5, sha256, sha512)
    pub algorithm: String,
    /// Output format (hex, base64)
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashGeneratorResult {
    /// The computed hash
    pub hash: String,
    /// Algorithm used
    pub algorithm: String,
    /// Output format used
    pub format: String,
    /// Length of the hash in bytes
    pub byte_length: usize,
    /// Length of the hash string
    pub string_length: usize,
    /// Input text length
    pub input_length: usize,
}

pub fn generate_hash(input: HashGeneratorInput) -> Result<HashGeneratorResult, String> {
    let algorithm = input.algorithm.to_lowercase();
    let format = input.format.as_ref().map(|s| s.to_lowercase()).unwrap_or_else(|| "hex".to_string());
    
    // Validate format
    if format != "hex" && format != "base64" {
        return Err(format!("Unsupported format: {}. Use 'hex' or 'base64'", format));
    }
    
    // Generate hash based on algorithm
    let (hash_bytes, byte_length) = match algorithm.as_str() {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(input.text.as_bytes());
            let result = hasher.finalize();
            (result.to_vec(), 16)
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(input.text.as_bytes());
            let result = hasher.finalize();
            (result.to_vec(), 32)
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(input.text.as_bytes());
            let result = hasher.finalize();
            (result.to_vec(), 64)
        }
        _ => {
            return Err(format!("Unsupported algorithm: {}. Use 'md5', 'sha256', or 'sha512'", algorithm));
        }
    };
    
    // Format output
    let hash_string = match format.as_str() {
        "hex" => hex::encode(&hash_bytes),
        "base64" => {
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD.encode(&hash_bytes)
        }
        _ => unreachable!(), // Already validated above
    };
    
    Ok(HashGeneratorResult {
        hash: hash_string.clone(),
        algorithm: algorithm.clone(),
        format,
        byte_length,
        string_length: hash_string.len(),
        input_length: input.text.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_hex() {
        let input = HashGeneratorInput {
            text: "hello world".to_string(),
            algorithm: "md5".to_string(),
            format: Some("hex".to_string()),
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
        assert_eq!(result.algorithm, "md5");
        assert_eq!(result.format, "hex");
        assert_eq!(result.byte_length, 16);
        assert_eq!(result.string_length, 32);
    }

    #[test]
    fn test_sha256_hex() {
        let input = HashGeneratorInput {
            text: "hello world".to_string(),
            algorithm: "sha256".to_string(),
            format: None, // Default to hex
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(result.algorithm, "sha256");
        assert_eq!(result.format, "hex");
        assert_eq!(result.byte_length, 32);
        assert_eq!(result.string_length, 64);
    }

    #[test]
    fn test_sha512_hex() {
        let input = HashGeneratorInput {
            text: "hello world".to_string(),
            algorithm: "sha512".to_string(),
            format: Some("hex".to_string()),
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.hash, "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f");
        assert_eq!(result.algorithm, "sha512");
        assert_eq!(result.byte_length, 64);
        assert_eq!(result.string_length, 128);
    }

    #[test]
    fn test_empty_string() {
        let input = HashGeneratorInput {
            text: "".to_string(),
            algorithm: "sha256".to_string(),
            format: None,
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_case_insensitive_algorithm() {
        let input = HashGeneratorInput {
            text: "test".to_string(),
            algorithm: "SHA256".to_string(),
            format: None,
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.algorithm, "sha256");
    }

    #[test]
    fn test_invalid_algorithm() {
        let input = HashGeneratorInput {
            text: "test".to_string(),
            algorithm: "sha1".to_string(),
            format: None,
        };
        let result = generate_hash(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported algorithm"));
    }

    #[test]
    fn test_invalid_format() {
        let input = HashGeneratorInput {
            text: "test".to_string(),
            algorithm: "sha256".to_string(),
            format: Some("binary".to_string()),
        };
        let result = generate_hash(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported format"));
    }

    #[test]
    fn test_unicode_input() {
        let input = HashGeneratorInput {
            text: "Hello 世界 🌍".to_string(),
            algorithm: "sha256".to_string(),
            format: None,
        };
        let result = generate_hash(input).unwrap();
        assert!(!result.hash.is_empty());
        assert_eq!(result.input_length, 17); // UTF-8 byte count (emojis take more bytes)
    }

    #[test]
    fn test_known_sha256_vectors() {
        let test_cases = vec![
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
            ("The quick brown fox jumps over the lazy dog", "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"),
        ];
        
        for (text, expected_hash) in test_cases {
            let input = HashGeneratorInput {
                text: text.to_string(),
                algorithm: "sha256".to_string(),
                format: None,
            };
            let result = generate_hash(input).unwrap();
            assert_eq!(result.hash, expected_hash);
        }
    }

    #[test]
    fn test_base64_format() {
        let input = HashGeneratorInput {
            text: "hello world".to_string(),
            algorithm: "sha256".to_string(),
            format: Some("base64".to_string()),
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.hash, "uU0nuZNNPgilLlLX2n2r+sSE7+N6U4DukIj3rOLvzek=");
        assert_eq!(result.format, "base64");
    }

    #[test]
    fn test_md5_base64() {
        let input = HashGeneratorInput {
            text: "hello world".to_string(),
            algorithm: "md5".to_string(),
            format: Some("base64".to_string()),
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.hash, "XrY7u+Ae7tCTyyK7j1rNww==");
    }

    #[test]
    fn test_large_input() {
        let large_text = "a".repeat(10000);
        let input = HashGeneratorInput {
            text: large_text,
            algorithm: "sha256".to_string(),
            format: None,
        };
        let result = generate_hash(input).unwrap();
        assert_eq!(result.input_length, 10000);
        assert_eq!(result.string_length, 64);
    }
}