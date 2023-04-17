// https://rust-unofficial.github.io/patterns/idioms/ctor.html

// it is common to have BOTH an associated function `new` method and a `Default` impl

// default can be derived if all fields have Default
#[derive(Default)]
pub struct Time {
    value: u64,
    output: Option<Vec<i32>>,
    used: bool,
}

impl Time {
    pub fn new() -> Self {
        Time {
            value: 0,
            output: None,
            used: false,
        }
    }

    pub fn val(&self) -> u64 {
        self.value
    }
}

// if it cound not derive, you could implement manually
// impl Default for Time {
//     fn default() -> Self {
//         Self { value: 0 }
//     }
// }

pub fn construct() {
    let time = Time::default();

    let partial_time = Time {
        value: 10,
        ..Default::default()
    };
}
