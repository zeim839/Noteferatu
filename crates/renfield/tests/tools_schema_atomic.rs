use renfield::tools::Schema;

#[test]
fn test_atomic_bool() {
    assert_eq!(core::sync::atomic::AtomicBool::schema(), bool::schema());
}

#[test]
fn test_atomic_i8() {
    assert_eq!(core::sync::atomic::AtomicI8::schema(), i8::schema());
}

#[test]
fn test_atomic_i16() {
    assert_eq!(core::sync::atomic::AtomicI16::schema(), i16::schema());
}

#[test]
fn test_atomic_i32() {
    assert_eq!(core::sync::atomic::AtomicI32::schema(), i32::schema());
}

#[test]
fn test_atomic_i64() {
    assert_eq!(core::sync::atomic::AtomicI64::schema(), i64::schema());
}

#[test]
fn test_atomic_isize() {
    assert_eq!(core::sync::atomic::AtomicIsize::schema(), isize::schema());
}

#[test]
fn test_atomic_u8() {
    assert_eq!(core::sync::atomic::AtomicU8::schema(), u8::schema());
}

#[test]
fn test_atomic_u16() {
    assert_eq!(core::sync::atomic::AtomicU16::schema(), u16::schema());
}

#[test]
fn test_atomic_u32() {
    assert_eq!(core::sync::atomic::AtomicU32::schema(), u32::schema());
}

#[test]
fn test_atomic_u64() {
    assert_eq!(core::sync::atomic::AtomicU64::schema(), u64::schema());
}

#[test]
fn test_atomic_usize() {
    assert_eq!(core::sync::atomic::AtomicUsize::schema(), usize::schema());
}
