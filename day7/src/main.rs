use std::{collections::HashMap, error::Error, fs, process, str::FromStr};
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
    input_left: Option<GateIO>,
    input_right: Option<GateIO>,
    operation: Operation,
    value: Option<u32>
}

impl LogicGate{
    pub fn new(input_left: Option<GateIO>, input_right: Option<GateIO>, operation: Operation) -> Self{
        Self{
            input_left,
            input_right,
            operation,
            value: None
        }
    }
}

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
            static ref GATE_REGEX:Regex = Regex::new(r"^([a-z]{1,2}|[0-9]+)? ?([A-Z]+)? ?([a-z]{1,2}|[0-9]+)? -> ([a-z]{1,2})").unwrap();
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
            let left_input=
            if let Some(x) = captures.get(1){
                let input_str = x.as_str();
                match input_str.parse::<u32>(){
                    Ok(val) => Some(GateIO::Const(val)),
                    Err(_) => Some(GateIO::Wire(self.get_or_create_index(&input_str)))
                }
            } else{
                None
            };
            
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

    pub fn measure_node(&mut self, node: usize) -> Result<u32, String>{
        let gate = self.gates[node].unwrap();
        if let Some(x) = gate.value{
            Ok(x)
        } else{ // if it's none we have to process it
            let l_input_value = match gate.input_left{
                Some(GateIO::Const(x)) => Some(x),
                Some(GateIO::Wire(x)) => Some(self.measure_node(x)?), // ooh look recursion rears its ugly head!
                None => None
            };

            let r_input_value = match gate.input_right{
                Some(GateIO::Const(x)) => Some(x),
                Some(GateIO::Wire(x)) => Some(self.measure_node(x)?),
                None => None 
            };
            
            let value = match gate.operation{
                Operation::And => l_input_value.ok_or_else(|| String::from("Insufficient Inputs for AND"))?
                     & r_input_value.ok_or_else(|| String::from("Insufficient Inputs Inputs for AND"))?,
                Operation::LShift => l_input_value.ok_or_else(|| String::from("Insufficient Inputs for LSHIFT"))?
                    << r_input_value.ok_or_else(|| String::from("Insufficient Inputs for LSHIFT"))?,
                Operation::RShift => l_input_value.ok_or_else(|| String::from("Insufficient Inputs for RSHIFT"))?
                     >> r_input_value.ok_or_else(|| String::from("Insufficient Inputs for RSHIFT"))?,
                Operation::Or => l_input_value.ok_or_else(|| String::from("Insufficient Inputs for OR"))?
                     | r_input_value.ok_or_else(|| String::from("Insufficient Inputs for OR"))?,
                Operation::Not => !r_input_value.ok_or_else(|| String::from("Insufficient Inputs for NOT"))?,
                Operation::Nop => l_input_value.ok_or_else(|| String::from("Insufficient Inputs for NOP"))?
            };

            if let Some( ref mut x) = self.gates[node] { x.value = Some(value) };
            Ok(value)
        }
    }

    pub fn reset(&mut self){
        for x in self.gates.iter_mut(){
            if let Some(ref mut gate) = x { gate.value = None }
        }
    }
}

fn main() {
    let input = fs::read_to_string("day7/input.txt").unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });

    let mut board = CircuitBoard::new();
    input
    .lines()
    .try_for_each(|x| board.insert(x))
    .unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });

    let idx = board.get_or_create_index("a");
    let part_one_soln = board.measure_node(idx).unwrap();
    println!("Part One Solution: {}", part_one_soln);

    // for part 2 we reset the boards and change circuit b
    board.reset();
    let idx = board.get_or_create_index("b");
    board.gates[idx] = Some(LogicGate::new(Some(GateIO::Const(part_one_soln)), None, Operation::Nop));

    let idx = board.get_or_create_index("a");
    let part_two_solution = board.measure_node(idx).unwrap();
    println!("Part Two Solution: {}", part_two_solution);
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
            Some(GateIO::Wire(0)),
            Some(GateIO::Const(2)),
            Operation::RShift
        ));

        // if we insert an existing element the gate size shouldn't grow
        board.insert("2 -> bn").unwrap();
        let index = board.get_or_create_index("bn");
        assert_eq!(board.gates.len(), 2);
        assert_eq!(board.gates[index].unwrap(), LogicGate::new(
            Some(GateIO::Const(2)),
            None,
            Operation::Nop,
        ));

    }

    #[test]
    fn test_circuit_board_creation(){
        let simple_circuit = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

        // make a new circuit board
        let mut board = CircuitBoard::new();
        simple_circuit
        .lines()
        .try_for_each(|x| {
            board.insert(x)
        }).unwrap();

        // evaluate teh value of node y
        assert_eq!(board.gates.len(), 8);
        let idx = board.get_or_create_index("d");
        assert_eq!(board.measure_node(idx).unwrap(), 72);
        assert_eq!(board.gates[idx].unwrap().value, Some(72));
    }
}