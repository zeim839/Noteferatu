#![allow(clippy::zero_prefixed_literal)]

/// Derive the [Schema] trait for an arbitrary Rust type.
///
/// Generates a [`Schema`] implementation for an algebraic type
/// (e.g. `enum` or `struct`).
///
/// # The `schema` helper attribute
///
/// ## Container attributes
///
/// Supported container attributes are `rename_all` and `desc`.
/// `rename_all` operates just as it would in [serde], i.e. it renames
/// all fields according to the following identifier naming schemes:
///  * `#[schema(rename_all = "UPPERCASE")]`
///  * `#[schema(rename_all = "lowercase")]`
///  * `#[schema(rename_all = "PascalCase")]`
///  * `#[schema(rename_all = "camelCase")]`
///  * `#[schema(rename_all = "snake_case")]`
///  * `#[schema(rename_all = "SCREAMING_SNAKE_CASE")]`
///  * `#[schema(rename_all = "kebab-case")]`
///  * `#[schema(rename_all = "SCREAMING-KEBAB-CASE")]`
///
/// Additionally, adding a docstring is the same as adding using the
/// `desc` attribute. However, the `#[schema(desc = "...")]` attribute
/// takes precedence over any docstrings.
///
/// ### Example
///
/// ```
/// use renfield::tools::schema;
///
/// #[derive(schema)]
/// #[schema(rename_all = "UPPERCASE")]
/// struct Foo {
///     // This field will be renamed to "MY_FIELD".
///     my_field: i32,
/// }
/// ```
///
/// ## Field attributes
///
/// Field attributes operate on struct fields. Supported attributes
/// are:
/// * `#[schema(skip)]`: Skip serializing this field.
/// * `#[schema(rename)]`: Rename this field. This takes precedence
///   over any container-level `rename_all` attribute.
/// * `#[schema(desc = "...")]`: Add a field description.
/// * `#[schema(optional = true/false)]`: Mark the field as optional
///   (i.e. excludes the field from the `required` list).
///
/// `Option<T>` fields will be automatically marked as `optional`,
/// unless `#[schema(optional = false)]` is specified. Similar to
/// containers, field docstrings may serve as descriptions.
///
/// ### Example
///
/// ```
/// use renfield::tools::schema;
///
/// #[derive(schema)]
/// struct Foo {
///
///     #[schema(skip)]
///     skipped_field: u8,
///
///     /// This is a field description.
///     /// Same as calling #[schema(desc = "...")]
///     some_field: u8,
///
///     // This field is required.
///     #[schema(optional = false)]
///     force_required: Option<u8>,
///
///     //This field is optional.
///     optional: Option<u8>,
///
///     // Add an explicit description.
///     #[schema(desc = "my description")]
///     described_field: u8,
///
///     // Override the field's default name.
///     #[schema(rename = "RENAMEDFIELD")]
///     renamed_field: u8,
/// }
/// ```
///
/// ## Variant Attributes
///
/// Variant attributes operate on variant fields. Supported attributes
/// are:
/// * `#[schema(skip)]`: Skip serializing a variant.
/// * `#[schema(rename = "...")]`: Rename a variant.
/// * `#[schema(desc = "...")]`: Add a description to the variant.
///
/// ### Example
///
/// ```
/// use renfield::tools::schema;
///
/// #[derive(schema)]
/// enum Foo {
///
///     // This variant is skipped.
///     SkippedVariant,
///
///     /// Docstrings are variant descriptions.
///     /// Same as calling #[schema(desc = "...")]
///     SomeVariant,
///
///     // Add an explicit description.
///     #[schema(desc = "...")]
///     DescribedVariant,
///
///     // Rename this variant.
///     #[schema(rename = "RENAMED_VARIANT")]
///     RenamedVariant,
///
///     // Unnamed fields are supported.
///     UnnamedFields(i32, u8, String),
///
///     // Struct fields are supported.
///     StructFields {
///
///         // Add struct fields as you like!
///         #[schema(desc = "my description...")]
///         my_field: u8,
///     }
/// }
/// ```
///
/// # Examples
///
/// ## Basic schema derivation
///
/// ```
/// use renfield::tools::{Schema, schema};
///
/// /// Docstrings are descriptions!
/// #[derive(schema)]
/// #[schema(rename_all = "kebab-case")]
/// struct Foo {
///     pub my_field: u8,
///
///     // Optional field.
///     pub other_field: Option<u8>,
/// }
///
/// /// Docstrings are descriptions!
/// #[derive(schema)]
/// #[schema(rename_all = "SCREAMING_SNAKE_CASE")]
/// enum Bar {
///     Variant0,
///     Variant1,
///
///     // Skip this one.
///     #[schema(skip)]
///     Variant2,
/// }
///
/// assert_eq!(Foo::schema(), serde_json::json!({
///     "type": "object",
///     "description": "Docstrings are descriptions!",
///     "properties": {
///         "my-field": u8::schema(),
///         "other-field": u8::schema(),
///     },
///     "required": [ "my-field" ],
/// }));
///
/// assert_eq!(Bar::schema(), serde_json::json!({
///     "enum": ["VARIANT_0", "VARIANT_1"],
///     "description": "Docstrings are descriptions!",
/// }));
/// ```
pub use codegen::schema;

