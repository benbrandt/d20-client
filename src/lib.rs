use futures::Future;
use js_sys::Date;
use seed::prelude::*;
use seed::{
    attrs, button, class, div, empty, fetch, form, h2, input, option, select, span, strong, style,
    Method, Request,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::JsValue;
use web_sys::Event;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(debug_assertions)]
const BACKEND_URL: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
const BACKEND_URL: &str = "https://morning-eyrie-18336.herokuapp.com";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RollInstruction {
    num: i32,
    die: i32,
    modifier: i32,
}

impl fmt::Display for RollInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.num, self.die)?;
        if self.modifier != 0 {
            write!(
                f,
                " {} {}",
                if self.modifier > 0 { "+" } else { "-" },
                self.modifier
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Deserialize, Debug)]
struct DiceResult {
    die: i32,
    value: i32,
}

#[derive(Clone, Deserialize, Debug)]
struct RollResult {
    instruction: RollInstruction,
    rolls: Vec<DiceResult>,
    total: i32,
}

#[derive(Debug)]
struct RollWithTime {
    result: RollResult,
    time: Date,
}

// Model
struct Model {
    error: Option<String>,
    form: RollInstruction,
    rolls: Vec<RollWithTime>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            error: None,
            form: RollInstruction {
                num: 1,
                die: 20,
                modifier: 0,
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
    GetRoll(Event),
    ReceiveRoll(Box<fetch::FetchObject<RollResult>>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::ChangeDie(val) => model.form.die = val.parse().unwrap_or(model.form.die),
        Msg::ChangeModifier(val) => {
            model.form.modifier = val.parse().unwrap_or(model.form.modifier)
        }
        Msg::ChangeNum(val) => model.form.num = val.parse().unwrap_or(model.form.num),
        Msg::GetRoll(event) => {
            event.prevent_default();
            orders.skip().perform_cmd(get_roll(&model.form));
        }
        Msg::ReceiveRoll(fetch_object) => match fetch_object.response() {
            Ok(response) => model.rolls.push(RollWithTime {
                result: response.data,
                time: Date::new_0(),
            }),
            Err(fail_reason) => model.error = Some(format!("Error: {:#?}", fail_reason)),
        },
    }
}

fn get_roll(instruction: &RollInstruction) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(format!("{}/roll/", BACKEND_URL))
        .method(Method::Post)
        .body_json(instruction)
        .fetch_json(|r| Msg::ReceiveRoll(Box::new(r)))
}

fn dice_option(form: &RollInstruction, die: i32) -> El<Msg> {
    let mut attributes = attrs! {At::Value => die;};
    if form.die == die {
        attributes.add(At::Selected, "");
    }
    option![attributes, format!("d{}", die)]
}

fn roll_result(rolls: &[RollWithTime]) -> El<Msg> {
    let roll_view: Vec<El<Msg>> = rolls
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
                        .map(|r| format!("{}", r.value))
                        .collect::<Vec<String>>()
                        .join(", ")
                ],
                div![
                    class!["column", "col-12", "text-center"],
                    style! {"font-size" => "75%";},
                    String::from(time.to_locale_string("default", &JsValue::UNDEFINED))
                ]
            ]
        })
        .collect();
    div![class!["container"], roll_view]
}

// View
fn view(Model { error, form, rolls }: &Model) -> El<Msg> {
    div![
        class!["container", "grid-lg", "p-2"],
        h2![class!["pt-2", "text-center"], "Dice Roller"],
        match error {
            Some(e) => div![class!["toast", "toast-error"], e],
            None => empty(),
        },
        form![
            raw_ev(Ev::Submit, Msg::GetRoll),
            class!["p-2"],
            div![
                class!["input-group"],
                span![class!["input-group-addon"], "#"],
                input![
                    attrs! {
                        At::Class => "form-input";
                        At::Id => "num";
                        At::Min => 1;
                        At::Max => 99;
                        At::Name => "num";
                        At::Type => "number";
                        At::Value => form.num;
                    },
                    input_ev(Ev::Input, Msg::ChangeNum),
                ],
                span![class!["input-group-addon"], "d"],
                select![
                    attrs! {
                        At::Class => "form-select";
                        At::Id => "die";
                        At::Name => "die";
                        At::Value => form.die;
                    },
                    input_ev(Ev::Input, Msg::ChangeDie),
                    dice_option(form, 4),
                    dice_option(form, 6),
                    dice_option(form, 8),
                    dice_option(form, 10),
                    dice_option(form, 12),
                    dice_option(form, 20),
                    dice_option(form, 100),
                ],
                span![class!["input-group-addon"], "+"],
                input![
                    attrs! {
                        At::Class => "form-input";
                        At::Id => "modifier";
                        At::Name => "modifier";
                        At::PlaceHolder => "Modifier";
                        At::Type => "number";
                        At::Value => form.modifier;
                    },
                    input_ev(Ev::Input, Msg::ChangeModifier),
                ],
                button![
                    attrs! {At::Class => "btn btn-primary input-group-btn"; At::Type => "submit" },
                    "Roll"
                ]
            ]
        ],
        roll_result(&rolls),
    ]
}

// Called by our JS entry point
#[wasm_bindgen]
pub fn render() {
    set_panic_hook();
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
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
