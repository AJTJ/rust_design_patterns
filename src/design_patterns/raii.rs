/*

https://rust-unofficial.github.io/patterns/patterns/behavioural/RAII.html
 "Resource Acquisition is Initialization"
 - essentially, resource initialization is done in the constructor
 - and finalization in the destructor
 - a RAII object is used as a guard of some resource

 Mutex guards are the classic example.
*/

use std::ops::Deref;

struct Foo {}

impl Foo {
    pub fn foo(&self) {
        println!("it does foo");
    }
}

struct Mutex<T> {
    // we would keep a reference to our data here
    internal_data: T,
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
}

// locking the mutex is explicit
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        // lock the underlying OS mutex

        // ..

        // MutexGuard keeps a reference to self
        MutexGuard { data: self }
    }
}

// destructor for unlocking the mutex
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // unlock the underlying OS mutex
    }
}

// impling Deref means we can treat the mutexguard like a pointer to T
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

pub fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo();
}
