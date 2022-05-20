#[test]
fn test_assert_impl() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/codes/fail.rs")
}
