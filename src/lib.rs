//!Simple utility type to allow to skip value for purpose of deserialization.
//!
//!Why would you want to use it?
//!
//!Because sometimes you just do not care for value of map, and need key alone
//!
//!Unfortunately serde interface is too dumb to just skip next value

#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

use core::fmt;

struct IgnoredAnyVisitor;
impl<'de> serde::de::Visitor<'de> for IgnoredAnyVisitor {
    type Value = IgnoredAny;

    #[inline(always)]
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Any")
    }

    #[inline(always)]
    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_borrowed_str<E: serde::de::Error>(self, _: &'de str) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while let Some(_) = seq.next_element::<IgnoredAny>()? {
        }

        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(_) = map.next_entry::<IgnoredAny, IgnoredAny>()? {
        }
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_enum<A: serde::de::EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        data.variant::<IgnoredAny>()?;
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_char<E>(self, _: char) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_i128<E>(self, _: i128) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline(always)]
    fn visit_newtype_struct<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_ignored_any(Self)
    }

    #[inline(always)]
    fn visit_some<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_ignored_any(Self)
    }

    #[inline(always)]
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }
}

#[derive(Copy, Clone, Debug)]
///Value whose deserialization implementation just returns success with empty self
pub struct IgnoredAny;
impl<'de> serde::Deserialize<'de> for IgnoredAny {
    #[inline(always)]
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_ignored_any(IgnoredAnyVisitor)
    }
}
