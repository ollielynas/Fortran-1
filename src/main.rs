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
        let sense_switches = create_signal(cx, vec![true; 20]);
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
            sense_switches: sense_switches.get().to_vec(),
            sense_lights: sense_lights.get().to_vec(),
            display: 0,
            stop_light: true,
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
            info!("current line: {}", current_line2);
            info!("num of lines: {}", num_of_lines);
            
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
                    new_line_info.remove(current_line2);
                }
                // new_line_info.remove(current_line2-1);
            }
            
        
            for i in 0..new_line_info.len() {
                new_line_info[i].number = i as i32;
            }

            line_info.set(new_line_info);

        });
        

        

        view! { cx,
            div(class="horizontal") {
            Keyed(
                iterable=sense_lights,
                view=|cx, x| view! { cx,
                    input(type="radio", checked=x) {}
                },
                key=|x| *x,
            )
        }
        input(type="radio", class="on-off", checked=!*do_loop.get()) {}
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
                        div(class="comment") {
                            Keyed(
                                iterable=line_info,
                                view=move |cx, x| view! { cx,
                                    div(class="comment-box", on:click=move |_| {
                                        line_info.set(line_info.get().iter().map(|y| {
                                            if y.number == x.number {
                                                info!("clicked {} {}", x.number, y.number);
                                                LineData {
                                                    comment: !y.comment,
                                                    ..y.clone()
                                                }
                                            } else {
                                                *y
                                            }
                                        }).collect::<Vec<LineData>>());
                                        info!("clicked {:?}", line_info.get());
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
                                        a.set(x.label.to_string());
                                    },
                                     on:focusout=move |_| {
                                        line_info.set(line_info.get().iter().map(|z| {
                                            if z.number == x.number {
                                                info!("input {} {}", x.number, z.number);
                                                LineData {
                                                    label: a.get().parse::<i32>().unwrap_or(0).max(0),
                                                    ..z.clone()
                                                }
                                            } else {
                                                *z
                                            }
                                        }).collect::<Vec<LineData>>());
                                        info!("input {:?}", line_info.get());
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
                                                info!("clicked {} {}", x.number, y.number);
                                                LineData {
                                                    continuation: !y.continuation,
                                                    ..y.clone()
                                                }
                                            } else {
                                                *y
                                            }
                                        }).collect::<Vec<LineData>>());
                                        info!("clicked {:?}", line_info.get());
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

            

            button(class="start", on:click=move |_| {
                do_loop.set(true);
                current_line.set(0);
            }) {
                "start"
            }

            button(class="run",id="run-click",disabled=!*do_loop.get(), on:click=move |_| {
                if *current_line.get() == 0 {
                sense_lights.set(vec![false;4]);
                variables.set(HashMap::<String, Vec<Token>>::new());
                do_statements.set(vec![]);
                }

                // let mut a = 0;
                if *do_loop.get() {
                    let (loop_2, io_2, a2, do_st, vari) = run(
                        tokens.get().to_vec(),
                        (*io.get()).clone(),
                        *current_line.get(),
                        (*variables.get()).clone(),
                        (*do_statements.get()).clone(),

                    );
                    current_line.set(a2);
                    do_loop.set(loop_2);
                    sense_lights.set(io_2.sense_lights.clone());
                    sense_switches.set(io_2.sense_switches.clone());
                    display.set(io_2.display);
                    stop_light.set(io_2.stop_light);
                    variables.set(vari);
                    do_statements.set(do_st);


                }
            }) {
                "Compile"
            }


            p(){(format!("{:?}", io.get().display))}
            p{(format!("{:?}", tokens.get()))}
        }
    });
}
