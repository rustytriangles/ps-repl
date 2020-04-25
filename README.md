#
A simple REPL for a subset of the PostScript language. PostScript is a
stack based language with reverse Polish Notation. For example, the
following:

```
cargo run
>> 3 2 div =
 1.5
```

pushes the numbers 3 and 2 onto the stack. Then it executes the div command,
which pops them off, and pushes the result of dividing them back onto the stack.
Then it executes the = command, which pops the top of the stack and prints it.

This doesn't include any of the graphics features of PostScript, so it's really
only useful as an intro to how to write a simple REPL in Rust.

### Modules used
* [rand](https://docs.rs/rand/0.7.3/rand/) Random number generator
* [rustyline](https://docs.rs/rustyline/6.1.2/rustyline/) Input handling
