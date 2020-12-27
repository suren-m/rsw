use std::mem;

pub fn string_index() {
    let x = 10;
    x.to_string();

    "hello";

    let s1 = String::from("hello \u{1f980}🎉");
    let s2 = String::from("\u{1f980}🎉 Hello");

    let subset = &s1[0..5];
    println!("{}", subset);

    let subset = &s2[0..4];
    println!("{}", subset);

    // let subset = &s2[0..6];
    // println!("{}", subset);
}

pub fn array_size() {
    let a = [1, 2, 3, 4, 5, 6];
    let b = ["a", "hello", "\u{1f980}"];

    println!("array a occupies {} bytes", mem::size_of_val(&a));
    println!("array b occupies {} bytes", mem::size_of_val(&b));
}
