use std::io;
use rand::Rng;
use crate::{Line, Program, Var};

pub fn end(mut p: Program){
    p.end = true
}
impl Program{
    pub fn set( mut self,nameTF: &String, valueTS: &String,)->Program {
        for mut i in &mut self.vars {
            if &i.name == nameTF {
                i.value = valueTS.to_string();
                return self;
            }
        }
        return self;
        //panic!("variable not found at line {}. maybe you didn't declare the variable",p.pp)
    }
    pub fn get( self,nameTF: &String) -> String {
        let mut value = String::new();
        
        for mut i in &self.vars {
            if self.debug{
                println!("var{}:{}",i.name,i.value);
            }
            if i.name == nameTF.to_string() {
                value = i.value.to_string();
                break;
            }
        }
        if self.debug {
            println!("\nDEBUG:{}, N:{}, V:{}",self.vars.len(),nameTF,value);
        }
        if value=="".to_string(){
            panic!("variable not found at line {}. maybe you didnt declare the variable",self.pp+1)
        }
        return value;
    }
    pub fn mov(self,nameTF1: &String,nameTF2: &String)-> Program {
        let mut p = self;
        let ntf = p.clone().get(nameTF2);
        p = p.set(nameTF1,&ntf);
        p
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
    pub fn usi(self,varTS:String)->Program{
        let mut p = self;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputTrimd = input.trim();
        p = p.set(&varTS, &inputTrimd.to_string());
        if p.debug {
            println!("usi:{}",varTS);
        }
        return p;
    }
    
    pub fn rnd(self,varTS:String,begin:i64,end:i64)->Program{
        let mut p = self;
        let mut rng = rand::thread_rng();
        p = p.set(&varTS,&rng.gen_range(begin..end).to_string());
        return p;
    }
    pub fn dif(mut self,conl:String,op:String,conr:String,tj:i64,)->Program{
        let mut p = self;
        match op.as_str() {
            "=="=>{if p.clone().get(&conr) != p.clone().get(&conl){
                p = p.jmp(tj);
                return p;
            }
                return p;
            },
            "!="=>{if p.clone().get(&conr) == p.clone().get(&conl){
                p = p.jmp(tj);
                return p;
            }
                return p;
            },
            "<"=>{
                let conln = p.clone().get(&conl).parse::<i64>().expect("NOT A NUMBER");
                let conrn = p.clone().get(&conr).parse::<i64>().expect("NOT A NUMBER");
                if  !(conln < conrn) {
                    p =p.jmp(tj);
                    return p;
                }
                return p;
            },
            ">"=>{
                let conln = p.clone().get(&conl).parse::<i64>().expect("NOT A NUMBER");
                let conrn = p.clone().get(&conr).parse::<i64>().expect("NOT A NUMBER");
                if !(conln > conrn)  {
                    p = p.jmp(tj);
                    return p;
                }
                return p;
            },
            _ => {return p;}
        }
    }
    pub fn add(self,varTS:String,valTA:String,)->Program{
        let mut p = self;
        let valueTA = valTA.parse::<i64>().expect("NOT A NUMBER");
        let valueO = p.clone().get(&varTS).parse::<i64>().expect("NOT A NUMBER");
        p = p.set(&varTS,&(valueTA+valueO).to_string());
        p
    }
    pub fn sub(self,varTS:String,valTS:String)->Program{
        let mut p = self;
        let valueTS = valTS.parse::<i64>().expect("NOT A NUMBER");
        let valueO = p.clone().get(&varTS,).parse::<i64>().expect("NOT A NUMBER");
        p = p.set(&varTS,&(valueO-valueTS).to_string());
        p
    }
}

impl Program{
    pub fn jmp(self, place:i64)->Program{
        let mut p = self;
        let placetojmp = p.pp as i64 + place;
        if p.lines.len()>(placetojmp-1) as usize{
            p.pp = (placetojmp-1) as u64;
            p
        }else { 
            panic!("cannot jump to {placetojmp}");
        }
    }
}