use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};


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
                break;
            }
        }
        if self.debug {
            println!("\nDEBUG:{}, N:{}, V:{}", self.vars.len(), nameTF, value);
        }
        if value == "".to_string() {
            panic!("variable not found at line {}. maybe you didnt declare the variable", self.pp + 1)
        }
        return value;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn getFuncs()->Vec<String>{
    //println!("Getting functions...");
    return vec!["ps".to_string(),];
}

#[unsafe(no_mangle)]
pub extern "C" fn ps(p:Program,opperhand:Vec<String>)->Program{
    let path = opperhand[0].as_str();
    if p.debug {
        println!("{}",path);
    }
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
    return p;
}