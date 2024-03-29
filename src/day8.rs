use std::collections::HashMap;

fn main(){
    let puzzle_input = include_str!("puzzle_input.txt");
    println!("{}", largest_value_after_processing(puzzle_input));
}

#[derive(Debug)]
enum Change {
    Increment,
    Decrement
}



impl Change {
    fn from_str(input: &str) -> Change {
        match input {
            "dec" => Change::Decrement,
            "inc" => Change::Increment,
            _ => panic!("Unsupported instruction."),
        }
    }
}

#[derive(Debug)]
enum Comparison {
    GreaterThan,
    EqualTo,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    NotEqualTo,
}

impl Comparison {
    fn from_str(input: &str) ->Comparison {
        match input {
            ">" => Comparison::GreaterThan,
            "==" => Comparison::EqualTo,
            ">=" => Comparison::GreaterThanOrEqualTo,
            "<" => Comparison::LessThan,
            "<=" => Comparison::LessThanOrEqualTo,
            "!=" => Comparison::NotEqualTo,
            _ => panic!("Operator not recognized."),
        }
    }
}

#[derive(Debug)]
struct Condition {
    other_register: String,
    comparison: Comparison,
    comparison_value: i32,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    change: Change,
    change_amount: i32,
    condition: Condition,
}

impl Instruction {
    fn from(raw_instruction: &str) -> Instruction {
        let mut parts: Vec<&str> = raw_instruction.split_whitespace().collect();
        //e.g b inc 5 if a > 1
        Instruction {
            register: parts[0].to_owned(),
            change: Change::from_str(parts[1]), 
            change_amount: parts[2].parse::<i32>().unwrap(),
            condition: Condition {
                other_register: parts[4].to_owned(),
                comparison: Comparison::from_str(parts[5]),
                comparison_value: parts[6].parse::<i32>().unwrap(),
            },
        }
    }
}
#[derive(Debug)]
struct Computer {
    registers: HashMap<String, i32>,
}
impl Computer{
    fn new()->Computer{
        Computer{
            registers:HashMap::new(),
        }
    }
    fn execute(&mut self, instructions: Vec<Instruction>) {
        instructions.iter().for_each(|instruction|{
            let condition_result = self.condition_is_true(&instruction.condition);
            let register_value = self.registers
                .entry(instruction.register.to_owned())
                .or_insert(0);

            if condition_result {
                match instruction.change {
                    Change::Increment => *register_value += instruction.change_amount,
                    Change::Decrement => *register_value -= instruction.change_amount,
                }
            }
        })
    }

    fn largest_value(&self) -> i32 {
        let mut values: Vec<&i32> = self.registers.values().collect();
        values.sort();
        **values.last().unwrap()
    }

    fn condition_is_true(&self, condition: &Condition) ->  bool {
        let other_value = match self.registers.get(&condition.other_register){
            Some(value) => value,
            None => &0,
        };
        match condition.comparison{
            Comparison::EqualTo => *other_value == condition.comparison_value,
            Comparison::GreaterThan => *other_value > condition.comparison_value,
            Comparison::GreaterThanOrEqualTo => *other_value >= condition.comparison_value,
            Comparison::LessThan => *other_value < condition.comparison_value,
            Comparison::LessThanOrEqualTo => *other_value <= condition.comparison_value,
            Comparison::NotEqualTo => *other_value != condition.comparison_value,
        }
    }
}

fn largest_value_after_processing(input: &str) -> i32{
    let instructions:Vec<Instruction> = input.lines().map(|line| Instruction::from(line)).collect();
    let mut computer = Computer::new();
    computer.execute(instructions);

    computer.largest_value()
}

#[test]
fn example(){
    let puzzle_input = include_str!("example.txt");
    assert_eq!(1, largest_value_after_processing(puzzle_input));
}