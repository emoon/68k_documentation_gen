pub struct Description {
    pub operation: &'static str,
    pub assembler: &'static [&'static str],  
    pub attributes: &'static str,
}

pub const ADD_DESC: Description = Description {
    operation: "Add < ea >, < ea >",
    assembler: &["Test", "Test"], 
    attributes: "Word, Long",
};
