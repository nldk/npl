use crate::{Line, Program};

pub fn parch(file:String,program: &mut Program,debug:bool) ->Program{
    let lines = file.lines().collect::<Vec<&str>>();
    let mut program_lines:Vec<Line> = Vec::new();
    for i in lines{
        let instruction_and_arguments = i.split_once(" ").unwrap();
        let (instruction,arguments) = instruction_and_arguments;
        let arguments = arguments.to_string();
        let arguments = split_Arguments(arguments,debug);
        if debug{
            println!("{:?}",arguments);
        }
        program_lines.push(Line{
            instruction:instruction.to_string(),
            arguments: arguments,
        })
    }
    if debug{
        println!("{:?}", program_lines);
    }
    program.lines = program_lines;
    program.to_owned()
}

fn split_Arguments(string:String,debug:bool)->Vec<String>{
    if debug {
        println!("string{:?}",string);
    }
    let mut arguments:Vec<String> = Vec::new();
    let mut isString = false;
    let mut argument:String = String::new();
    for i in string.chars(){
        match i { 
            '"' =>{isString=!isString;},
            ','=>{
                if debug{
                    println!("isSting:{:?},{}",isString,i);
                }
                if isString{
                    argument.push(i);
                }else {
                    if debug{
                        println!("argument{:?}",argument);
                        
                    }
                    arguments.push(argument.clone());
                    argument = String::new();
                }
            },
            _=>{argument.push(i);}
        }
    }
    arguments.push(argument);
    arguments
}