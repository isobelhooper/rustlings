// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

#[macro_use]  // See https://veykril.github.io/tlborm/decl-macros/minutiae/scoping.html
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
