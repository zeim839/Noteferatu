use renfield::tools::Schema;

#[test]
fn test_string() {
    assert_eq!(
        String::schema(),
        serde_json::json!({
            "type": "string"
        })
    );
}

#[test]
fn test_string_array() {
    assert_eq!(
        <[String]>::schema(),
        serde_json::json!({
            "type": "array",
            "items": { "type": "string" }
        })
    );
}

#[test]
fn test_vector() {
    assert_eq!(
        <Vec<String>>::schema(),
        serde_json::json!({
            "type": "array",
            "items": { "type" : "string" }
        })
    );
    assert_eq!(
        <Vec<usize>>::schema(),
        serde_json::json!({
            "type": "array",
            "items": {
                "type": "integer",
                "minimum": usize::MIN,
                "maximum": usize::MAX,
            },
        })
    );
    assert_eq!(
        <Vec<Vec<i32>>>::schema(),
        serde_json::json!({
            "type": "array",
            "items": {
                "type": "array",
                "items": {
                    "type": "integer",
                    "minimum": i32::MIN,
                    "maximum": i32::MAX,
                },
            }
        })
    );
}

#[test]
fn test_option() {
    assert_eq!(<Option<i32>>::schema(), i32::schema());
    assert_eq!(<Option<String>>::schema(), String::schema());
    assert_eq!(<Option<Vec<i32>>>::schema(), <Vec<i32>>::schema());
    assert_eq!(<Option<Option<i64>>>::schema(), i64::schema());
    assert_eq!(<Option<f32>>::schema(), f32::schema());
}
