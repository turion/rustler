use super::{atoms, error::Error};
use crate::{types::atom, ListIterator, MapIterator, Term, TermType};
use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;
use std::iter;

/// Converts a native Erlang term to a native Rust type.
///
/// See the [conversion table] for details about deserialization behavior.
///
/// [conversion table]: https://github.com/rusterlium/rustler/tree/master#conversion-table
pub fn from_term<'de, 'a: 'de, T>(term: Term<'a>) -> Result<T, Error>
where
    T: Deserialize<'de>,
{
    T::deserialize(term)
}

macro_rules! try_parse_number {
    ($term:expr, $type:ty, $visitor:expr, $visit_fn:ident) => {
        if let Ok(num) = crate::serde::parse_number(&$term) as Result<$type, Error> {
            return $visitor.$visit_fn(num);
        }
    };
}

impl<'de, 'a: 'de> de::Deserializer<'de> for Term<'a> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.get_type() {
            TermType::Atom => {
                if crate::serde::is_nil(&self) {
                    self.deserialize_unit(visitor)
                } else if let Ok(b) = crate::serde::parse_bool(&self) {
                    visitor.visit_bool(b)
                } else {
                    // unit variant (atom)
                    let string = atoms::term_to_string(&self)?;
                    visitor.visit_string(string)
                }
            }
            // i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 (i128, u128)
            TermType::Number => {
                try_parse_number!(self, u64, visitor, visit_u64);
                try_parse_number!(self, i64, visitor, visit_i64);
                try_parse_number!(self, f64, visitor, visit_f64);

                Err(Error::ExpectedNumber)
            }
            // char
            // string
            // byte array
            TermType::Binary => self.deserialize_str(visitor),
            // seq
            TermType::EmptyList | TermType::List => self.deserialize_seq(visitor),
            // map
            // struct
            // struct variant
            TermType::Map => {
                let iter = MapIterator::new(self).ok_or(Error::ExpectedMap)?;
                let de = match crate::serde::validate_struct(&self, None) {
                    Err(_) => MapDeserializer::new(iter, None),
                    Ok(struct_name_term) => MapDeserializer::new(iter, Some(struct_name_term)),
                };

                visitor.visit_map(de)
            }
            // newtype struct
            // newtype variant (atom, len 2)
            // tuple struct (atom, len 3+)
            // tuple variant (atom, len 3+)
            // => if nothing else, tuple (any len)
            TermType::Tuple => {
                let tuple = crate::serde::validate_tuple(self, None)?;
                visitor.visit_seq(SequenceDeserializer::new(tuple.into_iter()))
            }
            _ => Err(Error::TypeHintsRequired),
        }
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if crate::serde::is_nil(&self) {
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedNil)
        }
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(crate::serde::parse_bool(&self)?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if crate::serde::is_nil(&self) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(crate::serde::parse_number(&self)?)
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.list_length().or(Err(Error::ExpectedChar))? != 1 {
            return Err(Error::ExpectedChar);
        }

        let mut iter: ListIterator = self.decode().or(Err(Error::ExpectedList))?;
        let c: Option<char> = iter
            .next()
            .unwrap()
            .decode()
            .map(std::char::from_u32)
            .or(Err(Error::ExpectedChar))?;
        if let Some(c) = c {
            visitor.visit_char(c)
        } else {
            Err(Error::ExpectedChar)
        }
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(crate::serde::parse_str(self)?)
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(crate::serde::parse_binary(self)?)
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(crate::serde::parse_binary(self)?)
    }

    #[inline]
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let tuple = crate::serde::validate_tuple(self, Some(2))?;
        let name_term =
            atoms::str_to_term(&self.get_env(), name).or(Err(Error::ExpectedStructName))?;

        if tuple[0].ne(&name_term) {
            return Err(Error::InvalidStructName);
        }

        visitor.visit_newtype_struct(tuple[1])
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if !(self.is_list() | self.is_empty_list()) {
            return Err(Error::ExpectedList);
        }

        let iter: ListIterator = self.decode().or(Err(Error::ExpectedList))?;
        visitor.visit_seq(SequenceDeserializer::new(iter))
    }

    #[inline]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let tuple = crate::serde::validate_tuple(self, Some(len))?;
        visitor.visit_seq(SequenceDeserializer::new(tuple.into_iter()))
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut tuple = crate::serde::validate_tuple(self, Some(len + 1))?;
        let name_term =
            atoms::str_to_term(&self.get_env(), name).or(Err(Error::ExpectedStructName))?;

        if tuple[0].ne(&name_term) {
            return Err(Error::InvalidStructName);
        }

        let iter = tuple.split_off(1).into_iter();
        visitor.visit_seq(SequenceDeserializer::new(iter))
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // TODO: support keyword lists
        if self.is_map() {
            let iter = MapIterator::new(self).ok_or(Error::ExpectedMap)?;
            visitor.visit_map(MapDeserializer::new(iter, None))
        } else {
            Err(Error::ExpectedMap)
        }
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let struct_name_term = crate::serde::validate_struct(&self, Some(name))?;
        let iter = MapIterator::new(self).ok_or(Error::ExpectedStruct)?;
        visitor.visit_map(MapDeserializer::new(iter, Some(struct_name_term)))
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use EnumDeserializerType as EnumType;

        let variant: Option<(EnumType, Term<'a>)> = match self.get_type() {
            // unit variant
            TermType::Atom => Some((EnumType::Unit, self)),
            TermType::Binary => Some((EnumType::Unit, self)),
            TermType::Number => Some((EnumType::Unit, self)),
            // newtype or tuple variant
            TermType::Tuple => {
                let tuple = crate::serde::validate_tuple(self, None)?;
                match tuple.len() {
                    0 | 1 => None,
                    2 => Some((EnumType::Newtype, tuple[0])),
                    _ => Some((EnumType::Tuple, tuple[0])),
                }
            }
            // struct variant
            TermType::Map => {
                let struct_name_term = crate::serde::validate_struct(&self, None)?;
                Some((EnumType::Struct, struct_name_term))
            }
            _ => None,
        };

        variant.ok_or(Error::ExpectedEnum).and_then(|variant| {
            let (vtype, term) = variant;
            let enum_de = EnumDeserializer::new(vtype, term, variants, Some(self))?;
            visitor.visit_enum(enum_de)
        })
    }

    // TODO: is this right?
    #[inline]
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_type() {
            TermType::Atom => self.deserialize_str(visitor),
            TermType::Binary => self.deserialize_str(visitor),
            TermType::Number => self.deserialize_i64(visitor),
            _ => Err(Error::ExpectedAtom),
        }
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Just skip over this by calling visit_unit.
        visitor.visit_unit()
    }
}

