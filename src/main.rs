use std::fs::File;
use std::{env, io};
use std::io::Read;
use std::path::Path;
use crate::instructions::{add, dif, mov, rnd, set, sub, usi};

mod instructions;

fn main() {
    execute();
}

fn execute() {
    let mut program = Program {
        pp: 0,
        lines: Vec::new(),
        vars: Vec::new(),
    };
    let file = rfile();
    let mut begin = true;
    parch(file, &mut program);
    while program.pp < (program.lines.len() - 1) as u64 {
        if !begin {
            program.pp += 1;
        }
        let mut line = Line {
            instruction: program.lines[program.pp as usize].instruction.clone(),
            opperhand: program.lines[program.pp as usize].opperhand.clone(),
        };
        line.perform(&mut program);
        begin = false;
    }
}

fn rfile() -> String {
    //let args: Vec<String> = env::args().collect();
    //let pathStr:String = args[1].clone();
    let pathStr: String = "test.npl".to_string();
    let path = Path::new(&pathStr);
    let mut data_file = File::open(path).unwrap();
    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();
    return file_content;
}

fn parch(file: String, mut program: &mut Program) {
    let splitedFile = file.trim().split(';');
    let mut parchFile: Vec<&str> = splitedFile.collect();
    parchFile.pop();
    let mut parchetFile: Vec<Line> = Vec::new();
    for i in parchFile {
        let (first_part, second_part) = i.split_once(' ').unwrap();
        let contentOld: Vec<String> = vec![first_part.parse().unwrap(), second_part.parse().unwrap()];
        let content = contentOld[0].trim_start_matches('\n');
        let instructionP = content.to_string();
        let mut opperhand = String::new();
        for y in contentOld[1].chars() {
            match y {
                '"' => {}
                _ => { opperhand += &*y.to_string(); }
            }
        }
        let opperhandV: Vec<&str> = opperhand.split(',').collect();
        let mut opperhandP: Vec<String> = Vec::new();
        for y in opperhandV {
            opperhandP.push(y.to_string());
        }
        let line = Line {
            instruction: instructionP,
            opperhand: opperhandP,
        };
        parchetFile.push(line);
    }
    println!("{:?}", parchetFile);
    program.lines = parchetFile;
}

struct Program {
    pp: u64,
    lines: Vec<Line>,
    vars: Vec<Var>,
}

struct Var {
    name: String,
    value: String,
}

#[derive(Debug)]
struct Line {
    instruction: String,
    opperhand: Vec<String>,
}

fn newVar(nameN: &String, valueN: &String, p: &mut Program) {
    p.vars.push(
        Var {
            name: nameN.to_string(),
            value: valueN.to_string(),
        }
    );
}

impl Line {
    fn perform(&self, program: &mut Program) {
        match self.instruction.as_str() {
            "ptl" => Self::ptl(self, program),
            "pt" => Self::pt(self, program),
            "sav" => { newVar(&self.opperhand[0], &self.opperhand[1], program) }
            "set" => { set(&self.opperhand[0], &self.opperhand[1], program) }
            "mov" => { mov(&self.opperhand[0], &self.opperhand[1], program) }
            "jmp" => {
                match &self.opperhand[0].parse::<u64>() {
                    Ok(number) => {
                        program.jmp(*number)
                    }
                    Err(e) => {
                        panic!("NOT A NUMBER!!!")
                    }
                }
            }
            "usi"=>{usi(self.opperhand[0].to_string(),program)},
            "rnd"=>{rnd(self.opperhand[0].to_string(),self.opperhand[1].parse::<i64>().expect("NOT A NUMBER"),self.opperhand[2].parse::<i64>().expect("NOT A NUMBER"),program)},
            "add"=>{add(self.opperhand[0].to_string(),self.opperhand[1].to_string(),program)},
            "sub"=>{sub(self.opperhand[0].to_string(),self.opperhand[1].to_string(),program)},
            "dif"=>{dif(self.opperhand[0].to_string(), self.opperhand[1].to_string(), self.opperhand[2].to_string(), self.opperhand[3].to_string().parse::<u64>().expect("NOT A NUMBER"), program) }
            _=>{
                println!("error:invalid instruction");
                panic!("e:{}", self.instruction)
            }
        };
    }
}