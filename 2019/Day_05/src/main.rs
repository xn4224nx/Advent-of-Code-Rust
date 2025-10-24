/*
 * --- Day 5: Sunny with a Chance of Asteroids ---
 *
 * You're starting to sweat as the ship makes its way toward Mercury. The Elves
 * suggest that you get the air conditioner working by upgrading your ship
 * computer to support the Thermal Environment Supervision Terminal.
 *
 * The Thermal Environment Supervision Terminal (TEST) starts by running a
 * diagnostic program (your puzzle input). The TEST diagnostic program will run
 * on your existing Intcode computer after a few modifications:
 *
 * First, you'll need to add two new instructions:
 *
 *      -   Opcode 3 takes a single integer as input and saves it to the
 *          position given by its only parameter. For example, the instruction
 *          3,50 would take an input value and store it at address 50.
 *
 *      -   Opcode 4 outputs the value of its only parameter. For example, the
 *          instruction 4,50 would output the value at address 50.
 *
 * Programs that use these instructions will come with documentation that
 * explains what should be connected to the input and output. The program
 * 3,0,4,0,99 outputs whatever it gets as input, then halts.
 *
 * Second, you'll need to add support for parameter modes:
 *
 * Each parameter of an instruction is handled based on its parameter mode.
 * Right now, your ship computer already understands parameter mode 0, position
 * mode, which causes the parameter to be interpreted as a position - if the
 * parameter is 50, its value is the value stored at address 50 in memory. Until
 * now, all parameters have been in position mode.
 *
 * Now, your ship computer will also need to handle parameters in mode 1,
 * immediate mode. In immediate mode, a parameter is interpreted as a value - if
 * the parameter is 50, its value is simply 50.
 *
 * Parameter modes are stored in the same value as the instruction's opcode. The
 * opcode is a two-digit number based only on the ones and tens digit of the
 * value, that is, the opcode is the rightmost two digits of the first value in
 * an instruction. Parameter modes are single digits, one per parameter, read
 * right-to-left from the opcode: the first parameter's mode is in the hundreds
 * digit, the second parameter's mode is in the thousands digit, the third
 * parameter's mode is in the ten-thousands digit, and so on. Any missing modes
 * are 0.
 *
 * For example, consider the program 1002,4,3,4,33.
 *
 * The first instruction, 1002,4,3,4, is a multiply instruction - the rightmost
 * two digits of the first value, 02, indicate opcode 2, multiplication. Then,
 * going right to left, the parameter modes are 0 (hundreds digit), 1 (thousands
 * digit), and 0 (ten-thousands digit, not present and therefore zero):
 *
 *      ABCDE
 *       1002
 *
 *      DE - two-digit opcode,      02 == opcode 2
 *      C - mode of 1st parameter,  0 == position mode
 *      B - mode of 2nd parameter,  1 == immediate mode
 *      A - mode of 3rd parameter,  0 == position mode,
 *                                       omitted due to being a leading zero
 *
 * This instruction multiplies its first two parameters. The first parameter,
 * 4 in position mode, works like it did before - its value is the value stored
 * at address 4 (33). The second parameter, 3 in immediate mode, simply has
 * value 3. The result of this operation, 33 * 3 = 99, is written according to
 * the third parameter, 4 in position mode, which also works like it did before
 * - 99 is written to address 4.
 *
 * Parameters that an instruction writes to will never be in immediate mode.
 *
 * Finally, some notes:
 *
 *      -   It is important to remember that the instruction pointer should
 *          increase by the number of values in the instruction after the
 *          instruction finishes. Because of the new instructions, this amount
 *          is no longer always 4.
 *
 *      -   Integers can be negative: 1101,100,-1,4,0 is a valid program (find
 *          100 + -1, store the result in position 4).
 *
 * The TEST diagnostic program will start by requesting from the user the ID of
 * the system to test by running an input instruction - provide it 1, the ID for
 * the ship's air conditioner unit.
 *
 * It will then perform a series of diagnostic tests confirming that various
 * parts of the Intcode computer, like parameter modes, function correctly. For
 * each test, it will run an output instruction indicating how far the result of
 * the test was from the expected value, where 0 means the test was successful.
 * Non-zero outputs mean that a function is not working correctly; check the
 * instructions that were run before the output instruction to see which one
 * failed.
 *
 * Finally, the program will output a diagnostic code and immediately halt. This
 * final output isn't an error; an output followed immediately by a halt means
 * the program finished. If all outputs were zero except the diagnostic code,
 * the diagnostic program ran successfully.
 *
 * PART 1:  After providing 1 to the only input instruction and passing all the
 *          tests, what diagnostic code does the program produce?
 */

