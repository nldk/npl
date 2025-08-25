use crate::{Line, Program};

pub fn parch(file:String,program: &mut Program,debug:bool) ->Program{
    let lines = file.lines().collect::<Vec<&str>>();
    let mut program_lines:Vec<Line> = Vec::new();
    for i in lines{
        let instruction_and_arguments = i.split_once(" ").unwrap();
        let (instruction,arguments) = instruction_and_arguments;
        let arguments = arguments.to_string();
        let arguments = arguments.split(",").collect::<Vec<&str>>();
        let mut arguments_to_put_in_the_line:Vec<String> = Vec::new();
        for e in arguments{
            arguments_to_put_in_the_line.push(e.to_string());
        }
        program_lines.push(Line{
            instruction:instruction.to_string(),
            arguments: arguments_to_put_in_the_line,
        })
    }
    if debug{
        println!("{:?}", program_lines);
    }
    program.lines = program_lines;
    program.to_owned()
}