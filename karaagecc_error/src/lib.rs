// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::fmt;
use thiserror::Error;

use karaagecc_source::Loc;

#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ErrorKind {
    #[error("{0}")]
    Message(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub loc: Option<Loc>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.loc {
            Some(loc) => write!(f, "{}:{} {}", loc.line, loc.column, self.kind),
            _ => write!(f, "{}", self.kind),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind, loc: None }
    }

    pub fn with_loc(mut self, loc: Loc) -> Self {
        self.loc = Some(loc);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use karaage_asserts::*;

    type ResultString = Result<String>;
    fn_test_data_traits!(Error);
    fn_test_data_traits!(ResultString);

    #[test]
    fn test_new_error() {
        use ErrorKind::*;
        let msg = "message".to_string();
        assert_eq!(format!("{}", Error::new(Message(msg.clone()))), msg);

        let loc = Loc::new(0, 2, 1, 1);
        assert_eq!(
            format!("{}", Error::new(Message(msg.clone())).with_loc(loc.clone())),
            format!("{}:{} {}", loc.line, loc.column, msg),
        );
    }
}
