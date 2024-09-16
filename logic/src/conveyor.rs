use std::collections::HashMap;

struct LogicGate {
    instruction: Instruction,
    inputs: Vec<String>,
    output: String,
}

/// Main struct in project
/// 
/// Used for containing instructions and values
pub struct Conveyor {
    conveyor: Vec<LogicGate>,
    values: HashMap<String, bool>,
}

/// Instructions for control-flow conveyor
pub enum Instruction {
    AND,
    OR,
    XOR,
    NOT,
}

impl Conveyor {
    /// Creating new instance of conveyor
    pub fn new() -> Conveyor {
        Conveyor {
            conveyor: Vec::new(),
            values: HashMap::new(),
        }
    }
    
    /// Adding new value to conveyor
    pub fn add_value(&mut self, k: String, v: bool) {
        self.values.insert(k, v);
    }
    
    /// Adding new instruction to conveyor
    pub fn add_instruction(
        &mut self, 
        instruction: Instruction,
        inputs: Vec<String>,
        output: String,
    ) {
        self.conveyor.push(LogicGate {
            instruction,
            inputs,
            output,
        });
    }
    
    /// Start computing instructions in conveyor
    /// 
    /// Return HashMap with computed values
    /// 
    /// Return error if any instruction has error
    pub fn run(&mut self) -> Result<HashMap<String, bool>, String> {
        for value in &self.conveyor {
            let result = match value.instruction {
                Instruction::AND => self.run_and_instruction(&value.inputs),
                Instruction::OR => self.run_or_instruction(&value.inputs),
                Instruction::NOT => self.run_not_instruction(&value.inputs),
                Instruction::XOR => self.run_xor_instruction(&value.inputs),
            };
            if let None = result {
                return Err("operation panic: breaking".to_string())
            }
            self.values.insert(value.output.clone(), result.unwrap());
        }
        Ok(self.values.clone())
    }
    
    fn run_and_instruction(&self, inputs: &[String]) -> Option<bool> {
        Some(inputs.iter().all(|x| *self.values.get(x).unwrap() == true))
    }
    
    fn run_or_instruction(&self, inputs: &[String]) -> Option<bool> {
        Some(inputs.iter().any(|x| *self.values.get(x).unwrap() == true))
    }
    
    fn run_xor_instruction(&self, inputs: &[String]) -> Option<bool> {
        Some(inputs.iter().filter(|x| *self.values.get(*x).unwrap_or(&false)).count() % 2 == 0)
    }
    
    fn run_not_instruction(&self, inputs: &[String]) -> Option<bool> {
        Some(!self.values.get(&inputs[0]).unwrap_or(&false))
    }
}

