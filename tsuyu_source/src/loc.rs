// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{cmp, fmt, rc::Rc};

use crate::Source;
use tsuyu_utils::{clone_option_rc, define_with_params_and_init};

define_with_params_and_init! {
    #[derive(PartialEq, Eq, Clone)]
    pub struct Loc {
        source: Option<Rc<Source>>,
        pub index: usize,
        pub len: usize,
        pub line: usize,
        pub column: usize,
    }
}

#[macro_export]
macro_rules! loc {
    ($source:expr => { $begin:expr , $end:expr ; $line:expr , $column:expr $(;)? }) => {
        $crate::loc! { $source => $begin,$end; $line,$column; }
    };
    ($source:expr => $begin:expr , $end:expr ; $line:expr , $column:expr $(;)?) => {
        $crate::Loc::init($crate::LocInitParams {
            source: Some(::std::rc::Rc::clone(&$source)),
            index: $begin,
            len: $end - $begin,
            line: $line,
            column: $column,
        })
    };
    ($begin:expr , $end:expr ; $line:expr , $column:expr $(;)?) => {
        $crate::Loc::init($crate::LocInitParams {
            source: None,
            index: $begin,
            len: $end - $begin,
            line: $line,
            column: $column,
        })
    };
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (if let (Some(lhs_s), Some(rhs_s)) = (self.source(), other.source()) {
            Rc::ptr_eq(lhs_s, rhs_s)
        } else {
            true
        })
        .then(|| self.index.partial_cmp(&other.index))
        .flatten()
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source: {s}, index: {index}, len: {len}, line: {line}, column: {column} }}",
            s = self
                .source()
                .map(|s| format!("{:?}", s.path.to_string_lossy()))
                .unwrap_or_else(|| "None".to_string()),
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
    pub fn head(s: Option<&Rc<Source>>) -> Self {
        Self {
            source: clone_option_rc(s),
            index: 0,
            len: 1,
            line: 1,
            column: 1,
        }
    }

    pub fn to_short_string(&self) -> String {
        format!(
            "{}{}:{}",
            self.source()
                .map(|s| format!("{}:", s.path.to_string_lossy()))
                .unwrap_or_default(),
            self.line,
            self.column,
        )
    }

    pub fn source(&self) -> Option<&Rc<Source>> {
        self.source.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tsuyu_asserts::{assert_eq, assert_ne, *};

    fn_test_data_traits!(Loc);

    #[test]
    fn test_head() {
        let s = Rc::new(Source::inline(""));
        assert_eq!(Loc::head(Some(&s)).index, 0);
        assert_eq!(Loc::head(Some(&s)).source.unwrap(), s);
    }

    #[test]
    fn test_short_string() {
        let s = Rc::new(Source {
            path: std::path::PathBuf::from("abc").into_boxed_path(),
            code: crate::Code::from(""),
        });
        assert_eq!((loc! { s => 4,6;2,4 }).to_short_string(), "abc:2:4");
        assert_eq!((loc! { 4,6;2,4 }).to_short_string(), "2:4");
    }

    #[test]
    fn test_loc_macro() {
        let s = Rc::new(Source::inline(""));
        assert_eq!(
            loc! {s => 2,3; 1,2},
            Loc {
                source: Some(Rc::clone(&s)),
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
                source: None,
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
        assert_eq!(
            format!("{}", loc! {1,3; 2,3}),
            "{ source: None, index: 1, len: 2, line: 2, column: 3 }",
        );
        assert_eq!(
            format!("{:?}", loc! {0,1;1,2}),
            "Loc { source: None, index: 0, len: 1, line: 1, column: 2 }",
        );
    }
}
