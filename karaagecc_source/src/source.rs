// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;
use std::{fs, io};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Source {
    pub path: PathBuf,
    pub code: String,
}

impl AsRef<Source> for Source {
    fn as_ref(&self) -> &Source {
        self
    }
}

impl Source {
    pub fn inline(code: impl Into<String>) -> Self {
        use nanoid::nanoid;
        Self {
            path: PathBuf::from(format!("__inline{}", nanoid!())),
            code: code.into(),
        }
    }

    pub fn read_file(path: impl Into<PathBuf>) -> io::Result<Source> {
        let path_value = path.into();
        let code = fs::read_to_string(&path_value)?;
        Ok(Self {
            path: path_value,
            code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline() {
        let source = Source::inline("abc");
        assert!(source
            .path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("inline"));
        assert_eq!(source.code, "abc");
    }

    #[test]
    fn test_read_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let source = Source::read_file(&path).unwrap();
        assert_eq!(source.path, path);
        assert_eq!(source.code, fs::read_to_string(&path).unwrap());
        assert_ne!(source.code.len(), 0);
    }

    #[test]
    fn test_read_file_not_found() {
        Source::read_file("xxxxxxxx").expect_err("");
    }
}
