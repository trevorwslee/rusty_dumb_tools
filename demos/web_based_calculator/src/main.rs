#![deny(warnings)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use leptos::{logging::log, *};
use rusty_dumb_tools::{calculator, prelude::*};
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
    // DumbCalculator settings to enable undo and history
    let settings = DumbCalculatorSettings {
        enable_undo: true,
        enable_history: true,
        ..DumbCalculatorSettings::default()
    };
    // create an instance of DumbCalculator and wrap it in a RefCell, so that it can be got back as mutable
    let calculator_ref = RefCell::new(DumbCalculator::new_ex(settings));
    let (pressed_key, set_pressed_key) = create_signal(String::from(""));
    let (history, set_history) = create_signal(String::from(""));
    let on_key_pressed = move |ev: MouseEvent| {
        let pressed_chars = event_target_value(&ev);
        set_pressed_key.set(pressed_chars);
    };
    view! {
        <div class="container">
            // display row
            <div class="item display"> {
                // since not to re-render when "key pressed" signal changes, need to use a closure
                move || {
                    // get the calculator instance and make it mutable
                    let mut calculator = calculator_ref.borrow_mut();
                    // get the "input key" from the signal
                    let pressed_chars = pressed_key.get();
                    if pressed_chars == "<" {
                        calculator.undo();
                    } else if pressed_chars == "ac" {
                        calculator.reset();
                    } else if !pressed_chars.is_empty() {
                        calculator.push(pressed_chars.as_str());
                    }
                    let display = calculator.get_display_sized(DISPLAY_LEN);
                    let history = calculator.get_history_string(true);
                    let op_indicator = get_op_indicator(&calculator);
                    let bracket_indicator = get_bracket_indicator(&calculator);
                    if ENABLE_LOGGING {
                        log!("* display:[{}] ... history:[{:?}]", display, history);
                    }
                    match &history {
                        Some(history) => set_history.set(history.to_string()),
                        None => set_history.set("".to_string()),
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
                                    let c = if c == ' ' { "".to_string() } else { c.to_string() };
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
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="sin">{"sin"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="cos">{"cos"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="tan">{"tan"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="asin">{"sin"}<span class="ss_span">-1</span></button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="acos">{"cos"}<span class="ss_span">-1</span></button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="atan">{"tan"}<span class="ss_span">-1</span></button></div>

            // keys row 2
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="square">x<span class="ss_span">2</span></button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="sqrt">{"√"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="inv">{"1/x"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="abs">{"|x|"}</button></div>
            <div class="item key" style="background-color:lightyellow"><button class="key_button" on:click=on_key_pressed value="(">{"("}</button></div>
            <div class="item key" style="background-color:lightyellow"><button class="key_button" on:click=on_key_pressed value=")">{")"}</button></div>

            // keys row 3
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="pow10">10<span class="ss_span">x</span></button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=7>{"7️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=8>{"8️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=9>{"9️⃣"}</button></div>
            <div class="item key span2" style="background-color:orange"><button class="key_button" on:click=on_key_pressed value="ac">{"AC"}</button></div>

            // keys row 4
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="log">{"log"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=4>{"4️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=5>{"5️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=6>{"6️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="*">{"✖️"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="/">{"➗"}</button></div>

            // keys row 5
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="ln">{"ln"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=1>{"1️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=2>{"2️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=3>{"3️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="+">{"➕"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="-">{"➖"}</button></div>

            // keys row 6
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="%">{"%"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value="neg">{"±"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=0>{"0️⃣"}</button></div>
            <div class="item key"><button class="key_button" on:click=on_key_pressed value=".">{"•"}</button></div>
            <div class="item key span2" style="background-color:lightgreen"><button class="key_button" on:click=on_key_pressed value="=">{"🟰"}</button></div>

            // history row 6
            <div class="item history span5"> {
                // again, since the history portion will be updated when the "history" signal changes, need to use a closure
                move || view! {
                    {history.get()}
                }
            } </div>
            <div class="item key" style="background-color:tomato"><button class="key_button" on:click=on_key_pressed value="<">{"⬅"}</button></div>
        </div>
    }
}

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
