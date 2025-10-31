use renfield::tools::Schema;

#[test]
fn test_i8() {
    assert_eq!(i8::schema(), serde_json::json!({
        "type": "integer",
        "minimum": i8::MIN,
        "maximum": i8::MAX,
    }));
}

#[test]
fn test_i8_array() {
    assert_eq!(<[i8]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": i8::MIN,
            "maximum": i8::MAX,
        },
    }));
}

#[test]
fn test_i16() {
    assert_eq!(i16::schema(), serde_json::json!({
        "type": "integer",
        "minimum": i16::MIN,
        "maximum": i16::MAX,
    }));
}

#[test]
fn test_i16_array() {
    assert_eq!(<[i16]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": i16::MIN,
            "maximum": i16::MAX,
        },
    }));
}

#[test]
fn test_i32() {
    assert_eq!(i32::schema(), serde_json::json!({
        "type": "integer",
        "minimum": i32::MIN,
        "maximum": i32::MAX,
    }));
}

#[test]
fn test_i32_array() {
    assert_eq!(<[i32]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": i32::MIN,
            "maximum": i32::MAX,
        },
    }));
}

#[test]
fn test_i64() {
    assert_eq!(i64::schema(), serde_json::json!({
        "type": "integer",
        "minimum": i64::MIN,
        "maximum": i64::MAX,
    }));
}

#[test]
fn test_i64_array() {
    assert_eq!(<[i64]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": i64::MIN,
            "maximum": i64::MAX,
        },
    }));
}

#[test]
fn test_i128() {
    assert_eq!(i128::schema(), serde_json::json!({
        "type": "integer",
        "minimum": i128::MIN,
        "maximum": i128::MAX,
    }));
}

#[test]
fn test_i128_array() {
    assert_eq!(<[i128]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": i128::MIN,
            "maximum": i128::MAX,
        },
    }));
}

#[test]
fn test_isize() {
    assert_eq!(isize::schema(), serde_json::json!({
        "type": "integer",
        "minimum": isize::MIN,
        "maximum": isize::MAX,
    }));
}

#[test]
fn test_isize_array() {
    assert_eq!(<[isize]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": isize::MIN,
            "maximum": isize::MAX,
        },
    }));
}

#[test]
fn test_u8() {
    assert_eq!(u8::schema(), serde_json::json!({
        "type": "integer",
        "minimum": u8::MIN,
        "maximum": u8::MAX,
    }));
}

#[test]
fn test_u8_array() {
    assert_eq!(<[u8]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": u8::MIN,
            "maximum": u8::MAX,
        },
    }));
}

#[test]
fn test_u16() {
    assert_eq!(u16::schema(), serde_json::json!({
        "type": "integer",
        "minimum": u16::MIN,
        "maximum": u16::MAX,
    }));
}

#[test]
fn test_u16_array() {
    assert_eq!(<[u16]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": u16::MIN,
            "maximum": u16::MAX,
        },
    }));
}

#[test]
fn test_u32() {
    assert_eq!(u32::schema(), serde_json::json!({
        "type": "integer",
        "minimum": u32::MIN,
        "maximum": u32::MAX,
    }));
}

#[test]
fn test_u32_array() {
    assert_eq!(<[u32]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": u32::MIN,
            "maximum": u32::MAX,
        },
    }));
}

#[test]
fn test_u64() {
    assert_eq!(u64::schema(), serde_json::json!({
        "type": "integer",
        "minimum": u64::MIN,
        "maximum": u64::MAX,
    }));
}

#[test]
fn test_u64_array() {
    assert_eq!(<[u64]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": u64::MIN,
            "maximum": u64::MAX,
        },
    }));
}

#[test]
fn test_u128() {
    assert_eq!(u128::schema(), serde_json::json!({
        "type": "integer",
        "minimum": u128::MIN,
        "maximum": u128::MAX,
    }));
}

#[test]
fn test_u128_array() {
    assert_eq!(<[u128]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": u128::MIN,
            "maximum": u128::MAX,
        },
    }));
}

#[test]
fn test_usize() {
    assert_eq!(usize::schema(), serde_json::json!({
        "type": "integer",
        "minimum": usize::MIN,
        "maximum": usize::MAX,
    }));
}

#[test]
fn test_usize_array() {
    assert_eq!(<[usize]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "integer",
            "minimum": usize::MIN,
            "maximum": usize::MAX,
        },
    }));
}

#[test]
fn test_f32() {
    assert_eq!(f32::schema(), serde_json::json!({
        "type": "number",
        "minimum": f32::MIN,
        "maximum": f32::MAX,
    }));
}

#[test]
fn test_f32_array() {
    assert_eq!(<[f32]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "number",
            "minimum": f32::MIN,
            "maximum": f32::MAX,
        },
    }));
}

#[test]
fn test_f64() {
    assert_eq!(f64::schema(), serde_json::json!({
        "type": "number",
        "minimum": f64::MIN,
        "maximum": f64::MAX,
    }));
}

#[test]
fn test_f64_array() {
    assert_eq!(<[f64]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "number",
            "minimum": f64::MIN,
            "maximum": f64::MAX,
        },
    }));
}

#[test]
fn test_bool() {
    assert_eq!(bool::schema(), serde_json::json!({
        "type": "boolean",
    }));
}

#[test]
fn test_bool_array() {
    assert_eq!(<[bool]>::schema(), serde_json::json!({
        "type": "array",
        "items": {
            "type": "boolean",
        },
    }));
}

#[test]
fn test_char() {
    assert_eq!(char::schema(), serde_json::json!({
        "type": "string",
        "minLength": 1,
        "maxLength": 1,
    }));
}

#[test]
fn test_char_array() {
    assert_eq!(<[char]>::schema(), serde_json::json!({
        "type": "array",
        "items": char::schema(),
    }));
}
