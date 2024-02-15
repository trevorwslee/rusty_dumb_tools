#![deny(warnings)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use leptos::*;
use rusty_dumb_tools::{calculator, prelude::*};
use web_sys::MouseEvent;

const DISPLAY_LEN: usize = 14;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! { <App/> }
    });
}

fn App() -> impl IntoView {
    let settings = DumbCalculatorSettings {
        enable_undo: true,
        enable_history: true
    };
    let mut calculator_ref = RefCell::new(DumbCalculator::new_ex(settings));
    let (pressed_key, set_pressed_key) = create_signal(String::from(""));
    let (history, set_history) = create_signal(String::from(""));
    let on_key_pressed = move |ev: MouseEvent| {
        let pressed_chars = event_target_value(&ev);
        set_pressed_key.set(pressed_chars);
    };
    view! {
        <table class="main_table">
            <tr><td class="display_td" colspan=6> {
                move || {
                    let mut calculator = calculator_ref.borrow_mut();
                    let pressed_chars = pressed_key.get();
                    if pressed_chars == "<" {
                        calculator.undo();
                    } else if pressed_chars == "ac" {
                        calculator.reset();
                    } else if !pressed_chars.is_empty() {
                        calculator.push(pressed_chars.as_str());
                    }
                    let display = calculator.get_display_sized(DISPLAY_LEN);
                    let history = calculator.get_history_string();
                    let op_indicator = get_op_indicator(&calculator);
                    let bracket_indicator = get_bracket_indicator(&calculator);
                    match &history {
                        Some(history) => set_history.set(history.to_string()),
                        None => set_history.set("".to_string()),
                    }
                    view! {
                        // { match &history {
                        //     Some(hist) => {
                        //         view! { {format!("hist: {:?}", history)} }
                        //     },
                        //     None => view! { {"none".to_string()} },
                        // } }
                        <div class="display_indicator_div">
                            <span class="display_indicator_span">{op_indicator}</span>
                            <span class="display_indicator_span">{bracket_indicator}</span>
                        </div>
                        <div>
                            {
                                display.chars().map(|c| {
                                    let c = if c == ' ' { "".to_string() } else { c.to_string() };
                                    view! {
                                        <span class="display_digit_button">{c}</span>
                                    }
                                }).collect_view()
                            }
                        </div>
                    }
                }
            }
            </td></tr>
            <tr>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="sin">{"sin"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="cos">{"cos"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="tan">{"tan"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="asin">{"sin"}<span class="ss_span">-1</span></button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="acos">{"cos"}<span class="ss_span">-1</span></button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="atan">{"tan"}<span class="ss_span">-1</span></button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="square">x<span class="ss_span">2</span></button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="sqrt">{"√"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="inv">{"1/x"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="abc">{"|x|"}</button></td>
                <td class="key_td" style="background-color:lightyellow"><button class="digit_button" on:click=on_key_pressed value="(">{"("}</button></td>
                <td class="key_td" style="background-color:lightyellow"><button class="digit_button" on:click=on_key_pressed value=")">{")"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="pow10">10<span class="ss_span">x</span></button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=7>{"7️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=8>{"8️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=9>{"9️⃣"}</button></td>
                <td class="key_td" style="background-color:orange" colspan=2><button class="digit_button" on:click=on_key_pressed value="ac">{"AC"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="log">{"log"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=4>{"4️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=5>{"5️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=6>{"6️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="*">{"✖️"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="/">{"➗"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="ln">{"ln"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=1>{"1️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=2>{"2️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=3>{"3️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="+">{"➕"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="-">{"➖"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="%">{"%"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value="neg">{"±"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=0>{"0️⃣"}</button></td>
                <td class="key_td"><button class="digit_button" on:click=on_key_pressed value=".">{"•"}</button></td>
                <td class="key_td" style="background-color:lightgreen" colspan=2><button class="digit_button" on:click=on_key_pressed value="=">{"🟰"}</button></td>
            </tr>
            <tr>
                <td class="history_td" colspan=5> {
                    move || view! {
                        <div class="history_div">{history.get()}</div>
                    }
                } </td>
                <td class="key_td" style="background-color:tomato"><button class="digit_button" on:click=on_key_pressed value="<">{"⬅"}</button></td>
            </tr>
        </table>
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
