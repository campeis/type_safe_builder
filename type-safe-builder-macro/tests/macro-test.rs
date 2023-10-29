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
        #[builder(default)]
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
        #[builder(default = 10)]
        f1: i64,
    }

    let built = StructWithFieldBuilder::builder().build();

    assert_eq!(10, built.f1);
}

#[test]
fn default_fields_can_be_set_as_code() {
    #[derive(Builder)]
    struct StructWithField {
        #[builder(default = <i64 as Default>::default() + 1)]
        f1: i64,
    }

    let built = StructWithFieldBuilder::builder().build();

    assert_eq!(<i64 as Default>::default() + 1, built.f1);
}

#[test]
fn default_fields_can_be_overridden() {
    #[derive(Builder)]
    struct StructWithField {
        f1: String,
        #[builder(default)]
        f2: i64,
    }

    let built = StructWithFieldBuilder::builder()
        .f1("value f1".into())
        .f2(1)
        .build();

    assert_eq!("value f1", built.f1);
    assert_eq!(1, built.f2);
}

#[test]
fn default_fields_with_value_can_be_overridden() {
    #[derive(Builder)]
    struct StructWithField {
        #[builder(default = 10)]
        f1: i64,
    }

    let built = StructWithFieldBuilder::builder().f1(1).build();

    assert_eq!(1, built.f1);
}

#[test]
fn struct_can_be_configured_with_default_as_standard() {
    #[derive(Builder)]
    #[builder(default)]
    struct StructWithField {
        f1: i64,
    }

    let built = StructWithFieldBuilder::builder().build();

    assert_eq!(i64::default(), built.f1);
}

#[test]
fn struct_can_be_configured_with_default_as_standard_and_mandatory_fields() {
    #[derive(Builder)]
    #[builder(default)]
    struct StructWithField {
        f1: i64,
        #[builder(mandatory)]
        f2: i64,
    }

    let built = StructWithFieldBuilder::builder().f2(1).build();

    assert_eq!(i64::default(), built.f1);
    assert_eq!(1, built.f2);
}

#[test]
fn field_default_with_value_works_with_struct_default() {
    #[derive(Builder)]
    #[builder(default)]
    struct StructWithField {
        f1: i64,
        #[builder(default = 1)]
        f2: i64,
    }

    let built = StructWithFieldBuilder::builder().build();

    assert_eq!(i64::default(), built.f1);
    assert_eq!(1, built.f2);
}

#[test]
fn accepts_fields_with_generic_args() {
    #[derive(Builder)]
    struct StructWithField {
        f1: Option<i64>,
    }

    let built = StructWithFieldBuilder::builder().f1(Some(1)).build();

    assert_eq!(Some(1), built.f1);
}

#[test]
fn works_with_fields_of_generic_type() {
    #[derive(Builder)]
    struct GenericStruct<T> {
        f1: T,
    }

    let built = GenericStructBuilder::builder().f1(1).build();

    assert_eq!(1, built.f1);
}

#[test]
fn generic_fields_could_have_where() {
    trait TraitForField {}
    struct FieldStruct {
        content: i64,
    }
    impl TraitForField for FieldStruct {}

    #[derive(Builder)]
    struct GenericStruct<T>
    where
        T: TraitForField,
    {
        f1: T,
    }
    let built = GenericStructBuilder::builder()
        .f1(FieldStruct { content: 1 })
        .build();

    assert_eq!(1, built.f1.content)
}

#[test]
fn more_then_one_field_could_have_generics_and_where_clause() {
    trait TraitForField {}
    struct FieldStruct {
        content: i64,
    }
    impl TraitForField for FieldStruct {}

    trait OtherTraitForField {}
    struct OtherFieldStruct {
        content: i64,
    }
    impl OtherTraitForField for OtherFieldStruct {}

    #[derive(Builder)]
    struct GenericStruct<T, K>
    where
        T: TraitForField,
        K: OtherTraitForField,
    {
        f1: T,
        f2: K,
    }
    let built = GenericStructBuilder::builder()
        .f1(FieldStruct { content: 1 })
        .f2(OtherFieldStruct { content: 2 })
        .build();

    assert_eq!(1, built.f1.content);
    assert_eq!(2, built.f2.content);
}

#[test]
fn generic_fields_can_have_defaults() {
    #[derive(Default, Eq, PartialEq, Debug)]
    struct StructWithDefault {
        v: i64,
    }
    #[derive(Builder)]
    struct GenericStruct<T, K: Default>
    where
        T: Default,
    {
        #[builder(default)]
        f1: T,
        #[builder(default)]
        f2: K,
    }
    let built = GenericStructBuilder::builder().build();

    assert_eq!(StructWithDefault::default(), built.f1);
    assert_eq!(StructWithDefault::default(), built.f2);
}

#[test]
fn can_build_structs_with_timelines() {
    #[derive(Builder)]
    struct TimelineStruct<'a> {
        f1: &'a String,
    }

    let string = "a string".to_string();
    let built = TimelineStructBuilder::builder().f1(&string).build();

    assert_eq!("a string".to_string(), built.f1.to_owned())
}

#[test]
fn can_build_structs_with_generics_timelines_and_constraints() {
    trait TraitForField {
        fn get_content(&self) -> i64;
    }
    struct FieldStruct {
        content: i64,
    }
    impl TraitForField for FieldStruct {
        fn get_content(&self) -> i64 {
            self.content
        }
    }

    #[derive(Builder)]
    struct TimelineStruct<'a, T: TraitForField> {
        f1: &'a T,
    }

    let referenced_struct = FieldStruct { content: 1 };
    let built = TimelineStructBuilder::builder()
        .f1(&referenced_struct)
        .build();

    assert_eq!(1, built.f1.get_content());
}

#[test]
fn works_with_const_generics() {
    #[derive(Builder)]
    struct ConstGenericStruct<const T: bool> {}

    let builder: ConstGenericStructBuilderState<true> = ConstGenericStructBuilder::builder();

    let _built = builder.build();
}
