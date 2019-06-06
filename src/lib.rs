use futures::Future;
use seed::prelude::*;
use seed::{
    attrs, button, class, div, error, form, h1, h5, input, option, select, span, strong, Method,
    Request,
};
use serde::{Deserialize, Serialize};
use std::fmt;
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

// Model
struct Model {
    form: RollInstruction,
    rolls: Vec<RollResult>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
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
    OnFetchErr(JsValue),
    ReceiveRoll(RollResult),
}

fn update(msg: Msg, Model { form, rolls }: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::ChangeDie(val) => form.die = val.parse().unwrap_or(form.die),
        Msg::ChangeModifier(val) => form.modifier = val.parse().unwrap_or(form.modifier),
        Msg::ChangeNum(val) => form.num = val.parse().unwrap_or(form.num),
        Msg::GetRoll(event) => {
            event.prevent_default();
            orders.skip().perform_cmd(get_roll(form));
        }
        Msg::OnFetchErr(err) => {
            error!(format!("Fetch error: {:?}", err));
            orders.skip();
        }
        Msg::ReceiveRoll(result) => rolls.push(result),
    }
}

fn get_roll(instruction: &RollInstruction) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(&format!("{}/roll/", BACKEND_URL))
        .method(Method::Post)
        .body_json(instruction)
        .fetch_json()
        .map(Msg::ReceiveRoll)
        .map_err(Msg::OnFetchErr)
}

fn dice_option(form: &RollInstruction, die: i32) -> El<Msg> {
    let mut attributes = attrs! {At::Value => die;};
    if form.die == die {
        attributes.add(At::Selected, "");
    }
    option![attributes, format!("d{}", die)]
}

fn roll_result(rolls: &[RollResult]) -> El<Msg> {
    let roll_view: Vec<El<Msg>> = rolls
        .iter()
        .rev()
        .map(|result| {
            let roll_data: Vec<El<Msg>> = result
                .rolls
                .iter()
                .map(|r| div![format!("d{}: ", r.die), strong![format!("{}", r.value)]])
                .collect();
            div![
                class!["columns"],
                h5![
                    class!["col-6 p-2 text-right"],
                    format!("{}: ", result.instruction),
                    strong![class!["text-large"], format!("{}", result.total)],
                ],
                div![class!["col-6 p-2"], roll_data],
            ]
        })
        .collect();
    div![class!["container"], roll_view]
}

// View
fn view(Model { form, rolls }: &Model) -> El<Msg> {
    div![
        class!["container", "grid-lg"],
        h1![class!["text-center"], "Dice Roller"],
        form![
            raw_ev(Ev::Submit, Msg::GetRoll),
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
