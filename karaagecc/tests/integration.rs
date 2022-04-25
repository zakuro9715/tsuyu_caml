#[test]
fn run() {
    assert_eq!(karaagecc::run().unwrap().status.code().unwrap(), 42,)
}
