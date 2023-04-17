pub fn sorted_vec() {
    let my_vec = vec![5, 3, 6, 2];

    fn get_vec(v: &[i32]) -> Vec<i32> {
        v.to_vec()
    }

    let data = {
        let mut data = get_vec(&my_vec);
        data.sort();
        data
    };

    println!("ini: {my_vec:?}, sorted: {data:?}");
}
