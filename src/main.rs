pub mod constructors_default;
pub mod destructor;
pub mod iterate_option;
pub mod mem_take_replace;
pub mod non_exhaustive;
pub mod pass_var_closure;

fn main() {
    println!("Hello, world!");
    destructor::bar().unwrap();
    iterate_option::it_op();
}
