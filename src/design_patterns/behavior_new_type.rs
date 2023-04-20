/*
When you need a type to behave similar to another type, or enforce some behavior at compile time.

- Purpose
  - Abstraction
    - Sharing impl details between types while controlling the interface.
- Advantages
  - The wrapped and wrapper types are not type compatible. And thus cannot be confused.
  - zero-cost Abstraction

- Disadvantages
  - You need a "pass through" method for every method you want to expose on the wrapped type, and every impl too.
 */

struct Foo {
    // some type
}
impl Foo {
    // fns that would NOT be present on Bar
}

// the newtype
pub struct Bar(Foo);

impl Bar {
    pub fn new() {}
    // etc
}

pub fn new_type() {
    // Foo and Bar are now incompatible
}
