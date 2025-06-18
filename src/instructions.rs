use std::io;
use rand::Rng;
use crate::{Line, Program, Var};

pub fn end(mut p: Program){
    p.end = true
}
impl Program{
    pub fn set( mut self,nameTF: &String, valueTS: &String,) {
        for mut i in &mut self.vars {
            if &i.name == nameTF {
                i.value = valueTS.to_string();
                return;
            }
        }
        //panic!("variable not found at line {}. maybe you didn't declare the variable",p.pp)
    }
    pub fn get( self,nameTF: &String) -> String {
        let mut value = String::new();
        
        for mut i in &self.vars {
            println!("{}",self.debug);
            if self.debug{
                println!("var{}:{}",i.name,i.value);
            }
            if i.name == nameTF.to_string() {
                value = i.value.to_string();
                break;
            }
        }
        if value=="".to_string(){
            panic!("variable not found at line {}. maybe you didnt declare the variable",self.pp+1)
        }
        return value;
    }
    pub fn mov(self,nameTF1: &String,nameTF2: &String) {
        let ntf = self.clone().get(nameTF2);
        self.set(nameTF1,&ntf);
    }
}


impl Line {
    pub fn ptl(line: &Line,program: &Program) {
        if line.opperhand.len() == 2 {
            if line.opperhand[1] == "?" {
                println!("{}",program.clone().get(&line.opperhand[0]));
            }
        }else {
            println!("{}", line.opperhand[0])
        }
    }
    pub fn pt(line: &Line,program: &Program) {
        if line.opperhand.len() == 2 {
            if line.opperhand[1] == "?" {
                print!("{}",program.clone().get(&line.opperhand[0]));
            }
        }else {
            print!("{}", line.opperhand[0])
        }
    }
}
impl Program {
    pub fn usi(self,varTS:String){
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputTrimd = input.trim();
        self.set(&varTS, &inputTrimd.to_string());
    }
    
    pub fn rnd(self,varTS:String,begin:i64,end:i64){
        let mut rng = rand::thread_rng();
        self.set(&varTS,&rng.gen_range(begin..end).to_string());
    }
    pub fn dif(mut self,conl:String,op:String,conr:String,tj:i64,){
        match op.as_str() {
            "=="=>{if self.clone().get(&conr) != self.clone().get(&conl){
                self.jmp(tj);
            }},
            "!="=>{if self.clone().get(&conr) == self.clone().get(&conl){
                self.jmp(tj);
            }},
            "<"=>{
                let conln = self.clone().get(&conl).parse::<i64>().expect("NOT A NUMBER");
                let conrn = self.clone().get(&conr).parse::<i64>().expect("NOT A NUMBER");
                if  !(conln < conrn) {
                    self.jmp(tj);
                }},
            ">"=>{
                let conln = self.clone().get(&conl).parse::<i64>().expect("NOT A NUMBER");
                let conrn = self.clone().get(&conr).parse::<i64>().expect("NOT A NUMBER");
                if !(conln > conrn)  {
                    self.jmp(tj);
                }},
            _ => {}
        }
    }
    pub fn add(self,varTS:String,valTA:String,){
        let valueTA = valTA.parse::<i64>().expect("NOT A NUMBER");
        let valueO = self.clone().get(&varTS).parse::<i64>().expect("NOT A NUMBER");
        self.set(&varTS,&(valueTA+valueO).to_string())
    }
    pub fn sub(self,varTS:String,valTS:String){
        let valueTS = valTS.parse::<i64>().expect("NOT A NUMBER");
        let valueO = self.clone().get(&varTS,).parse::<i64>().expect("NOT A NUMBER");
        self.set(&varTS,&(valueO-valueTS).to_string())
    }
}

impl Program{
    pub fn jmp(&mut self, place:i64){
        let placetojmp = self.pp as i64 + place;
        if self.lines.len()>(placetojmp-1) as usize{
            self.pp = placetojmp as u64;
        }else { 
            panic!("cannot jump to {placetojmp}");
        }
        
    }
}