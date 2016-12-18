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

const FLAGS_EXT: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X - Not Affected"),
    n: Flag::Set("N — Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Clear("V — Always cleared."),
    c: Flag::Clear("C — Always cleared."),
};

const FLAGS_MUL: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X - Not Affected"),
    n: Flag::Set("N — Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Set("V — Set if overflow; cleared otherwise."),
    c: Flag::Clear("C — Always cleared."),
};

const FLAGS_NEG: FlagsDesc = FlagsDesc {
    x: Flag::Set("X — Set the same as the carry bit."),
    n: Flag::Set("N — Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Set("V — Set if an overflow occurs; cleared otherwise."),
    c: Flag::Set("C — Cleared if the result is zero; set otherwise."),
};

const FLAGS_NEGX: FlagsDesc = FlagsDesc {
    x: Flag::Set("X — Set the same as the carry bit."),
    n: Flag::Set("N — Set if the result is negative; cleared otherwise."),
    z: Flag::Clear("Z — Cleared if the result is nonzero; unchanged otherwise."),
    v: Flag::Set("V — Set if an overflow occurs; cleared otherwise."),
    c: Flag::Set("C — Set if a borrow occurs; cleared otherwise."),
};

const FLAGS_ROL: FlagsDesc = FlagsDesc {
    x: Flag::Set("X — Not affected."),
    n: Flag::Set("N — Set if the most significant bit of the result is set; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Clear("V - Always cleared."),
    c: Flag::Set("C — Set according to the last bit rotated out of the operand; cleared when the rotate count is zero."),
};

const FLAGS_SUB: FlagsDesc = FlagsDesc {
    x: Flag::Set("X - Set the same as carry"),
    n: Flag::Set("N - Set if the result is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the result is zero; cleared otherwise."),
    v: Flag::Set("V — Set if an overflow is generated; cleared otherwise."),
    c: Flag::Set("C — Set if a borrow is generated; cleared otherwise."),
};

const FLAGS_SWAP: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X - Not affected"),
    n: Flag::Set("Set if the most significant bit of the 32-bit result is set; cleared otherwise."),
    z: Flag::Set("Set if the 32-bit result is zero; cleared otherwise."),
    v: Flag::Clear("V — Always cleared."),
    c: Flag::Clear("C — Always cleared."),
};

const FLAGS_TAS: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X - Not affected"),
    n: Flag::Set("N - Set if the most significant bit of the operand is currently set; cleared otherwise."),
    z: Flag::Set("Z - Set if the 32-bit result is zero; cleared otherwise."),
    v: Flag::Clear("V — Always cleared."),
    c: Flag::Clear("C — Always cleared."),
};

