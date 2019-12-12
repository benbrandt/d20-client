#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
use futures::Future;
use js_sys::Date;
use seed::prelude::*;
use seed::{
    attrs, button, class, div, empty, error, fetch, form, header, input, option, section, select,
    span, strong, style, Method, Request,
};
use serde::Deserialize;
use std::fmt;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(debug_assertions)]
const BACKEND_URL: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
const BACKEND_URL: &str = "https://morning-eyrie-18336.herokuapp.com";

#[derive(Debug)]
struct Form {
    num: String,
    die: String,
    modifier: String,
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.num, self.die)?;
        let modifier: i32 = self.modifier.parse().unwrap_or_default();
        if modifier != 0 {
            write!(
                f,
                "{}{}",
                if modifier < 0 { "-" } else { "+" },
                modifier.abs()
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
/// Result of a roll
pub struct RollResult {
    /// The instruction passed in to roll the dice
    pub instruction: String,
    /// The results of all rolls made
    pub rolls: Vec<i32>,
    /// The total value of the entire roll
    pub total: i32,
}

#[derive(Clone, Debug)]
struct RollWithTime {
    result: RollResult,
    time: Date,
}

// Model
struct Model {
    error: Option<String>,
    form: Form,
    rolls: Vec<RollWithTime>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            error: None,
            form: Form {
                num: "1".into(),
                die: "20".into(),
                modifier: "0".into(),
            },
            rolls: vec![],
        }
    }
}

// Update
#[derive(Clone)]
enum Msg {
    ChangeDie(String),
    ChangeModifier(String),
    ChangeNum(String),
    GetRoll,
    ReceiveRoll(RollResult),
    ReceiveError,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeDie(val) => model.form.die = val,
        Msg::ChangeModifier(val) => model.form.modifier = val,
        Msg::ChangeNum(val) => model.form.num = val,
        Msg::GetRoll => {
            orders.skip().perform_cmd(get_roll(&model.form));
        }
        Msg::ReceiveRoll(result) => model.rolls.push(RollWithTime {
            result,
            time: Date::new_0(),
        }),
        Msg::ReceiveError => model.error = Some("Request failed".into()),
    }
}

fn get_roll(form: &Form) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(format!("{}/roll/?roll={}", BACKEND_URL, form))
        .method(Method::Get)
        .fetch_json_data(|r: fetch::ResponseDataResult<RollResult>| match r {
            Ok(response) => Msg::ReceiveRoll(response),
            Err(fail_reason) => {
                error(fail_reason);
                Msg::ReceiveError
            }
        })
}

fn dice_option(form: &Form, die: &str) -> Node<Msg> {
    let mut attributes = attrs! {At::Value => die};
    if form.die == die {
        attributes.add(At::Selected, "");
    }
    option![attributes, format!("d{}", die)]
}

fn roll_result(rolls: &[RollWithTime]) -> Node<Msg> {
    let roll_view: Vec<Node<Msg>> = rolls
        .iter()
        .rev()
        .map(|RollWithTime { result, time }| {
            div![
                class!["columns", "flex-centered", "py-2"],
                div![
                    class!["column", "col-6", "h5", "text-right"],
                    format!("{}: ", result.instruction),
                    strong![class!["text-large"], format!("{}", result.total)],
                ],
                div![
                    class!["column", "col-6"],
                    strong!["Rolls: "],
                    result
                        .rolls
                        .iter()
                        .map(|r| format!("{}", r))
                        .collect::<Vec<String>>()
                        .join(", ")
                ],
                div![
                    class!["column", "col-12", "text-center"],
                    style! {St::FontSize => "75%";},
                    String::from(time.to_locale_string("default", &JsValue::UNDEFINED))
                ]
            ]
        })
        .collect();
    div![class!["container"], roll_view]
}

// View
fn view(Model { error, form, rolls }: &Model) -> impl View<Msg> {
    div![
        class!["container", "grid-lg", "p-2"],
        header![
            class!["navbar p-2"],
            section![
                class!["navbar-section"],
                span![class!["navbar-brand mr-2"], "Dice Roller"],
            ]
        ],
        match error {
            Some(e) => div![class!["toast", "toast-error"], e],
            None => empty![],
        },
        form![
            raw_ev(Ev::Submit, |event| {
                event.prevent_default();
                Msg::GetRoll
            }),
            class!["p-2"],
            div![
                class!["input-group"],
                span![class!["input-group-addon"], "#"],
                input![
                    attrs! {
                        At::Class => "form-input",
                        At::Id => "num",
                        At::Min => 1,
                        At::Max => 99,
                        At::Name => "num",
                        At::Type => "number",
                        At::Value => form.num,
                    },
                    input_ev(Ev::Input, Msg::ChangeNum),
                ],
                span![class!["input-group-addon"], "d"],
                select![
                    attrs! {
                        At::Class => "form-select",
                        At::Id => "die",
                        At::Name => "die",
                        At::Value => form.die,
                    },
                    input_ev(Ev::Input, Msg::ChangeDie),
                    dice_option(form, "4"),
                    dice_option(form, "6"),
                    dice_option(form, "8"),
                    dice_option(form, "10"),
                    dice_option(form, "12"),
                    dice_option(form, "20"),
                    dice_option(form, "100"),
                ],
                span![class!["input-group-addon"], "+"],
                input![
                    attrs! {
                        At::Class => "form-input",
                        At::Id => "modifier",
                        At::Name => "modifier",
                        At::Placeholder => "Modifier",
                        At::Type => "number",
                        At::Value => form.modifier,
                    },
                    input_ev(Ev::Input, Msg::ChangeModifier),
                ],
                button![
                    attrs! {At::Class => "btn btn-primary input-group-btn",At::Type => "submit" },
                    "Roll"
                ]
            ]
        ],
        roll_result(rolls),
    ]
}

// Called by our JS entry point
#[wasm_bindgen(start)]
pub fn render() {
    set_panic_hook();
    seed::App::builder(update, view).build_and_start();
}

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
