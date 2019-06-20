# Logic

Parser, Compiler and Interpreted for logical expressions,
outputting Truthtables in Markdown and CSV format.

## about Logical Expressions
logical expressions consist of variables/symbols, the constants 0 and 1 and the operations:
    - And (^)
    - Or  (|)
    - Implication (->)
    - Biimplication (<->)
    - Not

## Examples of Logical Expressions
```
a -> b

1 | a

a ^ (b -> c)

a | (1^c) | (a^b^1 -> 0)
```


## TODO
- ~~implementing Parser~~
- ~~Bytecode generation~~
- implement Stack Machine to interpret bytecode
- implement evalutuation of all possible states for expression
- Markdown output
- CSV output
- Extend README with Truthtables (generated by `logic`) showing the behaviour of all operations {^, |, ->, <->} 