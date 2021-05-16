use std::{collections::HashMap, error::Error, str::FromStr};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GateIO{
    Const(u32), // the input is a constant value
    Wire(usize) //the input references a different gate by name
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Operation{
    And,
    LShift,
    RShift,
    Not,
    Or,
    Nop,
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct LogicGate{
    input_left: GateIO,
    input_right: Option<GateIO>,
    operation: Operation,
    value: Option<u32>
}

impl LogicGate{
    pub fn new(input_left: GateIO, input_right: Option<GateIO>, operation: Operation) -> Self{
        Self{
            input_left,
            input_right,
            operation,
            value: None
        }
    }
}

// impl FromStr for LogicGate{
//     type Err=String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         lazy_static!{
//             static ref GATE_REGEX:Regex = Regex::new(r"([a-z]{1,2}|[0-9]+)? ?([A-Z]+)? ?([a-z]{1,2}|[0-9]+)? -> ([a-z]{1,2})").unwrap();
//         }

//         if let Some(captures) = GATE_REGEX.captures(s){
//             // Pull out the Operation Name
//             let mut operation = Operation::Nop; // make the default operation
//             if let Some(op_str) = captures.get(2){
//                 operation = match op_str.as_str(){
//                     "AND" => Operation::And,
//                     "OR" => Operation::Or,
//                     "NOT" => Operation::Not,
//                     "LSHIFT" => Operation::LShift,
//                     "RSHIFT" => Operation::RShift,
//                     x => return Err(format!("Could not Parse Operation from {}, got {}",s, x)),
//                 };
//             }

//             // get the left input
//             let left_input: GateIO;
//             if let Some(x) = captures.get(1){
//                 match x.as_str().parse::<u32>(){
//                     Ok(val) => left_input = GateIO::Const(val),
//                     Err(_) => {
//                         // if it's a wire we need to see if it exists in the hash and assign that key
//                         self.
//                 }
//             } else{
//                 return Err(format!("could not parse circuit's left input"))
//             }
            
//             // get the right input
//             let right_input = match captures.get(3){
//                 Some(x) => Some(GateIO::from_str(&x.as_str()).unwrap()),
//                 None => None
//             };

//             // get the output
//             let output = GateIO::from_str(&captures.get(4).ok_or_else(|| format!("Could not Extract Gate Output from {}",s))?.as_str()).unwrap();

//             Ok(LogicGate::new(left_input, right_input, output, operation))
//         } else {
//             return Err(format!("Could not Parse String from {}",s))
//         }
//     }
// }

struct CircuitBoard{
    gates: Vec<Option<LogicGate>>,
    gate_lut: HashMap<String, usize>,
}

impl CircuitBoard{
    pub fn new() -> Self{
        Self{
            gates: vec![],
            gate_lut: HashMap::new(),
        }
    }

    fn get_or_create_index(&mut self, id: &str) -> usize{
        // returns the vector index of the logic cell with the corresponding label, if it does not exist a new cell is pushed onto the vector
        match self.gate_lut.get(id){
            Some(x) => *x,
            None => {
                //push a None object to the gates vec
                let new_index = self.gates.len();
                self.gate_lut.insert(id.to_string(), new_index);
                self.gates.push(None);
                new_index
            }
        }
    }

    pub fn insert(&mut self, gate_descriptor: &str) -> Result<(), String>{
        lazy_static!{
            static ref GATE_REGEX:Regex = Regex::new(r"([a-z]{1,2}|[0-9]+)? ?([A-Z]+)? ?([a-z]{1,2}|[0-9]+)? -> ([a-z]{1,2})").unwrap();
        }

        if let Some(captures) = GATE_REGEX.captures(gate_descriptor){
            // Pull out the Operation Name
            let operation:Operation = // make the default operation
            if let Some(op_str) = captures.get(2){
                match op_str.as_str(){
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "NOT" => Operation::Not,
                    "LSHIFT" => Operation::LShift,
                    "RSHIFT" => Operation::RShift,
                    x => return Err(format!("Could not Parse Operation from {}, got {}",gate_descriptor, x)),
                }
            } else{
                Operation::Nop
            };

            // get the left input
            let left_input: GateIO;
            if let Some(x) = captures.get(1){
                let input_str = x.as_str();
                left_input = match input_str.parse::<u32>(){
                    Ok(val) => GateIO::Const(val),
                    Err(_) => GateIO::Wire(self.get_or_create_index(&input_str))
                }
            } else{
                return Err("could not parse circuit's left input".to_string())
            }
            
            let right_input=
            if let Some(x) = captures.get(3){
                let input_str = x.as_str();
                match input_str.parse::<u32>(){
                    Ok(val) => Some(GateIO::Const(val)),
                    Err(_) => Some(GateIO::Wire(self.get_or_create_index(&input_str)))
                }
            } else{
                None
            };

            // get the output and store it in the LUT
            // TODO:
            let gate_idx = self.get_or_create_index(
                captures.get(4)
                .ok_or_else(|| format!("Could not Extract Gate Output from {}",gate_descriptor))?.as_str()
            );
            
            //insert it into the vec gate, but raise an error if something is already there
            match self.gates[gate_idx]{
                None => {
                    self.gates[gate_idx] = Some(LogicGate::new(left_input, right_input, operation));
                    Ok(())},
                Some(_) => Err(format!("gate Index {} is not empty",gate_idx))
            }
        } else{
            Err(format!("Could Not Process Gate Structure from String {}", gate_descriptor))
        }
    }

    // pub fn measure_node(&mut self, node: &str) -> Result<u32, String>{
    //     let gate_idx = *self.gate_lut.get(node).ok_or_else(|| format!("Node {} does not exist in Circuit",node))?;
        
    //     if let Some(x) = self.gates[gate_idx].value{
    //         Ok(x)
    //     } else{ // if it's none we have to process it
    //         let l_input_value = match self.gates[gate_idx].input_left{
    //             Some(GateIO::Const(x)) => x,
    //             Some(GateIO::Wire(x)) => self.measure_node(&x)?, // ooh look recursion rears its ugly head!
    //             None => return Err(format!("Gate {} has no left input!", node)), 
    //         };

    //         let r_input_value = match self.gates[gate_idx].input_right{
    //             Some(GateIO::Const(x)) => Some(x),
    //             Some(GateIO::Wire(x)) => Some(self.measure_node(&x)?),
    //             None => None 
    //         };
            
    //         let value = match self.gates[gate_idx].operation{
    //             Operation::And => l_input_value & r_input_value.ok_or_else(|| String::from("Insufficient Inputs"))?,
    //             Operation::LShift => l_input_value << 0x02,
    //             Operation::RShift => l_input_value >> 0x02,
    //             Operation::Or => l_input_value | r_input_value.ok_or_else(|| String::from("Insufficient Inputs"))?,
    //             Operation::Not => !l_input_value,
    //             Operation::Nop => l_input_value
    //         };

    //         self.gates[gate_idx].value = Some(value);
    //         Ok(value)
    //     }
    // }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_logic_gate_creation() {
        let mut board = CircuitBoard::new();
        board.insert("bn RSHIFT 2 -> bo").unwrap();
        assert_eq!(board.gates.len(),2);
        let index = board.get_or_create_index("bo");
        assert_eq!(board.gates[index].unwrap(), LogicGate::new(
            GateIO::Wire(0),
            Some(GateIO::Const(2)),
            Operation::RShift
        ))

    }

    // #[test]
    // fn test_circuit_board_creation(){
    //     let simple_circuit = r#"123 -> x
    //     456 -> y
    //     x AND y -> d
    //     x OR y -> e
    //     x LSHIFT 2 -> f
    //     y RSHIFT 2 -> g
    //     NOT x -> h
    //     NOT y -> i"#;

    //     // make a new circuit board
    //     let mut board = CircuitBoard::new();
    //     simple_circuit
    //     .lines()
    //     .map(|x| LogicGate::from_str(x))
    //     .try_for_each(|x| {
    //         board.insert(x?)
    //     }).unwrap();

    //     // evaluate teh value of node y
    //     assert_eq!(board.measure_node("y").unwrap(), 72);


    // }
}