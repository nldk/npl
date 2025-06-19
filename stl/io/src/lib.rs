struct Program {
    pp: u64,
    lines: Vec<Line>,
    vars: Vec<Var>,
    end: bool,
    debug: bool,
    libs: Vec<Lib>,
}
struct Var {
    name: String,
    value: String,
}
struct Line {
    instruction: String,
    opperhand: Vec<String>,
}
struct Lib{
    path: String,
    func:Vec<String>,
}

#[unsafe(no_mangle)]
pub extern "C" fn getFuncs()->Vec<String>{
    return vec!["print".to_string(),];
}

#[unsafe(no_mangle)]
pub extern "C" fn print(p:Program,opperhand:Vec<String>)->Program{
    println!("hallo");
    println!("{:?}",opperhand);
    return p;
}