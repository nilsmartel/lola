use bitvec::Bitvec;
use bytecode::Compile;
use std::collections::HashMap;
use std::sync::mpsc;

mod ast;
mod bitvec;
mod bytecode;
mod parse;
mod stack_machine;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let (string, expr) = parse::expression(&input).expect("Failed to parse expression");

    let mut symbols: Vec<String> = expr.get_symbols().into_iter().collect();
    symbols.sort_by(|a, b| a.cmp(b));

    let len = symbols.len();
    if len > std::mem::size_of::<usize>() * 8 {
        eprintln!(
            "This program is not suited to solve expressions with over {} variables",
            std::mem::size_of::<usize>() * 8
        );
        std::process::exit(0);
    }

    // maps symbols to addresses
    let addresses = symbols
        .iter()
        .cloned()
        .zip(0..)
        .collect::<HashMap<String, u8>>();

    let bytecode = expr.compile(&addresses);

    struct Information {
        symbols: Vec<String>,
        string: String,
        results: Vec<bool>,
    }

    let mut info = Information {
        symbols,
        string,
        results: vec![false; len],
    };

    use std::sync::mpsc::{Receiver, Sender};
    let (sender, receiver): (Sender<(usize, bool)>, Receiver<(usize, bool)>) = mpsc::channel();

    // Iterate over Receiver to get information about progress

    for (idx, value) in receiver {
        info.results[idx] = value;
    }
}

/// Executes the provided function (in form of bytecode) for all possible states
fn evaluate(
    code: &'static Vec<bytecode::Code>,
    symbol_count: usize,
    sender: mpsc::Sender<(usize, bool)>,
) -> Vec<std::thread::JoinHandle<()>> {
    (0usize..1 << symbol_count)
        .map(|index| {
            let sender = sender.clone();
            std::thread::spawn(move || {
                let result = stack_machine::StackMachine::new(Bitvec::new(index), code).evaluate();
                sender.send((index, result)).unwrap();
            })
        })
        .collect()
}
