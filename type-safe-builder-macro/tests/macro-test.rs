use type_safe_builder_macro::Builder;

#[test]
fn can_derive_builder_for_struct_with_no_field() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithNoField {}

    let _ = StructWithNoFieldBuilder::builder();
}

#[test]
fn can_derive_builder_for_struct_with_fields() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithField {
        f: i64,
    }

    let _ = StructWithFieldBuilder::builder();
}

#[test]
fn builder_has_set_methods() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithField {
        f1: i64,
        f2: String,
    }

    let _ = StructWithFieldBuilder::builder()
        .f1(1)
        .f2("string".to_string());
}

#[test]
fn built_struct_has_fields_set() {
    #[derive(Builder)]
    struct StructWithField {
        f1: String,
        f2: String,
    }

    let built = StructWithFieldBuilder::builder()
        .f1("value f1".into())
        .f2("value f2".into())
        .build();

    assert_eq!("value f1", built.f1);
    assert_eq!("value f2", built.f2);
}

#[test]
fn default_fields_do_not_need_to_be_set() {
    #[derive(Builder)]
    struct StructWithField {
        f1: String,
        #[build_default]
        f2: i64,
    }

    let built = StructWithFieldBuilder::builder()
        .f1("value f1".into())
        .build();

    assert_eq!("value f1", built.f1);
    assert_eq!(<i64 as Default>::default(), built.f2);
}

#[test]
fn default_fields_can_be_set_by_macro() {
    #[derive(Builder)]
    struct StructWithField {
        #[build_default(10)]
        f1: i64,
    }

    let built = StructWithFieldBuilder::builder().build();

    assert_eq!(10, built.f1);
}

#[test]
fn default_fields_can_be_overridden() {
    #[derive(Builder)]
    struct StructWithField {
        f1: String,
        #[build_default]
        f2: i64,
    }

    let built = StructWithFieldBuilder::builder()
        .f1("value f1".into())
        .f2(1)
        .build();

    assert_eq!("value f1", built.f1);
    assert_eq!(1, built.f2);
}
