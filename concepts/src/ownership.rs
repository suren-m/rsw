fn demo() {
    let x = "hello";
    let y = x;
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_test() {
        demo();
    }
}
