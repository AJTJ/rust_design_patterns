/*
https://rust-unofficial.github.io/patterns/patterns/structural/compose-structs.html

 */
struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}

fn foo(a: &mut A) -> &u32 {
    &a.f2
}

fn bar(a: &mut A) -> u32 {
    a.f1 + a.f3
}

fn baz(a: &mut A) {
    // this first mutable borrow means that we cannot do it again.
    let x = foo(a);

    // since a is borrowed, we cannot also have a mutable reference
    // interestingly the actual error is:
    // ERROR: cannot borrow `*a` as mutable more than once at a time
    // let y = bar(a);

    // this later usage of x, causes a to be borrowed for the rest of the fn
    println!("{x}");
}

// A is now composed of two structs - B and C.
struct A2 {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

// These functions take a B or C, rather than A2.
fn foo_2(b: &mut B) -> &u32 {
    &b.f2
}
fn bar_2(c: &mut C) -> u32 {
    c.f1 + c.f3
}

fn baz_2(a: &mut A2) {
    let x = foo_2(&mut a.b);
    // Now it's OK!
    let y = bar_2(&mut a.c);
    println!("{}", x);
}
