use anyhow::Result;
use std::str::pattern::Pattern;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Failed to convert {0}")]
    Failed(char),
    #[error("Failed to convert from string")]
    NumberFromStr,
}

/// Convert from char to numeric type T (such as u8, u32, etc).
/// Returns an error if the char cannot be converted to a number
pub fn c_to_num<T: std::convert::From<u8>>(c: char) -> Result<T> {
    let result = c.to_digit(10).ok_or(ConvertError::Failed(c))? as u8;
    Ok(result.into())
}

/// Convert a single string to digits
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

/// Splits a line on `pat` and parses each into T
pub fn line_to_numbers<T: std::str::FromStr>(line: &str, pat: impl Pattern) -> Result<Vec<T>> {
    line.trim()
        .split(pat)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.trim()
                .parse::<T>()
                .map_err(|_| ConvertError::NumberFromStr.into())
        })
        .collect::<Vec<Result<T>>>()
        .into_iter()
        .collect()
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

pub fn grid_line<T: std::convert::From<char>>(line: &str) -> Vec<T> {
    line.chars().map(|c| T::from(c)).collect()
}
/// Convert data input into type T
pub fn data_to_grid<T: std::convert::From<char>>(data: &str) -> Vec<Vec<T>> {
    data.lines().map(grid_line).collect()
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

    #[test]
    fn test_line_to_numbers() {
        let line = "  1    2 3";
        assert_eq!(vec![1, 2, 3], line_to_numbers(line, ' ').expect("oops"));
    }
}
