
#[derive(Clone)]
struct Program {
    pp: u64,
    lines: Vec<Line>,
    vars: Vec<Var>,
    end: bool,
    debug: bool,
    libs: Vec<Lib>,
}
#[derive(Clone)]
struct Var {
    name: String,
    value: String,
}
#[derive(Clone)]
struct Line {
    instruction: String,
    opperhand: Vec<String>,
}
#[derive(Clone)]
struct Lib{
    path: String,
    func:Vec<String>,
}
impl Program {
    pub fn set(mut self, nameTF: &String, valueTS: &String, ) -> Program {
        for mut i in &mut self.vars {
            if &i.name == nameTF {
                i.value = valueTS.to_string();
                return self;
            }
        }
        return self;
        //panic!("variable not found at line {}. maybe you didn't declare the variable",p.pp)
    }
    pub fn get(self, nameTF: &String) -> String {
        let mut value = String::new();

        for mut i in &self.vars {
            if self.debug {
                println!("var{}:{}", i.name, i.value);
            }
            if i.name == nameTF.to_string() {
                value = i.value.to_string();
                return value;
            }
        }
        if self.debug {
            println!("\nDEBUG:{}, N:{}, V:{}", self.vars.len(), nameTF, value);
        }
        if value == "".to_string() {
            panic!("variable not found at line {} , name {nameTF}. maybe you didnt declare the variable", self.pp + 1)
        }
        return value;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn getFuncs()->Vec<String>{
    //println!("Getting functions...");
    return vec!["pow".to_string(),"sqrt".to_string()];
}
#[unsafe(no_mangle)]
pub extern "C" fn pow(p:Program,opperhand:Vec<String>)->Program{
    let mut program = p;
    let exponent= match opperhand[1].parse(){
        Ok(i) => i,
        Err(e) => {
            if program.debug {
                println!("Errore{:?},Num{}",e,opperhand[1]);
            }
            program.clone().get(&opperhand[1]).parse::<i64>().unwrap()}
    };
    let grondtal = match opperhand[0].parse(){
        Ok(i) => i,
        Err(e) => {
            if program.debug {
                println!("Errorg{:?},Num{},return{}",e,opperhand[0],program.clone().get(&opperhand[0]))
            }
            program.clone().get(&opperhand[0]).parse::<i64>().unwrap()}
    };
    program = program.set(&opperhand[2],&grondtal.pow(exponent as u32).to_string());
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
