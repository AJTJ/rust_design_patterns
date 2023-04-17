mod MyMod {
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    #[non_exhaustive]
    pub enum AdmintMoreVariants {
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

    let some_enum = MyMod::AdmintMoreVariants::VarA;

    match some_enum {
        MyMod::AdmintMoreVariants::VarA => println!("is A"),
        MyMod::AdmintMoreVariants::VarB => println!("is B"),
        MyMod::AdmintMoreVariants::VarC { a } => println!("is C"),
    }
}
