use type_safe_builder_macro::Builder;

struct WithNoDefault {
    field: i64,
}
#[derive(Builder)]
struct StructWithFieldWithNoDefault {
    #[build_default]
    field: WithNoDefault,
}

fn main() {}
