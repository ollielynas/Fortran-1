mod compute;
use compute::*;
use log::info;
use log::Level;
// extern crate console_error_panic_hook;
use std::fmt::Display;
use std::pin::Pin;
use std::panic;
use sycamore::rt::Event;


use sycamore::prelude::*;
#[derive(Clone, Debug)]
pub struct IO704 {
    pub sense_switches: Vec<bool>,
    pub sense_lights: Vec<bool>,
    pub display: i32,
    pub stop_light: bool,
    

}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug);
    sycamore::render(|cx| {
        let sense_switches = create_signal(cx, vec![true; 20]);
        let sense_lights = create_signal(cx, vec![false; 4]);
        let current_card = create_signal(cx, 0);
        let input = create_signal(cx, "".to_owned());
        let tokens = create_memo(cx, move || process(input.get().to_string()));
        let display = create_signal(cx, 0);
        let stop_light = create_signal(cx, true);

        // let print_text = cre

        let io = create_memo(cx, move || IO704 {
            sense_switches: sense_switches.get().to_vec(),
            sense_lights: sense_lights.get().to_vec(),
            display: 0,
            stop_light: true,
        });

        // create_effect(cx, || {
        //     sense_lights = io.get().sense_lights.clone();
        //     sense_switches = io.get().sense_switches.clone();
            
        // });

        

        view! { cx,
            div(class="horizontal") {
            Keyed(
                iterable=sense_lights,
                view=|cx, x| view! { cx,
                    input(type="radio", checked=x, disabled=true) {}
                },
                key=|x| *x,
            )
        }
            div(class="horizontal") {
            Keyed(
                iterable=sense_switches,
                view= |cx, x| view! { cx,
                    input(type="checkbox") {}
                },
                key=|x| *x,
            )
            }

            div(class="card") {
                div(class="card-header") {
                    div(class="card-header-title") {
                        "Punch Card"
                    }
                }
                div(class="card-content") {
                    div(class="content") {
                        
                        textarea(bind:value=input) {
                            
                        }

                    }
                }

            }

            button(on:click=move |_| {
                info!("Button clicked!");
                let mut loop_ = true;
                sense_lights.set(vec![false;4]);
                let mut a = 0;
                while loop_ {
                    let (loop_2, io_2, a2) = run(tokens.get().to_vec(),(*io.get()).clone(), a);
                    a=a2;
                    info!("Looping");
                    loop_ = loop_2;
                    sense_lights.set(io_2.sense_lights.clone());
                    sense_switches.set(io_2.sense_switches.clone());
                    display.set(io_2.display);
                    stop_light.set(io_2.stop_light);
                }
            }) {
                "Compile"
            }


            p(){(format!("{:?}", io.get().display))}
            p{(format!("{:?}", tokens.get()))}
        }
    });
}
