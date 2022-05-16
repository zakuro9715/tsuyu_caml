// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use karaagecc_source::Loc;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Error {
    pub message: String,
    pub loc: Option<Loc>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = &self.message;
        match &self.loc {
            Some(loc) => write!(f, "{}:{} {}", loc.line, loc.column, msg),
            _ => write!(f, "{}", msg),
        }
    }
}

impl Error {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_message(msg: impl Into<String>) -> Self {
        Self::new().with_message(msg)
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }

    pub fn with_loc(mut self, loc: impl Into<Loc>) -> Self {
        self.loc = Some(loc.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new_error() {
        let mut err = Error::new();
        assert_eq!(
            err,
            Error {
                message: String::default(),
                loc: None,
            },
        );

        let msg = "message";
        err = err.with_message(msg);
        assert_eq!(err, Error::from_message(msg));

        let loc = Loc::new(0, 2, 1, 1);
        err = err.with_loc(&loc);
        assert_eq!(
            err,
            Error {
                message: msg.to_string(),
                loc: Some(loc),
            },
        );
    }
}
