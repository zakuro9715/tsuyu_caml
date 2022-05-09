#[test]
fn run() {
    assert_eq!(karaagecc::run("10").unwrap().status.code().unwrap(), 10);
    assert_eq!(karaagecc::run("42").unwrap().status.code().unwrap(), 42);
}

#[test]
fn compile_error() {
    assert!(format!("{}", karaagecc::compile("xx").expect_err("")).contains("error"));
}