const FLAGS_TST: FlagsDesc = FlagsDesc {
    x: Flag::NotAffected("X - Not affected"),
    n: Flag::Set("N — Set if the operand is negative; cleared otherwise."),
    z: Flag::Set("Z — Set if the operand is zero; cleared otherwise."),
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

pub const EOR_DESC: Description = Description {
    description: "Performs an exclusive-OR operation on the destination operand using the source operand and stores the result in the destination location. The size of the operation may be specified to be byte, word, or long. The source operand must be a data register. The destination operand is specified in the effective address field.",
    operation: "Destination EOR Source → Destination",
    assembler: &["eor Dn, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_AND,
};

pub const EXG_DESC: Description = Description {
    description: "Exchanges the contents of two 32-bit registers. The instruction performs three types of exchanges.
					1. Exchange data registers.
					2. Exchange address registers.
					3. Exchange a data register and an address register.",
    operation: "Rx ←→ Ry",
    assembler: &["exg Dx,Dy", "exg Ax,Ay", "exg Dx,Ay"],
    attributes: "Long",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const EXT_DESC: Description = Description {
    description: " Extends a byte in a data register to a word or a long word, or a word in a data register to a long word, by replicating the sign bit to the left. If the operation extends a byte to a word, bit 7 of the designated data register is copied to bits 15 – 8 of that data register. If the operation extends a word to a long word, bit 15 of the designated data register is copied to bits 31 – 16 of the data register.",
    operation: "Destination Sign-Extended → Destination",
    assembler: &["ext.w Dn - extend byte to word", "ext.l Dn - extend word to long"],
    attributes: "Word, Long",
    flags: &FLAGS_EXT,
};

pub const JMP_DESC: Description = Description {
    description: "Program execution continues at the effective address specified by the instruction. The addressing mode for the effective address must be a control addressing mode.",
    operation: "Destination Address → PC",
    assembler: &["jmp < ea > "],
    attributes: "Unsized",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const JSR_DESC: Description = Description {
    description: "Pushes the long-word address of the instruction immediately following the JSR instruction onto the system stack. Program execution then continues at the address specified in the instruction.",
    operation: "SP – 4 → Sp; PC → (SP); Destination Address → PC",
    assembler: &["jsr < ea > "],
    attributes: "Unsized",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const LEA_DESC: Description = Description {
    description: "Loads the effective address into the specified address register. All 32 bits of the address register are affected by this instruction.",
    operation: "< ea > → An",
    assembler: &["lea < ea >, An "],
    attributes: "Long",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const LINK_DESC: Description = Description {
    description: "Pushes the contents of the specified address register onto the stack. Then loads the updated stack pointer into the address register. Finally, adds the displacement value to the stack pointer. For word-size operation, the displacement is the sign-extended word following the operation word. For long size operation, the displacement is the long word following the operation word. The address register occupies one long word on the stack. The user should specify a negative displacement in order to allocate stack area.",
    operation: "SP – 4 → SP; An → (SP); SP → An; SP + dn → SP",
    assembler: &["link An, # < displacement >"],
    attributes: "Word",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const LSL_LSR_DESC: Description = Description {
    description: "Shifts the bits of the operand in the direction specified (L or R). The carry bit receives the last bit shifted out of the operand. The shift count for the shifting of a register is specified in two different ways:
					1. Immediate—The shift count (1 – 8) is specified in the instruction.
					2. Register—The shift count is the value in the data register specified in the in- struction modulo 64.
					The size of the operation for register destinations may be specified as byte, word, or long. The contents of memory, < ea > , can be shifted one bit only, and the operand size is restricted to a word.
					The LSL instruction shifts the operand to the left the number of positions specified as the shift count. Bits shifted out of the high-order bit go to both the carry and the extend bits; zeros are shifted into the low-order bit.",
    operation: "Destination Shifted By Count → Destination",
    assembler: &["LSd Dx,Dy", "LSd # < data > ,Dy", "LSd < ea >", "where d is direction, L or R"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_SHIFT,
};

pub const MOVE_DESC: Description = Description {
    description: "Moves the data at the source to the destination location and sets the condition codes according to the data. The size of the operation may be specified as byte, word, or long.",
    operation: "Source → Destination",
    assembler: &["move < ea >, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_EXT,
};

pub const MOVEM_DESC: Description = Description {
    description: "Moves the contents of selected registers to or from consecutive memory locations starting at the location specified by the effective address. A register is selected if the bit in the mask field corresponding to that register is set. The instruction size determines whether 16 or 32 bits of each register are transferred. In the case of a word transfer to either address or data registers, each word is sign-extended to 32 bits, and the resulting long word is loaded into the associated register.
				  Selecting the addressing mode also selects the mode of operation of the MOVEM instruction, and only the control modes, the predecrement mode, and the postincre- ment mode are valid. If the effective address is specified by one of the control modes, the registers are transferred starting at the specified address, and the address is incre- mented by the operand length (2 or 4) following each transfer. The order of the regis- ters is from D0 to D7, then from A0 to A7.
				  If the effective address is specified by the predecrement mode, only a register-to-mem- ory operation is allowed. The registers are stored starting at the specified address minus the operand length (2 or 4), and the address is decremented by the operand length following each transfer. The order of storing is from A7 to A0, then from D7 to D0. When the instruction has completed, the decremented address register contains the address of the last operand stored. For the MC68020, MC68030, MC68040, and CPU32, if the addressing register is also moved to memory, the value written is the ini- tial register value decremented by the size of the operation. The MC68000 and MC68010 write the initial register value (not decremented).
				  If the effective address is specified by the postincrement mode, only a memory-to-reg- ister operation is allowed. The registers are loaded starting at the specified address; the address is incremented by the operand length (2 or 4) following each transfer. The order of loading is the same as that of control mode addressing. When the instruction has completed, the incremented address register contains the address of the last oper- and loaded plus the operand length. If the addressing register is also loaded from memory, the memory value is ignored and the register is written with the postincre- mented effective address.",
    operation: "Registers → Destination; Source → Registers",
    assembler: &["movem < list >, < ea >", "movem < list >, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const MOVEQ_DESC: Description = Description {
    description: "Moves a byte of immediate data to a 32-bit data register. The data in an 8-bit field within the operation word is sign- extended to a long operand in the data register as it is transferred.",
    operation: "Immediate Data → Destination",
    assembler: &["moveq # < data >, Dn"],
    attributes: "Long",
    flags: &FLAGS_EXT,
};

pub const MULS_DESC: Description = Description {
    description: "Multiplies two signed operands yielding a signed result. The multiplier and multiplicand are both word operands, and the result is a long-word operand. A register operand is the low-order word; the upper word of the register is ignored. All 32 bits of the product are saved in the destination data register.",
    operation: "Source * Destination → Destination",
    assembler: &["muls.w < ea > ,Dn - 16 x 16 → 32"],
    attributes: "Long",
    flags: &FLAGS_MUL,
};

pub const MULU_DESC: Description = Description {
    description: "Multiplies two signed operands yielding a unsigned result. The multiplier and multiplicand are both word operands, and the result is a long-word operand. A register operand is the low-order word; the upper word of the register is ignored. All 32 bits of the product are saved in the destination data register.",
    operation: "Source * Destination → Destination",
    assembler: &["muls.w < ea > ,Dn - 16 x 16 → 32"],
    attributes: "Long",
    flags: &FLAGS_MUL,
};

pub const NEG_DESC: Description = Description {
    description: "Subtracts the destination operand from zero and stores the result in the destination location. The size of the operation is specified as byte, word, or long.",
    operation: "0 – Destination → Destination",
    assembler: &["neg < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_NEG,
};

pub const NEGX_DESC: Description = Description {
    description: "Subtracts the destination operand and the extend bit from zero. Stores the result in the destination location. The size of the operation is specified as byte, word, or long.",
    operation: "0 – Destination – X → Destination",
    assembler: &["negx < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_NEGX,
};

pub const NOP_DESC: Description = Description {
    description: "Performs no operation. The processor state, other than the program counter, is unaffected. Execution continues with the instruction following the NOP instruction. The NOP instruction does not begin execution until all pending bus cycles have completed. This synchronizes the pipeline and prevents instruction overlap.",
    operation: "None",
    assembler: &["nop"],
    attributes: "Undefined",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const NOT_DESC: Description = Description {
    description: "Calculates the ones complement of the destination operand and stores the result in the destination location. The size of the operation is specified as byte, word, or long.",
    operation: "~ Destination → Destination",
    assembler: &["not < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_EXT,
};

pub const OR_DESC: Description = Description {
    description: "Performs an inclusive-OR operation of the source operand with the destination operand and stores the result in the destination location. The size of the operation can be specified as byte, word, or long. The contents of an address register may not be used as an operand.",
    operation: "Source | Destination → Destination",
    assembler: &["or < ea > ,Dn", "or Dn, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_AND,
};

pub const PEA_DESC: Description = Description {
    description: "Computes the effective address and pushes it onto the stack. The effective address is a long address.",
    operation: "SP – 4 → SP; < ea > → (SP)",
    assembler: &["pea < ea >"],
    attributes: "Long",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const ROL_ROR_DESC: Description = Description {
    description: "Rotates the bits of the operand in the direction specified (L or R). The extend bit is not included in the rotation. The rotate count for the rotation of a register is specified in either of two ways:
					1. Immediate—The rotate count (1 – 8) is specified in the instruction.
					2. Register—The rotate count is the value in the data register specified in the in- struction, modulo 64.
				  The size of the operation for register destinations is specified as byte, word, or long. The contents of memory, (ROd < ea > ), can be rotated one bit only, and operand size is restricted to a word.
				  The ROL instruction rotates the bits of the operand to the left; the rotate count deter- mines the number of bit positions rotated. Bits rotated out of the high-order bit go to the carry bit and also back into the low-order bit.",
    operation: "Destination Rotated By Count → Destination",
    assembler: &["ROd Dx,Dy", "ROd # < data > ,Dy", "ROd < ea >", "where d is direction, L or R"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_ROL,
};


pub const ROXL_ROXR_DESC: Description = Description {
    description: "Rotates the bits of the operand in the direction specified (L or R). The extend bit is included in the rotation. The rotate count for the rotation of a register is specified in either of two ways:
					1. Immediate—The rotate count (1 – 8) is specified in the instruction.
					2. Register—The rotate count is the value in the data register specified in the in- struction, modulo 64.
				  The size of the operation for register destinations is specified as byte, word, or long. The contents of memory, < ea > , can be rotated one bit only, and operand size is restricted to a word. The ROXL instruction rotates the bits of the operand to the left; the rotate count determines the number of bit positions rotated. Bits rotated out of the high- order bit go to the carry bit and the extend bit; the previous value of the extend bit rotates into the low-order bit.",
    operation: "Destination Rotated With X By Count → Destination",
    assembler: &["ROXd Dx,Dy", "ROXd # < data > ,Dy", "ROXd < ea >", "where d is direction, L or R"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_ROL,
};

pub const RTS_DESC: Description = Description {
    description: "Pulls the program counter value from the stack. The previous program counter value is lost.",
    operation: "(SP) → PC; SP + 4 → SP",
    assembler: &["rts"],
    attributes: "Unsized",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const SCC_DESC: Description = Description {
    description: "Tests the specified condition code; if the condition is true, sets the byte specified by the effective address to TRUE (all ones). Otherwise, sets that byte to FALSE (all zeros).",
    operation: "If Condition True
				   Then 1s → Destination
				Else 0s → Destination",
    assembler: &["Scc < ea >"],
    attributes: "Byte",
    flags: &FLAGS_NOT_AFFECTED,
};

pub const SUB_DESC: Description = Description {
    description: "Subtracts the source operand from the destination operand and stores the result in the destination. The size of the operation is specified as byte, word, or long. The mode of the instruction indicates which operand is the source, which is the destination, and which is the operand size.",
    operation: "Source - Destination → Destination",
    assembler: &["sub < ea > ,Dn", "sub Dn, < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_SUB,
};

pub const SUBQ_DESC: Description = Description {
    description: "Subtracts the immediate data (1 – 8) from the destination operand. The size of the operation is specified as byte, word, or long. Only word and long operations can be used with address registers, and the condition codes are not affected. When subtracting from address registers, the entire destination address register is used, despite the operation size.",
    operation: "Immidate - Destination → Destination",
    assembler: &["subq # < data > , < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_ARC,
};

pub const SUBX_DESC: Description = Description {
    description: "Subtracts the source operand and the extend bit from the destination operand and stores the result in the destination location.
					The instruction has two modes:
				  1. Data register to data register—the data registers specified in the instruction con- tain the operands.
				  2. Memory to memory—the address registers specified in the instruction access the operands from memory using the predecrement addressing mode.",
    operation: "Source - Destination + X → Destination",
    assembler: &["subx Dy,Dx", "subx -(Ay),-(Ax)"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_X,
};

pub const SWAP_DESC: Description = Description {
    description: "Exchange the 16-bit words (halves) of a data register.",
    operation: "Register 31 – 16 ←→ Register 15 – 0",
    assembler: &["swap dn"],
    attributes: "Word",
    flags: &FLAGS_SWAP,
};

pub const TAS_DESC: Description = Description {
    description: "Tests and sets the byte operand addressed by the effective address field. The instruction tests the current value of the operand and sets the N and Z condition bits appropriately. TAS also sets the high-order bit of the operand. The operation uses a locked or read-modify-write transfer sequence. This instruction supports use of a flag or semaphore to coordinate several processors.",
    operation: "Destination Tested → Condition Codes; 1 → Bit 7 of Destination",
    assembler: &["tas < ea >"],
    attributes: "Byte",
    flags: &FLAGS_TAS,
};

pub const TST_DESC: Description = Description {
    description: "Compares the operand with zero and sets the condition codes according to the results of the test. The size of the operation is specified as byte, word, or long.",
    operation: "Destination Tested → Condition Codes; 1 → Bit 7 of Destination",
    assembler: &["tst < ea >"],
    attributes: "Byte, Word, Long",
    flags: &FLAGS_TST,
};

pub const UNLK_DESC: Description = Description {
    description: "Loads the stack pointer from the specified address register, then loads the address register with the long word pulled from the top of the stack.",
    operation: "An → SP; (SP) → An; SP + 4 → SP",
    assembler: &["unlk An"],
    attributes: "Unsized",
    flags: &FLAGS_NOT_AFFECTED,
};