/// Generate a JSON schema for an arbitrary Rust type.
///
/// See [JSON schema specification](https://json-schema.org/)
pub trait Schema {

    /// Generate a JSON schema object for an arbitrary type.
    fn schema() -> serde_json::Value;

    /// Generate a JSON schema with additional metadata.
    ///
    /// Same as [`schema`] but optionally adds a description field.
    fn schema_with_meta(desc: Option<&str>) -> serde_json::Value {
        let mut schema = Self::schema();
        if let Some(desc) = desc {
            if let Some(obj) = schema.as_object_mut() {
                obj.insert(
                    "description".to_string(),
                    serde_json::Value::String(desc.to_string()),
                );
            }
        }
        schema
    }
}

impl Schema for core::num::NonZeroI8 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroI8::MAX,
            "minimum": core::num::NonZeroI8::MIN,
        })
    }
}

impl Schema for core::num::NonZeroI16 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroI16::MAX,
            "minimum": core::num::NonZeroI16::MIN,
        })
    }
}

impl Schema for core::num::NonZeroI32 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroI32::MAX,
            "minimum": core::num::NonZeroI32::MIN,
        })
    }
}

impl Schema for core::num::NonZeroI64 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroI64::MAX,
            "minimum": core::num::NonZeroI64::MIN,
        })
    }
}

impl Schema for core::num::NonZeroI128 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroI128::MAX,
            "minimum": core::num::NonZeroI128::MIN,
        })
    }
}

impl Schema for core::num::NonZeroIsize {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroIsize::MAX,
            "minimum": core::num::NonZeroIsize::MIN,
        })
    }
}

impl Schema for core::num::NonZeroU8 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroU8::MAX,
            "minimum": core::num::NonZeroU8::MIN,
        })
    }
}

impl Schema for core::num::NonZeroU16 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroU16::MAX,
            "minimum": core::num::NonZeroU16::MIN,
        })
    }
}

impl Schema for core::num::NonZeroU32 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroU32::MAX,
            "minimum": core::num::NonZeroU32::MIN,
        })
    }
}

impl Schema for core::num::NonZeroU64 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroU64::MAX,
            "minimum": core::num::NonZeroU64::MIN,
        })
    }
}

impl Schema for core::num::NonZeroU128 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroU128::MAX,
            "minimum": core::num::NonZeroU128::MIN,
        })
    }
}

impl Schema for core::num::NonZeroUsize {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "not": { "const": 0 },
            "maximum": core::num::NonZeroUsize::MAX,
            "minimum": core::num::NonZeroUsize::MIN,
        })
    }
}

#[cfg(target_has_atomic = "8")]
impl Schema for core::sync::atomic::AtomicBool {
    fn schema() -> serde_json::Value {
        bool::schema()
    }
}

#[cfg(target_has_atomic = "8")]
impl Schema for core::sync::atomic::AtomicI8 {
    fn schema() -> serde_json::Value {
        i8::schema()
    }
}

#[cfg(target_has_atomic = "16")]
impl Schema for core::sync::atomic::AtomicI16 {
    fn schema() -> serde_json::Value {
        i16::schema()
    }
}

#[cfg(target_has_atomic = "32")]
impl Schema for core::sync::atomic::AtomicI32 {
    fn schema() -> serde_json::Value {
        i32::schema()
    }
}

