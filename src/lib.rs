use seed::prelude::*;
use seed::{attrs, button, class, div, form, h1, input, select, span};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Model
struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self { val: 0 }
    }
}

// Update
#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.val += 1,
    }
    Render.into()
}

// View
fn view(model: &Model) -> El<Msg> {
    div![
        class!["content", "section"],
        h1![class!["title", "has-text-centered"], "Dice Roller"],
        div![
            class!["container"],
            form![div![
                class!["field", "has-addons", "has-addons-centered"],
                div![
                    class!["control", "has-icons-left"],
                    input![attrs! {
                        At::Class => "input";
                        At::Id => "num";
                        At::Min => 1;
                        At::Max => 99;
                        At::Name => "num";
                        At::Type => "number"
                    }],
                    span![class!["icon", "is-small", "is-left"], "#"]
                ],
                div![
                    class!["control", "has-icons-left"],
                    span![
                        class!["select"],
                        select![attrs! {At::Id => "die"; At::Name => "die"}]
                    ],
                    span![class!["icon", "is-small", "is-left"], "d20"]
                ],
                div![
                    class!["control", "has-icons-left"],
                    input![attrs! {
                        At::Class => "input";
                        At::Id => "modifier";
                        At::Name => "modifier";
                        At::PlaceHolder => "Modifier";
                        At::Type => "number";
                    }],
                    span![class!["icon", "is-small", "is-left"], "+"]
                ],
                div![
                    class!["control"],
                    button![
                        attrs! {At::Class => "button is-primary"; At::Type => "submit" },
                        "Roll"
                    ]
                ],
            ]],
            button![
                simple_ev(Ev::Click, Msg::Increment),
                format!("Hello, World × {}", model.val)
            ]
        ]
    ]
}

// Called by our JS entry point
#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}

#[allow(dead_code)]
fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
}