#[derive(PartialEq)]
pub enum OPCode {
    ADD = 1,
    MULT = 2,
    INPUT = 3,
    OUTPUT = 4,
    EXIT = 99,
}

pub struct IntCodeProgram {
    pub memory: Vec<i32>,
    pub pntr: usize,
    pub input_val: i32,
    pub outputs: Vec<i32>,
}

impl IntCodeProgram {
    pub fn from_file(mem_file: &str) -> Self {
        return IntCodeProgram {
            memory: std::fs::read_to_string(mem_file)
                .unwrap()
                .trim()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            pntr: 0,
            input_val: 0,
            outputs: Vec::new(),
        };
    }

    /// Extract the opcode and modes from a single value
    pub fn extract_modes(value: i32) -> (i32, i32, i32, i32) {
        return (
            value % 100,
            value % 1000 / 100,
            value % 10000 / 1000,
            value % 100000 / 10000,
        );
    }

    /// Change the state of the program according to the referenced command
    pub fn execute_cmd(&mut self) -> OPCode {
        let (op_code, prm_1, prm_2, _prm_3) = Self::extract_modes(self.memory[self.pntr]);

        /* No further actions take place if exit detected. */
        if op_code == OPCode::EXIT as i32 {
            return OPCode::EXIT;
        }

        /* Resolve the first parameter. */
        let value_1 = if prm_1 == 0
            && self.pntr + 1 < self.memory.len()
            && self.memory[self.pntr + 1] < self.memory.len() as i32
        {
            self.memory[self.memory[self.pntr + 1] as usize]
        } else if self.pntr + 1 < self.memory.len() {
            self.memory[self.pntr + 1]
        } else {
            0
        };

        /* Resolve the second parameter. */
        let value_2 = if prm_2 == 0
            && self.pntr + 2 < self.memory.len()
            && self.memory[self.pntr + 2] < self.memory.len() as i32
        {
            self.memory[self.memory[self.pntr + 2] as usize]
        } else if self.pntr + 2 < self.memory.len() {
            self.memory[self.pntr + 2]
        } else {
            0
        };

        /* Convert the op_code value to its enum equivilent. */
        let ret_code = match op_code {
            1 => OPCode::ADD,
            2 => OPCode::MULT,
            3 => OPCode::INPUT,
            4 => OPCode::OUTPUT,
            99 => OPCode::EXIT,
            _ => panic!("Unknown opcode: '{}'", op_code),
        };

        /* Run the specified operation. */
        match ret_code {
            OPCode::ADD => {
                let idx = self.memory[self.pntr + 3] as usize;
                self.memory[idx] = value_1 + value_2;
            }
            OPCode::MULT => {
                let idx = self.memory[self.pntr + 3] as usize;
                self.memory[idx] = value_1 * value_2;
            }
            OPCode::INPUT => {
                let idx = self.memory[self.pntr + 1] as usize;
                self.memory[idx] = self.input_val;
            }
            OPCode::OUTPUT => {
                self.outputs.push(value_1);
            }
            OPCode::EXIT => {}
        }

        /* Change the pointer ready for the next command */
        self.pntr += match ret_code {
            OPCode::ADD | OPCode::MULT => 4,
            OPCode::INPUT | OPCode::OUTPUT => 2,
            _ => 0,
        };
        return ret_code;
    }

    /// After all possible command executions determine the final diagnostic code
    pub fn final_diag_code(&mut self, init_value: i32) -> i32 {
        self.input_val = init_value;

        /* Generate diagnostic codes until the stop code is reached. */
        while self.execute_cmd() != OPCode::EXIT {}
        return self.outputs[self.outputs.len() - 1];
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        IntCodeProgram::from_file("./data/input_0.txt").final_diag_code(1)
    );
}
