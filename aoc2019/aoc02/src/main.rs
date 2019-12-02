type Result<T> = std::result::Result<T, Box<dyn (std::error::Error)>>;

fn main() -> Result<()> {
    println!("{:?}", std::env::current_dir());

    let input = std::fs::read_to_string("aoc02/input.txt")?;

    let original_program: Vec<usize> = input
        .split(",")
        .filter_map(|code| code.parse::<usize>().ok())
        .collect();

    println!("Original program: {:?}", original_program);

    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let mut program = original_program.clone();
            program[1] = i;
            program[2] = j;
            process_program(&mut program);

            if program[0] == 19690720 {
                println!("The pair of inputs is {} and {}", i, j);
                break 'outer;
            }
        }
    }

    Ok(())
}

fn process_program(program: &mut [usize]) {
    let mut opcode_index = 0;

    while opcode_index < program.len() {
        if program[opcode_index] == 99 {
            return;
        }

        process_opcode(program, opcode_index);
        //println!("{:?}", program);
        opcode_index += 4;
    }
}

fn process_opcode(program: &mut [usize], opcode_index: usize) {
    let opcode = program[opcode_index];

    //println!("Processing opcode {} at position {}", opcode, opcode_index);

    match opcode {
        1 => {
            let n1_pos = opcode_index + 1;
            let n1_code = program[n1_pos];

            let n2_pos = opcode_index + 2;
            let n2_code = program[n2_pos];

            let output_pos = opcode_index + 3;
            let output_code = program[output_pos];

            let sum = program[n1_code] + program[n2_code];

            //println!("Writing sum {} to position {}", sum, output_pos);
            program[output_code] = sum;
        }
        2 => {
            let n1_pos = opcode_index + 1;
            let n1_code = program[n1_pos];

            let n2_pos = opcode_index + 2;
            let n2_code = program[n2_pos];

            let output_pos = opcode_index + 3;
            let output_code = program[output_pos];

            let product = program[n1_code] * program[n2_code];

            //println!("Writing product {} to position {}", product, output_pos);
            program[output_code] = product;
        }
        99 => {
            panic!("Why are we processing the 99 this far? Abort, abort!");
        }
        _ => {
            panic!("Something went wrong! Abort, abort!");
        }
    }
}
