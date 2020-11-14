use subprocess::{Exec, Redirection};

pub fn lightblue() {
    let input = "太郎はカステラが好きだ。";
    let command = format!("echo {} | lightblue parse -s xml", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .capture()
        .expect("lightblue maybe exists");
    dbg!(captured.stdout_str());
}
