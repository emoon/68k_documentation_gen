use std::process::Command;
use std::fs::File;
use std::io::Write;

#[derive(PartialEq)]
enum OpType {
    Word,
    Long,
}

#[derive(PartialEq, Clone)]
enum EaType {
    Any,
    Immidate,
    DataRegister,
    AddressRegister,
    Memory,
}

struct Op {
    name: &'static str,
    count: usize,
    ea_type: EaType,
}

impl Op {
    fn new(name: &'static str, count: usize, ea_type: EaType) -> Op {
        Op {
            name: name,
            count: count,
            ea_type: ea_type,
        }
    }
}

#[derive(Clone)]
struct CycleRule {
    count: usize,
    source: EaType,
    dest: EaType,
}

struct Instruction<'a> {
    name: &'static str,
    op_type: OpType,
    cycle_rules: &'a [CycleRule],
}

fn calculate_cycle_count(inst: &Instruction, src: &Op, dest: &Op) -> usize {
    let mut cycle_count = 0;

    for rule in inst.cycle_rules {
        if (src.ea_type == rule.source || rule.source == EaType::Any) &&
           (dest.ea_type == rule.dest || rule.dest == EaType::Any) {
            cycle_count = rule.count;
            break;
        }
    }

    cycle_count += src.count + dest.count;

    if inst.op_type == OpType::Long {
        if src.count != 0 {
            cycle_count += 4;
        }

        if dest.count != 0 {
            cycle_count += 4;
        }
    }

    cycle_count
}

fn main() {
    let dest_types = [Op::new("d0", 0, EaType::DataRegister),
                      Op::new("a0", 0, EaType::AddressRegister),
                      Op::new("(a0)", 4, EaType::Memory),
                      Op::new("(a0)+", 4, EaType::Memory),
                      Op::new("-(a0)", 6, EaType::Memory),
                      Op::new("2(a0)", 8, EaType::Memory),
                      Op::new("2(a0,d0)", 10, EaType::Memory),
                      Op::new("$4.W", 8, EaType::Memory),
                      Op::new("$4.L", 12, EaType::Memory)];

    let src_types = [Op::new("d0", 0, EaType::DataRegister),
                     Op::new("a0", 0, EaType::AddressRegister),
                     Op::new("(a0)", 4, EaType::Memory),
                     Op::new("(a0)+", 4, EaType::Memory),
                     Op::new("-(a0)", 6, EaType::Memory),
                     Op::new("2(a0)", 8, EaType::Memory),
                     Op::new("2(a0,d0)", 10, EaType::Memory),
                     Op::new("$4.W", 8, EaType::Memory),
                     Op::new("$4.L", 12, EaType::Memory),
                     Op::new("2(pc)", 8, EaType::Memory),
                     Op::new("2(pc,d0)", 10, EaType::Memory),
                     Op::new("#2", 4, EaType::Immidate)];

    let mut compile_status = Vec::new();

    let filename = "temp.s";

    let dst_print = [" Dn ",
                     " An ",
                     " (An) ",
                     " (An)+ ",
                     " -(An) ",
                     " d(An) ",
                     " d(An,Dn) ",
                     " xxx.W ",
                     " xxx.L "];
    let src_print = ["| Dn       ",
                     "| An       ",
                     "| (An)     ",
                     "| (An)+    ",
                     "| -(An)    ",
                     "| d(An)    ",
                     "| d(An,Dn) ",
                     "| xxx.W    ",
                     "| xxx.L    ",
                     "| d(Pc)    ",
                     "| d(Pc,Dn) ",
                     "| #xxx     "];

    let instructions = [// Instruction {
                        // name: "clr.w",
                        // op_type: OpType::Word,
                        // cycle_rules: &[
                        // CycleRule { count: 4, source: EaType::DataRegister, dest: EaType::Any },
                        // CycleRule { count: 8, source: EaType::DataRegister, dest: EaType::Memory },
                        // ]
                        // },
                        //
                        Instruction {
                            name: "add.w",
                            op_type: OpType::Word,
                            cycle_rules: &[CycleRule {
                                               count: 4,
                                               source: EaType::Immidate,
                                               dest: EaType::DataRegister,
                                           },
                                           CycleRule {
                                               count: 8,
                                               source: EaType::Immidate,
                                               dest: EaType::Memory,
                                           },
                                           CycleRule {
                                               count: 8,
                                               source: EaType::Any,
                                               dest: EaType::AddressRegister,
                                           },
                                           CycleRule {
                                               count: 4,
                                               source: EaType::Any,
                                               dest: EaType::DataRegister,
                                           },
                                           CycleRule {
                                               count: 8,
                                               source: EaType::DataRegister,
                                               dest: EaType::Memory,
                                           }],
                        },
                        Instruction {
                            name: "add.l",
                            op_type: OpType::Long,
                            cycle_rules: &[CycleRule {
                                               count: 8,
                                               source: EaType::Immidate,
                                               dest: EaType::DataRegister,
                                           },
                                           CycleRule {
                                               count: 8,
                                               source: EaType::Immidate,
                                               dest: EaType::Memory,
                                           },
                                           CycleRule {
                                               count: 8,
                                               source: EaType::Any,
                                               dest: EaType::AddressRegister,
                                           },
                                           CycleRule {
                                               count: 6,
                                               source: EaType::Any,
                                               dest: EaType::DataRegister,
                                           },
                                           CycleRule {
                                               count: 12,
                                               source: EaType::DataRegister,
                                               dest: EaType::Memory,
                                           }],
                        } /* Instruction {
                           * name: "and.l",
                           * op_type: OpType::Word,
                           * cycle_rules: &[
                           * CycleRule { count: 6, source: EaType::Any, dest: EaType::DataRegister },
                           * CycleRule { count: 12, source: EaType::DataRegister, dest: EaType::Memory },
                           * ]
                           * }
                           * */];

    for inst in &instructions {
        compile_status.clear();
        for src in &src_types {
            for dst in &dest_types {

                {
                    let mut file = File::create(filename).unwrap();
                    write!(file, " {} {},{}", inst.name, src.name, dst.name).unwrap();
                }

                let output = Command::new("vasmm68k_mot")
                    .arg("-quiet")
                    .arg(filename)
                    .output()
                    .expect("failed to execute process");

                if output.status.success() {
                    compile_status.push(Some(calculate_cycle_count(&inst, &src, &dst)));
                } else {
                    compile_status.push(None);
                }
            }
        }

        // print header
        print!("| {name:<width$}", name = inst.name, width = 9);

        for dst in &dst_print {
            print!("|{}", dst);
        }

        println!("|");
        println!("|----------|----|----|------|-------|-------|-------|----------|-------|-------|");

        let mut index = 0;

        for src in &src_print {
            print!("{}", src);

            for dest_name in &dst_print {
                if let Some(cycle_count) = compile_status[index] {
                    print!("|{number:^width$}",
                           number = cycle_count,
                           width = dest_name.len());
                } else {
                    print!("|{number:^width$}", number = "*", width = dest_name.len());
                }
                index += 1;
            }

            println!("|");
        }

        println!("");
    }
}
