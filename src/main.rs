use std::io;
use std::str::FromStr;

fn main() {
    println!("*** Rust Calculator ***");

    // Prompt the user to input the first number
    let first_number: f64;
    loop {
        let mut number = String::new();
        println!("Inform the first number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the first number!");
        if let Ok(val) = number.trim().parse::<f64>() {
            first_number = val;
            break;
        } else {
            println!("Please enter a valid number!");
        }
    }

    // Prompt the user to input the first number
    let second_number: f64;
    loop {
        let mut number = String::new();
        println!("Inform the second number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the second number!");

        if let Ok(val) = number.trim().parse::<f64>() {
            second_number = val;
            break;
        } else {
            println!("Please enter a valid number!");
        }
    }

    // Prompt the user to input the operation.
    let operation: Operation;
    loop {
        let mut oper = String::new();
        println!("Inform the operation (add, subtract, multiply, or divide): ");

        io::stdin()
            .read_line(&mut oper)
            .expect("Failed to read the operation!");

        oper = String::from(oper.trim());

        let val = match Operation::from_str(&oper) {
            Ok(oper) => oper,
            Err(err) => panic!("Please enter a valid operation - must be one of the following: add, subtract, multiply, or divide! - {}", err),
        };

        operation = val;
        break;
    }

    let final_operation = match operation {
        Operation::Add { op1: _, op2: _ } => Operation::Add { op1: first_number, op2: second_number },
        Operation::Subtract { op1: _, op2: _  } => Operation::Subtract { op1: first_number, op2: second_number },
        Operation::Multiply { op1: _, op2: _  } => Operation::Multiply { op1: first_number, op2: second_number },
        Operation::Divide { op1: _, op2: _ } => Operation::Divide { op1: first_number, op2: second_number },
    };

    println!("Result: [{}]", calculate(final_operation));
}

// Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
enum Operation {
    Add { op1: f64, op2: f64 },
    Subtract { op1: f64, op2: f64 },
    Multiply { op1: f64, op2: f64 },
    Divide { op1: f64, op2: f64 },
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.to_lowercase().as_str() {
        "add" => Ok(Operation::Add {op1: 0.0, op2: 0.0}),
        "subtract" => Ok(Operation::Subtract {op1: 0.0, op2: 0.0}),
        "multiply" => Ok(Operation::Multiply {op1: 0.0, op2: 0.0}),
        "divide" => Ok(Operation::Divide {op1: 0.0, op2: 0.0}),
        _ => Err("Invalid operation string"),
      }
    }
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add { op1, op2 } => {
           return op1 + op2;
        }

        Operation::Subtract { op1, op2 } => {
            return op1 - op2;
        }

        Operation::Multiply { op1, op2 } => {
            return op1 * op2;
        }

        Operation::Divide { op1, op2 } => {
            return op1 / op2;
        }
    }
}
