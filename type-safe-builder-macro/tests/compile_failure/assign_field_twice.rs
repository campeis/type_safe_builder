use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(multi)]
struct StructToBuild {
    #[builder(single)]
    field: i64,
}

fn main() {
    let _ = StructToBuildBuilder::builder().field(1).field(2).build();
}
