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

#[derive(PartialEq, Clone, Copy)]
enum Ea {
    Any,
    Immidate,
    DataRegister,
    AddressRegister,
    Memory,
}

#[derive(PartialEq, Copy, Clone)]
enum Size {
    _Byte,
    Word,
    Long,
}

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
    count: usize,
    ea_type: Ea,
}

impl Op {
    fn new(name: &'static str, print_name: &'static str, count: usize, ea_type: Ea) -> Op {
        Op {
            name: name,
            print_name: print_name,
            count: count,
            ea_type: ea_type,
        }
    }
}
struct Instruction {
    name: &'static str,
    _desc: Option<Description>,
}


fn compile_statement(filename: &str, file_out: &str, statement: &str) -> bool {
    {
        let mut file = File::create(filename).unwrap();
        write!(file, " {}", statement).unwrap();
    }

    let output = Command::new(VASM_EXE)
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
    if let Some(ref desc) = inst._desc {
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

    println!("Size buffer {}", instructions.len());
}

fn generate_table(inst: &Instruction,
                  name: &str,
                  size: Size,
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
            let statement = format!("{} {},{}", name, v.src.unwrap().name, v.dst.name);
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

        print_table(name, &statements, dest_table);
    }
}

fn main() {
    let dest_types = [Op::new("d0", "Dn", 0, Ea::DataRegister),
    Op::new("a0", "An", 0, Ea::AddressRegister),
    Op::new("(a0)", "(An)", 4, Ea::Memory),
    Op::new("(a0)+", "(An)+", 4, Ea::Memory),
    Op::new("-(a0)", "-(An)", 6, Ea::Memory),
    Op::new("2(a0)", "d(An)", 8, Ea::Memory),
    Op::new("2(a0,d0)", "d(An,Dn)", 10, Ea::Memory),
    Op::new("$4.W", "xxx.W", 8, Ea::Memory),
    Op::new("$4.L", "xxx.L", 12, Ea::Memory)];

    let src_types = [Op::new("d0", "Dn", 0, Ea::DataRegister),
    Op::new("a0", "An", 0, Ea::AddressRegister),
    Op::new("(a0)", "(An)", 4, Ea::Memory),
    Op::new("(a0)+", "(An)+", 4, Ea::Memory),
    Op::new("-(a0)", "-(An)", 6, Ea::Memory),
    Op::new("2(a0)", "d(An)", 8, Ea::Memory),
    Op::new("2(a0,d0)", "d(An,Dn)", 10, Ea::Memory),
    Op::new("$4.W", "xxx.W", 8, Ea::Memory),
    Op::new("$4.L", "xxx.L", 12, Ea::Memory),
    Op::new("2(pc)", "d(PC)", 8, Ea::Memory),
    Op::new("2(pc,d0)", "d(PC,Dn)", 10, Ea::Memory),
    Op::new("#2", "#xxx", 4, Ea::Immidate)];

    unsafe {
        m68k_wrapper_init();
    }

    let inst_2_ops_000 =
        [/*Instruction {
           name: "abcd",
           _desc: None,
           cycle_rules: &[CycleRule::new(6, 6, Ea::DataRegister, Ea::DataRegister),
           CycleRule::new(6, 10, Ea::Any, Ea::Any)],
           },*/
        Instruction {
            name: "add",
            _desc: Some(ADD_DESC),
        }/*,
           Instruction {
           name: "addq",
           _desc: None,
           cycle_rules: &[CycleRule::new(0, 0, Ea::Immidate, Ea::DataRegister),
           CycleRule::new(4, 4, Ea::Immidate, Ea::Memory)],
           },
           Instruction {
           name: "addx",
           _desc: None,
           cycle_rules: &[CycleRule::new(4, 8, Ea::DataRegister, Ea::DataRegister),
           CycleRule::new(6, 10, Ea::Any, Ea::Any)],
           },
           Instruction {
           name: "and",
           _desc: None,
           cycle_rules: &[CycleRule::new(4, 8, Ea::Immidate, Ea::DataRegister),
           CycleRule::new(8, 8, Ea::Immidate, Ea::Memory),
           CycleRule::new(4, 6, Ea::Any, Ea::DataRegister),
           CycleRule::new(8, 12, Ea::DataRegister, Ea::Memory)],
           },
           Instruction {
           name: "bchg",
           _desc: None,
           cycle_rules: &[CycleRule::new(8, 12, Ea::Immidate, Ea::Memory),
           CycleRule::new(8, 8, Ea::DataRegister, Ea::Any)],
           }*/];

    let inst_1_ops_000 = [Instruction {
        name: "clr",
        _desc: None,
    }];

    for inst in &inst_2_ops_000 {
        let name_long = format!("{}.l", inst.name);

        print_instruction_header(inst);
        generate_table(&inst, inst.name, Size::Word, Some(&src_types), &dest_types);
        generate_table(&inst, &name_long, Size::Long, Some(&src_types), &dest_types);
    }

    // Generate instructions with one op

    for inst in &inst_1_ops_000 {
        let name_long = format!("{}.l", inst.name);

        print_instruction_header(inst);
        generate_table(&inst, inst.name, Size::Word, None, &dest_types);
        generate_table(&inst, &name_long, Size::Long, None, &dest_types);
    }
}

extern "C" {
    fn m68k_wrapper_init();
    fn m68k_run_instructions(instructions: *const raw::c_void, count: u32, cycle_res: *mut u32);
}


