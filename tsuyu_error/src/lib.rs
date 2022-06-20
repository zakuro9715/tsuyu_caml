// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::fmt;

use tsuyu_utils::derives::{Display, Error};
use tsuyu_source::Loc;

#[derive(Display, Debug, Eq, PartialEq, Clone)]
pub enum ErrorKind {
    Message(String),
}

impl From<String> for ErrorKind {
    fn from(s: String) -> Self {
        Self::Message(s)
    }
}

impl From<&str> for ErrorKind {
    fn from(s: &str) -> Self {
        Self::Message(s.to_string())
    }
}

#[derive(Error, PartialEq, Eq, Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub loc: Option<Loc>,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ComposedResult<T> = std::result::Result<T, Vec<Error>>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.loc {
            Some(loc) => write!(f, "{} {}", loc.to_short_string(), self.kind),
            _ => write!(f, "{}", self.kind),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind, loc: None }
    }

    pub fn with_loc(mut self, loc: Loc) -> Self {
        self.loc = Some(loc);
        self
    }
}

#[macro_export]
macro_rules! error {
    ($e:expr) => {
        $crate::Error::new($crate::ErrorKind::from($e))
    };
    ($e:expr, $loc:expr) => {
        $crate::Error::new($crate::ErrorKind::from($e)).with_loc($loc)
    };
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::*;
    use tsuyu_asserts::{assert_eq, *};
    use tsuyu_source::{loc, Source};

    type ResultString = Result<String>;
    fn_test_data_traits!(Error);
    fn_test_data_traits!(ResultString);

    #[test]
    fn test_error() {
        use ErrorKind::*;
        let s = Rc::new(Source::inline(""));

        assert_eq!(error!("123"), Error::new(Message("123".into())));
        assert_eq!(error!("123".to_string()), Error::new(Message("123".into())));

        let loc = loc! {s => 2,4; 1,3};
        assert_eq!(error!("123", loc.clone()).loc.unwrap(), loc);
        assert_eq!(
            error!("123", loc.clone()),
            Error::new(Message("123".into())).with_loc(loc),
        )
    }

    #[test]
    fn test_error_format() {
        assert_eq!(format!("{}", error!("abc")), "abc");
        let s = Rc::new(Source::inline(""));

        let loc = loc! {s => 0,2;1,1};
        assert_eq!(
            format!("{}", error!("efg", loc)),
            format!("{}:1:1 efg", s.path.to_string_lossy()),
        );
    }
}
