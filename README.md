# Lola - Logic Expression Evaluator
Parser, Compiler and Interpreted for logical expressions,
outputting Truthtables in Markdown and CSV format.

## about Logical Expressions
logical expressions consist of variables/symbols, the constants 0 and 1 and the operations:
    - And (^)
    - Or  (|)
    - Implication (->)
    - Biimplication (<->)
    - Not (-)

## Examples of Logical Expressions
```
-a | a

1 ^ a

a -> b

1 | a

a ^ (b -> c)

a | (1^c) | (a^-b^1 -> 0)
```

## Why is it called `lola`?
Since this programm parses source code (in form of logical expressions) and parses the AST, it is more or less an logic compiler. (We'll ignore, that it also evaluates these statements)
From the perspective of `lola`, those expressions are source code, making logic an programming language.
Now, by convention programming languages often attach the keyword `lang` behind their respective names, to be easier identified as programming languages.
That is why you'll hear some people write/talk about `golang` or `rustlang`, when they mean `go` and `rust`.
Following this convention, `lola` becomes a compiler for `logiclang`.
You see what I did there, `lola` is just the short form of `logiclang`.



## TODO
- ~~implementing Parser~~
- ~~Bytecode generation~~
- ~~implement Stack Machine to interpret bytecode~~
- ~~implement evalutuation of all possible states for expression~~
- ~~implement parrallel evalutation~~
- ~~read from file~~
- CLI
- Markdown output
- ~~CSV output~~
- ~~short output (1 instead of true, 0 instead of false)~~
- *(important)* Verify results! _crazy I never did that, right??_
- Extend README with Truthtables (generated by `lola`) showing the behaviour of all operations {^, |, ->, <->}
