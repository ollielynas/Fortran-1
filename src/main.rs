mod compute;
use compute::*;
use log::Level;
use log::info;
extern crate console_error_panic_hook;
use std::panic;

use sycamore::prelude::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug);
        sycamore::render(|cx| {
        let switches = create_signal(cx, vec![true;20]);
        let lights = create_signal(cx, vec![false;20]);
        let input = create_signal(cx,include_str!("input.txt").to_owned());
        let tokens = create_memo(cx, move || process(input.get().to_string()));
        view! { cx,
        div(class="horizontal") {
        Keyed(
            iterable=lights,
            view=|cx, x| view! { cx,
                input(type="radio", checked=x, disabled=true) {}
            },
            key=|x| *x,
        )
    }
        div(class="horizontal") {
        Keyed(
            iterable=switches,
            view=|cx, x| view! { cx,
                input(type="checkbox") {}
            },
            key=|x| *x,
        )
    }
        textarea(bind:value=input) {}
        p{(format!("{:?}", tokens.get()))}
    }});
}
