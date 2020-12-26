use pindilogji::{format_sr, lightblue};

fn main() {
    let input = "太郎はカステラが好きだ。しかし牛乳も好きだ。";
    let sr = lightblue(input).unwrap();
    println!("{}", format_sr(&sr));
}
