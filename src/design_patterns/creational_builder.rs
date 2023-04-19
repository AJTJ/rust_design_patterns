/*
https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
https://crates.io/crates/derive_builder

ADVANTAGES
- Separates methods for building from other methods
- Prevents having tons of constructors
- Useful for one-liner initialisation, or for more complex construction

NOTES
- More common in rust because rust has no overloading
 */

#[derive(Debug, PartialEq)]
pub struct Foo {
    // many complicated fields
    bar: String,
}

impl Foo {
    // helpful method to help users to discover builder
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    // many optional fields
    bar: String,
}

impl FooBuilder {
    pub fn new(/* many properties here */) -> FooBuilder {
        FooBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    // avoid consuming the builder here
    // we can then use the FooBuilder as a template for constructing many Foos
    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}

#[test]
fn builder_tester() {
    let my_foo = Foo {
        bar: String::from("Y"),
    };

    let foo_from_builder: Foo = FooBuilder::new().name("Y".to_string()).build();

    assert_eq!(my_foo, foo_from_builder);
}
