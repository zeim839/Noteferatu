// Ignore "unused fields" warning in structs.
#![allow(dead_code)]

use renfield::tools::{schema, Schema};

#[test]
fn test_struct_basic() {
    #[derive(schema)]
    struct BasicTool {
        pub integer_type: i64,
        pub boolean_type: bool,
        pub numerical_type: f32,
    }
    assert_eq!(
        BasicTool::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "integer_type": i64::schema(),
                "boolean_type": bool::schema(),
                "numerical_type": f32::schema(),
            },
            "required": vec![
                "integer_type",
                "boolean_type",
                "numerical_type",
            ],
        })
    );
}

#[test]
fn test_struct_basic_nested() {
    #[derive(schema)]
    struct Child {
        pub child_0: Vec<f32>,
        pub child_2: i128,
        pub child_3: String,
    }
    #[derive(schema)]
    struct Parent {
        pub child: Child,
        pub inner: String,
    }
    assert_eq!(
        Parent::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "child": Child::schema(),
                "inner": String::schema(),
            },
            "required": [
                "child",
                "inner",
            ],
        })
    );
}

#[test]
fn test_struct_optional_field() {
    #[derive(schema)]
    struct Foo {
        pub bar: Option<String>,
        #[schema(optional)]
        pub foo: String,
    }
    assert_eq!(
        Foo::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "bar": <Option<String>>::schema(),
                "foo": String::schema(),
            },
        })
    );
}

#[test]
fn test_struct_generic() {
    #[derive(schema)]
    struct Foo<T> {
        bar: T,
    }
    assert_eq!(
        Foo::<String>::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "bar": String::schema(),
            },
            "required": [ "bar" ],
        })
    );
    #[derive(schema)]
    struct Bar<T: std::fmt::Display> {
        foo: Option<T>,
    }
    assert_eq!(
        Bar::<i64>::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "foo": i64::schema(),
            },
        })
    );
}

#[test]
fn test_struct_pointer_types() {
    #[derive(schema)]
    struct GetWeather<'a> {
        location: &'a str,
    }
    assert_eq!(
        GetWeather::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "location": str::schema(),
            },
            "required": [ "location" ]
        })
    );
}

#[test]
fn test_tuple_struct() {
    #[derive(schema)]
    struct TupleStruct(i64, u32, char, String);
    assert_eq!(
        TupleStruct::schema(),
        serde_json::json!({
            "type": "array",
            "prefixItems": [
                i64::schema(),
                u32::schema(),
                char::schema(),
                String::schema(),
            ]
        })
    );
}

#[test]
fn test_tuple_struct_generics() {
    #[derive(schema)]
    struct TupleStruct<'a, T>(&'a str, &'a T, T);
    assert_eq!(
        TupleStruct::<String>::schema(),
        serde_json::json!({
            "type": "array",
            "prefixItems": [
                str::schema(),
                String::schema(),
                String::schema(),
            ],
        })
    );
}

#[test]
fn test_struct_member_tuples() {
    #[derive(schema)]
    struct TupleStruct(i32, i32);
    #[derive(schema)]
    struct Parent {
        tuple_struct: TupleStruct,
        tuple_child: (f32, f32),
    }
    assert_eq!(
        Parent::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "tuple_struct": TupleStruct::schema(),
                "tuple_child": <(f32, f32)>::schema(),
            },
            "required": [
                "tuple_struct",
                "tuple_child",
            ]
        })
    );
}

#[test]
fn test_unit_struct() {
    #[derive(schema)]
    struct Unit;
    assert_eq!(
        Unit::schema(),
        serde_json::json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false,
        })
    );
}

#[test]
fn test_struct_unnamed_fields() {
    #[derive(schema)]
    struct Foo();
    assert_eq!(Foo::schema(), <()>::schema());
    #[derive(schema)]
    #[schema(desc = "hello")]
    struct Bar(i32);
    assert_eq!(Bar::schema(), <i32>::schema_with_meta(Some("hello")));
    #[derive(schema)]
    struct FooBar(i32, i64);
    assert_eq!(FooBar::schema(), <(i32, i64)>::schema());
}

#[test]
fn test_struct_field_attributes() {
    #[derive(schema)]
    #[schema(rename_all = "SCREAMING-KEBAB-CASE")]
    struct Child {
        child_field: u8,
    }
    #[derive(schema)]
    #[schema(rename_all = "UPPERCASE", desc = "description")]
    struct Parent {
        #[schema(optional = false)]
        child: Child,
        #[schema(rename = "_other", desc = "description")]
        other: i32,
        #[schema(skip)]
        skipped: bool,
    }
    assert_eq!(
        Parent::schema(),
        serde_json::json!({
            "type": "object",
            "description": "description",
            "properties": {
                "CHILD": {
                    "type": "object",
                    "properties": {
                        "CHILD-FIELD": u8::schema(),
                    },
                    "required": [
                        "CHILD-FIELD",
                    ]
                },
                "_other": i32::schema_with_meta(Some("description")),
            },
            "required": [
                "CHILD", "_other"
            ],
        })
    );
}

#[test]
fn test_struct_container_attributes() {
    #[derive(schema)]
    #[schema(rename_all = "kebab-case", desc = "my description")]
    struct Container {
        my_child: i32,
        my_special_child: u64,
    }
    assert_eq!(
        Container::schema(),
        serde_json::json!({
            "type": "object",
            "description": "my description",
            "properties": {
                "my-child": i32::schema(),
                "my-special-child": u64::schema(),
            },
            "required": [
                "my-child",
                "my-special-child",
            ],
        })
    );
}

#[test]
fn test_overriding_attributes() {
    #[derive(schema)]
    #[schema(rename_all = "kebab-case", desc = "...")]
    #[schema(rename_all = "UPPERCASE", desc = "description")]
    struct Container {
        #[schema(optional = true, rename = "ccchild", desc = "...")]
        #[schema(optional = false, rename = "child0", desc = "description")]
        child0: i32,
        child1: u64,
    }
    assert_eq!(
        Container::schema(),
        serde_json::json!({
            "type": "object",
            "description": "description",
            "properties": {
                "child0": i32::schema_with_meta(Some("description")),
                "CHILD1": u64::schema(),
            },
            "required": [
                "child0", "CHILD1"
            ]
        })
    );
}

#[test]
fn test_docstring_description() {
    /// This is a description.
    #[derive(schema)]
    struct Foo {}
    assert_eq!(Foo::schema(), serde_json::json!({
        "type": "object",
        "properties": {},
        "description": "This is a description.",
    }));
    /// This description is overwritten.
    #[derive(schema)]
    #[schema(desc = "new description")]
    struct Bar {}
    assert_eq!(Bar::schema(), serde_json::json!({
        "type": "object",
        "properties": {},
        "description": "new description",
    }));
}
