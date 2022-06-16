// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Code;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Source {
    pub path: Box<Path>,
    pub code: Code,
}

impl AsRef<Source> for Source {
    fn as_ref(&self) -> &Source {
        self
    }
}

impl Source {
    pub fn inline(code: impl Into<Code>) -> Self {
        Self::dummy(format!("__inline{}", nanoid::nanoid!()), code)
    }

    pub fn dummy(name: impl Into<String>, code: impl Into<Code>) -> Self {
        Self {
            path: PathBuf::from(name.into()).into_boxed_path(),
            code: code.into(),
        }
    }

    pub fn read_file(path: impl Into<PathBuf>) -> io::Result<Source> {
        fn inner(path: Box<Path>) -> io::Result<Source> {
            let code = fs::read_to_string(&path)?;
            Ok(Source {
                path,
                code: code.into(),
            })
        }
        inner(path.into().into_boxed_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use karaage_asserts::{assert_eq, assert_ne, *};

    fn_test_data_traits!(Source);

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
    fn test_dummy() {
        let source = Source::dummy("name.c", "code");
        assert_eq!(source.path.file_name().unwrap().to_str().unwrap(), "name.c",);
        assert_eq!(source.code, "code");
    }

    #[test]
    fn test_read_file() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let source = Source::read_file(&path).unwrap();
        let code = fs::read_to_string(&path).unwrap();
        assert_eq!(source.path, path.into_boxed_path());
        assert_eq!(source.code, code);
        assert_ne!(source.code.len(), 0);
    }

    #[test]
    fn test_read_file_not_found() {
        Source::read_file("xxxxxxxx").expect_err("");
    }
}
