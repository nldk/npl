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
    arguments: Vec<String>,
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
    pub fn get(self, nameTF: &String,allowNonVariableVallue:bool,onlyNumbers:bool) -> String {
        let mut value = String::new();
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
        if onlyNumbers {
            if value.parse::<f64>().is_ok(){
                return value.to_string();
            }else if nameTF.parse::<f64>().is_ok() {
                return nameTF.to_string();
            }else {
                panic!("not a number")
            }
        }
        if value == "".to_string() {
            if allowNonVariableVallue {
                return nameTF.to_string();
            }else {
                panic!("variable not found at line {} , name {nameTF}. maybe you didnt declare the variable", self.pp + 1)
            }
        }
        return value;

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
pub struct Function {
    functions:Vec<String>,
}
impl Function {
    pub fn add(mut self,name:String){
        self.functions.push(name);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn getFuncs()->Vec<String>{
    let mut functions = Vec::new();
    for i in function.functions.iter() {
        functions.push(i.to_string());
    }
    functions
}