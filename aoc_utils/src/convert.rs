use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Failed to convert {0}")]
    Failed(char),
}

/// Convert from char to numeric type T (such as u8, u32, etc).
/// Returns an error if the char cannot be converted to a number
pub fn c_to_num<T: std::convert::From<u8>>(c: char) -> Result<T> {
    let result = c.to_digit(10).ok_or(ConvertError::Failed(c))? as u8;
    Ok(result.into())
}

/// Convert a single string to numbers
/// "123" becomes [1,2,3]
/// This method is unsafe, and will panic if character is not a digit.
pub fn line_to_digits<T: std::convert::From<u8>>(line: &str) -> Result<Vec<T>> {
    line.chars().map(c_to_num).collect()
}

/// Convert a single string to numbers
/// "123" becomes [1,2,3]
/// This method is unsafe, and will panic if character is not a digit.
pub fn unsafe_line_to_digits<T: std::convert::From<u8>>(line: &str) -> Vec<T> {
    line.chars()
        .map(|c| (c.to_digit(10).unwrap() as u8).into())
        .collect()
}

/// Convert `\n` delimited lines to digits
/// "123\n456" becomes [[1, 2, 3], [4, 5, 6]]
pub fn data_to_digits<T: std::convert::From<u8>>(data: &str) -> Result<Vec<Vec<T>>> {
    data.lines().map(line_to_digits).collect()
}

/// Convert `\n` delimited lines to digits
/// "123\n456" becomes [[1, 2, 3], [4, 5, 6]]
pub fn unsafe_data_to_digits<T: std::convert::From<u8>>(data: &str) -> Vec<Vec<T>> {
    data.lines().map(unsafe_line_to_digits).collect()
}

/// Convert a single string to ASCII bytes
/// "123" becomes [49, 50, 51]
pub fn line_to_ascii_bytes(line: &str) -> Vec<u8> {
    line.as_bytes().to_vec()
}

/// Convert `\n` delimited lines to ascii bytes.
/// "abc\n123" becomes [[97, 98, 99], [49, 50, 51]]
pub fn data_to_ascii_bytes(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(line_to_ascii_bytes).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_to_ascii() {
        let input = "abc\n123";
        let bytes = data_to_ascii_bytes(input);
        println!("{:?}", bytes);
    }

    #[test]
    fn test_digits() {
        let input = "123\n456";
        println!("{:?}", data_to_digits::<u32>(input));
    }
}
