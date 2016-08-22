use std::process::Command;
use std::fs::File;
use std::io::Write;

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

const TEMP_FILE: &'static str = "target/temp.s";
const TEMP_FILE_OUT: &'static str = "target/temp.o";

#[derive(PartialEq, Clone)]
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

#[derive(Clone)]
struct CycleRule {
    word_count: usize,
    long_count: usize,
    source: Ea,
    dest: Ea,
}

impl CycleRule {
    fn new(word_count: usize, long_count: usize, source: Ea, dest: Ea) -> CycleRule {
        CycleRule {
            word_count: word_count,
            long_count: long_count,
            source: source,
            dest: dest,
        }
    }
}

struct Instruction<'a> {
    name: &'static str,
    operation: &'static str,
    syntax: &'static str,
    attributes: &'static str,
    description: &'static str,
    cycle_rules: &'a [CycleRule],
}

fn calculate_cycle_count(inst: &Instruction, src: &Op, dest: &Op, size: Size) -> usize {
    let mut cycle_count = 0;

    for rule in inst.cycle_rules {
        if (src.ea_type == rule.source || rule.source == Ea::Any) &&
           (dest.ea_type == rule.dest || rule.dest == Ea::Any) {
            match size {
                Size::Word => cycle_count = rule.word_count,
                Size::Long => cycle_count = rule.long_count,
                _ => panic!("Not supported!"),
            }
            break;
        }
    }

    cycle_count += src.count + dest.count;

    if size == Size::Long {
        if src.count != 0 {
            cycle_count += 4;
        }

        if dest.count != 0 {
            cycle_count += 4;
        }
    }

    cycle_count
}

fn calc_cycle_count_one_op(inst: &Instruction, arg: &Op, size: Size) -> usize {
    let mut cycle_count = 0;

    for rule in inst.cycle_rules {
        if arg.ea_type == rule.dest || rule.dest == Ea::Any {
            match size {
                Size::Word => cycle_count = rule.word_count,
                Size::Long => cycle_count = rule.long_count,
                _ => panic!("Not supported!"),
            }
            break;
        }
    }

    cycle_count += arg.count;

    if size == Size::Long {
        if arg.count != 0 {
            cycle_count += 4;
        }
    }

    cycle_count
}


fn compile_statement(statement: &str) -> bool {
    {
        let mut file = File::create(TEMP_FILE).unwrap();
        write!(file, " {}", statement).unwrap();
    }

    let output =
        Command::new(VASM_EXE)
            .arg(TEMP_FILE)
            .arg("-Fbin")
            .arg("-o")
            .arg(TEMP_FILE_OUT)
            .output().expect("failed to execute process");

    output.status.success()
}

