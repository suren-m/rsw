use rsw_lib::add;

fn main() {
    let res = add(2, 2);
    println!("println result is {}", res);
    dbg!("Debug Result is {}", res);
    println!("eprintln! Result is {}", res);
}
