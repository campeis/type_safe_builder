use type_safe_builder_macro::Builder;

struct WithNoDefault {
    field: i64,
}
#[derive(Builder)]
struct StructWithFieldWithNoDefault {
    #[builder(default)]
    field: WithNoDefault,
}

fn main() {}
