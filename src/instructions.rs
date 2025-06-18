use std::io;
use crate::{Line, Program, Var};

pub fn end(mut p: &mut Program){
    p.end = true
}
pub fn set( nameTF: &String, valueTS: &String, mut p: &mut Program) {
    for mut i in &mut p.vars {
        if &i.name == nameTF {
            i.value = valueTS.to_string();
            return;
        }
    }
    //panic!("variable not found at line {}. maybe you didnt declare the variable",p.pp)
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
    io::stdin().read_line(&mut input).unwrap();
    let inputTrimd = input.trim();
    set(&varTS, &inputTrimd.to_string(), &mut p);
}
use rand::Rng;
pub fn rnd(varTS:String,begin:i64,end:i64,p: &mut Program){
    let mut rng = rand::thread_rng();
    set(&varTS,&rng.gen_range(begin..end).to_string(),p);
}
pub fn dif(conl:String,op:String,conr:String,tj:u64, p:&mut Program){
    match op.as_str() {
        "=="=>{if get(&conr,&p) != get(&conl,&p){
            p.jmp(tj);
        }},
        "!="=>{if get(&conr,&p) == get(&conl,&p){
            p.jmp(tj);
        }},
        "<"=>{
            let conln = get(&conl,&p).parse::<i64>().expect("NOT A NUMBER");
            let conrn = get(&conr,&p).parse::<i64>().expect("NOT A NUMBER");
            if  !(conln < conrn) {
                p.jmp(tj);
            }},
        ">"=>{
            let conln = get(&conl,&p).parse::<i64>().expect("NOT A NUMBER");
            let conrn = get(&conr,&p).parse::<i64>().expect("NOT A NUMBER");
            if !(conln > conrn)  {
            p.jmp(tj);
        }},
        _ => {}
    }
}
pub fn add(varTS:String,valTA:String,  p:&mut Program){
    let valueTA = valTA.parse::<i64>().expect("NOT A NUMBER");
    let valueO = get(&varTS,&p).parse::<i64>().expect("NOT A NUMBER");
    set(&varTS,&(valueTA+valueO).to_string(),p)
}
pub fn sub(varTS:String,valTS:String, p:&mut Program){
    let valueTS = valTS.parse::<i64>().expect("NOT A NUMBER");
    let valueO = get(&varTS,&p).parse::<i64>().expect("NOT A NUMBER");
    set(&varTS,&(valueO-valueTS).to_string(),p)
}
impl Program{
    pub fn jmp(&mut self, place:u64){
        let placetojmp = self.pp + place;
        self.pp = placetojmp;
    }
}