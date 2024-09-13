use serde_json::json;

use simulation::{compute_taxes, Op};
use std::io::{self, BufRead};

mod simulation;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }

        // Parse JSON input
        let operations: Vec<Op> = serde_json::from_str(&line).expect("JSON inválido");
        let taxes = compute_taxes(&operations);
        // Print JSON output
        println!("{}", json!(taxes));
    }
}
