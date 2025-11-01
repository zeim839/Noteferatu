// Ignore "unused fields" warnings in enums.
#![allow(dead_code)]

use renfield::tools::{schema, Schema};

#[test]
fn test_unit() {
    #[derive(schema)]
    enum BasicTool {
        Variant0,
        #[schema(rename = "variant_1")]
        Variant1,
        Variant2,
        #[schema(skip)]
        Variant3,
    }
    assert_eq!(
        BasicTool::schema(),
        serde_json::json!({
            "enum": ["Variant0", "variant_1", "Variant2"]
        })
    );
}

#[test]
fn test_unnamed() {
    #[derive(schema)]
    enum UnnamedFields {
        Variant0(i64, String, u8),
        Variant1,
    }
    assert_eq!(UnnamedFields::schema(), serde_json::json!({
        "enum": [
            <(i64, String, u8)>::schema(),
            "Variant1",
        ]
    }));
}

#[test]
fn test_unnamed_nested() {
    #[derive(schema)]
    enum Child {
        Child0(u8),
        Child1(i32, String, f32),
    }
    #[derive(schema)]
    enum Parent {
        Nested(Child),
        Inner(String, char, bool),
    }
    assert_eq!(Parent::schema(), serde_json::json!({
        "enum": [
            Child::schema(),
            <(String, char, bool)>::schema(),
        ]
    }));
}

#[test]
fn test_unnamed_field_attributes() {
    #[derive(schema)]
    #[schema(desc = "description", rename_all = "kebab-case")]
    enum Child {
        #[schema(desc = "hello world")]
        Child0(u8),
        #[schema(rename = "child_1")]
        Child1(i32, String, f32),
        #[schema(skip)]
        Child2,
    }
    #[derive(schema)]
    #[schema(desc = "description")]
    enum Parent {
        Nested(Child),
        Inner(String, char, bool),
    }
    assert_eq!(Parent::schema(), serde_json::json!({
        "description": "description",
        "enum": [
            {
                "description": "description",
                "enum": [
                    u8::schema_with_meta(Some("hello world")),
                    <(i32, String, f32)>::schema(),
                ],
            },
            <(String, char, bool)>::schema(),
        ],
    }));
}

#[test]
fn test_empty_unnamed() {
    #[derive(schema)]
    enum Empty {
        Field(),
    }
    assert_eq!(Empty::schema(), serde_json::json!({
        "enum": [ <()>::schema() ]
    }));
}

#[test]
fn test_named() {
    #[derive(schema)]
    struct Child {
        child_field: u8,
    }
    #[derive(schema)]
    enum NamedFields {
        ChildField(Child),
        Field {
            child_field0: u8,
            child_field1: String,
        },
    }
    assert_eq!(NamedFields::schema(), serde_json::json!({
        "enum": [
            Child::schema(),
            {
                "type": "object",
                "properties": {
                    "child_field0": u8::schema(),
                    "child_field1": String::schema(),
                },
                "required": [
                    "child_field0",
                    "child_field1",
                ],
            },
        ],
    }));
}
