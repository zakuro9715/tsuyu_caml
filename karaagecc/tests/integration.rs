use insta::{assert_toml_snapshot, glob};
use karaagecc_source::Source;
use serde::Serialize;
use std::{path::Path, process};

mod karaage_testutil {
    use super::*;
    pub fn walk_files<F: FnMut(&str, &Path)>(dir: &str, ext: &str, mut f: F) {
        glob!(&format!("{}/*{}", dir, ext), |path| insta::with_settings!({
            snapshot_path => dir,
            prepend_module_to_snapshot => false,
            snapshot_suffix => "",
            input_file => path.to_path_buf(),
        }, {
            let name = path.file_name().unwrap().to_string_lossy();
            f(&name, path);
        }));
    }

    pub fn run_source(path: impl AsRef<Path>) -> Output {
        let source = Source::read_file(path.as_ref()).unwrap();
        Output::from(karaagecc::run(source).unwrap())
    }
}

#[test]
fn correct_sources() {
    let sources = "sources";
    karaage_testutil::walk_files(sources, ".c", |name, path| {
        assert_toml_snapshot!(name, karaage_testutil::run_source(path));
    });
}

#[test]
fn compile_error() {
    assert!(format!(
        "{}",
        karaagecc::compile(Source::inline("xx")).expect_err("")
    )
    .contains("error"));
}

#[derive(Serialize)]
pub struct Output {
    pub stdout_text: String,
    pub stdout_bytes: Vec<u8>,
    pub stderr_text: String,
    pub stderr_bytes: Vec<u8>,
    pub code: Option<i32>,
}

impl From<process::Output> for Output {
    fn from(output: process::Output) -> Self {
        fn parse_out(data: Vec<u8>) -> (String, Vec<u8>) {
            match String::from_utf8(data) {
                Ok(text) => (text, vec![]),
                Err(err) => (String::new(), err.into_bytes()),
            }
        }

        let (stdout_text, stdout_bytes) = parse_out(output.stdout);
        let (stderr_text, stderr_bytes) = parse_out(output.stderr);
        Self {
            stdout_text,
            stdout_bytes,
            stderr_text,
            stderr_bytes,
            code: output.status.code(),
        }
    }
}
