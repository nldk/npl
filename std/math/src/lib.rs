

#[unsafe(no_mangle)]
pub extern "C" fn getFuncs()->Vec<String>{
    //println!("Getting functions...");
    return vec!["pow".to_string(),"sqrt".to_string()];
}
#[unsafe(no_mangle)]
pub extern "C" fn pow(p:Program, arguments:Vec<String>) ->Program{
    let mut program = p;
    let exponent= match arguments[1].parse(){
        Ok(i) => i,
        Err(e) => {
            if program.debug {
                println!("Errore{:?},Num{}", e, arguments[1]);
            }
            program.clone().get(&arguments[1]).parse::<i64>().unwrap()}
    };
    let grondtal = match arguments[0].parse(){
        Ok(i) => i,
        Err(e) => {
            if program.debug {
                println!("Errorg{:?},Num{},return{}", e, arguments[0], program.clone().get(&arguments[0]))
            }
            program.clone().get(&arguments[0]).parse::<i64>().unwrap()}
    };
    program = program.set(&arguments[2], &grondtal.pow(exponent as u32).to_string());
    return program;
}
pub extern "C" fn sqrt(p:Program,opperhand:Vec<String>)->Program{
    let mut program = p;
    let grondtal = match opperhand[0].parse(){
        Ok(i) => i,
        Err(e) => {
            if program.debug {
                println!("Errorg{:?},Num{},return{}",e,opperhand[0],program.clone().get(&opperhand[0]))
            }
            program.clone().get(&opperhand[0]).parse::<i64>().unwrap()}
    };
    program = program.set(&opperhand[2],&grondtal.sqrt().to_string());
    return program;
}
