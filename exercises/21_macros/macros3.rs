// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // use macros::my_macro;
    my_macro!();
}
