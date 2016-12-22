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
    matrix: &'a [&'a [Op]],
    override_imm: Option<usize>,
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

fn print_instruction_header(inst: &Instruction) {
    let name = inst.name.to_uppercase();
    println!("## {}\n", name);
    if let Some(ref desc) = inst.desc {
        println!("**Operation:**      {}\n", desc.operation);

        print!("**Assembler:** ");
        for assem in desc.assembler {
            print!("{}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;", assem);
        }
        println!("\n\n**Attributes:** Size = ({})\n", desc.attributes);

        println!("**Description:** {}\n", desc.description);
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

fn generate_table(name: &str,
                  is_long: bool,
                  src_table: Option<&[Op]>,
                  dest_table: &[Op]) {
    let mut statements = Vec::with_capacity(20 * 20);
    let mut count = 0;

    if let Some(src_opts) = src_table {
        for src in src_opts {
            for dst in dest_table {
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
            let src = v.src.unwrap();
            let statement;

            if src.print_name == "#xxx" && is_long {
                statement = format!("{} {},{}", name, "#$ffffff", v.dst.name);
            } else {
                statement = format!("{} {},{}", name, v.src.unwrap().name, v.dst.name);
            }
            
            //println!("Statement {}", statement);
            if compile_statement(&v.temp_file, &v.temp_out, &statement) {
                v.cycle_count = Some(0); // indicate that this should be processed
            }
        });

        compile_cycle_counts(&mut statements);

        print_grid_table(name, &statements, &src_opts, dest_table);
    } else {
        for dst in dest_table {
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

        compile_cycle_counts(&mut statements);

        print_table(name, &statements, dest_table);
    }
}

fn generate_statements_two_args(name: &str, inst: &Instruction, is_long: bool) -> Vec<BuildResult> {
    let mut statements = Vec::with_capacity(20 * 20);
    let mut count = 0;

    for src in inst.matrix[0] {
        for dst in inst.matrix[1] {
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
        let src = v.src.unwrap();
        let statement;

        if src.print_name == "#xxx" && is_long {
            statement = format!("{} {},{}", name, "#$ffffff", v.dst.name);
        } else {
            statement = format!("{} {},{}", name, v.src.unwrap().name, v.dst.name);
        }
        
        if compile_statement(&v.temp_file, &v.temp_out, &statement) {
            v.cycle_count = Some(0); // indicate that this should be processed
        }
    });

    statements
}

fn generate_statements_one_arg(name: &str, inst: &Instruction, is_long: bool) -> Vec<BuildResult> {
    let mut statements = Vec::with_capacity(20 * 20);
    let mut count = 0;

    for dst in inst.matrix[0] {
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
        println!("one arg statement {}", statement);
        if compile_statement(&v.temp_file, &v.temp_out, &statement) {
            v.cycle_count = Some(0);
        }
    });

    statements
}

fn generate_statements_no_args(name: &str) -> Vec<BuildResult> {
    let mut statement = Vec::with_capacity(1);
    let file_in = format!("target/temp_{}.s", 0);
    let file_out = format!("target/temp_{}.o", 0);

    statement.push(BuildResult {
        src: None,
        dst: Op::new("", ""),
        temp_file: "".to_owned(), 
        temp_out:  "".to_owned(),
        cycle_count: None,
    });

    if compile_statement(&file_in, &file_out, name) {
        statement[0].cycle_count = Some(0);
    }

    statement
}

fn generate_table_2(name: &str, inst: &Instruction, is_long: bool) {
    let mut statements;

    if inst.matrix.len() == 2 {
        statements = generate_statements_two_args(name, inst, is_long);
        compile_cycle_counts(&mut statements);
        print_grid_table(name, &statements, inst.matrix[0], inst.matrix[1]);
    } else if inst.matrix.len() == 1 {
        statements = generate_statements_one_arg(name, inst, is_long);
        compile_cycle_counts(&mut statements);
        print_table(name, &statements, inst.matrix[0]);
    } else {
        statements = generate_statements_no_args(name);
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
        Op::new("#2000", "#xxx")];

    //let dest_types_none = [Op::new("", "")];

    //let mut two_ops = Vec::<&[Op]>::new();
    let two_ops: &[&[Op]] = &[&src_types, &dest_types];
    let one_op: &[&[Op]] = &[&dest_types];

    //two_ops.push(&src_types);
    //two_ops.push(&dest_types);

    unsafe {
        m68k_wrapper_init();
    }

    let inst_2_ops_000 =
        [
        /*
        Instruction {
            name: "abcd",
            desc: Some(ABCD_DESC),
            matrix: two_ops,
            override_imm: None,
        },
        */

        Instruction {
            name: "add",
            desc: Some(ADD_DESC),
            matrix: two_ops,
            override_imm: None,
        },
        Instruction {
            name: "clr",
            desc: Some(CLR_DESC),
            matrix: one_op,
            override_imm: None,
        },
        /*
        Instruction {
            name: "addq",
            desc: Some(ADDQ_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "addx",
            desc: Some(ADDX_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "and",
            desc: Some(AND_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "bchg",
            desc: Some(BCHG_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "bclr",
            desc: Some(BCLR_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "bset",
            desc: Some(BSET_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "btst",
            desc: Some(BTST_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "cmp",
            desc: Some(CMP_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "divu",
            desc: Some(DIVS_DIVU_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "divs",
            desc: Some(DIVS_DIVU_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "eor",
            desc: Some(EOR_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "exg",
            desc: Some(EXG_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "lea",
            desc: Some(LEA_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "move",
            desc: Some(MOVE_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "muls",
            desc: Some(MULS_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "mulu",
            desc: Some(MULU_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "or",
            desc: Some(OR_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "sub",
            desc: Some(SUB_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "subq",
            desc: Some(SUBQ_DESC),
            .. Instruction::default()
        },
        Instruction {
            name: "subx",
            desc: Some(SUBX_DESC),
            .. Instruction::default()
        },
        ];

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
        */
        ];

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


    for inst in &inst_2_ops_000 {
        let name_long = format!("{}.l", inst.name);

        print_instruction_header(inst);
        generate_table_2(inst.name, &inst, false);
        generate_table_2(&name_long, &inst, true);


        //generate_table(inst.name, false, Some(&src_types), &dest_types);
        //generate_table(&name_long, true, Some(&src_types), &dest_types);
    }

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


