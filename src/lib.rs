mod xml_parse;
use subprocess::{Exec, Redirection};

pub fn lightblue(input: &str) {
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()
        .expect("lightblue should exists");
    let err = &captured.stderr_str();
    let xml_str = captured.stdout_str();
    dbg!(&err.is_empty());
    dbg!(err);
    dbg!(&xml_str);
    let xml_struct = xml_parse::parse(&xml_str);
    dbg!(xml_struct);
}
