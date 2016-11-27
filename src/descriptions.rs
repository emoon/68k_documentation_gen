pub enum Flag {
    Set(&'static str),
    Clear(&'static str),
    NotAffected(&'static str),
    Undefined,
}

pub struct FlagsDesc {
    pub x: Flag,
    pub n: Flag,
    pub z: Flag,
    pub v: Flag,
    pub c: Flag,
}

const FLAGS_ARC: FlagsDesc = FlagsDesc {
    x: Flag::Set("X - Set the same as carry"),
    n: Flag::Set("N - Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Set("V — Set if an overflow is generated; cleared otherwise."),
    c: Flag::Set("C — Set if a carry is generated; cleared otherwise."),
};

const FLAGS_X: FlagsDesc = FlagsDesc {
    x: Flag::Set("X - Set the same as carry"),
    n: Flag::Set("N - Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Cleared if the result is non-zero; unchanged otherwise."),
    v: Flag::Set("V — Set if an overflow is generated; cleared otherwise."),
    c: Flag::Set("C — Set if a carry is generated; cleared otherwise."),
};

const FLAGS_ABCD: FlagsDesc = FlagsDesc {
    x: Flag::Set("X - Set the same as carry"),
    n: Flag::Undefined,
    z: Flag::Clear("Z — Cleared if the result is nonzero; unchanged otherwise"),
    v: Flag::Undefined,
    c: Flag::Set("C — Set if a decimal carry was generated; cleared otherwise."),
};

const FLAGS_AND: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X - Not Affected"),
    n: Flag::Set("N — Set if the most significant bit of the result is set; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Clear("V — Always cleared."),
    c: Flag::Clear("C — Always cleared."),
};

pub struct Description {
    pub description: &'static str,
    pub operation: &'static str,
    pub assembler: &'static [&'static str],
    pub attributes: &'static str,
    pub flags: &'static FlagsDesc,
}

pub const ABCD_DESC: Description = Description {
    description: "Adds the source operand to the destination operand along with the extend bit, and stores the result in the destination location. The addition is performed using binary- coded decimal arithmetic. The operands, which are packed binary-coded decimal numbers, can be addressed in two different ways\n
        1. Data Register to Data Register: The operands are contained in the data registers specified in the instruction.\n
        2. Memory to Memory: The operands are addressed with the predecrement addressing mode using the address registers specified in the instruction.\n
        This operation is a byte operation only.",
    operation: "Source10 + Destination10 + X → Destination",
    assembler: &["abcd < ea > ,Dn", "Add Dn, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_ABCD,
};

pub const ADD_DESC: Description = Description {
    description: "Adds the source operand to the destination operand using binary addition and stores the result in the destination location. The size of the operation may be specified as byte, word, or long. The mode of the instruction indicates which operand is the source and which is the destination, as well as the operand size.",
    operation: "Source + Destination → Destination",
    assembler: &["Add < ea > ,Dn", "Add Dn, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_ARC,
};

pub const ADDQ_DESC: Description = Description {
    description: "Adds an immediate value of one to eight to the operand at the destination location. The size of the operation may be specified as byte, word, or long. Word and long operations are also allowed on the address registers. When adding to address registers, the condition codes are not altered, and the entire destination address register is used regardless of the operation size.",
    operation: "Immidate + Destination → Destination",
    assembler: &["addq # < data > , < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_ARC,
};

pub const ADDX_DESC: Description = Description {
    description: "Adds the source operand and the extend bit to the destination operand and stores the result in the destination location. The operands can be addressed in two different ways:
                    1. Data register to data register—The data registers specified in the instruction contain the operands.
                    2. Memory to memory—The address registers specified in the instruction address the operands using the predecrement addressing mode.
                    The size of the operation can be specified as byte, word, or long.";
    operation: "Source + Destination + X → Destination",
    assembler: &["addx Dy,Dx", "addx -(Ay),-(Ax)"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_X,
};

pub const AND_DESC: Description = Description {
    description: "Performs an AND operation of the source operand with the destination operand and stores the result in the destination location. The size of the operation can be specified as byte, word, or long. The contents of an address register may not be used as an operand.",
    operation: "Source & Destination → Destination",
    assembler: &["and < ea > ,Dn", "Add Dn, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_AND,
};


