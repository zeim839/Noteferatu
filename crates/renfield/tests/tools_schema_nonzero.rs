use renfield::tools::Schema;

#[test]
fn test_nonzero_i8() {
    assert_eq!(core::num::NonZeroI8::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroI8::MAX,
        "minimum": core::num::NonZeroI8::MIN,
    }));
}

#[test]
fn test_nonzero_i16() {
    assert_eq!(core::num::NonZeroI16::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroI16::MAX,
        "minimum": core::num::NonZeroI16::MIN,
    }));
}

#[test]
fn test_nonzero_i32() {
    assert_eq!(core::num::NonZeroI32::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroI32::MAX,
        "minimum": core::num::NonZeroI32::MIN,
    }));
}

#[test]
fn test_nonzero_i64() {
    assert_eq!(core::num::NonZeroI64::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroI64::MAX,
        "minimum": core::num::NonZeroI64::MIN,
    }));
}

#[test]
fn test_nonzero_i128() {
    assert_eq!(core::num::NonZeroI128::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroI128::MAX,
        "minimum": core::num::NonZeroI128::MIN,
    }));
}

#[test]
fn test_nonzero_isize() {
    assert_eq!(core::num::NonZeroIsize::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroIsize::MAX,
        "minimum": core::num::NonZeroIsize::MIN,
    }));
}

#[test]
fn test_nonzero_u8() {
    assert_eq!(core::num::NonZeroU8::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroU8::MAX,
        "minimum": core::num::NonZeroU8::MIN,
    }));
}

#[test]
fn test_nonzero_u16() {
    assert_eq!(core::num::NonZeroU16::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroU16::MAX,
        "minimum": core::num::NonZeroU16::MIN,
    }));
}

#[test]
fn test_nonzero_u32() {
    assert_eq!(core::num::NonZeroU32::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroU32::MAX,
        "minimum": core::num::NonZeroU32::MIN,
    }));
}

#[test]
fn test_nonzero_u64() {
    assert_eq!(core::num::NonZeroU64::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroU64::MAX,
        "minimum": core::num::NonZeroU64::MIN,
    }));
}

#[test]
fn test_nonzero_u128() {
    assert_eq!(core::num::NonZeroU128::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroU128::MAX,
        "minimum": core::num::NonZeroU128::MIN,
    }));
}

#[test]
fn test_nonzero_usize() {
    assert_eq!(core::num::NonZeroUsize::schema(), serde_json::json!({
        "type": "integer",
        "not": {"const": 0},
        "maximum": core::num::NonZeroUsize::MAX,
        "minimum": core::num::NonZeroUsize::MIN,
    }));
}
