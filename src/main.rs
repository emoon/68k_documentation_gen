use std::process::Command;
use std::fs::File;
use std::io::Write;

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

fn compile_statement(statement: &str) -> bool {
    let filename = "temp.s";

    {
        let mut file = File::create(filename).unwrap();
        write!(file, " {}", statement).unwrap();
    }

    let output =
        Command::new("vasmm68k_mot").arg(filename).output().expect("failed to execute process");

    output.status.success()
}

fn print_grid_table(name: &str, cycles: &Vec<Option<usize>>, src_table: &[Op], dest_table: &[Op]) {
    print!("| {name:<width$}", name = name, width = 9);

    for dst in dest_table {
        print!("| {} ", dst.print_name);
    }

    println!("|");
    println!("|----------|----|----|------|-------|-------|-------|----------|-------|-------|");

    let mut index = 0;

    for src in src_table {
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

    println!("");
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

    let inst_2_ops_00 = [Instruction {
                             name: "add",
                             cycle_rules: &[CycleRule::new(4, 8, Ea::Immidate, Ea::DataRegister),
                                            CycleRule::new(8, 8, Ea::Immidate, Ea::Memory),
                                            CycleRule::new(8, 8, Ea::Any, Ea::AddressRegister),
                                            CycleRule::new(4, 6, Ea::Any, Ea::DataRegister),
                                            CycleRule::new(8, 12, Ea::DataRegister, Ea::Memory)],
                         }];

    for inst in &inst_2_ops_00 {
        // generate for .w and .l

        let name_word = format!("{}.w", inst.name);
        let name_long = format!("{}.l", inst.name);

        generate_table(&inst, &name_word, Size::Word, Some(&src_types), &dest_types);
        generate_table(&inst, &name_long, Size::Long, Some(&src_types), &dest_types);

    }
}
