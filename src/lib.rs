mod xml_parse;
use std::io;
use subprocess::{Exec, Redirection};

pub fn lightblue(input: &str) -> Result<(), io::Error> {
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()
        .expect("lightblue should exists");
    let command_err = &captured.stderr_str();
    let xml_str = captured.stdout_str();
    if !command_err.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("lightblue error: {}", command_err),
        ));
    }
    let xml_struct = xml_parse::parse(&xml_str)
        .expect("xml parsing should success (assuming correct xml structure)");
    let parsed_input = xml_struct.document.sentences.sentence;
    dbg!(parsed_input);
    Ok(())
}
