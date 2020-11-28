mod xml_parse;
use std::io::{Error, ErrorKind};
use subprocess::{Exec, Redirection};

pub fn lightblue(input: &str) -> Result<(), Error> {
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()
        .expect("lightblue should exists");
    let err = &captured.stderr_str();
    let xml_str = captured.stdout_str();
    if !err.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("lightblue error: {}", err),
        ));
    }
    let xml_struct = xml_parse::parse(&xml_str);
    dbg!(xml_struct);
    Ok(())
}
