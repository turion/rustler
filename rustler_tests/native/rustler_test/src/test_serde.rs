use rustler::types::atom;
use rustler::{Encoder, Env, Error, NifResult, Term};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UnitVariant {
    #[serde(rename = "UnitVariant::A")]
    A,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NewtypeVariant {
    #[serde(rename = "Elixir.RustlerTest.SerdeTest.NewtypeVariant.N")]
    N(u8),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.RustlerTest.SerdeTest.NewtypeStruct")]
pub struct NewtypeStruct(pub u8);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.RustlerTest.SerdeTest.TupleStruct")]
pub struct TupleStruct(pub u8, pub u8, pub u8);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TupleVariant {
    #[serde(rename = "Elixir.RustlerTest.SerdeTest.TupleVariant.T")]
    T(u8, u8),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.RustlerTest.SerdeTest.Struct")]
pub struct Struct {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StructVariant {
    #[serde(rename = "Elixir.RustlerTest.SerdeTest.StructVariant.S")]
    S { r: u8, g: u8, b: u8 },
}

#[rustler::nif]
pub fn transcode<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    match serde_transcode::transcode(term, env) {
        Ok(term) => Ok((atom::ok(), term).encode(env)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn serde_test<'a>(
    env: Env<'a>,
    typ: &str,
    name: &str,
    term: Term<'a>,
) -> Result<Term<'a>, Error> {
    macro_rules! run_test {
        ($actual:expr) => {
            run_test(env, typ, $actual, term)
        };
    }

    match name {
        "none" => run_test!(None as Option<u8>),
        "some" => run_test!(Some(100)),
        "true" => run_test!(true),
        "false" => run_test!(false),

        // Signed Integers
        "i8 (min)" => run_test!(i8::min_value()),
        "i8 (0)" => run_test!(0 as i8),
        "i8 (max)" => run_test!(i8::max_value()),
        "i16 (min)" => run_test!(i16::min_value()),
        "i16 (0)" => run_test!(0 as i16),
        "i16 (max)" => run_test!(i16::max_value()),
        "i32 (min)" => run_test!(i32::min_value()),
        "i32 (0)" => run_test!(0 as i32),
        "i32 (max)" => run_test!(i32::max_value()),
        "i64 (min)" => run_test!(i64::min_value()),
        "i64 (0)" => run_test!(0 as i64),
        "i64 (max)" => run_test!(i64::max_value()),
        "i128 (min)" => run_test!(i128::min_value()),
        "i128 (0)" => run_test!(0 as i128),
        "i128 (max)" => run_test!(i128::max_value()),

        // Unsigned Integers
        "u8 (min)" => run_test!(u8::min_value()),
        "u8 (max)" => run_test!(u8::max_value()),
        "u16 (min)" => run_test!(u16::min_value()),
        "u16 (max)" => run_test!(u16::max_value()),
        "u32 (min)" => run_test!(u32::min_value()),
        "u32 (max)" => run_test!(u32::max_value()),
        "u64 (min)" => run_test!(u64::min_value()),
        "u64 (max)" => run_test!(u64::max_value()),
        "u128 (min)" => run_test!(u128::min_value()),
        "u128 (max)" => run_test!(u128::max_value()),

        // Float32
        "f32 (0)" => run_test!(f32::from_bits(0x0000_0000)),
        "f32 (-0)" => run_test!(f32::from_bits(0x8000_0000)),
        "f32 (one)" => run_test!(f32::from_bits(0x3f80_0000)),
        "f32 (smallest subnormal)" => run_test!(f32::from_bits(0x0000_0001)),
        "f32 (largest subnormal)" => run_test!(f32::from_bits(0x007f_ffff)),
        "f32 (smallest normal)" => run_test!(f32::from_bits(0x0080_0000)),
        "f32 (largest normal)" => run_test!(f32::from_bits(0x7f7f_ffff)),
        "f32 (smallest number < 1)" => run_test!(f32::from_bits(0x3f80_0001)),
        "f32 (largest number < 1)" => run_test!(f32::from_bits(0x3f7f_ffff)),
        // "f32 (infinity)" => run_test!(f32::from_bits(0x7f800000)),
        // "f32 (-infinity)" => run_test!(f32::from_bits(0xff800000)),

        // Float64
        "f64 (0)" => run_test!(f64::from_bits(0x0000_0000_0000_0000)),
        "f64 (-0)" => run_test!(f64::from_bits(0x8000_0000_0000_0000)),
        "f64 (one)" => run_test!(f64::from_bits(0x3f80_0000_0000_0000)),
        "f64 (smallest subnormal)" => run_test!(f64::from_bits(0x0000_0000_0000_0001)),
        "f64 (largest subnormal)" => run_test!(f64::from_bits(0x007f_ffff_ffff_ffff)),
        "f64 (smallest normal)" => run_test!(f64::from_bits(0x0080_0000_0000_0000)),
        "f64 (largest normal)" => run_test!(f64::from_bits(0x7f7f_ffff_ffff_ffff)),
        "f64 (smallest number < 1)" => run_test!(f64::from_bits(0x3f80_0000_0000_0001)),
        "f64 (largest number < 1)" => run_test!(f64::from_bits(0x3f7f_ffff_ffff_ffff)),
        // "f64 (infinity)" => run_test!(f64::from_bits(0x7f80000000000000)),
        // "f64 (-infinity)" => run_test!(f64::from_bits(0xff80000000000000)),

        // Chars, Strings and Binaries
        "char (ascii)" => run_test!(std::char::from_u32(65)),
        "char (replacement)" => run_test!(std::char::from_u32(65533)),
        "str (empty)" => run_test!(""),
        "str" => run_test!("hello world"),
        "bytes" => run_test!(Bytes::new(&[3, 2, 1, 0])),

        // Unit Types
        "unit" => run_test!(()),
        "unit struct" => run_test!(Unit {}),
        "unit variant" => run_test!(UnitVariant::A),

        // Newtype Types
        "newtype struct" => run_test!(NewtypeStruct(u8::max_value())),
        "newtype variant" => run_test!(NewtypeVariant::N(u8::max_value())),
        "newtype variant (ok tuple)" => {
            let ok: Result<u8, String> = Ok(u8::max_value());
            run_test!(ok)
        }
        "newtype variant (error tuple)" => {
            let err: Result<u8, String> = Err(String::from("error reason"));
            run_test!(err)
        }

        // Sequences
        "sequences (empty)" => run_test!(Vec::new() as Vec<u8>),
        "sequences (primitive)" => run_test!(vec!["hello", "world"]),
        "sequences (complex)" => {
            let a = NewtypeStruct(u8::min_value());
            let b = NewtypeStruct(u8::max_value());
            run_test!(vec![a, b])
        }

        // Tuple Types
        "tuple (empty)" => run_test!(()), // same as unit
        "tuple" => run_test!((0, 255)),
        "tuple struct" => run_test!(TupleStruct(0, 128, 255)),
        "tuple variant" => run_test!(TupleVariant::T(0, 255)),

        // Map and Struct Types
        "map (primitive)" => {
            let mut map = HashMap::new();
            map.insert("key", "hello");
            map.insert("val", "world");

            run_test!(map)
        }
        "map (complex)" => {
            let mut map = HashMap::new();
            map.insert("key", Struct { r: 0, g: 0, b: 0 });
            map.insert(
                "val",
                Struct {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            );

            run_test!(map)
        }
        "struct" => run_test!(Struct {
            r: 0,
            g: 128,
            b: 255
        }),
        "struct variant" => run_test!(StructVariant::S {
            r: 0,
            g: 128,
            b: 255
        }),
        _ => Ok(error_tuple(env, "nonexistant test".encode(env))),
    }
}

enum TestResult<'a> {
    Ok,
    Err(Term<'a>),
}

fn run_test<'a, T>(
    env: Env<'a>,
    typ: &str,
    actual: T,
    expected: Term<'a>,
) -> Result<Term<'a>, Error>
where
    T: Debug + PartialEq + Serialize + Deserialize<'a>,
{
    let res = match typ {
        "serialize" => run_ser_test(env, &actual, expected),
        "deserialize" => run_de_test(env, &actual, expected),
        _ => TestResult::Err(error_tuple(env, "nonexistant test".encode(env))),
    };

    match res {
        TestResult::Ok => Ok(atom::ok().encode(env)),
        TestResult::Err(term) => Ok(term),
    }
}

fn run_ser_test<'a, T>(env: Env<'a>, actual: &T, expected: Term<'a>) -> TestResult<'a>
where
    T: PartialEq + Serialize,
{
    match rustler::to_term(env, actual) {
        Ok(actual) => {
            if expected.eq(&actual) {
                TestResult::Ok
            } else {
                TestResult::Err(error_tuple(env, actual))
            }
        }
        Err(reason) => {
            let reason_term = reason.to_string().encode(env);
            TestResult::Err(error_tuple(env, reason_term))
        }
    }
}

fn run_de_test<'a, T>(env: Env<'a>, actual: &T, expected: Term<'a>) -> TestResult<'a>
where
    T: Debug + PartialEq + Deserialize<'a>,
{
    match rustler::from_term(expected) {
        Ok(expected) => {
            if actual.eq(&expected) {
                TestResult::Ok
            } else {
                TestResult::Err(error_tuple(env, format!("{:?}", actual).encode(env)))
            }
        }
        Err(reason) => {
            let reason_term = reason.to_string().encode(env);
            TestResult::Err(error_tuple(env, reason_term))
        }
    }
}

fn error_tuple<'a>(env: Env<'a>, term: Term<'a>) -> Term<'a> {
    (atom::error(), term).encode(env)
}
