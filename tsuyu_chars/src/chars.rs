// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Char;

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

pub struct Chars<'a> {
    graphemes: Graphemes<'a>,
}

impl<'a> Chars<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            graphemes: s.graphemes(true),
        }
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = Char<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.graphemes.size_hint()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.graphemes.next().map(Char::from)
    }
}

impl<'a> From<&'a str> for Chars<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use tsuyu_asserts::*;
    use rstest::*;

    #[rstest]
    #[case("abcd", vec!["a", "b", "c", "d"])]
    #[case("あa\n🐈a̐", vec!["あ", "a", "\n", "🐈", "a̐"])]
    fn test_iter(#[case] text: &str, #[case] char_strs: Vec<&str>) {
        assert_iter_eq!(Chars::from(text), char_strs);
    }
}
