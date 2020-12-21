use rsw_lib::add;


fn main() {
    let res = add(3, 2);
    println!("println result is {}", res);
    dbg!("Debug Result is {}", res);
    eprintln!("eprintln! Result is {}", res);

}
