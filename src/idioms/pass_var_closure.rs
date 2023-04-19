use std::rc::Rc;

pub fn passing_nums() {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);

    /* The principle lesson here is that you con't need to create variables outside the closure. Pass them INTO the closure.
    so, don't do this outside the closure:
    let num2_cloned = num2.cloned();
    */
    let closure = {
        let num2 = num2.clone(); // cloned
        let num3 = num3.as_ref(); // borrowed

        move || *num1 + *num2 * num3;
    };
}
