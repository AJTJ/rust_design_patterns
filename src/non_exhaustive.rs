mod MyMod {
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VarA,
        VarB,
        #[non_exhaustive]
        VarC {
            a: String,
        },
    }
}

pub fn print_matched_variants(s: MyMod::S) {
    let MyMod::S { foo: the_foo } = s;

    let some_enum = MyMod::AdmitMoreVariants::VarA;

    match some_enum {
        MyMod::AdmitMoreVariants::VarA => println!("is A"),
        MyMod::AdmitMoreVariants::VarB => println!("is B"),
        MyMod::AdmitMoreVariants::VarC { a } => println!("is C"),
    }
}
