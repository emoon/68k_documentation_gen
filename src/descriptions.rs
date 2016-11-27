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

const FLAGS_SHIFT: FlagsDesc = FlagsDesc {
    x: Flag::Set("X — Set according to the last bit shifted out of the operand; unaffected for a shift count of zero."),
    n: Flag::Set("N — Set if the most significant bit of the result is set; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Set("V — Set if the most significant bit is changed at any time during the shift operation; cleared otherwise."),
    c: Flag::Set("C — Set according to the last bit shifted out of the operand; cleared for a shift count of zero."),
};

const FLAGS_BINST: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X — Not Affected."),
    n: Flag::NotAffected("N — Not Affected."),
    z: Flag::Set("Z — Set if the bit tested is zero; cleared otherwise."),
    v: Flag::NotAffected("V — Not Affected."),
    c: Flag::NotAffected("C — Not Affected."),
};

const FLAGS_NOT_AFFECTED: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X — Not Affected."),
    n: Flag::NotAffected("N — Not Affected."),
    z: Flag::NotAffected("Z — Not Affected."),
    v: Flag::NotAffected("V — Not Affected."),
    c: Flag::NotAffected("C — Not Affected."),
};

const FLAGS_CLR: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X — Not Affected."),
    n: Flag::Clear("N — Always cleared."),
    z: Flag::Set("Z — Always set."),
    v: Flag::Clear("V — Always cleared."),
    c: Flag::Clear("C — Always cleared."),
};

const FLAGS_CMP: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X — Not Affected."),
    n: Flag::Set("N — Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Set("V — Set if an overflow occurs; cleared otherwise."),
    c: Flag::Set("C — Set if a borrow occurs; cleared otherwise."),
};

const FLAGS_DIV: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X — Not Affected."),
    n: Flag::Set("N — Set if the quotient is negative; cleared otherwise; undefined if overflow or divide by zero occurs."),
    z: Flag::Set("Z — Set if the quotient is zero; cleared otherwise; undefined if overflow or divide by zero occurs."),
    v: Flag::Set("V — Set if division overflow occurs; undefined if divide by zero occurs; cleared otherwise."),
    c: Flag::Set("C — Always cleared."),
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
                    The size of the operation can be specified as byte, word, or long.",
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

