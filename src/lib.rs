// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        terms: vec![Term::new()],
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
    terms: Vec<Term>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    CreateTerm,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::CreateTerm => model.terms.push(Term::new()),
    }
}

// ------ ------
//     View
// ------ ------

fn view_term(term: &Term) -> Node<Msg> {
    div![
        C!["term"],
        input![
            C!["input"],
            attrs! {
                At::Value => term.word,
            }
        ],
        term.lambda.to_string(),
    ]
}

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    let mut ret: Vec<Node<Msg>> = vec![];
    for term in &model.terms {
        ret.push(view_term(term));
    }
    ret
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
