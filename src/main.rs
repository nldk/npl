use std::fs::File;
use std::{env, io};
use std::io::Read;
use std::path::Path;
use libloading::{Library, Symbol};
use crate::instructions::{end};

mod instructions;

fn main() {
    execute();
}

fn execute() {
    let mut program = Program {
        pp: 0,
        lines: Vec::new(),
        vars: Vec::new(),
        end: false,
        debug: false,
        libs: Vec::new(),
    };
    let file = rfile();
    let mut begin = true;
    let args: Vec<String> = env::args().collect();
    let mut debugS="".to_string();
    let mut debug = false;
    if args.len() > 2 {
        debugS = args[2].clone();
        debug = debugS == "-d";
    }
    program.debug=debug;
    program = parch(file, &program,debug);
    let mut p:Program =program.clone();
    while p.pp < (p.lines.len() - 1) as u64 && !p.end{
        if !begin {
            p.pp += 1;
        }
        let mut line = Line {
            instruction: p.lines[p.pp as usize].instruction.clone(),
            opperhand: p.lines[p.pp as usize].opperhand.clone(),
        };
        p = line.perform(p);
        begin = false;
    }
}

fn rfile() -> String {
    let args: Vec<String> = env::args().collect();
    let pathStr:String = args[1].clone();
    //let pathStr: String = "test.npl".to_string();
    let path = Path::new(&pathStr);
    let mut data_file = File::open(path).unwrap();
    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();
    return file_content;
}

fn parch(file: String, program: &Program,debug:bool)->Program {
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
    if debug{
        println!("{:?}", parchetFile);
    }
    let mut p = program.clone();
    p.lines = parchetFile;
    return p;
}
#[derive(Clone, Debug)]
struct Program {
    pp: u64,
    lines: Vec<Line>,
    vars: Vec<Var>,
    end: bool,
    debug: bool,
    libs: Vec<Lib>,
}

#[derive(Clone,Debug)]
struct Var {
    name: String,
    value: String,
}

#[derive(Debug)]
#[derive(Clone)]
struct Line {
    instruction: String,
    opperhand: Vec<String>,
}
#[derive(Clone,Debug)]
struct Lib{
    path: String,
    func:Vec<String>,
}
impl Program {
    fn newVar(mut self, nameN: &String, valueN: &String,)-> Program {
        
        for i in &mut self.vars {
            if &i.name == nameN {
                let pS = self.set(nameN,valueN);
                return pS;
            }
        }
        self.vars.push(
            Var {
                name: nameN.to_string(),
                value: valueN.to_string(),
            }
        );
        if self.debug{
            println!("\nDEBUG {},N{},V{}",self.vars.len(),nameN,valueN);
        }
        return self;
        
    }
}


impl Line {
    fn perform(&self, mut program: Program)->Program {
        let mut programTR = program.clone();
        match self.instruction.as_str() {
            "ptl" => Self::ptl(self, &program),
            "pt" => Self::pt(self, &program),
            "sav" => { programTR = program.newVar(&self.opperhand[0], &self.opperhand[1]) },
            "set" => { programTR = program.set(&self.opperhand[0], &self.opperhand[1]) },
            "mov" => {  programTR =program.mov(&self.opperhand[0], &self.opperhand[1]) },
            "jmp" => { programTR =
                match &self.opperhand[0].parse::<i64>() {
                    Ok(number) => {
                        program.jmp(*number)
                    }
                    Err(e) => {
                        panic!("{}", e.to_string())
                    }
                }
            }
            "usi"=>{programTR = program.usi(self.opperhand[0].to_string())},
            "rnd"=>{ programTR =program.rnd(self.opperhand[0].to_string(),self.opperhand[1].parse::<i64>().expect("rnd NOT A NUMBER"),self.opperhand[2].parse::<i64>().expect("rnd NOT A NUMBER"))},
            "add"=>{programTR =program.add(self.opperhand[0].to_string(),self.opperhand[1].to_string())},
            "sub"=>{programTR =program.sub(self.opperhand[0].to_string(), self.opperhand[1].to_string())},
            "dif"=>{ programTR =program.dif(self.opperhand[0].to_string(), self.opperhand[1].to_string(), self.opperhand[2].to_string(), self.opperhand[3].to_string().parse::<i64>().expect("dif NOT A NUMBER")) },
            "end"=>{end(program)},
            "init"=>{
                let path = self.opperhand[0].clone();
                let fullPath:String = "./".to_string()+path.as_str()+".so";
                if program.debug{
                    println!("path{}",fullPath);
                }
                unsafe {
                    let lib = Library::new(fullPath).unwrap();
                    let func: Symbol<unsafe extern fn()->Vec<String>> = lib.get(b"getFuncs").unwrap();
                    let funcs = func();
                    program.libs.push(
                        Lib{
                            path:path,
                            func: funcs,
                        }
                    )
                }
                programTR = program
            }
            _=>{
                let mut skip = false;
                for i in program.clone().libs{
                    if program.debug{
                        println!("Libs{:?}",i.func)
                    }
                    for name in i.func{
                        if program.debug{
                            println!("Libs:{},ins:{}",name,name.to_string() == self.instruction.to_string())
                        }
                        if name.to_string() == self.instruction.to_string() {
                            unsafe {
                                let lib = Library::new("./".to_string()+i.path.to_string().as_str()+".so").unwrap();
                                let func: Symbol<unsafe extern fn(Program,Vec<String>)->Program> = lib.get(self.instruction.as_bytes()).unwrap();
                                let returnValue = func(program.clone(),self.opperhand.clone());
                                if program.debug{
                                    println!("{:?}",returnValue);
                                }
                                skip =true;
                                return returnValue;
                            }
                        }
                    }
                    
                }
                if !skip {
                    println!("error:invalid instruction");
                    panic!("e:{}", self.instruction)
                }
                
            }
        };
        return programTR;
    }
}