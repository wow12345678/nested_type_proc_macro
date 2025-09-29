use rep_proc_macro_test::nested_type;

#[test]
fn simple() {
    let test: nested_type!(2, Vec, u32, "<>") = vec![vec![3]];

    println!("{}", test[0][0]);
}

#[test]
fn not_so_simple() {
    struct Foo {
        bar: nested_type!(2, Vec, u32, "<>"),
    }

    let test = Foo {
        bar: vec![vec![2, 3, 4, 5]],
    };

    println!("{:?}", test.bar);
}

// #[test]
// fn variable_size() {
//     const SIZE:u32 = 3;
//
//     struct Foo {
//         bar: nested_type!(size, Vec, u32, "<>"),
//     }
//
//     let test = Foo {
//         bar: vec![vec![2, 3, 4, 5]],
//     };
//
//     println!("{:?}", test.bar);
// }
