use subprocess::{Exec, Redirection};

pub fn lightblue(input: &str) {
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .capture()
        .expect("lightblue maybe exists");
    dbg!(captured.stdout_str());
}
