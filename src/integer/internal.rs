use crate::integer::{ISize, USize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::layout::{Abi, AsBytes};

macro_rules! impl_aligned_integer {
    ( $($kind:literal, $name:ident, $inner:tt, $size:literal),* $(,)?) => {
        $(
            #[doc = concat!($kind, " ", "integer type with explicit alignment requirements")]
            #[doc = ""]
            #[doc = "Without the explicit `align` representation hint, this type may have different"]
            #[doc = "alignment requirements on different machines. This helps to ensure that the type"]
            #[doc = "has a predictable layout in memory and that operations assuming a particular"]
            #[doc = "alignment value are sound."]
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(C, align($size))]
            pub struct $name($inner);

            impl $name {
                #[doc = concat!("Creates a new [`", stringify!($name), "`] from a fixed size array of bytes.")]
                #[inline]
                pub const fn new(value: $inner) -> Self {
                    Self(<$inner>::from_le(value))
                }

                pub const fn from_be(value: $inner) -> Self {
                    Self(<$inner>::from_be(value))
                }

                pub const fn from_le(value: $inner) -> Self {
                    Self(<$inner>::from_le(value))
                }

                pub const fn from_le_chunk(chunk: $crate::contiguous::Chunk<$size>) -> Self {
                    Self::from_le_bytes(chunk.into_array())
                }

                pub const fn from_be_chunk(chunk: $crate::contiguous::Chunk<$size>) -> Self {
                    Self::from_be_bytes(chunk.into_array())
                }

                pub fn from_chunk<E: $crate::endian::Endian>(
                    chunk: $crate::contiguous::Chunk<$size>,
                    endian: E,
                ) -> Self {
                    if endian.is_big_endian() {
                        Self::from_le_chunk(chunk)
                    } else {
                        Self::from_le_chunk(chunk)
                    }
                }

                #[doc = "Create a native endian integer value from its representation as a byte array in little endian."]
                #[inline]
                pub const fn from_le_bytes(bytes: [u8; $size]) -> Self {
                    Self(<$inner>::from_le_bytes(bytes))
                }

                #[doc = "Create a native endian integer value from its representation as a byte array in big endian."]
                #[inline]
                pub const fn from_be_bytes(bytes: [u8; $size]) -> Self {
                    Self(<$inner>::from_be_bytes(bytes))
                }

                #[doc = "Return the memory representation of this integer as a byte array in little-endian byte order."]
                #[inline]
                pub const fn to_le_bytes(self) -> [u8; $size] {
                    self.0.to_le_bytes()
                }
                #[doc = "Return the memory representation of this integer as a byte array in little-endian byte order."]
                #[inline]
                pub const fn to_be_bytes(self) -> [u8; $size] {
                    self.0.to_be_bytes()
                }

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in the the specified byte order.")]
                #[inline(always)]
                pub const fn get(self, endian: $crate::endian::Endianness) -> $inner {
                    use $crate::endian::Endianness::*;
                    match endian {
                        Big => <$inner>::from_be(self.0),
                        Little => <$inner>::from_le(self.0),
                    }
                }

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in native-endian byte order.")]
                #[doc = ""]
                #[doc = "This method uses the target endian value, calculated at compile time, to determine"]
                #[doc = "which byte order serialization variant to use."]
                #[inline(always)]
                pub const fn get_ne(self) -> $inner {
                    #[cfg(target_endian = "big")]
                    {
                        use $crate::endian::Endianness::*;
                        self.get(Big)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        use $crate::endian::Endianness::*;
                        self.get(Little)
                    }
                }

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in little-endian byte order.")]
                #[inline(always)]
                pub const fn get_le(self) -> $inner {
                    use $crate::endian::Endianness::*;
                    self.get(Little)
                }
                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in big-endian byte order.")]
                #[inline(always)]
                pub const fn get_be(self) -> $inner {
                    use $crate::endian::Endianness::*;
                    self.get(Big)
                }
            }

            impl ::core::convert::From<$inner> for $name {
                fn from(value: $inner) -> $name {
                    $name(value)
                }
            }

            impl $crate::util::FromInner<$inner> for $name {
                fn from_inner(inner: $inner) -> $name {
                    $name(inner)
                }
            }
            impl $crate::util::IntoInner<$inner> for $name {
                fn into_inner(self) -> $inner {
                    #[cfg(target_endian = "big")]
                    {
                        self.get_be()
                    }
                    #[cfg(target_endian = "little")]
                    {
                        self.get_le()
                    }
                }
            }

            impl ::core::fmt::Display for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::LowerHex for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::UpperHex for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::Binary for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::Octal for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
        )*
        };
}

/// A trait defining integer types with explicit or implicit byte order
/// serialization (endianness) that can be converted to and from slices of bytes.
///
/// # Safety
///
/// This trait may only be implemented for integer types, such as Rust's built-in
/// integer primitives. Newtype wrappers for these primitives are provided by the
/// crate. You are strongly encouraged to use them when performing operations on
/// bytes where endianness matters.
///
/// Implementing this trait for non-integer types is immediate **undefined
/// behaviour**. You are strongly encouraged to use the types provided by this crate
/// or, whenever possible, deriving the unsafe traits for your types. The derive
/// macros provided by [`abio_derive`][abio-derive] validate the layout of your
/// custom type(s) at compile time, ensuring that they will work as intended at
/// runtime.
pub unsafe trait Integer: Abi + AsBytes {
    type Value: Integer;

    /// Get this [`Integer`] type as a raw pointer.
    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /// Gets the value of this instance at runtime.
    fn value(&self) -> Self::Value {
        unsafe { self.as_ptr().cast::<Self::Value>().read() }
    }
}

macro_rules! impl_integer_for_primitives {
    ($($ty:ty: $inner:ty),* $(,)?) => {
        $(
            unsafe impl Integer for $ty {
                type Value = $inner;
            }
        )*
    };
}

impl_integer_for_primitives! {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    usize: usize,

    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    isize: isize,

    U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
    U128: u128,
    USize: usize,

    I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
    I128: i128,
    ISize: isize,
}