/// SequenceDeserializer
pub struct SequenceDeserializer<'a, I>
where
    I: Iterator<Item = Term<'a>>,
{
    iter: iter::Fuse<I>,
}

impl<'a, I> SequenceDeserializer<'a, I>
where
    I: Iterator<Item = Term<'a>>,
{
    #[inline]
    fn new(iter: I) -> Self {
        SequenceDeserializer { iter: iter.fuse() }
    }
}

impl<'de, 'a: 'de, I> SeqAccess<'de> for SequenceDeserializer<'a, I>
where
    I: Iterator<Item = Term<'a>>,
{
    type Error = Error;

    #[inline]
    fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            None => Ok(None),
            Some(term) => seed.deserialize(term).map(Some),
        }
    }
}

/// MapDeserializer
pub struct MapDeserializer<'a, I>
where
    I: Iterator,
{
    struct_name_term: Option<Term<'a>>,
    iter: iter::Fuse<I>,
    current_value: Option<Term<'a>>,
}

impl<'a, I> MapDeserializer<'a, I>
where
    I: Iterator,
{
    #[inline]
    fn new(iter: I, struct_name_term: Option<Term<'a>>) -> Self {
        MapDeserializer {
            struct_name_term,
            iter: iter.fuse(),
            current_value: None,
        }
    }
}

impl<'de, 'a: 'de, I> MapAccess<'de> for MapDeserializer<'a, I>
where
    I: Iterator<Item = (Term<'a>, Term<'a>)>,
{
    type Error = Error;

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.current_value.is_some() {
            panic!("MapDeserializer.next_key_seed was called twice in a row")
        }

        self.iter
            .next()
            .and_then(|pair| match pair {
                (key, _) if atom::__struct__().eq(&key) => self.iter.next(),
                pair => Some(pair),
            })
            .map_or(Ok(None), |pair| {
                let (key, value) = pair;
                self.current_value = Some(value);

                if self.struct_name_term.is_some() {
                    seed.deserialize(VariantNameDeserializer::from(key))
                        .map(Some)
                } else {
                    seed.deserialize(key).map(Some)
                }
            })
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.current_value {
            None => Err(Error::ExpectedStructValue),
            Some(value) => {
                self.current_value = None;
                seed.deserialize(value)
            }
        }
    }
}

