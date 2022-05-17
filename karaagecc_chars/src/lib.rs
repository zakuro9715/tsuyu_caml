// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub type Char = char;

pub fn is_whitespace(c: Char) -> bool {
    c.is_whitespace()
}

pub fn is_octal_digit(c: Char) -> bool {
    c.is_digit(8)
}

pub fn is_decimal_digit(c: Char) -> bool {
    c.is_digit(10)
}

pub fn is_hex_digit(c: Char) -> bool {
    c.is_digit(16)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_whitespace() {
        for c in vec![' ', '\t', '\n', '\r'] {
            assert!(is_whitespace(c));
        }
        for c in vec!['a', 'あ'] {
            assert!(!is_whitespace(c));
        }
    }

    #[test]
    fn test_is_digit() {
        for c in vec!['0', '3', '7'] {
            assert!(is_decimal_digit(c));
            assert!(is_hex_digit(c));
            assert!(is_octal_digit(c));
        }
        for c in vec!['8', '9'] {
            assert!(!is_octal_digit(c));
            assert!(is_decimal_digit(c));
            assert!(is_hex_digit(c));
        }
        for c in vec!['a', 'A', 'f', 'F'] {
            assert!(!is_octal_digit(c));
            assert!(!is_decimal_digit(c));
            assert!(is_hex_digit(c));
        }
        for c in vec!['g', '三'] {
            assert!(!is_octal_digit(c));
            assert!(!is_decimal_digit(c));
            assert!(!is_hex_digit(c));
        }
    }
}
