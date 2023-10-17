#[test]
fn will_not_compile_if_field_with_attr_does_not_implement_default() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_failure/*.rs");
}
