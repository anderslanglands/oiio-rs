#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum BaseType {
    UNKNOWN,
    NONE,
    UINT8,
    INT8,
    UINT16,
    INT16,
    UINT32,
    INT32,
    UINT64,
    INT64,
    HALF,
    FLOAT,
    DOUBLE,
    STRING,
    PTR,
    LASTBASE,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Aggregate {
    SCALAR = 1,
    VEC2 = 2,
    VEC3 = 3,
    VEC4 = 4,
    MATRIX33 = 9,
    MATRIX44 = 16,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum VecSemantics {
    NOSEMANTICS = 0,
    COLOR,
    POINT,
    VECTOR,
    NORMAL,
    TIMECODE,
    KEYCODE,
    RATIONAL,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TypeDesc {
    pub basetype: BaseType,
    pub aggregate: Aggregate,
    pub vecsemantics: VecSemantics,
    pub reserved: u8,
    pub arraylen: i32,
}

pub const UINT8: TypeDesc = TypeDesc {
    basetype: BaseType::UINT8,
    aggregate: Aggregate::SCALAR,
    vecsemantics: VecSemantics::NOSEMANTICS,
    reserved: 0,
    arraylen: 0,
};

pub const UINT16: TypeDesc = TypeDesc {
    basetype: BaseType::UINT16,
    aggregate: Aggregate::SCALAR,
    vecsemantics: VecSemantics::NOSEMANTICS,
    reserved: 0,
    arraylen: 0,
};

pub const HALF: TypeDesc = TypeDesc {
    basetype: BaseType::HALF,
    aggregate: Aggregate::SCALAR,
    vecsemantics: VecSemantics::NOSEMANTICS,
    reserved: 0,
    arraylen: 0,
};

pub const FLOAT: TypeDesc = TypeDesc {
    basetype: BaseType::FLOAT,
    aggregate: Aggregate::SCALAR,
    vecsemantics: VecSemantics::NOSEMANTICS,
    reserved: 0,
    arraylen: 0,
};