pub const ASL_ASR_DESC: Description = Description {
    description: "Arithmetically shifts the bits of the operand in the direction (L or R) specified. The carry bit receives the last bit shifted out of the operand. The shift count for the shifting of a register may be specified in two different ways:
                    1. Immediate—The shift count is specified in the instruction (shift range, 1 – 8).
                    2. Register—The shift count is the value in the data register specified in instruction modulo 64.
                    The size of the operation can be specified as byte, word, or long. An operand in mem- ory can be shifted one bit only, and the operand size is restricted to a word.
                    For ASL, the operand is shifted left; the number of positions shifted is the shift count. Bits shifted out of the high-order bit go to both the carry and the extend bits; zeros are shifted into the low-order bit. The overflow bit indicates if any sign changes occur dur- ing the shift.",
    operation: "Destination Shifted By Count → Destination",
    assembler: &["ASd Dx,Dy", "ASd # < data > ,Dy", "ASd < ea >", "where d is direction, L or R"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_SHIFT,
};

pub const BCC_DESC: Description = Description {
    description: "If the specified condition is true, program execution continues at location (PC) + displacement.
                  The program counter contains the address of the instruction word for the Bcc instruction plus two.
                  The displacement is a twos-complement integer that represents the relative distance in bytes from the current program counter to the destination program counter.
                  If the 8-bit displacement field in the instruction word is zero, a 16-bit displacement (the word immediately following the instruction) is used.
                  If the 8-bit displacement field in the instruction word is all ones ($FF), the 32-bit displacement (long word immediately following the instruction) is used.
                  Condition code cc specifies one of the following conditional tests.",
    operation: "If Condition True Then PC + dn → PC",
    assembler: &["bcc < label >"],
    attributes: "Byte, Word",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const BCHG_DESC: Description = Description {
    description: "Tests a bit in the destination operand and sets the Z condition code appropriately, then inverts the specified bit in the destination. When the destination is a data register, any of the 32 bits can be specified by the modulo 32-bit number. When the destination is a memory location, the operation is a byte operation, and the bit number is modulo 8. In all cases, bit zero refers to the least significant bit. The bit number for this operation may be specified in either of two ways:
                    1. Immediate—The bit number is specified in a second word of the instruction.
                    2. Register—The specified data register contains the bit number.",
    operation: "TEST ( < number > of Destination) → Z;
                TEST ( < number > of Destination) → < bit number > of Destination",
    assembler: &["bchg dn, < ea >", "bchg # < data > , < ea >"],
    attributes: "Byte, Long",
    flags: &FLAGS_BINST,
};

pub const BCLR_DESC: Description = Description {
    description: "Tests a bit in the destination operand and sets the Z condition code appropriately, then clears the specified bit in the destination. When a data register is the destination, any of the 32 bits can be specified by a modulo 32-bit number. When a memory location is the destination, the operation is a byte operation, and the bit number is modulo 8. In all cases, bit zero refers to the least significant bit. The bit number for this operation can be specified in either of two ways:
                    1. Immediate—The bit number is specified in a second word of the instruction.
                    2. Register—The specified data register contains the bit number.",
    operation: "TEST ( < bit number > of Destination) → Z; 0 → < bit number > of Destination",
    assembler: &["bclr dn, < ea >", "bclr # < data > , < ea >"],
    attributes: "Byte, Long",
    flags: &FLAGS_BINST,
};

pub const BRA_DESC: Description = Description {
    description: "Program execution continues at location (PC) + displacement. The program counter contains the address of the instruction word of the BRA instruction plus two. The displacement is a twos complement integer that represents the relative distance in bytes from the current program counter to the destination program counter. If the 8-bit displacement field in the instruction word is zero, a 16-bit displacement (the word immediately following the instruction) is used. If the 8-bit displacement field in the instruction word is all ones ($FF), the 32-bit displacement (long word immediately following the instruction) is used.",
    operation: "PC + dn → PC",
    assembler: &["bra < label >"],
    attributes: "Byte, Long",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const BSET_DESC: Description = Description {
    description: "Description: Tests a bit in the destination operand and sets the Z condition code appropriately, then sets the specified bit in the destination operand. When a data register is the destination, any of the 32 bits can be specified by a modulo 32-bit number. When a memory location is the destination, the operation is a byte operation, and the bit number is modulo 8. In all cases, bit zero refers to the least significant bit. The bit number for this operation can be specified in either of two ways:
                    1. Immediate—The bit number is specified in the second word of the instruction.
                    2. Register—The specified data register contains the bit number.",
    operation: "TEST ( < bit number > of Destination) → Z; 1 → < bit number > of Destination",
    assembler: &["btest dn, < ea >", "btest # < data > , < ea >"],
    attributes: "Byte, Long",
    flags: &FLAGS_BINST,
};

pub const BSR_DESC: Description = Description {
    description: "Pushes the long-word address of the instruction immediately following the BSR instruction onto the system stack. The program counter contains the address of the instruction word plus two. Program execution then continues at location (PC) + displacement. The displacement is a twos complement integer that represents the relative distance in bytes from the current program counter to the destination program counter. If the 8-bit displacement field in the instruction word is zero, a 16-bit displacement (the word immediately following the instruction) is used. If the 8-bit displacement field in the instruction word is all ones ($FF), the 32-bit displacement (long word immediately following the instruction) is used.",
    operation: "SP – 4 → SP; PC → (SP); PC + dn → PC",
    assembler: &["bsr < label >"],
    attributes: "Byte, Word",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const BTST_DESC: Description = Description {
    description: "Tests a bit in the destination operand and sets the Z condition code appropriately. When a data register is the destination, any of the 32 bits can be specified by a modulo 32- bit number. When a memory location is the destination, the operation is a byte operation, and the bit number is modulo 8. In all cases, bit zero refers to the least significant bit. The bit number for this operation can be specified in either of two ways:
                 1. Immediate—The bit number is specified in a second word of the instruction.
                 2. Register—The specified data register contains the bit number.",
    operation: "TEST ( < bit number > of Destination) → Z",
    assembler: &["btest dn, < ea >", "btest # < data > , < ea >"],
    attributes: "Byte, Long",
    flags: &FLAGS_BINST,
};

pub const CLR_DESC: Description = Description {
    description: "Clears the destination operand to zero. The size of the operation may be specified as byte, word, or long.",
    operation: " 0 → Destination",
    assembler: &["clr < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_CLR,
};

pub const CMP_DESC: Description = Description {
    description: "Subtracts the source operand from the destination data register and sets the condition codes according to the result; the data register is not changed. The size of the operation can be byte, word, or long.",
    operation: "Destination – Source → cc",
    assembler: &["cmp < ea > , Dn"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_CMP,
};

pub const CMPM_DESC: Description = Description {
    description: "Subtracts the source operand from the destination operand and sets the condition codes according to the results; the destination location is not changed. The operands are always addressed with the postincrement addressing mode, using the address registers specified in the instruction. The size of the operation may be specified as byte, word, or long.",
    operation: "Destination – Source → cc",
    assembler: &["cmpm (Ay) + ,(Ax) +"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_CMP,
};

pub const DBCC_DESC: Description = Description {
    description: "Description: Controls a loop of instructions. The parameters are a condition code, a data register (counter), and a displacement value. The instruction first tests the condition for termination; if it is true, no operation is performed. If the termination condition is not true, the low-order 16 bits of the counter data register decrement by one. If the result is – 1, execution continues with the next instruction. If the result is not equal to – 1, execution continues at the location indicated by the current value of the program counter plus the sign-extended 16-bit displacement. The value in the program counter is the address of the instruction word of the DBcc instruction plus two. The displacement is a twos complement integer that represents the relative distance in bytes from the current program counter to the destination program counter. Condition code cc specifies one of the following conditional tests:",
    operation: " If Condition False
                    Then (Dn – 1 → Dn; If Dn not equal – 1 Then PC + dn → PC)",
    assembler: &["dbcc dn, < label >"],
    attributes: "Word",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const DIVS_DIVU_DESC: Description = Description {
    description: "Divides the signed destination operand by the signed source operand and stores the signed result in the destination. The result is a quotient in the lower word (least significant 16 bits) and a remainder in the upper word (most significant 16 bits). The sign of the remainder is the same as the sign of the dividend.
                    Two special conditions may arise during the operation:
                    1. Division by zero causes a trap.
                    2. Overflow may be detected and set before the instruction completes. If the instruction detects an overflow, it sets the overflow condition code, and the operands are unaffected.",
    operation: "Destination / Source → Destination",
    assembler: &["DIVS.W < ea > ,Dn32/16 → 16r – 16q"],
    attributes: "Word",
    flags: &FLAGS_DIV,
};



