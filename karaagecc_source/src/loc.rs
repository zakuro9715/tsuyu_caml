// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{cmp, fmt, rc::Rc};

use crate::Source;
use karaage_utils::must;

#[derive(PartialEq, Eq, Clone)]
pub struct Loc {
    pub source: Rc<Source>,
    pub index: usize,
    pub len: usize,
    pub line: usize,
    pub column: usize,
}

#[macro_export]
macro_rules! loc {
    ($begin:expr , $end:expr ; $line:expr , $column:expr $(;)?) => {
        $crate::loc! {
            ::std::rc::Rc::new($crate::Source::inline("")) => {
                $begin , $end;
                $line , $column;
            }
        }
    };
    ($source:expr => { $begin:expr , $end:expr ; $line:expr , $column:expr $(;)? }) => {
        $crate::loc! { $source => $begin,$end; $line,$column; }
    };
    ($source:expr => $begin:expr , $end:expr ; $line:expr , $column:expr $(;)?) => {
        $crate::Loc {
            source: ::std::rc::Rc::clone(&$source),
            index: $begin,
            len: $end - $begin,
            line: $line,
            column: $column,
        }
    };
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Rc::ptr_eq(&self.source, &other.source)
            .then(|| self.index.partial_cmp(&other.index))
            .flatten()
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source: {s:?}, index: {index}, len: {len}, line: {line}, column: {column} }}",
            s = must!(self.source.path.to_str()),
            index = self.index,
            len = self.len,
            line = self.line,
            column = self.column,
        )
    }
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loc {}", self)
    }
}

impl Loc {
    pub fn head(s: &Rc<Source>) -> Self {
        Self {
            source: Rc::clone(s),
            index: 0,
            len: 1,
            line: 1,
            column: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use karaage_asserts::{assert_eq, assert_ne, *};

    fn_test_data_traits!(Loc);

    #[test]
    fn test_head() {
        let s = Rc::new(Source::inline(""));
        assert_eq!(Loc::head(&s).index, 0);
    }

    #[test]
    fn test_loc_macro() {
        let s = Rc::new(Source::inline(""));
        assert_eq!(
            loc! {s => 2,3; 1,2},
            Loc {
                source: Rc::clone(&s),
                index: 2,
                len: 1,
                line: 1,
                column: 2,
            },
        );
        assert_eq!(
            loc! {s => 2,3; 1,2},
            loc! {
                s =>
                    2,3;
                    1,2;
            },
        );
        assert_eq!(
            loc! { s => { 2,3; 1,2 }},
            loc! {
                s => {
                    2,3;
                    1,2;
                }
            },
        );

        let loc = loc! {1,2; 2,3};
        assert_eq!(
            loc,
            Loc {
                source: Rc::clone(&loc.source),
                index: loc.index,
                len: loc.len,
                line: loc.line,
                column: loc.column,
            },
        );
    }

    #[test]
    fn test_eq() {
        let s1 = Rc::new(Source::inline(""));
        let s2 = Rc::new(Source::inline(""));
        assert_eq!(loc! {s1 => 0,1; 1,1}, loc! {s1 => 0,1; 1,1});
        assert_ne!(loc! {s1 => 0,1; 1,1}, loc! {s1 => 0,2; 1,1});
        assert_ne!(loc! {s1 => 0,1; 1,1}, loc! {s2 => 0,1; 1,1});
    }

    #[test]
    fn test_fmt() {
        use std::path::Path;
        let mut s = Source::inline("");
        s.path = Path::new("name.c").to_path_buf().into_boxed_path();
        let s = Rc::new(s);
        assert_eq!(
            format!("{}", loc! {s => 1,3; 2,3}),
            "{ source: \"name.c\", index: 1, len: 2, line: 2, column: 3 }",
        );
        assert_eq!(
            format!("{:?}", loc! {s => 0,1;1,2}),
            "Loc { source: \"name.c\", index: 0, len: 1, line: 1, column: 2 }",
        );
    }
}
