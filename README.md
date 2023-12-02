# type_safe_builder
A typesafe builder macro in Rust

The generated builder will make sure a struct can't be built if any of the required fields has not been set.
The check will be done at compile time, so there will be no need to handle any error in the code.

```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    field: String,
    a_field_that_has_not_been_set: String,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .build(); // this will not compile
}
```

## How to use

### Basic usage
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    field: String,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .build();
}
```

### Default fields
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    field: String,
    #[builder(default)]
    default_field: Option<String>,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .build();
}
```

### Default fields value override
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    field: String,
    #[builder(default=Some("default value"))]
    default_field: Option<&'static str>,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .build();
}
```

### All Default fields in the struct
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(default)]
struct Struct {
    field: Option<String>,
    other_field: Option<String>,
}

fn main() {
    let build = StructBuilder::builder()
        .build();
}
```

### If default values is set a specific field can be set to require a value
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(default)]
struct Struct {
    #[builder(mandatory)]
    field: String,
    default_field: Option<String>,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .build();
}
```

### Custom builder name
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(name=CustomBuilder)]
struct Struct {}

fn main() {
    let build = CustomBuilder::builder()
        .build();
}
```

### Custom setter name
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    #[builder(setter_name=custom_setter)]
    field: String,
}

fn main() {
    let build = StructBuilder::builder()
        .custom_setter("value".into())
        .build();
}
```

### Allow field value to be set multiple times
By default the builder will not allow a field to be set multiple times.

Because of this the following code would not compile.

```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    field: String,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .field("another value".into()) //this will mot compile
        .build();
}
```

This behaviour could be changed on a field by field basis.

```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
struct Struct {
    #[builder(multi)]
    field: String,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value".into())
        .field("value that will override the previous one".into())
        .build();
}
```

### Allow field value to be set multiple times can be configured for all fields
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(multi)]
struct Struct {
    field: String,
    other_field: String,
}

fn main() {
    let build = StructBuilder::builder()
        .field("value for field".into())
        .field("value that will override the value of field".into())
        .other_field("value for other field".into())
        .other_field("value that will override the value of other_field".into())
        .build();
}
```

### Allow field value to be set multiple times set at the struct level can be disabled for specific fields
```rust
use type_safe_builder_macro::Builder;

#[derive(Builder)]
#[builder(multi)]
struct Struct {
    #[builder(single)]
    field: String,
    other_field: String,
}

fn main() {
    let build = StructBuilder::builder()
        .field("this can't be overridden".into())
        .other_field("value for other field".into())
        .other_field("value that will override the value of other_field".into())
        .build();
}
```
