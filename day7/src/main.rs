use std::{collections::HashMap, error::Error, str::FromStr};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Clone)]
enum GateIO{
    Const(u32), // the input is a constant value
    Wire(String) //the input references a different gate by name
}

impl FromStr for GateIO{
    type Err= String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if it can be converted to a u32, it's a const, otherwise it's a wire label
        Ok(
            match s.parse::<u32>(){
            Ok(val) => GateIO::Const(val),
            Err(_) => GateIO::Wire(String::from(s))
        })
    }
}
#[derive(Eq, PartialEq, Debug, Clone)]
enum Operation{
    And,
    LShift,
    RShift,
    Not,
    Or,
    Nop,
}
#[derive(Clone)]
struct LogicGate{
    input_left: Option<GateIO>,
    input_right: Option<GateIO>,
    output: GateIO,
    operation: Operation,
}

impl LogicGate{
    pub fn new(input_left: Option<GateIO>, input_right: Option<GateIO>, output: GateIO, operation: Operation) -> Self{
        Self{
            input_left,
            input_right,
            output,
            operation,
        }
    }
}

impl FromStr for LogicGate{
    type Err=String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref GATE_REGEX:Regex = Regex::new(r"([a-z]{1,2}|[0-9]+)? ?([A-Z]+)? ?([a-z]{1,2}|[0-9]+)? -> ([a-z]{1,2})").unwrap();
        }

        if let Some(captures) = GATE_REGEX.captures(s){
            // Pull out the Operation Name
            let mut operation = Operation::Nop; // make the default operation
            if let Some(op_str) = captures.get(2){
                operation = match op_str.as_str(){
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "NOT" => Operation::Not,
                    "LSHIFT" => Operation::LShift,
                    "RSHIFT" => Operation::RShift,
                    x => return Err(format!("Could not Parse Operation from {}, got {}",s, x)),
                };
            }
            // get the left input
            let left_input = match captures.get(1){
                Some(x) => Some(GateIO::from_str(&x.as_str()).unwrap()),
                None => None
            };
            
            // get the right input
            let right_input = match captures.get(3){
                Some(x) => Some(GateIO::from_str(&x.as_str()).unwrap()),
                None => None
            };

            // get the output
            let output = GateIO::from_str(&captures.get(4).ok_or_else(|| format!("Could not Extract Gate Output from {}",s))?.as_str()).unwrap();

            Ok(LogicGate::new(left_input, right_input, output, operation))
        } else {
            return Err(format!("Could not Parse String from {}",s))
        }
    }
}

struct CircuitBoard{
    gate_lut: HashMap<String, LogicGate>,
    node_values: HashMap<String, u32>
}

impl CircuitBoard{
    pub fn new() -> Self{
        Self{
            gate_lut: HashMap::new(),
            node_values: HashMap::new()
        }
    }

    pub fn insert(&mut self, gate: LogicGate) -> Result<(), String>{
        let key = match gate.output{
            GateIO::Wire(x) => x,
            GateIO::Const(_) => return Err(String::from("Logic Gate has const output, cannot insert"))
        };
        self.gate_lut.insert(key, gate);
        Ok(())
    }

    fn get_node_value(& gate_lut: HashMap<String, >)

    pub fn measure_node(&mut self, node: &str) -> Result<u32, String>{
        
        if let Some(x) = self.node_values.get(node){
            Ok(*x)
        } else{ // if it's none we have to process it
            let gate = self.gate_lut.get(node).ok_or_else(|| format!("gate with label {} does not exist",node))?;
            let l_input_value = match &gate.input_left{
                Some(GateIO::Const(x)) => *x,
                Some(GateIO::Wire(x)) => self.measure_node(x)?, // ooh look recursion rears its ugly head!
                None => return Err(format!("Gate {} has no left input!", node)), 
            };

            let r_input_value = match &gate.input_right{
                Some(GateIO::Const(x)) => Some(*x),
                Some(GateIO::Wire(x)) => Some(self.measure_node(&x)?),
                None => None 
            };
            
            let value = match &gate.operation{
                Operation::And => l_input_value & r_input_value.ok_or_else(|| String::from("Insufficient Inputs"))?,
                Operation::LShift => l_input_value << 0x02,
                Operation::RShift => l_input_value >> 0x02,
                Operation::Or => l_input_value | r_input_value.ok_or_else(|| String::from("Insufficient Inputs"))?,
                Operation::Not => !l_input_value,
                Operation::Nop => l_input_value
            };

            self.node_values.insert(node.to_string(), value);
            Ok(value)
        }
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_logic_gate_creation() {
        let gate = LogicGate::from_str("bn RSHIFT 2 -> bo").unwrap();
        assert_eq!(gate.input_left,Some(GateIO::Wire(String::from("bn"))));
        assert_eq!(gate.input_right,Some(GateIO::Const(2)));
        assert_eq!(gate.output,GateIO::Wire(String::from("bo")));
        assert_eq!(gate.operation, Operation::RShift);
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
        .map(|x| LogicGate::from_str(x))
        .try_for_each(|x| {
            board.insert(x?)
        }).unwrap();

        // evaluate teh value of node y
        assert_eq!(board.measure_node("y").unwrap(), 72);


    }
}