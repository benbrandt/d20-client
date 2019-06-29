use futures::Future;
use graphql_client::{GraphQLQuery, Response};
use js_sys::{Date, Promise};
use seed::prelude::*;
use seed::{
    attrs, button, class, div, empty, fetch, form, header, input, option, section, select, span,
    strong, style, Method, Request,
};
use serde::{Deserialize, Serialize};
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/graphql/roll.graphql",
    response_derives = "Clone, Debug"
)]
struct RollQuery;

#[derive(Debug, Deserialize, Serialize)]
struct Form {
    num: i32,
    die: i32,
    modifier: i32,
}

#[derive(Debug)]
struct RollWithTime {
    result: Option<roll_query::ResponseData>,
    time: Date,
}

// Model
struct Model {
    authentication: bool,
    error: Option<String>,
    form: Form,
    rolls: Vec<RollWithTime>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            authentication: false,
            error: None,
            form: Form {
                num: 1,
                die: 20,
                modifier: 0,
            },
            rolls: vec![],
        }
    }
}

// Update
#[derive(Clone, Deserialize)]
enum Msg {
    Authenticated(bool),
    ChangeDie(String),
    ChangeModifier(String),
    ChangeNum(String),
    GetRoll,
    Login,
    Logout,
    ReceiveRoll(Option<roll_query::ResponseData>),
    ReceiveError(Option<String>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::Authenticated(authentication) => model.authentication = authentication,
        Msg::ChangeDie(val) => model.form.die = val.parse().unwrap_or(model.form.die),
        Msg::ChangeModifier(val) => {
            model.form.modifier = val.parse().unwrap_or(model.form.modifier)
        }
        Msg::ChangeNum(val) => model.form.num = val.parse().unwrap_or(model.form.num),
        Msg::Login => {
            d20Login();
        }
        Msg::Logout => {
            d20Logout();
        }
        Msg::GetRoll => {
            orders.skip().perform_cmd(get_roll(roll_query::Variables {
                num: model.form.num.into(),
                die: model.form.die.into(),
                modifier: model.form.modifier.into(),
            }));
        }
        Msg::ReceiveRoll(result) => model.rolls.push(RollWithTime {
            result,
            time: Date::new_0(),
        }),
        Msg::ReceiveError(error) => model.error = error,
    }
}

fn get_roll(variables: roll_query::Variables) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(format!("{}/graphql", BACKEND_URL))
        .method(Method::Post)
        .body_json(&RollQuery::build_query(variables))
        .fetch_json_data(
            |r: fetch::ResponseDataResult<Response<roll_query::ResponseData>>| match r {
                Ok(response) => Msg::ReceiveRoll(response.data),
                Err(fail_reason) => Msg::ReceiveError(Some(format!("{:#?}", fail_reason))),
            },
        )
}

fn dice_option(form: &Form, die: i32) -> El<Msg> {
    let mut attributes = attrs! {At::Value => die};
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
            if let Some(r) = result {
                div![
                    class!["columns", "flex-centered", "py-2"],
                    div![
                        class!["column", "col-6", "h5", "text-right"],
                        format!("{:?}: ", r.roll.instruction),
                        strong![class!["text-large"], format!("{}", r.roll.total)],
                    ],
                    div![
                        class!["column", "col-6"],
                        strong!["Rolls: "],
                        r.roll
                            .rolls
                            .iter()
                            .map(|r| format!("{}", r))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ],
                    div![
                        class!["column", "col-12", "text-center"],
                        style! {"font-size" => "75%";},
                        String::from(time.to_locale_string("default", &JsValue::UNDEFINED))
                    ]
                ]
            } else {
                empty()
            }
        })
        .collect();
    div![class!["container"], roll_view]
}

// View
fn view(
    Model {
        authentication,
        error,
        form,
        rolls,
    }: &Model,
) -> El<Msg> {
    div![
        class!["container", "grid-lg", "p-2"],
        header![
            class!["navbar p-2"],
            section![
                class!["navbar-section"],
                span![class!["navbar-brand mr-2"], "Dice Roller"],
                button![
                    simple_ev(
                        Ev::Click,
                        if *authentication {
                            Msg::Logout
                        } else {
                            Msg::Login
                        }
                    ),
                    class!["btn", "btn-link", "btn-sm"],
                    if *authentication { "Log in" } else { "Log out" },
                ],
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
        roll_result(&rolls),
    ]
}

// Called by our JS entry point
#[wasm_bindgen]
pub fn render() {
    set_panic_hook();
    seed::App::build(Model::default(), update, view)
        // `trigger_update_handler` processes JS event
        // and forwards it to `update` function.
        .window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn d20Login() -> Promise;
    fn d20Logout();
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
