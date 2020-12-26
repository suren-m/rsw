fn demo() {
    let x = String::from("hello");
    let y = x; // ownership moved to y
               // println!("{}", x); // compile time error

    let s1 = String::from("hello");

    let (s1, len) = get_len(s1);
    fn get_len(s: String) -> (String, usize) {
        let len = s.len();
        (s, len)
    }

    let len = get_len_v2(&s1);
    dbg!(len);
    fn get_len_v2(s: &String) -> usize {
        s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_test() {
        demo();
    }
}
