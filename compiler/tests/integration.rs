use karaagecc_compiler as karaagecc;

#[test]
fn run() {
    assert_eq!(
        std::str::from_utf8(&karaagecc::run().unwrap().stdout).unwrap(),
        "Hello compiler\n"
    )
}
