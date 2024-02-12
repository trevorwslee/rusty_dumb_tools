//#![deny(warnings)]
//#![allow(unused)]
#![allow(non_snake_case)]

use leptos::*;
use rusty_dumb_tools::prelude::*;


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

fn App() -> impl IntoView {
    let mut calculator = DumbCalculator::new();
    //let pressed = create_signal("0");
    view! {
        <table>
            <tr><td colspan=5>
             {move || {
                (0..12).map(|_| {
                    view! {
                        <span class="display_digit">
                            { 0 }
                        </span>
                     }
                }).collect_view()
             }}
            </td></tr>
            <tr>
                <td class="key_digit">{"7️⃣"}</td>
                <td class="key_digit">{"8️⃣"}</td>
                <td class="key_digit">{"9️⃣"}</td>
                <td class="key_digit" colspan=2>{"AC"}</td>
            </tr>
            <tr>
                <td class="key_digit">{"4️⃣"}</td>
                <td class="key_digit">{"5️⃣"}</td>
                <td class="key_digit">{"6️⃣"}</td>
                <td class="key_digit">{"✖️"}</td>
                <td class="key_digit">{"➗"}</td>
            </tr>
            <tr>
                <td class="key_digit">{"1️⃣"}</td>
                <td class="key_digit">{"2️⃣"}</td>
                <td class="key_digit">{"3️⃣"}</td>
                <td class="key_digit">{"➕"}</td>
                <td class="key_digit">{"➖"}</td>
            </tr>
            <tr>
                <td class="key_digit">{"±"}</td>
                <td class="key_digit">{"0️⃣"}</td>
                <td class="key_digit">{"•"}</td>
                <td class="key_digit" colspan=2>{"🟰"}</td>
            </tr>
        </table>
    }
}
