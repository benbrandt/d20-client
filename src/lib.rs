use seed::prelude::*;
use seed::{attrs, button, class, div, form, h1, input, option, select, span};

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

fn update(msg: Msg, model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
    }
}

// View
fn view(model: &Model) -> El<Msg> {
    div![
        class!["container", "grid-lg"],
        h1![class!["text-center"], "Dice Roller"],
        form![div![
            class!["input-group"],
            span![class!["input-group-addon"], "#"],
            input![attrs! {
                At::Class => "form-input";
                At::Id => "num";
                At::Min => 1;
                At::Max => 99;
                At::Name => "num";
                At::Type => "number";
                At::Value => 1;
            }],
            span![class!["input-group-addon"], "d"],
            select![
                attrs! {
                    At::Class => "form-select";
                    At::Id => "die";
                    At::Name => "die";
                },
                option![attrs! {At::Value => 4}, "d4"],
                option![attrs! {At::Value => 6}, "d6"],
                option![attrs! {At::Value => 8}, "d8"],
                option![attrs! {At::Value => 10}, "d10"],
                option![attrs! {At::Value => 12}, "d12"],
                option![attrs! {At::Value => 20}, "d20"],
                option![attrs! {At::Value => 100}, "d100"],
            ],
            span![class!["input-group-addon"], "+"],
            input![attrs! {
                At::Class => "form-input";
                At::Id => "modifier";
                At::Name => "modifier";
                At::PlaceHolder => "Modifier";
                At::Type => "number";
            }],
            button![
                attrs! {At::Class => "btn btn-primary input-group-btn"; At::Type => "submit" },
                "Roll"
            ]
        ]],
        button![
            simple_ev(Ev::Click, Msg::Increment),
            class!["btn"],
            format!("Hello, World × {}", model.val)
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
