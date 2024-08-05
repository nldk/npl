use std::io;
use crate::{Line, Program, Var};


pub fn set( nameTF: &String, valueTS: &String, mut p: &mut Program) {
    for mut i in &mut p.vars {
        if &i.name == nameTF {
            i.value = valueTS.to_string();
            return;
        }
    }
    panic!("variable not found at line {}. maybe you didnt declare the variable",p.pp)
}
pub fn get( nameTF: &String, mut p: &Program) -> String {
    let mut value = String::new();
    for mut i in &p.vars {
        if i.name == nameTF.to_string() {
            value = i.value.to_string();
            break;
        }
    }
    if value=="".to_string(){
        panic!("variable not found at line {}. maybe you didnt declare the variable",p.pp)
    }
    return value;
}
pub fn mov(nameTF1: &String,nameTF2: &String, mut p: &mut Program) {
    set(nameTF1, &get(nameTF2, p), p);
}
impl Line {
    pub fn ptl(line: &Line,program: &Program) {
        if line.opperhand.len() == 2 {
            if line.opperhand[1] == "?" {
                println!("{}",get(&line.opperhand[0],program));
            }
        }else {
            println!("{}", line.opperhand[0])
        }
    }
    pub fn pt(line: &Line,program: &Program) {
        if line.opperhand.len() == 2 {
            if line.opperhand[1] == "?" {
                print!("{}",get(&line.opperhand[0],program));
            }
        }else {
            print!("{}", line.opperhand[0])
        }
    }
}
pub fn usi(varTS:String,mut p: &mut Program){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let inputTrimd = input.trim();
    set(&varTS, &inputTrimd.to_string(), &mut p);
}
impl Program{
    pub fn jmp(&mut self, place:u64){
        self.pp = place-1
        ;
    }
}