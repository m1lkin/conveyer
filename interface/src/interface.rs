use logic::conveyor;

pub fn start() {
    let mut con = conveyor::Conveyor::new();
    
    con.add_value("A".to_string(), true);
    con.add_value("B".to_string(), false);
    con.add_instruction(conveyor::Instruction::AND, vec!["A".to_string(), "B".to_string()], "C".to_string());
    con.add_instruction(conveyor::Instruction::OR, vec!["C".to_string(), "A".to_string()], "B".to_string());
    
    let result = con.run().unwrap();
    println!("Результат: {:#?}", result);
}
