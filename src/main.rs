pub mod constructors_default;
pub mod destructor;
pub mod iterate_option;
pub mod mem_take_replace;
pub mod non_exhaustive;
pub mod pass_var_closure;
pub mod ret_consumed_arg_on_error;
pub mod temporary_mutability;

fn main() {
    // println!("Hello, world!");
    // destructor::bar().unwrap();
    // iterate_option::it_op();
    // temporary_mutability::sorted_vec();
    ret_consumed_arg_on_error::returned_error();
}
