mod xml_parse;
use subprocess::{Exec, Redirection};

pub fn lightblue(input: &str) {
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .capture()
        .expect("lightblue maybe exists");
    let xml_str = captured.stdout_str();
    let xml_struct = xml_parse::parse(&xml_str);
    dbg!(xml_struct);
}
