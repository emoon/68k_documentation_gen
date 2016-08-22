pub struct Description {
    pub description: &'static str,
    pub operation: &'static str,
    pub assembler: &'static [&'static str],  
    pub attributes: &'static str,
}

pub const ADD_DESC: Description = Description {
    description: "Adds the source operand to the destination operand using binary addition and stores the result in the destination location. The size of the operation may be specified as byte, word, or long. The mode of the instruction indicates which operand is the source and which is the destination, as well as the operand size.",
    operation: "Source + Destination → Destination",
    assembler: &["Add < ea > ,Dn", "Add Dn, < ea >"], 
    attributes: "Byte, Word, Long",
};
