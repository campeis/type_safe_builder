use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(default)]
struct StructToBuild {
    #[builder(mandatory)]
    field: i64,
}

fn main() {
    let _ = StructToBuildBuilder::builder().build();
}
