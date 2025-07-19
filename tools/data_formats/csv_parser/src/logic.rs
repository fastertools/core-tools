use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvParserInput {
    /// CSV content to parse
    pub content: String,
    /// Whether first row contains headers
    pub has_headers: Option<bool>,
    /// Custom delimiter (default: comma)
    pub delimiter: Option<String>,
    /// Whether to skip empty lines
    pub skip_empty_lines: Option<bool>,
    /// Whether to trim whitespace from fields
    pub trim_fields: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvParserResult {
    /// Parsed headers (if any)
    pub headers: Option<Vec<String>>,
    /// Parsed data rows
    pub rows: Vec<Vec<String>>,
    /// Number of rows parsed
    pub row_count: usize,
    /// Number of columns detected
    pub column_count: usize,
    /// Parsing statistics
    pub stats: ParsingStats,
    /// Error if parsing failed
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingStats {
    /// Total lines processed
    pub lines_processed: usize,
    /// Lines skipped (empty or malformed)
    pub lines_skipped: usize,
    /// Whether all rows have same column count
    pub uniform_columns: bool,
    /// Delimiter used
    pub delimiter_used: String,
}

pub fn parse_csv(input: CsvParserInput) -> Result<CsvParserResult, String> {
    let has_headers = input.has_headers.unwrap_or(true);
    let skip_empty = input.skip_empty_lines.unwrap_or(true);
    let trim_fields = input.trim_fields.unwrap_or(true);

    // Get delimiter (default to comma)
    let delimiter = match input.delimiter.as_deref() {
        Some(d) if d.len() == 1 => d.chars().next().unwrap() as u8,
        Some("\\t") => b'\t',
        Some(d) => {
            return Err(format!(
                "Invalid delimiter: '{d}'. Must be a single character."
            ));
        }
        None => b',',
    };

    let delimiter_str = match delimiter {
        b'\t' => "\\t".to_string(),
        d => (d as char).to_string(),
    };

    // Create CSV reader
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .trim(csv::Trim::All)
        .flexible(true) // Allow variable number of fields per record
        .from_reader(Cursor::new(input.content.as_bytes()));

    // Parse headers if present
    let headers = if has_headers {
        match reader.headers() {
            Ok(h) => Some(
                h.iter()
                    .map(|s| {
                        if trim_fields {
                            s.trim().to_string()
                        } else {
                            s.to_string()
                        }
                    })
                    .collect::<Vec<String>>(),
            ),
            Err(e) => {
                return Ok(CsvParserResult {
                    headers: None,
                    rows: vec![],
                    row_count: 0,
                    column_count: 0,
                    stats: ParsingStats {
                        lines_processed: 0,
                        lines_skipped: 0,
                        uniform_columns: true,
                        delimiter_used: delimiter_str,
                    },
                    error: Some(format!("Failed to parse headers: {e}")),
                });
            }
        }
    } else {
        None
    };

    // Parse rows
    let mut rows = Vec::new();
    let mut lines_processed = 0;
    let mut lines_skipped = 0;
    let mut column_counts = Vec::new();

    for result in reader.records() {
        lines_processed += 1;

        match result {
            Ok(record) => {
                // Skip empty records if requested
                if skip_empty && record.is_empty() {
                    lines_skipped += 1;
                    continue;
                }

                let row: Vec<String> = record
                    .iter()
                    .map(|field| {
                        if trim_fields {
                            field.trim().to_string()
                        } else {
                            field.to_string()
                        }
                    })
                    .collect();

                column_counts.push(row.len());
                rows.push(row);
            }
            Err(e) => {
                // Skip malformed rows but track them
                lines_skipped += 1;
                eprintln!("Skipping malformed row: {e}");
            }
        }
    }

    // Calculate statistics
    let column_count = if let Some(ref h) = headers {
        h.len()
    } else if !rows.is_empty() {
        rows[0].len()
    } else {
        0
    };

    let uniform_columns = if column_counts.is_empty() {
        true
    } else {
        column_counts.iter().all(|&count| count == column_count)
    };

    Ok(CsvParserResult {
        headers,
        row_count: rows.len(),
        rows,
        column_count,
        stats: ParsingStats {
            lines_processed,
            lines_skipped,
            uniform_columns,
            delimiter_used: delimiter_str,
        },
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_csv() {
        let input = CsvParserInput {
            content: "Name,Age,City\nJohn,30,New York\nJane,25,Boston".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(
            result.headers,
            Some(vec![
                "Name".to_string(),
                "Age".to_string(),
                "City".to_string()
            ])
        );
        assert_eq!(result.row_count, 2);
        assert_eq!(result.column_count, 3);
        assert_eq!(result.rows[0], vec!["John", "30", "New York"]);
        assert_eq!(result.rows[1], vec!["Jane", "25", "Boston"]);
    }

    #[test]
    fn test_no_headers() {
        let input = CsvParserInput {
            content: "John,30,New York\nJane,25,Boston".to_string(),
            has_headers: Some(false),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(result.headers, None);
        assert_eq!(result.row_count, 2);
        assert_eq!(result.rows[0], vec!["John", "30", "New York"]);
    }

    #[test]
    fn test_tab_delimiter() {
        let input = CsvParserInput {
            content: "Name\tAge\tCity\nJohn\t30\tNew York".to_string(),
            has_headers: Some(true),
            delimiter: Some("\\t".to_string()),
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(
            result.headers,
            Some(vec![
                "Name".to_string(),
                "Age".to_string(),
                "City".to_string()
            ])
        );
        assert_eq!(result.rows[0], vec!["John", "30", "New York"]);
        assert_eq!(result.stats.delimiter_used, "\\t");
    }

    #[test]
    fn test_pipe_delimiter() {
        let input = CsvParserInput {
            content: "Name|Age|City\nJohn|30|New York".to_string(),
            has_headers: Some(true),
            delimiter: Some("|".to_string()),
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(
            result.headers,
            Some(vec![
                "Name".to_string(),
                "Age".to_string(),
                "City".to_string()
            ])
        );
        assert_eq!(result.rows[0], vec!["John", "30", "New York"]);
        assert_eq!(result.stats.delimiter_used, "|");
    }

    #[test]
    fn test_trim_fields() {
        let input = CsvParserInput {
            content: " Name , Age , City \n John , 30 , New York ".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: Some(true),
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(
            result.headers,
            Some(vec![
                "Name".to_string(),
                "Age".to_string(),
                "City".to_string()
            ])
        );
        assert_eq!(result.rows[0], vec!["John", "30", "New York"]);
    }

    #[test]
    fn test_empty_lines() {
        let input = CsvParserInput {
            content: "Name,Age\nJohn,30\n\nJane,25\n\n".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: Some(true),
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(result.row_count, 2);
        assert_eq!(result.rows[0], vec!["John", "30"]);
        assert_eq!(result.rows[1], vec!["Jane", "25"]);
    }

    #[test]
    fn test_quoted_fields() {
        let input = CsvParserInput {
            content: r#"Name,Description,Price
"Product A","Contains, comma",10.99
"Product B","Has ""quotes""",20.50"#
                .to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(result.row_count, 2);
        assert_eq!(
            result.rows[0],
            vec!["Product A", "Contains, comma", "10.99"]
        );
        assert_eq!(result.rows[1], vec!["Product B", "Has \"quotes\"", "20.50"]);
    }

    #[test]
    fn test_non_uniform_columns() {
        let input = CsvParserInput {
            content: "A,B,C\n1,2,3\n4,5\n6,7,8,9".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert!(!result.stats.uniform_columns);
        assert_eq!(result.column_count, 3); // Based on headers
        assert_eq!(result.rows[0].len(), 3);
        assert_eq!(result.rows[1].len(), 2);
        assert_eq!(result.rows[2].len(), 4);
    }

    #[test]
    fn test_empty_csv() {
        let input = CsvParserInput {
            content: "".to_string(),
            has_headers: Some(false),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(result.row_count, 0);
        assert_eq!(result.column_count, 0);
        assert!(result.headers.is_none());
    }

    #[test]
    fn test_headers_only() {
        let input = CsvParserInput {
            content: "Name,Age,City".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(
            result.headers,
            Some(vec![
                "Name".to_string(),
                "Age".to_string(),
                "City".to_string()
            ])
        );
        assert_eq!(result.row_count, 0);
        assert_eq!(result.column_count, 3);
    }

    #[test]
    fn test_invalid_delimiter() {
        let input = CsvParserInput {
            content: "test".to_string(),
            has_headers: None,
            delimiter: Some(",,".to_string()),
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid delimiter"));
    }

    #[test]
    fn test_newlines_in_quoted_fields() {
        let input = CsvParserInput {
            content: "Name,Address\n\"John\",\"123 Main St\nApt 4\"".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: None,
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(result.row_count, 1);
        assert_eq!(result.rows[0], vec!["John", "123 Main St\nApt 4"]);
    }

    #[test]
    fn test_parsing_stats() {
        let input = CsvParserInput {
            content: "A,B\n1,2\n\n3,4\n5".to_string(),
            has_headers: Some(true),
            delimiter: None,
            skip_empty_lines: Some(true),
            trim_fields: None,
        };
        let result = parse_csv(input).unwrap();

        assert_eq!(result.row_count, 3); // 1,2 | 3,4 | 5
        assert!(!result.stats.uniform_columns); // Different column counts
        // With flexible parsing, empty lines are handled internally by the CSV parser
    }
}
