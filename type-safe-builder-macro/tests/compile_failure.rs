#[test]
fn check_compiler_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_failure/*.rs");
}
