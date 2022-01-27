use rustler::types::truthy::Truthy;
use rustler::{NifException, NifMap, NifRecord, NifStruct, NifTuple, NifUnitEnum, NifUntaggedEnum};

#[derive(NifTuple)]
pub struct AddTuple {
    lhs: i32,
    rhs: i32,
}

#[rustler::nif]
pub fn tuple_echo(tuple: AddTuple) -> AddTuple {
    tuple
}

#[derive(NifTuple)]
pub struct GenericTuple<T> {
    t1: T,
    t2: T,
}

#[rustler::nif]
pub fn generic_tuple_echo_usize(generic_tuple: GenericTuple<usize>) -> GenericTuple<usize> {
    generic_tuple
}

#[rustler::nif]
pub fn generic_tuple_echo_str(generic_tuple: GenericTuple<&str>) -> GenericTuple<&str> {
    generic_tuple
}

#[derive(NifTuple)]
pub struct GenericTuple2<T, U, V> {
    t: T,
    u: U,
    v: V,
}

#[rustler::nif]
pub fn generic_tuple2_echo(
    generic_tuple: GenericTuple2<usize, i64, &str>,
) -> GenericTuple2<usize, i64, &str> {
    generic_tuple
}

#[derive(NifRecord)]
#[rustler(encode, decode)] // Added to check encode/decode attribute, #180
#[must_use] // Added to check attribute order (see similar issue #152)
#[tag = "record"]
pub struct AddRecord {
    lhs: i32,
    rhs: i32,
}

#[rustler::nif]
pub fn record_echo(record: AddRecord) -> AddRecord {
    record
}

#[derive(NifRecord)]
#[tag = "generic_record"]
pub struct GenericRecord<T, U> {
    field1: T,
    field2: U,
}

#[rustler::nif]
pub fn generic_record_echo(record: GenericRecord<i32, &str>) -> GenericRecord<i32, &str> {
    record
}

#[derive(NifMap)]
pub struct AddMap {
    lhs: i32,
    rhs: i32,
}

#[rustler::nif]
pub fn map_echo(map: AddMap) -> AddMap {
    map
}

#[derive(NifMap)]
pub struct GenericMap<T, U> {
    lhs: T,
    rhs: U,
}

#[rustler::nif]
pub fn generic_map_echo(map: GenericMap<i32, &str>) -> GenericMap<i32, &str> {
    map
}

#[derive(Debug, NifStruct)]
#[must_use] // Added to test Issue #152
#[module = "AddStruct"]
pub struct AddStruct {
    lhs: i32,
    rhs: i32,
}

#[rustler::nif]
pub fn struct_echo(add_struct: AddStruct) -> AddStruct {
    add_struct
}

#[derive(Debug, NifStruct)]
#[module = "GenericStruct"]
pub struct GenericStruct<T, U> {
    lhs: T,
    rhs: U,
}

#[rustler::nif]
pub fn generic_struct_echo(generic_struct: GenericStruct<i32, &str>) -> GenericStruct<i32, &str> {
    generic_struct
}

#[derive(Debug, NifException)]
#[module = "AddException"]
pub struct AddException {
    message: String,
}

#[rustler::nif]
pub fn exception_echo(add_exception: AddException) -> AddException {
    add_exception
}

#[derive(Debug, NifException)]
#[module = "GenericException"]
pub struct GenericException<T> {
    message: String,
    t: T,
}

#[rustler::nif]
pub fn generic_exception_echo(generic_exception: GenericException<i32>) -> GenericException<i32> {
    generic_exception
}

#[derive(NifUnitEnum)]
pub enum UnitEnum {
    FooBar,
    Baz,
}

#[rustler::nif]
pub fn unit_enum_echo(unit_enum: UnitEnum) -> UnitEnum {
    unit_enum
}

#[derive(NifUntaggedEnum)]
pub enum UntaggedEnum {
    Foo(u32),
    Bar(String),
    Baz(AddStruct),
    Bool(bool),
}

#[rustler::nif]
pub fn untagged_enum_echo(untagged_enum: UntaggedEnum) -> UntaggedEnum {
    untagged_enum
}

#[derive(NifUntaggedEnum)]
pub enum GenericUntaggedEnum<T, U> {
    Foo(u32),
    Bar(T),
    Baz(U),
    Bool(bool),
}

#[rustler::nif]
pub fn generic_untagged_enum_echo(
    generic_untagged_enum: GenericUntaggedEnum<GenericMap<i32, i32>, &str>,
) -> GenericUntaggedEnum<GenericMap<i32, i32>, &str> {
    generic_untagged_enum
}

#[derive(NifUntaggedEnum)]
pub enum UntaggedEnumWithTruthy {
    Baz(AddStruct),
    Truthy(Truthy),
}

#[rustler::nif]
pub fn untagged_enum_with_truthy(untagged_enum: UntaggedEnumWithTruthy) -> UntaggedEnumWithTruthy {
    untagged_enum
}

#[derive(NifUntaggedEnum)]
pub enum UntaggedEnumForIssue370 {
    Vec(Vec<i64>),
}

#[rustler::nif]
pub fn untagged_enum_for_issue_370(
    untagged_enum: UntaggedEnumForIssue370,
) -> UntaggedEnumForIssue370 {
    untagged_enum
}

#[derive(NifTuple)]
pub struct Newtype(i64);

#[rustler::nif]
pub fn newtype_echo(newtype: Newtype) -> Newtype {
    newtype
}

#[derive(NifTuple)]
pub struct TupleStruct(i64, i64, i64);

#[rustler::nif]
pub fn tuplestruct_echo(tuplestruct: TupleStruct) -> TupleStruct {
    tuplestruct
}

#[derive(NifRecord)]
#[tag = "newtype"]
pub struct NewtypeRecord(i64);

#[rustler::nif]
pub fn newtype_record_echo(newtype: NewtypeRecord) -> NewtypeRecord {
    newtype
}

#[derive(NifRecord)]
#[tag = "tuplestruct"]
pub struct TupleStructRecord(i64, i64, i64);

#[rustler::nif]
pub fn tuplestruct_record_echo(tuplestruct: TupleStructRecord) -> TupleStructRecord {
    tuplestruct
}

pub mod reserved_keywords {
    use rustler::{NifMap, NifRecord, NifStruct, NifTuple, NifUntaggedEnum};

    #[derive(NifMap, Debug)]
    pub struct Map {
        r#override: i32,
    }

    #[derive(NifStruct, Debug)]
    #[module = "Struct"]
    pub struct Struct {
        r#override: i32,
    }

    #[derive(NifTuple, Debug)]
    pub struct Tuple {
        r#override: i32,
    }

    #[derive(NifRecord, Debug)]
    #[tag = "record"]
    pub struct Record {
        r#override: i32,
    }

    #[derive(NifUntaggedEnum)]
    pub enum ReservedKeywords {
        Struct(Struct),
        Map(Map),
        Tuple(Tuple),
        Record(Record),
    }

    #[rustler::nif]
    pub fn reserved_keywords_type_echo(reserved: ReservedKeywords) -> ReservedKeywords {
        reserved
    }
}
