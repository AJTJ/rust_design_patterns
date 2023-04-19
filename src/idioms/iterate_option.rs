pub fn it_op() {
    // Option impls `IntoIterator`
    let red_op = Some("red");
    let mut colors = vec!["yellow", "blue"];

    colors.extend(red_op);

    println!("{colors:?}");

    // you can also .chain()

    let mut just_orange = vec!["orange"];

    for c in just_orange.iter().chain(red_op.iter()) {
        println!("c in just_orange: {c}")
    }
}
