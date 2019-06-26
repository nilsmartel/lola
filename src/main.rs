use bitvec::Bitvec;
use bytecode::Compile;
use output::Output;
use rayon::prelude::*;
use std::collections::HashMap;

mod ast;
mod bitvec;
mod bytecode;
mod output;
mod parse;
mod stack_machine;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let (equation, expr) = parse::expression(&input).expect("Failed to parse expression");

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

    let results = evaluate(&bytecode, len);

    let info = Output {
        symbols,
        equation,
        results,
    };

    println!("{}", info.fmt_csv());
}

/// Executes the provided function (in form of bytecode) for all possible states
fn evaluate(code: &Vec<bytecode::Code>, symbol_count: usize) -> Vec<bool> {
    (0usize..1 << symbol_count)
        .into_par_iter()
        .map(|index| stack_machine::StackMachine::new(Bitvec::new(index), code).evaluate())
        .collect()
}