fn print_grid_table(name: &str, cycles: &Vec<Option<usize>>, src_table: &[Op], dest_table: &[Op]) {
    print!("| {name:<width$}", name = name, width = 9);

    for dst in dest_table {
        print!("| {} ", dst.print_name);    // prints top row
    }

    println!("|");
    println!("|----------|----|----|------|-------|-------|-------|----------|-------|-------|");

    let mut index = 0;

    for src in src_table {
        // precheck if row needs printed
        let mut skip_count = 0;

        for cycle in cycles {
            if let &Some(cc) = cycle { }
            else {                
                skip_count += 1;
            }
        }

        if skip_count != 9 {
            print!("| {name:<width$}", name = src.print_name, width = 9);

            for dest in dest_table {
                if let Some(cycle_count) = cycles[index] {
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
        }
    }

    println!("");
}

fn print_table(name: &str, cycles: &Vec<Option<usize>>, dest_table: &[Op]) {
    print!("| {name:<width$}", name = name, width = 9);

    for dst in dest_table {
        print!("| {} ", dst.print_name);
    }

    println!("|");
    println!("|----------|----|----|------|-------|-------|-------|----------|-------|-------|");

    let mut index = 0;

    print!("| {name:<width$}", name = " ", width = 9);

    for dest in dest_table {
        if let Some(cycle_count) = cycles[index] {
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
/*
    ## ADD

    **Operation:** Source + Destination → Destination

    **Assembler:** add < ea > ,Dn

    **Attributes:**  Size = (Byte, Word, Long)

    **Description:** 
*/
    println!("## {}\n", inst.name.to_uppercase());
    println!("**Operation:** {}\n", inst.operation);
    println!("**Assembler:** {}\n", inst.syntax);
    println!("**Attributes:** {}\n", inst.attributes);
    println!("**Description:** {}\n", inst.description);
}

fn generate_table(inst: &Instruction,
                  name: &str,
                  size: Size,
                  src_table: Option<&[Op]>,
                  dest_table: &[Op]) {
    let mut cycles = Vec::with_capacity(20 * 20);


    if let Some(src_opts) = src_table {
        for src in src_opts {
            for dst in dest_table {
                let statement = format!("{} {},{}", name, src.name, dst.name);
                if compile_statement(&statement) {
                    cycles.push(Some(calculate_cycle_count(inst, &src, &dst, size)));
                } else {
                    cycles.push(None);
                }
            }
        }
        print_grid_table(name, &cycles, &src_opts, dest_table);
    } else {
        for dst in dest_table {
            let statement = format!("{} {}", name, dst.name);
            if compile_statement(&statement) {
                cycles.push(Some(calc_cycle_count_one_op(inst, &dst, size)));
            } else {
                cycles.push(None);
            }
        }
        print_table(name, &cycles, dest_table);
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

    let inst_2_ops_000 = [
        Instruction {
                name: "abcd",
                operation: "",
                syntax: "",
                attributes:  "",
                description: "",
                cycle_rules: &[CycleRule::new(6, 6, Ea::DataRegister, Ea::DataRegister),
                                CycleRule::new(6, 10, Ea::Any, Ea::Any)],
        },
        Instruction {
                name: "add",
                operation: "Source + Destination → Destination",
                syntax: "ADD < ea > ,Dn  ADD Dn, < ea >",
                attributes: "Size = (Byte, Word, Long)",
                description: " Adds the source operand to the destination operand using binary addition and stores the result in the destination location. The size of the operation may be specified as byte, word, or long. The mode of the instruction indicates which operand is the source and which is the destination, as well as the operand size.",
                cycle_rules: &[CycleRule::new(4, 8, Ea::Immidate, Ea::DataRegister),
                                CycleRule::new(8, 8, Ea::Immidate, Ea::Memory),
                                CycleRule::new(8, 8, Ea::Any, Ea::AddressRegister),
                                CycleRule::new(4, 6, Ea::Any, Ea::DataRegister),
                                CycleRule::new(8, 12, Ea::DataRegister, Ea::Memory)],
        },
        Instruction {
                name: "addq",
                operation: "",
                syntax: "",
                attributes:  "",
                description: "",
                cycle_rules: &[CycleRule::new(0, 0, Ea::Immidate, Ea::DataRegister),
                                CycleRule::new(4, 4, Ea::Immidate, Ea::Memory)],
        },
        Instruction {
                name: "addx",
                operation: "",
                syntax: "",
                attributes:  "",
                description: "",
                cycle_rules: &[CycleRule::new(4, 8, Ea::DataRegister, Ea::DataRegister),
                                CycleRule::new(6, 10, Ea::Any, Ea::Any)],
        },
        Instruction {
                name: "and",
                operation: "",
                syntax: "",
                attributes:  "",
                description: "",
                cycle_rules: &[CycleRule::new(4, 8, Ea::Immidate, Ea::DataRegister),
                                CycleRule::new(8, 8, Ea::Immidate, Ea::Memory),
                                CycleRule::new(4, 6, Ea::Any, Ea::DataRegister),
                                CycleRule::new(8, 12, Ea::DataRegister, Ea::Memory)],
        },
        Instruction {
                name: "bchg",
                operation: "",
                syntax: "",
                attributes:  "",
                description: "",
                cycle_rules: &[CycleRule::new(8, 12, Ea::Immidate, Ea::Memory),
                                CycleRule::new(8, 8, Ea::DataRegister, Ea::Any)],
        },
    ];

    let inst_1_ops_000 = [Instruction {
                              name: "clr",
                              operation: "",
                syntax: "",
                attributes:  "",
                description: "",
                              cycle_rules: &[CycleRule::new(4, 8, Ea::Any, Ea::Any)],
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
