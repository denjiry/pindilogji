use std::io;
use subprocess::{Exec, Redirection};

fn parse_lightblue_sr(input: String) -> String {
    let mut top_level_sr = "";
    for (i, line) in input.lines().enumerate() {
        if i == 3 {
            top_level_sr = line;
        }
    }
    let top_level_sr = top_level_sr
        .splitn(3, ' ')
        .last()
        .expect("top_level_sr should not be empty.");
    let top_level_sr = top_level_sr
        .rsplitn(2, ' ')
        .last()
        .expect("top_level_sr should not be empty.");
    top_level_sr.to_owned()
}

pub fn lightblue(
    input: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command = format!("echo {} | lightblue parse -s text", input);
    let captured = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()
        .expect("lightblue should exists");
    let command_err = &captured.stderr_str();
    let output = captured.stdout_str();
    if !command_err.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("lightblue error: {}", command_err),
        )
        .into());
    }
    let semantic_representation = parse_lightblue_sr(output);
    Ok(semantic_representation)
}

pub fn format_sr(input: &str) -> String {
    let mut level = 0;
    let mut ret = String::new();
    for c in input.chars() {
        level += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        let new_c = match c {
            'x' => c.to_string(),
            _ => c.to_string(),
        };
        ret.push_str(&new_c);
    }
    ret
}
