use crate::tr_with;
use isbn2::{Isbn, IsbnError};
use std::str::FromStr;

/// Normalize and validate an ISBN string.
///
/// - If `is_plain == true`, returns the ISBN without hyphens.
/// - If `is_plain == false`, returns the ISBN with hyphens.
/// - Works with both ISBN-10 and ISBN-13.
/// - Returns an error string if validation fails.
///
/// # Examples
/// ```
/// # use librius::isbn::normalize_isbn;
///
/// let plain = normalize_isbn("978-88-203-8269-8", true).unwrap();
/// assert_eq!(plain, "9788820382698");
///
/// let pretty = normalize_isbn("9788820382698", false).unwrap();
/// assert_eq!(pretty, "978-88-203-8269-8");
/// ```
pub fn normalize_isbn(isbn_input: &str, is_plain: bool) -> Result<String, String> {
    let cleaned = isbn_input.trim().replace([' ', '-'], "");

    match Isbn::from_str(&cleaned) {
        Ok(isbn) => {
            let plain = isbn.to_string();
            let formatted = isbn
                .hyphenate()
                .unwrap_or_else(|_| plain.clone().parse().unwrap());

            Ok(if is_plain {
                plain
            } else {
                formatted.parse().unwrap()
            })
        }
        Err(IsbnError::InvalidChecksum) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.invalid_checksum", &[("isbn", isbn_input)]),
        )),
        Err(IsbnError::InvalidLength) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.invalid_length", &[("isbn", isbn_input)])
        )),
        Err(IsbnError::InvalidConversion) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.invalid_conversion", &[("isbn", isbn_input)])
        )),
        Err(IsbnError::InvalidDigit) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.invalid_digit", &[("isbn", isbn_input)])
        )),
        Err(IsbnError::DigitTooLarge) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.digit_too_large", &[("isbn", isbn_input)])
        )),
        Err(IsbnError::InvalidGroup) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.invalid_group", &[("isbn", isbn_input)])
        )),
        Err(IsbnError::UndefinedRange) => Err(format!(
            "\n{}",
            &tr_with("book.isbn.undefined_range", &[("isbn", isbn_input)])
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_output() {
        let r = normalize_isbn("978-88-203-8269-8", true).unwrap();
        assert_eq!(r, "9788820382698");
    }

    #[test]
    fn test_hyphenated_output() {
        let r = normalize_isbn("9788820382698", false).unwrap();
        assert_eq!(r, "978-88-203-8269-8");
    }

    #[test]
    fn test_invalid_length() {
        assert!(normalize_isbn("12345", true).is_err());
    }

    #[test]
    fn test_invalid_characters() {
        assert!(normalize_isbn("97A88203826B8", false).is_err());
    }
}
