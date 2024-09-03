# What I have learned on this project

**Imports :**
```rust
use lib_name
```
**Declare a function :**
Use `fn` to declare a function.
```rust
fn my_func() {}
```

### Print and retieve:

**Print a string on the output :** Use `println!("mystring")`
**Retrieving string from stdin :** Use `io::stdin.read_line(&mut variable_name)`, where `&` is the reference.

To catch errors from read_line, we must use `expect` (crashes program). If this funciton has not been called, compiler will display a warning.

```rust
let mut guess = String::new();
println!("You guessed: {}", guess);

let x = 5;
let y = 10;
println!("x = {x} and y + 2 = {}", y + 2);
// Resulsts in : x = 5 and y + 2 = 12
```

## Variables:

Declaring a variable:
```rust
let apple = 5
```
By default, in rust, variables are immutable. If we want to create a mutable variable, we must add `mut` in the declaration

Strings must be allocated with `String::new()`

## Match

match is cool, allows to map on all possible outputs for a method and adjust the comportment based on it

**Types**

To use a method of a type, we must use the syntax `Type::method_name`

## Crates

We love `cargo doc --open`