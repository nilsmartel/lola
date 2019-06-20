mod bitvec;
use bitvec::Bitvec;
use bytecode::Compile;
use std::collections::HashMap;
mod ast;
mod bytecode;
mod parse;
mod stack_machine;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let expr = parse::expression(&input).expect("Failed to parse expression");

    let mut symbols: Vec<String> = expr.get_symbols().into_iter().collect();
    symbols.sort_by(|a, b| a.cmp(b));

    // Just don't want to do it that way around
    if symbols.len() > 128 {
        eprintln!("This program is not suited to solve expressions with over 128 variables");
        std::process::exit(0);
    }

    // maps symbols to addresses
    let addresses = symbols
        .iter()
        .cloned()
        .zip(0..)
        .collect::<HashMap<String, u8>>();

    let bytecode = expr.compile(&addresses);

    let results = (0u128..1 << symbols.len())
        .into_iter()
        .map(|bitvec| stack_machine::StackMachine::new(Bitvec::new(bitvec), &bytecode).evaluate())
        .enumerate()
        .map(|(code, result)| {
            let mut row: Vec<bool> = symbols
                .iter()
                .map(|s| dbg!(code & 1 << addresses[s]) > 0)
                .collect();
            row.push(result);
            row
        })
        .collect::<Vec<Vec<bool>>>();

    println!("{:#?}", results);
}
