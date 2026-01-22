#![allow(non_snake_case)]

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/styles.css");

#[derive(Props, PartialEq, Clone)]
struct ButtonProps {
    label: String,
    on_click: EventHandler<MouseEvent>,
}

fn CalcButton(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "calc-button",
            onclick: move |evt| props.on_click.call(evt),
            "{props.label}"
        }
    }
}

#[component]
fn Calculator() -> Element {
    let mut current_input = use_signal(|| "0".to_string());
    let mut operator = use_signal(|| Option::<char>::None);
    let mut previous_input = use_signal(|| Option::<String>::None);

    let handle_number = move |num: &str| {
        let mut current = current_input.write();
        
        if *current == "0" {
            *current = num.to_string();
        } else {
            *current = format!("{}{}", *current, num);
        }
    };

    let handle_operator = move |op: char| {
        if !current_input().is_empty() {
            *previous_input.write() = Some(current_input().clone());
            *operator.write() = Some(op);
            *current_input.write() = "0".to_string();
        }
    };

    let handle_equals = move || {
        if let Some(ref prev_val) = *previous_input.read() {
            if let Ok(prev) = prev_val.parse::<f64>() {
                if let Ok(current) = current_input().parse::<f64>() {
                    let result = match operator().unwrap_or_default() {
                        '+' => prev + current,
                        '-' => prev - current,
                        '*' => prev * current,
                        '/' => prev / current,
                        '√' => current.sqrt(),
                        _ => current,
                    };
                    
                    // Handle invalid operations like square root of negative numbers
                    if result.is_nan() || result.is_infinite() {
                        *current_input.write() = "Error".to_string();
                    } else {
                        // Format the result to avoid unnecessary decimals
                        let formatted_result = if result.fract() == 0.0 {
                            result as i64
                        } else {
                            (result * 1000000.0).round() / 1000000.0
                        };
                        
                        *current_input.write() = if result.fract() == 0.0 {
                            format!("{}", formatted_result as i64)
                        } else {
                            format!("{}", (result * 1000000.0).round() / 1000000.0)
                        };
                    }
                    
                    *operator.write() = Option::<char>::None;
                    *previous_input.write() = Option::<String>::None;
                }
            }
        }
    };

    let handle_clear = move || {
        *current_input.write() = "0".to_string();
        *operator.write() = Option::<char>::None;
        *previous_input.write() = Option::<String>::None;
    };

    let handle_decimal = move || {
        let current = current_input.read();
        if !current.contains('.') {
            let mut current = current_input.write();
            *current = format!("{}{}", *current, ".");
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
                            *current_input.write() = result.to_string();
                        } else {
                            *current_input.write() = "Error".to_string();
                        }
                    } else {
                        *current_input.write() = "Error".to_string();
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