extern crate rayon;

use rayon::prelude::*;

use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::os::raw;
use std::io::Read;

pub mod descriptions;

use descriptions::*;


#[cfg(target_os="windows")]
const VASM_EXE: &'static str = "bin/win/vasmm68k_mot.exe";

#[cfg(target_os="macos")]
const VASM_EXE: &'static str = "bin/mac/vasmm68k_mot";

#[cfg(any(target_os="linux",
          target_os="freebsd",
          target_os="dragonfly",
          target_os="netbsd",
          target_os="openbsd"))]
const VASM_EXE: &'static str = "vasmm68k_mot";

struct BuildResult {
    src: Option<Op>,
    dst: Op,
    temp_file: String,
    temp_out: String,
    cycle_count: Option<usize>,
}

#[derive(Copy, Clone)]
struct Op {
    name: &'static str,
    print_name: &'static str,
}

impl Op {
    fn new(name: &'static str, print_name: &'static str) -> Op {
        Op {
            name: name,
            print_name: print_name,
        }
    }
}

struct Instruction<'a> {
    name: &'static str,
    desc: Option<Description>,
    matrix: Option<&'a [&'a [Op]]>,
    cc_codes: Option<&'a [&'a [&'static str]]>,
    override_output_b: Option<&'a [&'a [&'static str]]>,
    override_output_w: Option<&'a [&'a [&'static str]]>,
    override_output_l: Option<&'a [&'a [&'static str]]>,
}

impl <'a> Instruction<'a> {
    pub fn has_override(&self) -> bool {
        return self.override_output_b.is_some() ||
               self.override_output_w.is_some() ||
               self.override_output_l.is_some();
    }
}

impl <'a> Default for Instruction <'a> {
    fn default() -> Self {
        Instruction {
            name: "",
            desc: None,
            matrix: None,
            cc_codes: None,
            override_output_b: None,
            override_output_w: None,
            override_output_l: None,
        }
    }
}

fn compile_statement(filename: &str, file_out: &str, statement: &str) -> bool {
    {
        let mut file = File::create(filename).unwrap();
        write!(file, " {}", statement).unwrap();
    }

    let output = Command::new(VASM_EXE)
        .arg("-no-opt")
        .arg("-m68000")
        .arg(filename)
        .arg("-Fbin")
        .arg("-o")
        .arg(file_out)
        .output()
        .expect("failed to execute process");

    output.status.success()
}

fn print_grid_table(name: &str, cycles: &Vec<BuildResult>, src_table: &[Op], dest_table: &[Op]) {
    // Check if we have anything to print

    let mut should_print = false;

    for c in cycles {
        if c.cycle_count.is_some() {
            should_print = true;
            break;
        }
    }

    if !should_print {
        return;
    }

    print!("| {name:<width$}", name = name, width = 9);

    for dst in dest_table {
        print!("| {} ", dst.print_name);
    }

    println!("|");
    println!("|----------|----|----|------|-------|-------|-------|----------|-------|-------|");

    let mut index = 0;

    for src in src_table {
        let mut skip_count = 0;

        for i in index..index + 9 {
            if let None = cycles[i].cycle_count {
                skip_count += 1;
            }
        }

        if skip_count != 9 {
            print!("| {name:<width$}", name = src.print_name, width = 9);

            for dest in dest_table {
                if let Some(cycle_count) = cycles[index].cycle_count {
                    print!("|{number:^width$}",
                           number = cycle_count,
                           width = dest.print_name.len() + 2);
                } else {
                    print!("|{number:^width$}",
                           number = "*",
                           width = dest.print_name.len() + 2);
                }
                index += 1;
            }

            println!("|");
        } else {
            index += 9;
        }
    }

    println!("");
}

fn print_table(name: &str, cycles: &Vec<BuildResult>, dest_table: &[Op]) {
    print!("| {name:<width$}", name = name, width = 9);

    for dst in dest_table {
        print!("| {} ", dst.print_name);
    }

    println!("|");
    println!("|----------|----|----|------|-------|-------|-------|----------|-------|-------|");

    let mut index = 0;

    print!("| {name:<width$}", name = " ", width = 9);

    for dest in dest_table {
        if let Some(cycle_count) = cycles[index].cycle_count {
            print!("|{number:^width$}",
                   number = cycle_count,
                   width = dest.print_name.len() + 2);
        } else {
            print!("|{number:^width$}",
                   number = "*",
                   width = dest.print_name.len() + 2);
        }
        index += 1;
    }

    println!("|");
    println!("");
}

fn check_affected(flag_desc: &FlagsDesc) -> bool {
    flag_desc.x.is_affected() &&
    flag_desc.n.is_affected() &&
    flag_desc.z.is_affected() &&
    flag_desc.v.is_affected() &&
    flag_desc.c.is_affected()
}

fn get_flag_status(flag: &Flag) -> &'static str {
    match *flag {
        Flag::Set(_) => "*",
        Flag::Clear(_) => "0",
        Flag::NotAffected(_) => "-",
        Flag::Undefined => "U",
    }
}

fn print_flag_desc(name: &str, flag: &Flag) {
    match *flag {
        Flag::Set(text) => println!("{}  ", text),
        Flag::Clear(text) => println!("{}  ", text),
        Flag::NotAffected(text) => println!("{}  ", text),
        Flag::Undefined => println!("{} — Undefined  ", name),
    }
}

fn print_cc_codes(flag_desc: &FlagsDesc) {
    println!("### Condition Codes:\n");

    if !check_affected(flag_desc) {
        println!("Not affected.\n");
        return;
    }

    println!("| X | N | Z | V | C |");
    println!("|---|---|---|---|---|");
    println!("| {} | {} | {} | {} | {} |",
             get_flag_status(&flag_desc.x), get_flag_status(&flag_desc.n),
             get_flag_status(&flag_desc.z), get_flag_status(&flag_desc.v),
             get_flag_status(&flag_desc.c));

    print_flag_desc("X", &flag_desc.x);
    print_flag_desc("N", &flag_desc.n);
    print_flag_desc("Z", &flag_desc.z);
    print_flag_desc("V", &flag_desc.v);
    print_flag_desc("C", &flag_desc.c);

    println!("");
}

fn print_condition_codes(table: &[&[&'static str]]) {
    // it's assumed the first row is the header and name is the first entry always

    let mut title_row_lengths = Vec::new();
    let title_row = table[0];

    for entry in title_row {
        print!("| {} ", entry);
    }

    println!("|");

    for entry in title_row {
        print!("|");

        for _ in entry.chars() {
            print!("-")
        }

        title_row_lengths.push(entry.len());

        print!("--")
    }

    println!("|");

    for t in &table[1..] {
        for (i, entry) in t.iter().enumerate() {
            print!("| {name:<width$}", name = entry, width = title_row_lengths[i] + 1);
        }

        println!("|");
    }

    println!("");
}


fn print_instruction_header(inst: &Instruction) {
    let mut name = inst.name.to_uppercase();

    // Hack
    if name == "BCC" {
        name = "Bcc".to_owned();
    }

    if name == "SCC" {
        name = "Scc".to_owned();
    }

    if name == "DBCC" {
        name = "DBcc".to_owned();
    }

    println!("## {}\n", name);
    if let Some(ref desc) = inst.desc {
        println!("**Operation:**      {}\n", desc.operation);

        // print the syntax

        print!("| Assembler Syntax ");
        for assem in desc.assembler {
            print!("| {} ", assem);
        }

        println!("|");
        print!("|------------------");


        for assem in desc.assembler {
            print!("|");

            for _ in assem.chars() {
                print!("-");
            }
            print!("--");
        }

        println!("|");

        println!("\n**Attributes:** Size = ({})\n", desc.attributes);

        println!("**Description:** {}\n", desc.description);

        if let Some(cc_codes) = inst.cc_codes {
            print_condition_codes(cc_codes);
        }

        print_cc_codes(inst.desc.as_ref().unwrap().flags);

        println!("### Instruction Execution Times:");
    }
    else {
        println!("__No Description__\n");
    }
}

fn compile_cycle_counts(statements: &mut Vec<BuildResult>) {
    let mut instructions = Vec::new();
    let mut inst_count = 0u32;
    let mut cycle_count = Vec::<u32>::new();

    for statement in statements.iter() {
        if statement.cycle_count.is_some() {
            {
                let mut f = File::open(&statement.temp_out).unwrap();
                f.read_to_end(&mut instructions).unwrap();
                cycle_count.push(0);
                inst_count += 1;
            }
        }
    }

    unsafe {
        m68k_run_instructions(instructions.as_ptr() as *const raw::c_void, inst_count, cycle_count.as_mut_ptr());
    }

    inst_count = 0;

    for statement in statements.iter_mut() {
        if statement.cycle_count.is_some() {
            statement.cycle_count = Some(cycle_count[inst_count as usize] as usize);
            inst_count += 1;
        }
    }
}

fn generate_statements_two_args(name: &str, inst: &Instruction) -> Vec<BuildResult> {
    let mut statements = Vec::with_capacity(20 * 20);
    let mut count = 0;

    let matrix = inst.matrix.unwrap();

    for src in matrix[0] {
        for dst in matrix[1] {
            let file_in = format!("target/temp_{}.s", count);
            let file_out = format!("target/temp_{}.o", count);

            statements.push(BuildResult {
                src: Some(src.clone()),
                dst: dst.clone(),
                temp_file: file_in,
                temp_out: file_out,
                cycle_count: None,
            });

            count += 1;
        }
    }

    statements.par_iter_mut().weight_max().for_each(|v| {
        let statement = format!("{} {},{}", name, v.src.unwrap().name, v.dst.name);
        if compile_statement(&v.temp_file, &v.temp_out, &statement) {
            v.cycle_count = Some(0); // indicate that this should be processed
        }
    });

    statements
}

fn generate_statements_one_arg(name: &str, inst: &Instruction) -> Vec<BuildResult> {
    let mut statements = Vec::with_capacity(20 * 20);
    let mut count = 0;

    let matrix = inst.matrix.unwrap();

    for dst in matrix[0] {
        let file_in = format!("target/temp_{}.s", count);
        let file_out = format!("target/temp_{}.o", count);

        statements.push(BuildResult {
            src: None,
            dst: dst.clone(),
            temp_file: file_in,
            temp_out: file_out,
            cycle_count: None,
        });

        count += 1;
    }

    statements.par_iter_mut().weight_max().for_each(|v| {
        let statement = format!("{} {}", name, v.dst.name);
        if compile_statement(&v.temp_file, &v.temp_out, &statement) {
            v.cycle_count = Some(0);
        }
    });

    statements
}

fn generate_statements_no_args(name: &str) -> Vec<BuildResult> {
    let mut statement = Vec::with_capacity(1);
    let file_in = "target/temp_on_op.s";
    let file_out = "target/temp_one_op.o";

    statement.push(BuildResult {
        src: None,
        dst: Op::new("", ""),
        temp_file: file_in.to_owned(),
        temp_out:  file_out.to_owned(),
        cycle_count: None,
    });

    if compile_statement(&file_in, &file_out, name) {
        statement[0].cycle_count = Some(0);
    } 

    statement
}

fn fill_table_space(name: &str, extra_chars: usize) {
    for _ in name.chars() {
        print!("-")
    }

    for _ in 0..extra_chars {
        print!("-")
    }
}

fn print_predef_table(name: &str, table: &[&[&'static str]]) {
    // it's assumed the first row is the header and name is the first entry always

    let mut title_row_lengths = Vec::new();
    let title_row = table[0];

    print!("| {} ", name);

    for entry in title_row {
        print!("| {} ", entry);
    }

    println!("|");
    print!("|--");

    title_row_lengths.push(name.len());

    for _ in name.chars() {
        print!("-")
    }

    for entry in title_row {
        print!("|");

        for _ in entry.chars() {
            print!("-")
        }

        title_row_lengths.push(entry.len());

        print!("--")
    }

    println!("|");

    for t in &table[1..] {
        for (i, entry) in t.iter().enumerate() {
            print!("| {name:<width$}", name = entry, width = title_row_lengths[i] + 1);
        }

        println!("|");
    }

    println!("");
}

fn print_table_no_args(name: &str, build_res: &Vec<BuildResult>) {
    print!("| {} ", name);
    println!("| {} |", build_res[0].cycle_count.unwrap());

    print!("|");
    fill_table_space(name, 2);

    print!("|");

    fill_table_space("99", 2);
    println!("|\n");
}


fn generate_table(name: &str, inst: &Instruction) {
    let mut statements;

    if let Some(over) = inst.override_output_b {
        print_predef_table(name, over);
    }

    if let Some(over) = inst.override_output_w {
        print_predef_table(name, over);
    }

    if let Some(over) = inst.override_output_l {
        let name_long = format!("{}.l", name);
        print_predef_table(&name_long, over);
    }

    if inst.has_override() {
        return;
    }

    let matrix = inst.matrix.unwrap();

    if matrix.len() == 2 {
        statements = generate_statements_two_args(name, inst);
        compile_cycle_counts(&mut statements);
        print_grid_table(name, &statements, matrix[0], matrix[1]);
    } else if matrix.len() == 1 && matrix[0].len() > 0 {
        statements = generate_statements_one_arg(name, inst);
        compile_cycle_counts(&mut statements);
        print_table(name, &statements, matrix[0]);
    } else {
        statements = generate_statements_no_args(name);
        compile_cycle_counts(&mut statements);
        print_table_no_args(&name, &statements);
    }
}

fn main() {
    let dest_types = [
        Op::new("d0", "Dn"),
        Op::new("a0", "An"),
        Op::new("(a0)", "(An)"),
        Op::new("(a0)+", "(An)+"),
        Op::new("-(a0)", "-(An)"),
        Op::new("2(a0)", "d(An)"),
        Op::new("2(a0,d0)", "d(An,Dn)"),
        Op::new("$4.W", "xxx.W"),
        Op::new("$4.L", "xxx.L")];

    let src_types = [
        Op::new("d0", "Dn"),
        Op::new("a0", "An"),
        Op::new("(a0)", "(An)"),
        Op::new("(a0)+", "(An)+"),
        Op::new("-(a0)", "-(An)"),
        Op::new("2(a0)", "d(An)"),
        Op::new("2(a0,d0)", "d(An,Dn)"),
        Op::new("$4.W", "xxx.W"),
        Op::new("$4.L", "xxx.L"),
        Op::new("2(pc)", "d(PC)"),
        Op::new("2(pc,d0)", "d(PC,Dn)"),
        Op::new("#8", "#xxx")];

    let cc_codes: &[&[&'static str]] = &[
        &["Mnemonic", "Condition", "Mnemonic", "Condition"],
        &["CC (HI)", "Carry Clear","LS","Low or Same"],
        &["CS (LO)", "Carry Set",  "LT","Less Than"],
        &["EQ", "Equal", "MI","Minus"],
        &["GE", "Greater or Equal","NE","Not Equal"],
        &["GT", "Greather Than","PL","Plus"],
        &["HI", "High","VC","Overflow Clear"],
        &["LE", "Less or Equal","VS","Overflow Set"]];

    //let dest_types_none = [Op::new("", "")];

    //let mut Some(two_ops) = Vec::<&[Op]>::new();
    let two_ops: &[&[Op]] = &[&src_types, &dest_types];
    let one_op: &[&[Op]] = &[&dest_types];
    let no_ops: &[&[Op]] = &[&[]];

    let shift_desc: &[&[&'static str]] = &[
        &["Dn", "An", "(An)", "(An)+", "-(An)", "d(An)", "d(An,Dn)", "xxx.W", "xxx.L"],
        &["#1", "8","*","12","12","14","16","18","16","20"],
        &["#1-8", "6+2n", "*", "*", "*", "*", "*", "*", "*", "*"],
        &["Dn", "6+2n", "*", "*", "*", "*", "*", "*", "*", "*"]];

    let shift_desc_long: &[&[&'static str]] = &[
        &["Dn", "An", "(An)", "(An)+", "-(An)", "d(An)", "d(An,Dn)", "xxx.W", "xxx.L"],
        &["#1-8", "8+2n", "*", "*", "*", "*", "*", "*", "*", "*"],
        &["Dn", "8+2n", "*", "*", "*", "*", "*", "*", "*", "*"]];

    let bcc_desc: &[&[&'static str]] = &[
        &["Displacement", "Branch Taken", "Branch Not Taken"],
        &["", "Byte", "10", "8"],
        &["", "Word", "10", "12"]];

    let bsr_desc: &[&[&'static str]] = &[
        &["Displacement", "Branch Taken", "Branch Not Taken"],
        &["", "Byte", "18", "-"],
        &["", "Word", "18", "-"]];

    let dbcc_desc: &[&[&'static str]] = &[
        &["Displacement", "Branch Taken", "Branch Not Taken"],
        &["", "cc true", "-", "12"],
        &["", "cc false, Count not Expired", "10", "-"],
        &["", "cc false, Counter Expired", "-", "14"]];

    let branch_header: &[&'static str] = &["(An)", "(d16,An)", "(d8,An,Xn)", "(xxx).W", "(xxx).L", "(d16,PC)", "(d8,PC,Xn)"];

    let jmp_desc: &[&[&'static str]] = &[
       branch_header ,
       &["", "8", "10", "14", "10", "12", "10", "14"]];

    let jsr_desc: &[&[&'static str]] = &[
       branch_header ,
       &["", "16", "18", "22", "18", "20", "18", "22"]];

    let lea_desc: &[&[&'static str]] = &[
       branch_header ,
       &["", "4", "8", "12", "8", "12", "8", "12"]];

    //let pea_desc: &[&[&'static str]] = &[
     //  branch_header ,
     //  &["12", "16", "20", "16", "20", "16", "20"]];

    //Some(two_ops).push(&src_types);
    //Some(two_ops).push(&dest_types);

    unsafe {
        m68k_wrapper_init();
    }

    let inst_2_ops_000 =
        [
        Instruction {
            name: "abcd",
            desc: Some(ABCD_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "add",
            desc: Some(ADD_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "addq",
            desc: Some(ADDQ_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "addx",
            desc: Some(ADDX_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "and",
            desc: Some(AND_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "asl",
            desc: Some(ASL_ASR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "bcc",
            desc: Some(BCC_DESC),
            cc_codes: Some(&cc_codes),
            override_output_w: Some(&bcc_desc),
            .. Instruction::default()
        },
        Instruction {
            name: "bchg",
            desc: Some(BCHG_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "bclr",
            desc: Some(BCLR_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "bset",
            desc: Some(BSET_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "bsr",
            desc: Some(BSR_DESC),
            override_output_w: Some(&bsr_desc),
            .. Instruction::default()
        },
        Instruction {
            name: "btst",
            desc: Some(BTST_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "dbcc",
            desc: Some(DBCC_DESC),
            cc_codes: Some(&cc_codes),
            override_output_w: Some(&dbcc_desc),
            .. Instruction::default()
        },
        Instruction {
            name: "clr",
            desc: Some(CLR_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "cmp",
            desc: Some(CMP_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "divu",
            desc: Some(DIVS_DIVU_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "divs",
            desc: Some(DIVS_DIVU_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "eor",
            desc: Some(EOR_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "exg",
            desc: Some(EXG_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "ext",
            desc: Some(EXT_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "illegal",
            desc: Some(ILLEGAL_DESC),
            matrix: Some(no_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "jmp",
            desc: Some(JMP_DESC),
            override_output_w: Some(&jmp_desc),
            .. Instruction::default()
        },
        Instruction {
            name: "jsr",
            desc: Some(JSR_DESC),
            override_output_w: Some(&jsr_desc),
            .. Instruction::default()
        },
        Instruction {
            name: "lea",
            desc: Some(LEA_DESC),
            override_output_w: Some(&lea_desc),
            .. Instruction::default()
        },
        Instruction {
            name: "lsl",
            desc: Some(LSL_LSR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "lsr",
            desc: Some(LSL_LSR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "move",
            desc: Some(MOVE_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "muls",
            desc: Some(MULS_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "mulu",
            desc: Some(MULU_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "neg",
            desc: Some(NEG_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "negx",
            desc: Some(NEGX_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "nop",
            desc: Some(NOP_DESC),
            matrix: Some(no_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "not",
            desc: Some(NOT_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "or",
            desc: Some(OR_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "rol",
            desc: Some(ROL_ROR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "ror",
            desc: Some(ROL_ROR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "roxl",
            desc: Some(ROXL_ROXR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "roxr",
            desc: Some(ROXL_ROXR_DESC),
            matrix: Some(two_ops),
            override_output_w: Some(&shift_desc),
            override_output_l: Some(&shift_desc_long),
            .. Instruction::default()
        },
        Instruction {
            name: "rte",
            desc: Some(RTE_DESC),
            matrix: Some(no_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "rts",
            desc: Some(RTS_DESC),
            matrix: Some(no_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "scc",
            desc: Some(SCC_DESC),
            matrix: Some(one_op),
            cc_codes: Some(&cc_codes),
            .. Instruction::default()
        },
        Instruction {
            name: "sub",
            desc: Some(SUB_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "subq",
            desc: Some(SUBQ_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "subx",
            desc: Some(SUBX_DESC),
            matrix: Some(two_ops),
            .. Instruction::default()
        },
        Instruction {
            name: "swap",
            desc: Some(SWAP_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "swap",
            desc: Some(SWAP_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
        Instruction {
            name: "tst",
            desc: Some(TST_DESC),
            matrix: Some(one_op),
            .. Instruction::default()
        },
    ];

    /*
       let inst_1_ops_000 = [
       Instruction {
       name: "clr",
       desc: Some(CLR_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "ext",
       desc: Some(EXT_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "bsr",
       desc: Some(BSR_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "jsr",
       desc: Some(JSR_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "jmp",
       desc: Some(JMP_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "neg",
       desc: Some(NEG_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "not",
       desc: Some(NOT_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "swap",
       desc: Some(SWAP_DESC),
       .. Instruction::default()
       },
       ];
       */

    /*
       let inst_0_ops_000 = [
       Instruction {
       name: "nop",
       desc: Some(NOP_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "illegal",
       desc: None,
       .. Instruction::default()
       },
       Instruction {
       name: "rte",
       desc: Some(RTS_DESC),
       .. Instruction::default()
       },
       Instruction {
       name: "rts",
       desc: Some(RTS_DESC),
       .. Instruction::default()
       },
       ];
       */

    for inst in inst_2_ops_000.iter() {
        let name_long = format!("{}.l", inst.name);

        print_instruction_header(inst);
        generate_table(inst.name, &inst);

        if !inst.has_override() {
            generate_table(&name_long, &inst);
        }

        //generate_table(inst.name, false, Some(&src_types), &dest_types);
        //generate_table(&name_long, true, Some(&src_types), &dest_types);
    }

    /*
    {
        let inst = Instruction {
                name: "rts",
                desc: Some(RTS_DESC),
                matrix: Some(no_ops),
                //cc_codes: Some(&&cc_codes),
                //override_output_w: Some(bcc_desc),
                .. Instruction::default()
        };

        print_instruction_header(&inst);
        generate_table(inst.name, &inst);
    }
    */

    // Generate instructions with one op

    /*
       for inst in &inst_1_ops_000 {
       let name_long = format!("{}.l", inst.name);

       print_instruction_header(inst);
       generate_table(inst.name, None, &dest_types);
       generate_table(&name_long, None, &dest_types);
       }

    //

    // Generate instructions with no ops

    for inst in &inst_0_ops_000 {
    let name_long = format!("{}.l", inst.name);

    print_instruction_header(inst);
    generate_table(inst.name, None, &dest_types_none);
    generate_table(&name_long, None, &dest_types_none);
    }
    */
}

extern "C" {
    fn m68k_wrapper_init();
    fn m68k_run_instructions(instructions: *const raw::c_void, count: u32, cycle_res: *mut u32);
}

