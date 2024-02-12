#![deny(warnings)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use leptos::*;
use rusty_dumb_tools::{calculator, prelude::*};
use web_sys::MouseEvent;

const DISPLAY_LEN: usize = 15;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || view! { <App/> });
}

fn App() -> impl IntoView {
    let mut calculator_ref = RefCell::new(DumbCalculator::new());
    let (pressed_key, set_pressed_key) = create_signal(String::from(""));
    let on_key_pressed = move |ev: MouseEvent| {
        let pressed_chars = event_target_value(&ev);
        set_pressed_key.set(pressed_chars);
    };
    view! {
        <table>
            <tr><td class="display_td" colspan=5>{
                move || {
                    let mut calculator = calculator_ref.borrow_mut();
                    let pressed_chars = pressed_key.get();
                    if pressed_chars == "ac" {
                        calculator.reset();
                    } else {
                        calculator.push(pressed_chars.as_str());
                    }
                    let display = calculator.get_display_sized(DISPLAY_LEN);
                    view! {
                        {display.chars().map(|c| {
                                let c = if c == ' ' { "_".to_string() } else { c.to_string() };
                                view! {
                                    <span class="display_digit_span">
                                        { c }
                                    </span>
                                }
                            }).collect_view()
                    }}
                }
            }
            </td></tr>
            <tr>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=7>{"7️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=8>{"8️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=9>{"9️⃣"}</button></td>
                <td class="key_td" colspan=2><button class="digit_span" on:click=on_key_pressed value="ac">{"AC"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=4>{"4️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=5>{"5️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=6>{"6️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value="*">{"✖️"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value="/">{"➗"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=1>{"1️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=2>{"2️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=3>{"3️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value="+">{"➕"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value="-">{"➖"}</button></td>
            </tr>
            <tr>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value="neg">{"±"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=0>{"0️⃣"}</button></td>
                <td class="key_td"><button class="digit_span" on:click=on_key_pressed value=".">{"•"}</button></td>
                <td class="key_td" colspan=2><button class="digit_span" on:click=on_key_pressed value="=">{"🟰"}</button></td>
            </tr>
        </table>
    }
}
