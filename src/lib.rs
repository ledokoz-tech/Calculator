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

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn start_app() {
    dioxus::launch(App);
}

#[cfg(target_family = "wasm")]
pub fn start_app() {
    dioxus::launch(App);
}

#[cfg(not(any(target_os = "android", target_family = "wasm")))]
pub fn start_app() {
    dioxus::launch::launch_cfg(App, dioxus::launch::LaunchConfig::new().with_window(dioxus::desktop::WindowBuilder::new().with_title("Calculator").with_inner_size((320.0, 480.0))));
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
                        onclick: move |_| handle_number(state, num_str.clone()),
                        "{i}"
                    }
                }
            }) },

            // Zero button (spans two columns)
            button {
                class: "button zero",
                onclick: move |_| handle_number(state, "0".to_string()),
                "0"
            },

            // Operator buttons
            button { class: "button operator", onclick: move |_| handle_operator(state, '+'), "+" },
            button { class: "button operator", onclick: move |_| handle_operator(state, '-'), "-" },
            button { class: "button operator", onclick: move |_| handle_operator(state, '*'), "*" },
            button { class: "button operator", onclick: move |_| handle_operator(state, '/'), "/" },

            // Square Root button
            button { class: "button operator", onclick: move |_| handle_square_root(state), "√" },

            // Clear button
            button { class: "button clear", onclick: move |_| handle_clear(state), "C" },

            // Equals button
            button { class: "button equals", onclick: move |_| handle_equals(state), "=" }
        }
    }
}

// Event handler for number button clicks
fn handle_number(mut state: Signal<CalculatorState>, num_str: String) {
    let mut state_guard = state.write();
    if state_guard.waiting_for_second_operand {
        state_guard.display = num_str.clone();
        state_guard.waiting_for_second_operand = false;
    } else {
        // Append number to display, handling leading zeros
        state_guard.display = if state_guard.display == "0" { num_str } else { state_guard.display.clone() + &num_str };
    }
}

// Event handler for operator button clicks
fn handle_operator(mut state: Signal<CalculatorState>, op: char) {
    let mut state_guard = state.write();
    if let Ok(value) = state_guard.display.parse::<f64>() {
        state_guard.current_value = Some(value);
    }
    state_guard.operator = Some(op);
    state_guard.waiting_for_second_operand = true; // Next number will start a new input
}

// Event handler for the equals button
fn handle_equals(mut state: Signal<CalculatorState>) {
    let mut state_guard = state.write();
    if let (Some(val1), Some(op), Some(val2)) = (state_guard.current_value, state_guard.operator, state_guard.display.parse::<f64>().ok()) {
        let result = match op {
            '+' => val1 + val2,
            '-' => val1 - val2,
            '*' => val1 * val2,
            '/' => if val2 != 0.0 { val1 / val2 } else { f64::NAN }, // Handle division by zero, use NaN for error
            _ => val2, // Should not happen with current operators
        };
        state_guard.display = result.to_string();
        state_guard.current_value = Some(result); // Store result for potential chained operations
        state_guard.operator = None;
        state_guard.waiting_for_second_operand = false;
    }
}

// Event handler for square root button
fn handle_square_root(mut state: Signal<CalculatorState>) {
    let mut state_guard = state.write();
    if let Ok(value) = state_guard.display.parse::<f64>() {
        if value >= 0.0 {
            let result = (value as f64).sqrt();
            state_guard.display = result.to_string();
            state_guard.current_value = Some(result);
            state_guard.operator = None;
            state_guard.waiting_for_second_operand = false;
        } else {
            // Handle error for negative square root
            state_guard.display = "Error".to_string();
            *state_guard = CalculatorState::default(); // Reset state after error
        }
    } else {
        // Handle parsing error for display value
        state_guard.display = "Error".to_string();
        *state_guard = CalculatorState::default(); // Reset state after error
    }
}

// Event handler for the clear button
fn handle_clear(mut state: Signal<CalculatorState>) {
    let mut state_guard = state.write();
    *state_guard = CalculatorState::default(); // Reset to initial state
}
