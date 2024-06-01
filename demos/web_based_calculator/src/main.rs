#![deny(warnings)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use leptos::logging::log;
use leptos::*;
use rusty_dumb_tools::calculator::*;
use web_sys::MouseEvent;

const ENABLE_LOGGING: bool = false;
const DISPLAY_LEN: usize = 14;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! { <App/> }
    });
}

fn App() -> impl IntoView {
    // create an instance of DumbCalculator and wrap it in a RefCell, so that it can be got back as mutable
    let calculator_ref = RefCell::new(DumbCalculator::new());
    let (angle_mode, set_angle_mode) = create_signal(String::from("deg"));
    let (alt_mode, set_alt_mode) = create_signal(false);
    let (clicked_value, set_clicked_value) = create_signal(String::from(""));
    let (history, set_history) = create_signal(String::from(""));
    let on_clicked = move |ev: MouseEvent| {
        let value = event_target_value(&ev);
        set_clicked_value.set(value);
    };
    view! {
        <div class="container">
            // display row
            <div class="item display"> {
                // since want to re-render when "clicked_value" signal changes, need to use a closure
                move || {
                    // get the calculator instance and make it mutable
                    let mut calculator = calculator_ref.borrow_mut();
                    // get the "input" from the signal
                    let mut clicked_chars = clicked_value.get();
                    if clicked_chars == "alt" {
                        let alt = alt_mode.get();
                        set_alt_mode.set(!alt);
                        clicked_chars = "".to_owned()
                    } else if clicked_chars == "am" {
                        let angle_mode = angle_mode.get();
                        let new_angle_mode = if angle_mode == "deg" { "rad" } else { "deg" };
                        calculator.use_angle_mode(angle_mode.as_str());
                        set_angle_mode.set(new_angle_mode.to_string());
                        clicked_chars = "".to_owned()
                    }
                    if clicked_chars == "<" {
                        calculator.undo();
                    } else if clicked_chars == "ac" {
                        calculator.reset();
                    } else if !clicked_chars.is_empty() {
                        calculator.push(clicked_chars.as_str());
                    }
                    // get the calculator display
                    let display = calculator.get_display_sized(DISPLAY_LEN);
                    // get the calculator history
                    let history = calculator.get_history_string(true);
                    // get the "operator" indicator of the calculator
                    let op_indicator = get_op_indicator(&calculator);
                    // get the "bracket" indicator of the calculator
                    let bracket_indicator = get_bracket_indicator(&calculator);
                    if ENABLE_LOGGING {
                        log!("* display:[{}] ... history:[{:?}]", display, history);
                    }
                    match &history {
                        // set the "history" signal
                        Some(history) => set_history.set(history.to_string()),
                        None => set_history.set("".to_owned()),
                    }
                    // return the view that represents the calculator display
                    view! {
                        <div class="display_digits_div">
                            <div class="display_indicator_div">
                                <span class="display_indicator_span">{op_indicator}</span>
                                <span class="display_indicator_span">{bracket_indicator}</span>
                            </div>
                            {
                                display.chars().map(|c| {
                                    let c = if c == ' ' { "".to_owned() } else { c.to_string() };
                                    view! {
                                        <span class="display_digit_span">{c}</span>
                                    }
                                }).collect_view()
                            }
                        </div>
                    }
                }
            }
            </div>

            // keys row 1
            { move || view! {
                <div class="item key"><button class="key_button" style="color:green; background-color:ivory" on:click=on_clicked value="am">{
                    let angle_mode = angle_mode.get();
                    angle_mode.to_uppercase()
                }</button></div>
            }}
            { move || {
                let alt = alt_mode.get();
                let style = if alt { "color:darkgreen; background-color:orange" } else { "color:blue; background-color:lightgray" };
                view! {
                <div class="item key"><button class="key_button" style={style} on:click=on_clicked value="alt">{
                    //let angle_mode = angle_mode.get();
                    "ALT"
                }</button></div>
            }}}
            <div class="item key"><button class="key_button" on:click=on_clicked value="^">x<span class="ss_span">y</span></button></div>
            { move || {
                let alt = alt_mode.get();
                if alt { view! {
                        <div class="item key"><button class="key_button alt_key" on:click=on_clicked value="asin">{"sin"}<span class="ss_span">-1</span></button></div>
                        <div class="item key"><button class="key_button alt_key" on:click=on_clicked value="acos">{"cos"}<span class="ss_span">-1</span></button></div>
                        <div class="item key"><button class="key_button alt_key" on:click=on_clicked value="atan">{"tan"}<span class="ss_span">-1</span></button></div>
                    }
                } else { view! {
                        <div class="item key"><button class="key_button" on:click=on_clicked value="sin">{"sin"}</button></div>
                        <div class="item key"><button class="key_button" on:click=on_clicked value="cos">{"cos"}</button></div>
                        <div class="item key"><button class="key_button" on:click=on_clicked value="tan">{"tan"}</button></div>
                    }
                }
            }}


            // keys row 2
            <div class="item key"><button class="key_button" on:click=on_clicked value="square">x<span class="ss_span">2</span></button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value="sqrt">{"√"}</button></div>
            { move || {
                let alt = alt_mode.get();
                if alt {
                    view! {
                        <div class="item key"><button class="key_button alt_key" on:click=on_clicked value="PI">{"𝞹"}</button></div>
                        <div class="item key"><button class="key_button alt_key" on:click=on_clicked value="E">{"e"}</button></div>
                    }
                } else {
                    view! {
                        <div class="item key"><button class="key_button" on:click=on_clicked value="inv">{"1/x"}</button></div>
                        <div class="item key"><button class="key_button" on:click=on_clicked value="abs">{"|x|"}</button></div>
                    }
                }
            }}
            <div class="item key" style="background-color:lightyellow"><button class="key_button" on:click=on_clicked value="(">{"("}</button></div>
            <div class="item key" style="background-color:lightyellow"><button class="key_button" on:click=on_clicked value=")">{")"}</button></div>

            // keys row 3
            <div class="item key"><button class="key_button" on:click=on_clicked value="pow10">10<span class="ss_span">x</span></button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=7>{"7️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=8>{"8️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=9>{"9️⃣"}</button></div>
            <div class="item key span2" style="background-color:orangered"><button class="key_button" on:click=on_clicked value="ac">{"AC"}</button></div>

            // keys row 4
            <div class="item key"><button class="key_button" on:click=on_clicked value="log">{"log"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=4>{"4️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=5>{"5️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=6>{"6️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value="*">{"✖️"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value="/">{"➗"}</button></div>

            // keys row 5
            <div class="item key"><button class="key_button" on:click=on_clicked value="ln">{"ln"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=1>{"1️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=2>{"2️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=3>{"3️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value="+">{"➕"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value="-">{"➖"}</button></div>

            // keys row 6
            <div class="item key"><button class="key_button" on:click=on_clicked value="%">{"%"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value="neg">{"±"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=0>{"0️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_clicked value=".">{"•"}</button></div>
            <div class="item key span2" style="background-color:lightgreen"><button class="key_button" on:click=on_clicked value="=">{"🟰"}</button></div>

            // history row
            <div class="item history span5"> {
                // again, since the history portion will be updated when the "history" signal changes, need to use a closure
                move || view! {
                    {history.get()}
                }
            } </div>
            <div class="item key" style="background-color:tomato"><button class="key_button" on:click=on_clicked value="<">{"⬅"}</button></div>
        </div>
    }
}

// turn the "operator" indicator to something more human readable
fn get_op_indicator(calculator: &DumbCalculator) -> &'static str {
    let operator = calculator.get_last_operator();
    match operator {
        Some(operator) => match operator.as_str() {
            "+" => "+",
            "-" => "-",
            "*" => "x",
            "/" => "÷",
            _ => " ",
        },
        None => " ",
    }
}

// turn the "bracket" indicator to something more human readable
fn get_bracket_indicator(calculator: &DumbCalculator) -> &'static str {
    match calculator.count_opened_brackets() {
        1 => "⑴", // ⑴ ⑵ ⑶ ⑷ ⑸ ⑹ ⑺ ⑻ ⑼ ⑽ ⑾ ⑿ ⒀ ⒁ ⒂ ⒃ ⒄ ⒅ ⒆ ⒇
        2 => "⑵",
        3 => "⑶",
        4 => "⑷",
        5 => "⑸",
        6 => "⑹",
        7 => "⑺",
        8 => "⑻",
        9 => "⑼",
        10 => "⑽",
        _ => " ",
    }
}
