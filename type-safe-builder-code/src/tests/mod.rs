use crate::builder_for;
use quote::quote;

#[test]
fn test() {
    let input = quote! {
        struct Struct1 {
            #[builder(default)]
            field1: i64,
            field2: String,
        }
    };

    let actual = builder_for(input);

    assert!(actual.to_string().contains("struct Struct1Builder"));
}
