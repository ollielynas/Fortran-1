mod compute;
use compute::*;
use log::info;
use log::Level;
use std::collections::HashMap;
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
    pub print: String,
    

}
#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub struct LineData {
    pub number: i32,
    pub continuation: bool,
    pub comment: bool,
    pub label: i32,
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug);
    
    sycamore::render(|cx| {
        let sense_switches = create_signal(cx, vec![(0, false); 20].iter().enumerate().map(|(i, x)| {
            (i, x.1)
        }).collect::<Vec<(usize, bool)>>());

        let sense_lights = create_signal(cx, vec![false; 4]);
        let input = create_signal(cx, "".to_owned());
        
        let line_num_node_ref = create_node_ref(cx);
        let line_info: &Signal<Vec<LineData>> = create_signal(cx, vec![]);

        let a = create_signal(cx, "".to_owned());


        let tokens = create_memo(cx, move || process(input.get().to_string(), line_info.get().to_vec()));
        let display = create_signal(cx, 0);
        let stop_light = create_signal(cx, true);

        let edit_line_text = create_signal(cx, "".to_string());

        let do_loop = create_signal(cx, false);
        let current_line = create_signal(cx, 0);

        let variables = create_signal(cx, HashMap::<String, Vec<Token>>::new());
        let do_statements = create_signal(cx, vec![]);
        



        // let print_text = cre

        let io = create_memo(cx, move || IO704 {
            sense_switches: sense_switches.get().iter().map(|(i,x)| *x).collect::<Vec<bool>>(),
            sense_lights: sense_lights.get().to_vec(),
            display: 0,
            stop_light: true,
            print: "".to_string(),
        });

        // create_effect(cx, || {
        //     sense_lights = io.get().sense_lights.clone();
        //     sense_switches = io.get().sense_switches.clone();
            
        // });

        create_effect(cx, || {
            let num_of_lines = input.get().replace("\n","\n ").lines().count();
            if num_of_lines == 0 {return}
            let mut new_line_info:Vec<LineData> = (*line_info.get_untracked()).clone();
            // get element by id with web sys
            let document = web_sys::window().unwrap().document().unwrap();
            let edit = document.get_element_by_id("lineNo").unwrap();
            let current_line2 = edit.inner_html().parse::<usize>().unwrap();
            
            if num_of_lines > new_line_info.len() {
                let new_data = LineData {
                    number: 0,
                    continuation: false,
                    comment: false,
                    label: 0,
                };
                if current_line2 >= new_line_info.len() {
                    new_line_info.push(new_data);
                } else {
                    new_line_info.insert(current_line2, new_data);
                }
            }
            if num_of_lines < new_line_info.len() {
                if current_line2 >= new_line_info.len() {
                    new_line_info.pop();
                } else {
                    new_line_info.remove(current_line2-1);
                }
                // new_line_info.remove(current_line2-1);
            }
            
        
            for i in 0..new_line_info.len() {
                new_line_info[i].number = i as i32;
            }

            line_info.set(new_line_info);

        });
        

        

        view! { cx,

        div(class="io") {
            div(class="horizontal labeled") {
            p{("Sense Lights")}
            Keyed(
                iterable=sense_lights,
                view=|cx, x| view! { cx,
                    input(type="radio", checked=x) {}
                },
                key=|x| *x,
            )
        
        }
        div(class="horizontal labeled") {
            p{("Stop Light")}
            input(type="radio", checked=*stop_light.get()) {}
        }

            div(class="horizontal labeled") {
            p{("Sense Switches")}
            Keyed(
                iterable=sense_switches,
                view= move |cx, x| view! { cx,
                    input(type="checkbox", class="input-button", on:click=move |_|{
                        sense_switches.set(sense_switches.get().iter().map(|(i, y)| {
                            if i == &x.0 {
                                (*i, !y)
                            } else {
                                (*i, *y)
                            }
                        }).collect::<Vec<(usize, bool)>>());
                    }, checked=x.1) {}
                    
                },
                key=|x| *x,
            )
            }

            button(class="start", on:click=move |_| {
                do_loop.set(true);
                current_line.set(0);
            }) {
                "start"
            }
        }

            div(class="card") {
                div(class="card-header") {
                    div(class="card-header-title") {
                        "Punch Card"
                    }
                }
                div(class="card-content") {
                    div(class="content") {
                        div(class="comment") {
                            Keyed(
                                iterable=line_info,
                                view=move |cx, x| view! { cx,
                                    div(class="comment-box", on:click=move |_| {
                                        line_info.set(line_info.get().iter().map(|y| {
                                            if y.number == x.number {
                                                LineData {
                                                    comment: !y.comment,
                                                    ..y.clone()
                                                }
                                            } else {
                                                *y
                                            }
                                        }).collect::<Vec<LineData>>());
                                    }) {
                                        (if x.comment {
                                            "C"
                                        } else {
                                            " "
                                        })
                                    }
                                },
                                key=|x| *x,
                            )
                            
                        }
                        div(class="line-number") {
                            Keyed(
                                iterable=line_info,
                                view=move |cx, x| view! { cx,div(class="line-number-box"){
                                    div(class="move_me"){(if x.label != 0 {
                                        x.label.to_string()
                                    } else {
                                        " ".to_string()
                                    })}
                                    input(class="invisible",onclick="this.select();setTimeout(function () {this.select()}, 100)", type="text",id="line_input", bind:value=a,
                                    on:click=move |_| {
                                        if x.label != 0 {
                                            a.set(x.label.to_string());
                                        }else {
                                            a.set("".to_string());
                                        }
                                    },
                                     on:focusout=move |_| {
                                        line_info.set(line_info.get().iter().map(|z| {
                                            if z.number == x.number {
                                                LineData {
                                                    label: a.get().parse::<i32>().unwrap_or(0).max(0),
                                                    ..z.clone()
                                                }
                                            } else {
                                                *z
                                            }
                                        }).collect::<Vec<LineData>>());
                                    }) {
                                    }
                                }

                                },
                                key=|x| *x,
                            )
                        }
                        div(class="continuation") {
                            Keyed(
                                iterable=line_info,
                                view=move |cx, x| view! { cx,
                                    div(class="continuation-box", on:click=move |_| {
                                        line_info.set(line_info.get().iter().map(|y| {
                                            if y.number == x.number {
                                                LineData {
                                                    continuation: !y.continuation,
                                                    ..y.clone()
                                                }
                                            } else {
                                                *y
                                            }
                                        }).collect::<Vec<LineData>>());
                                    }) {
                                        (if x.continuation {
                                            "x"
                                        } else {
                                            " "
                                        })
                                    }
                                },
                                key=|x| *x,
                            )
                        }
                        div(ref=line_num_node_ref, id="lineNo") {}
                        textarea(class="not-epic",id="text-edit",bind:value=input, 
                        wrap="on",
                        spellcheck=false,
                        oninput=format!("this.style.height = 'calc( ( 1em + 7px ) * {} )'", 8.0*((input.get().lines().count()+3)as f32/8.0).ceil()),
                        onkeyup="getLineNumber(this, document.getElementById('lineNo'));", onmouseup="this.onkeyup();"
                    ) {}
                            
                        }

                    }
                }

            

            

            button(class="run",id="run-click",disabled=!*do_loop.get(), on:click=move |_| {
                if *current_line.get() == 0 {
                sense_lights.set(vec![false;4]);
                variables.set(HashMap::<String, Vec<Token>>::new());
                do_statements.set(vec![]);
                }

                let mut update_io = false;

                // let mut a = 0;
                if *do_loop.get() {
                while !update_io {
                    let (loop_2, io_2, a2, do_st, vari, update_io2) = run(
                        tokens.get().to_vec(),
                        (*io.get()).clone(),
                        *current_line.get(),
                        (*variables.get()).clone(),
                        (*do_statements.get()).clone(),

                    );
                    update_io = update_io2;
                    current_line.set(a2);
                    do_loop.set(loop_2);
                    display.set(io_2.display);
                    variables.set(vari);
                    do_statements.set(do_st);
                    if update_io2 {
                        sense_lights.set(io_2.sense_lights.clone());
                        stop_light.set(io_2.stop_light);
                        sense_switches.set(io_2.sense_switches.iter().enumerate().map(|(i, x)| {
                            (i, *x)
                        }).collect::<Vec<(usize, bool)>>());
                    }

                }
            }
            }) {
                "Run"
            }
        }
    });
}
