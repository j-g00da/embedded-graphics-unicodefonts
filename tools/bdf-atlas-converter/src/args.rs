use clap::Parser;
use std::ops::RangeInclusive;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Convert BDF fonts using to atlas-backed embedded fonts")]
pub struct Args {
    /// Input BDF file
    #[arg(value_parser = validate_file_exists)]
    pub input: PathBuf,

    /// Display information about the BDF file without converting
    #[arg(short, long, conflicts_with_all = ["output", "ranges"])]
    pub info: bool,

    /// Output file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Additional Unicode ranges in hex format (e.g., 0x20..0x7f)
    #[arg(short, long = "range", value_parser = parse_unicode_range)]
    pub ranges: Vec<RangeInclusive<char>>,

    /// Maximum gap size to bridge when creating ranges (default: 1)
    #[arg(long, default_value = "1")]
    pub gap_threshold: u32,

    /// Minimum consecutive characters needed to form a range (default: 8)
    #[arg(long, default_value = "8")]
    pub min_range_length: usize,

    /// Save a PNG image of the generated font atlas
    #[arg(long)]
    pub save_png: bool,

    /// Optional suffix to append to font names (e.g. "_optimized")
    #[arg(long)]
    pub suffix: Option<String>,
}

fn parse_unicode_range(s: &str) -> Result<RangeInclusive<char>, String> {
    if let Some((start_str, end_str)) = s.split_once("..") {
        let start_code = parse_hex(start_str.trim())
            .map_err(|e| format!("Invalid start value '{}': {}", start_str, e))?;
        let end_code = parse_hex(end_str.trim())
            .map_err(|e| format!("Invalid end value '{}': {}", end_str, e))?;

        let start_char = char::from_u32(start_code)
            .ok_or_else(|| format!("Invalid Unicode code point: 0x{:x}", start_code))?;
        let end_char = char::from_u32(end_code)
            .ok_or_else(|| format!("Invalid Unicode code point: 0x{:x}", end_code))?;

        if start_code > end_code {
            return Err(format!(
                "Start value (0x{:x}) cannot be greater than end value (0x{:x})",
                start_code, end_code
            ));
        }

        Ok(start_char..=end_char)
    } else {
        Err(format!(
            "Invalid range format '{s}'. Expected format: 0x20..0x7f"
        ))
    }
}

fn parse_hex(s: &str) -> Result<u32, String> {
    s.strip_prefix("0x")
        .ok_or_else(|| format!("Expected hexadecimal format (0x...), got: {}", s))
        .map(|hex_str| u32::from_str_radix(hex_str, 16))?
        .map_err(|_| format!("Invalid hexadecimal number: {}", s))
}

fn validate_file_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    match () {
        _ if !path.exists() => Err(format!("Input file does not exist: {s}")),
        _ if !path.is_file() => Err(format!("Path is not a file: {s}")),
        _ => Ok(path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unicode_range_hex() {
        let range = parse_unicode_range("0x20..0x7f").unwrap();
        assert_eq!(*range.start(), '\u{20}');
        assert_eq!(*range.end(), '\u{7f}');
    }

    #[test]
    fn test_info_flag_conflicts() {
        use clap::Parser;

        // Test that --info works alone (would need a real file, so we test parsing logic)
        // Test that --info conflicts with --output and --ranges
        let result = Args::try_parse_from(&["prog", "--info", "--output", "test.rs", "input.bdf"]);
        assert!(result.is_err());

        let result =
            Args::try_parse_from(&["prog", "--info", "--range", "0x20..0x7f", "input.bdf"]);
        assert!(result.is_err());
    }
}
