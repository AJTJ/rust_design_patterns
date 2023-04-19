/*
- Advantages
  - Destructors almost always run
- Dis
  - Infinite loops will NOT run destructors
  - If a desctructor panics while unwinding, rust aborts the thread
  - Cannot be relied on if the code in the destructor NEEDS to run

  Seems like a bad place to leave essential code
 */

pub fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The dtor of _exit will run however the function `bar` is exited.
    let _exit = Foo;
    // Implicit return with `?` operator.
    // baz()?;
    // Normal return.
    Ok(())
}
