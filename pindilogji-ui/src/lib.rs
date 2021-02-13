// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

const ENTER_KEY: &str = "Enter";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        term: Term::new(),
        user_message: String::new(),
    }
}

// ------ ------
//     Model
// ------ ------
type Lambda = String;
struct Term {
    word: String,
    lambda: Lambda,
}

impl Term {
    fn new() -> Self {
        Term {
            word: String::default(),
            lambda: Lambda::default(),
        }
    }
}

// `Model` describes our app state.
struct Model {
    term: Term,
    user_message: String,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    WordChanged(String),
    SendTerm,
    Fetched(fetch::Result<String>),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WordChanged(new_word) => model.term.word = new_word,
        Msg::SendTerm => {
            orders.skip().perform_cmd({
                let word = model.term.word.clone();
                async { Msg::Fetched(send_message(word).await) }
            });
        }
        Msg::Fetched(Ok(lambda)) => {
            model.user_message = "success".to_string();
            model.term.lambda = lambda
        }
        Msg::Fetched(Err(error)) => model.user_message = format!("{:#?}", error),
    }
}

async fn send_message(new_word: String) -> fetch::Result<String> {
    Request::new("127.0.0.1:8080/newterm")
        .method(Method::Post)
        .json(&new_word)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let term = &model.term;
    div![
        C!["term"],
        input![
            C!["input"],
            attrs! {
                At::Value => term.word,
            },
            input_ev(Ev::Input, Msg::WordChanged),
            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                IF!(keyboard_event.key() == ENTER_KEY => Msg::SendTerm)
            }),
        ],
        button!("send", ev(Ev::Click, |_| Msg::SendTerm)),
        li!(&model.user_message),
        div![term.lambda.to_string()],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
