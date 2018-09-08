use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;

//extern crate bitreader;
//use bitreader::BitReader;

extern crate bytes;
use self::bytes::{Bytes, Buf};


pub type Name = String;

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum IntSize {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Signedness {
    Unsigned,
    Signed,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum FloatPrim {
    F32(Endianness),
    F64(Endianness),
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct IntPrim {
    pub size : IntSize,
    pub signedness : Signedness,
    pub endianness : Endianness,
}

impl IntPrim {
  pub fn new(size : IntSize,
             signedness : Signedness,
             endianness : Endianness) -> Self {
    
    IntPrim{ size : size,
             signedness : signedness,
             endianness : endianness
           }
  }
}

// NOTE bits could be allow to be any size.
// currently limited to 8/16/32/64 fields
#[derive(Eq, PartialEq, Debug, Hash)]
pub struct BitPrim {
    pub entries : Vec<(Name, u32, IntPrim)>,
    pub num_bytes : IntSize,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Enum {
    pub map : BTreeMap<i64, Name>,
    pub int_prim : IntPrim,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Prim {
    Int(IntPrim),
    Float(FloatPrim),
    //Bytes(usize),
    Enum(Enum),
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Item {
    pub name : Name,
    pub typ : Prim,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Layout {
    Prim(Item),
    Seq(Vec<Layout>),
    All(Vec<Layout>),
    // maybe Placement(u64, Layout)
    Bits(BitPrim),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Packet {
    Seq(Vec<Packet>),
    Subcom(HashMap<Vec<Item>, Packet>),
    Layout(Layout),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Protocol {
    Seq(Vec<Protocol>),
    Branch(Vec<(Vec<Prim>, Protocol)>),
    Layout(Layout),
    Packet(Packet),
}

pub type Loc = usize;

pub type LayoutMap = HashMap<Name, (Loc, Prim)>;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    //Bytes(&[u8]),
    Enum(Name, i64),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub name : Name,
    pub val : Value,
}

pub type ValueMap = HashMap<Name, Value>;


impl Value {
    // NOTE this would work better with an IntValue separate
    // from the Value type
    pub fn value(&self) -> i64 {
        match self {
            Value::U8(int)  =>   *int as i64,
            Value::U16(int) =>   *int as i64,
            Value::U32(int) =>   *int as i64,
            Value::U64(int) =>   *int as i64,
            Value::I8(int)  =>   *int as i64,
            Value::I16(int) =>   *int as i64,
            Value::I32(int) =>   *int as i64,
            Value::I64(int) =>   *int as i64,
            Value::F32(int) =>   panic!("Found an F32 in a value, expecting an int!"),
            Value::F64(int) =>   panic!("Found an F64 in a value, expecting an int!"),
            //Value::Bytes(_) =>   panic!("Found an Bytes in a value, expecting an int!"),
            Value::Enum(_, _) => panic!("Found an Enum in a value, expecting an int!"),
        }
    }
}
