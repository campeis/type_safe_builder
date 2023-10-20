use type_safe_builder_macro::Builder;

trait TraitForField {}
struct FieldStruct {}
impl TraitForField for FieldStruct {}

#[derive(Builder)]
struct GenericStruct<T>
where
    T: TraitForField,
{
    f1: T,
}

struct StructWithoutRequiredTrait {}
fn main() {
    let _builder = GenericStructBuilder::builder().f1(StructWithoutRequiredTrait {});
}
