mod xml_parse;
use subprocess::{Exec, Redirection};

pub fn lightblue(input: &str) {
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .capture()
        .expect("lightblue maybe exists");
    let xml = captured.stdout_str();
    let sentences = xml_parse::parse(&xml);
    dbg!(sentences);
}