/// EnumDeserializerType
pub enum EnumDeserializerType {
    #[allow(dead_code)]
    Any,
    Unit,
    Newtype,
    Tuple,
    Struct,
}

/// EnumDeserializer
pub struct EnumDeserializer<'a> {
    variant_type: EnumDeserializerType,
    variant_term: Term<'a>,
    variant: String,
    term: Option<Term<'a>>,
}

impl<'a> EnumDeserializer<'a> {
    #[inline]
    fn new(
        variant_type: EnumDeserializerType,
        variant_term: Term<'a>,
        variants: &'static [&'static str],
        term: Option<Term<'a>>,
    ) -> Result<Self, Error> {
        let var_de = VariantNameDeserializer::from(variant_term);
        let variant = String::deserialize(var_de).or(Err(Error::InvalidVariantName))?;

        match variant_type {
            EnumDeserializerType::Any => Ok(EnumDeserializer {
                variant_type,
                variant_term,
                variant,
                term,
            }),
            _ => {
                if variants.contains(&variant.as_str()) {
                    Ok(EnumDeserializer {
                        variant_type,
                        variant_term,
                        variant,
                        term,
                    })
                } else {
                    Err(Error::InvalidVariantName)
                }
            }
        }
    }
}

impl<'de, 'a: 'de> EnumAccess<'de> for EnumDeserializer<'a> {
    type Error = Error;
    type Variant = Self;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let var_de = VariantNameDeserializer::from(self.variant_term);
        let val = seed.deserialize(var_de)?;
        Ok((val, self))
    }
}

impl<'de, 'a: 'de> VariantAccess<'de> for EnumDeserializer<'a> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Error> {
        match self.variant_type {
            EnumDeserializerType::Any | EnumDeserializerType::Unit => Ok(()),
            _ => Err(Error::ExpectedUnitVariant),
        }
    }

    #[inline]
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.variant_type {
            EnumDeserializerType::Any | EnumDeserializerType::Newtype => {
                if let Some(term) = self.term {
                    let tuple = crate::serde::validate_tuple(term, Some(2))?;
                    seed.deserialize(tuple[1])
                } else {
                    Err(Error::ExpectedNewtypeVariant)
                }
            }
            _ => Err(Error::ExpectedNewtypeVariant),
        }
    }

    #[inline]
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.variant_type {
            EnumDeserializerType::Any | EnumDeserializerType::Tuple => {
                if let Some(term) = self.term {
                    let mut tuple = crate::serde::validate_tuple(term, Some(len + 1))?;
                    let iter = tuple.split_off(1).into_iter();
                    visitor.visit_seq(SequenceDeserializer::new(iter))
                } else {
                    Err(Error::ExpectedTupleVariant)
                }
            }
            _ => Err(Error::ExpectedTupleVariant),
        }
    }

    #[inline]
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.variant_type {
            EnumDeserializerType::Struct => {
                if let Some(term) = self.term {
                    crate::serde::validate_struct(&term, Some(&self.variant))?;
                    let iter = MapIterator::new(term).ok_or(Error::ExpectedStruct)?;
                    visitor.visit_map(MapDeserializer::new(iter, Some(self.variant_term)))
                } else {
                    Err(Error::ExpectedStructVariant)
                }
            }
            _ => Err(Error::ExpectedStructVariant),
        }
    }
}

/// Deserializer for atoms and map keys.
pub struct VariantNameDeserializer<'a> {
    variant: Term<'a>,
}

impl<'a> From<Term<'a>> for VariantNameDeserializer<'a> {
    fn from(variant: Term<'a>) -> Self {
        VariantNameDeserializer { variant }
    }
}

impl<'de, 'a: 'de> de::Deserializer<'de> for VariantNameDeserializer<'a> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.variant.get_type() {
            TermType::Atom => {
                let string =
                    atoms::term_to_string(&self.variant).or(Err(Error::InvalidVariantName))?;
                visitor.visit_string(string)
            }
            TermType::Binary => visitor.visit_string(crate::serde::term_to_str(&self.variant)?),
            TermType::Number => visitor.visit_string(crate::serde::term_to_str(&self.variant)?),
            _ => Err(Error::ExpectedStringable),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map struct enum identifier ignored_any
    }
}
