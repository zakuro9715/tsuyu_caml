use karaagecc_source::Source;
#[test]
fn run() {
    assert_eq!(
        karaagecc::run(Source::inline("10")).unwrap().stdout,
        "10\n".as_bytes()
    );
    assert_eq!(
        karaagecc::run(Source::inline("42")).unwrap().stdout,
        "42\n".as_bytes()
    );
}

#[test]
fn compile_error() {
    assert!(format!(
        "{}",
        karaagecc::compile(Source::inline("xx")).expect_err("")
    )
    .contains("error"));
}
