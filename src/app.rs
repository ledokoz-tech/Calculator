#![allow(non_snake_case)]

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/styles.css");

#[derive(Props, PartialEq)]
struct ButtonProps {
    label: String,
    on_click: EventHandler<MouseEvent>,
    class: Option<String>,
}

fn CalcButton(props: ButtonProps) -> Element {
    let button_class = match &props.class {
        Some(class) => format!("calc-button {}", class),
        None => "calc-button".to_string(),
    };
    
    rsx! {
        button {
            class: "{button_class}",
            onclick: move |evt| props.on_click.call(evt),
            "{props.label}"
        }
    }
}

#[component]
fn Calculator() -> Element {
    let mut current_input = use_signal(|| "0".to_string());
    let mut operator = use_signal(|| None::<char>());
    let mut previous_input = use_signal(|| None::<String>());

    let handle_number = move |num: &str| {
        let mut current = current_input();
        
        if *current == "0" || current.is_empty() {
            current.set(num.to_string());
        } else {
            current.set(format!("{}{}", *current, num));
        }
    };

    let handle_operator = move |op: char| {
        if !current_input().is_empty() {
            previous_input.set(Some(current_input().clone()));
            operator.set(Some(op));
            current_input.set("0".to_string());
        }
    };

    let handle_equals = move || {
        if let Some(prev_val) = &*previous_input() {
            if let Ok(prev) = prev_val.parse::<f64>() {
                if let Ok(current) = current_input().parse::<f64>() {
                    let result = match *operator() {
                        Some('+') => prev + current,
                        Some('-') => prev - current,
                        Some('*') => prev * current,
                        Some('/') => prev / current,
                        Some('√') => current.sqrt(),
                        _ => current,
                    };
                    
                    // Handle invalid operations like square root of negative numbers
                    if result.is_nan() || result.is_infinite() {
                        current_input.set("Error".to_string());
                    } else {
                        // Format the result to avoid unnecessary decimals
                        if result.fract() == 0.0 {
                            current_input.set(format!("{}", result as i64));
                        } else {
                            // Round to 6 decimal places to avoid floating point precision issues
                            current_input.set(format!("{}", (result * 1_000_000.0).round() / 1_000_000.0));
                        }
                    }
                    
                    operator.set(None);
                    previous_input.set(None);
                }
            }
        }
    };

    let handle_clear = move || {
        current_input.set("0".to_string());
        operator.set(None);
        previous_input.set(None);
    };

    let handle_decimal = move || {
        let current = current_input();
        if !current.contains('.') {
            current_input.set(format!("{}.{}", *current, "."));
        }
    };

    rsx! {
        link { rel: "stylesheet", href: CSS }
        div { 
            class: "calculator-container",
            div {
                class: "calculator-display",
                "{current_input}"
            }
            div {
                class: "calculator-row",
                CalcButton { label: "C".to_string(), on_click: move |_| handle_clear() }
                CalcButton { label: "√".to_string(), on_click: move |_| {
                    if let Ok(val) = current_input().parse::<f64>() {
                        if val >= 0.0 {
                            let result = val.sqrt();
                            current_input.set(result.to_string());
                        } else {
                            current_input.set("Error".to_string());
                        }
                    } else {
                        current_input.set("Error".to_string());
                    }
                }}
                CalcButton { label: "/".to_string(), on_click: move |_| handle_operator('/') }
                CalcButton { label: "*".to_string(), on_click: move |_| handle_operator('*') }
            }
            div {
                class: "calculator-row",
                CalcButton { label: "7".to_string(), on_click: move |_| handle_number("7") }
                CalcButton { label: "8".to_string(), on_click: move |_| handle_number("8") }
                CalcButton { label: "9".to_string(), on_click: move |_| handle_number("9") }
                CalcButton { label: "-".to_string(), on_click: move |_| handle_operator('-') }
            }
            div {
                class: "calculator-row",
                CalcButton { label: "4".to_string(), on_click: move |_| handle_number("4") }
                CalcButton { label: "5".to_string(), on_click: move |_| handle_number("5") }
                CalcButton { label: "6".to_string(), on_click: move |_| handle_number("6") }
                CalcButton { label: "+".to_string(), on_click: move |_| handle_operator('+') }
            }
            div {
                class: "calculator-row",
                CalcButton { label: "1".to_string(), on_click: move |_| handle_number("1") }
                CalcButton { label: "2".to_string(), on_click: move |_| handle_number("2") }
                CalcButton { label: "3".to_string(), on_click: move |_| handle_number("3") }
                CalcButton { 
                    label: "=".to_string(), 
                    on_click: move |_| handle_equals(),
                    class: "equals-button"
                }
            }
            div {
                class: "calculator-row",
                CalcButton { label: "0".to_string(), on_click: move |_| handle_number("0") }
                CalcButton { label: ".".to_string(), on_click: move |_| handle_decimal() }
            }
        }
    }
}

pub fn App() -> Element {
    rsx! {
        Calculator {}
    }
}