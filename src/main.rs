#![allow(non_snake_case)]

use dioxus::prelude::*;

// Define the calculator's state
#[derive(Default)]
struct CalculatorState {
    display: String,
    current_value: Option<f64>,
    operator: Option<char>,
    waiting_for_second_operand: bool,
}

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    // Initialize the calculator state using use_state hook
    let state = use_signal(CalculatorState::default);

    rsx! {
        div {
            class: "calculator",
            // Display component
            div {
                class: "display",
                "{state.read().display}" // Display the current value from the state
            },

            // Number buttons
            { (1..=9).map(|i| {
                let num_str = i.to_string();
                rsx! {
                    button {
                        class: "button",
                        onclick: move |_| handle_number(&state, num_str.clone()),
                        "{i}"
                    }
                }
            }) },

            // Zero button (spans two columns)
            button {
                class: "button zero",
                onclick: move |_| handle_number(&state, "0".to_string()),
                "0"
            },

            // Operator buttons
            button { class: "button operator", onclick: move |_| handle_operator(&state, '+'), "+" },
            button { class: "button operator", onclick: move |_| handle_operator(&state, '-'), "-" },
            button { class: "button operator", onclick: move |_| handle_operator(&state, '*'), "*" },
            button { class: "button operator", onclick: move |_| handle_operator(&state, '/'), "/" },

            // Square Root button
            button { class: "button operator", onclick: move |_| handle_square_root(&state), "√" },

            // Clear button
            button { class: "button clear", onclick: move |_| handle_clear(&state), "C" },

            // Equals button
            button { class: "button equals", onclick: move |_| handle_equals(&state), "=" }
        }
    }
}

// Event handler for number button clicks
fn handle_number(state: &Signal<CalculatorState>, num_str: String) {
    state.modify(move |s| {
        if s.waiting_for_second_operand {
            s.display = num_str.clone();
            s.waiting_for_second_operand = false;
        } else {
            // Append number to display, handling leading zeros
            s.display = if s.display == "0" { num_str } else { s.display.clone() + &num_str };
        }
    });
}

// Event handler for operator button clicks
fn handle_operator(state: &Signal<CalculatorState>, op: char) {
    state.modify(move |s| {
        if let Ok(value) = s.display.parse::<f64>() {
            s.current_value = Some(value);
        }
        s.operator = Some(op);
        s.waiting_for_second_operand = true; // Next number will start a new input
    });
}

// Event handler for the equals button
fn handle_equals(state: &Signal<CalculatorState>) {
    state.modify(move |s| {
        if let (Some(val1), Some(op), Some(val2)) = (s.current_value, s.operator, s.display.parse::<f64>().ok()) {
            let result = match op {
                '+' => val1 + val2,
                '-' => val1 - val2,
                '*' => val1 * val2,
                '/' => if val2 != 0.0 { val1 / val2 } else { f64::NAN }, // Handle division by zero, use NaN for error
                _ => val2, // Should not happen with current operators
            };
            s.display = result.to_string();
            s.current_value = Some(result); // Store result for potential chained operations
            s.operator = None;
            s.waiting_for_second_operand = false;
        }
    });
}

// Event handler for square root button
fn handle_square_root(state: &Signal<CalculatorState>) {
    state.modify(move |s| {
        if let Ok(value) = s.display.parse::<f64>() {
            if value >= 0.0 {
                let result = (value as f64).sqrt();
                s.display = result.to_string();
                s.current_value = Some(result);
                s.operator = None;
                s.waiting_for_second_operand = false;
            } else {
                // Handle error for negative square root
                s.display = "Error".to_string();
                *s = CalculatorState::default(); // Reset state after error
            }
        } else {
            // Handle parsing error for display value
            s.display = "Error".to_string();
            *s = CalculatorState::default(); // Reset state after error
        }
    });
}

// Event handler for the clear button
fn handle_clear(state: &Signal<CalculatorState>) {
    state.modify(|s| {
        *s = CalculatorState::default(); // Reset to initial state
    });
}