#[cfg(target_has_atomic = "64")]
impl Schema for core::sync::atomic::AtomicI64 {
    fn schema() -> serde_json::Value {
        i64::schema()
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Schema for core::sync::atomic::AtomicIsize {
    fn schema() -> serde_json::Value {
        isize::schema()
    }
}

#[cfg(target_has_atomic = "8")]
impl Schema for core::sync::atomic::AtomicU8 {
    fn schema() -> serde_json::Value {
        u8::schema()
    }
}

#[cfg(target_has_atomic = "16")]
impl Schema for core::sync::atomic::AtomicU16 {
    fn schema() -> serde_json::Value {
        u16::schema()
    }
}

#[cfg(target_has_atomic = "32")]
impl Schema for core::sync::atomic::AtomicU32 {
    fn schema() -> serde_json::Value {
        u32::schema()
    }
}

#[cfg(target_has_atomic = "64")]
impl Schema for core::sync::atomic::AtomicU64 {
    fn schema() -> serde_json::Value {
        u64::schema()
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Schema for core::sync::atomic::AtomicUsize {
    fn schema() -> serde_json::Value {
        usize::schema()
    }
}

impl Schema for i8 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": i8::MIN,
            "maximum": i8::MAX,
        })
    }
}

impl Schema for i16 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": i16::MIN,
            "maximum": i16::MAX,
        })
    }
}

impl Schema for i32 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": i32::MIN,
            "maximum": i32::MAX,
        })
    }
}

impl Schema for i64 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": i64::MIN,
            "maximum": i64::MAX,
        })
    }
}

impl Schema for i128 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": i128::MIN,
            "maximum": i128::MAX,
        })
    }
}

impl Schema for isize {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": isize::MIN,
            "maximum": isize::MAX,
        })
    }
}

impl Schema for u8 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": u8::MIN,
            "maximum": u8::MAX,
        })
    }
}

impl Schema for u16 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": u16::MIN,
            "maximum": u16::MAX,
        })
    }
}

impl Schema for u32 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": u32::MIN,
            "maximum": u32::MAX,
        })
    }
}

impl Schema for u64 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": u64::MIN,
            "maximum": u64::MAX,
        })
    }
}

impl Schema for u128 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": u128::MIN,
            "maximum": u128::MAX,
        })
    }
}

impl Schema for usize {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "integer",
            "minimum": usize::MIN,
            "maximum": usize::MAX,
        })
    }
}

impl Schema for f32 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "number",
            "minimum": f32::MIN,
            "maximum": f32::MAX,
        })
    }
}

impl Schema for f64 {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "number",
            "minimum": f64::MIN,
            "maximum": f64::MAX,
        })
    }
}

impl Schema for bool {
    fn schema() -> serde_json::Value {
        serde_json::json!({"type": "boolean"})
    }
}

impl Schema for String {
    fn schema() -> serde_json::Value {
        serde_json::json!({"type": "string"})
    }
}

impl<T: Schema> Schema for Vec<T> {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "array",
            "items": T::schema(),
        })
    }
}

impl<T: Schema> Schema for Option<T> {
    fn schema() -> serde_json::Value {
        T::schema()
    }
}

impl<T: Schema + ?Sized> Schema for &T {
    fn schema() -> serde_json::Value {
        T::schema()
    }
}

impl<T: Schema> Schema for [T] {
    fn schema() -> serde_json::Value {
        <Vec<T>>::schema()
    }
}

impl Schema for str {
    fn schema() -> serde_json::Value {
        String::schema()
    }
}

impl Schema for char {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "minLength": 1,
            "maxLength": 1,
        })
    }
}

impl Schema for () {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "array",
            "maxLength": 0,
        })
    }
}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T: Schema> Schema for (T,) {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "array",
            "items": [T::schema()],
            "minItems": 1,
            "maxItems": 1
        })
    }
}

macro_rules! array_impls {
    ($($len:tt)+) => {
        $(
            impl<T: Schema> Schema for [T; $len] {
                fn schema() -> serde_json::Value {
                    serde_json::json!({
                        "type": "array",
                        "items": T::schema(),
                        "minItems": $len,
                        "maxItems": $len,
                    })
                }
            }
        )+
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            #[cfg_attr(docsrs, doc(hidden))]
            impl<$($name),+> Schema for ($($name,)+)
            where
                $($name: Schema,)+
            {
                tuple_impl_body!($len => ($($name)+));
            }
        )+
    };
}

macro_rules! tuple_impl_body {
    ($len:expr => ($($name:ident)+)) => {
        fn schema() -> serde_json::Value {
            serde_json::json!({
                "type": "array",
                "prefixItems": [
                    $($name::schema(),)+
                ],
            })
        }
    };
}

array_impls! {
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

tuple_impls! {
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
