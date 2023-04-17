use std::mem;

// basically how to deal with switching an enums

pub enum MyEnum {
    A { name: String, num: i32 },
    B { name: String },
    C { dankness: i32 },
}

fn switcheroo(e: &mut MyEnum) {
    use MyEnum::*;
    *e = match e {
        A { name, .. } => B {
            name: mem::take(name),
        },
        B { .. } => C { dankness: 0 },
        C { dankness } => A {
            name: mem::take(dankness).to_string(),
            num: *dankness,
        },
    };
}
