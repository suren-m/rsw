    let hello = String::from("hello 😂🎉");
    let subset = &hello[0..5];
    println!("{}", subset);

    let mixed = String::from("😂🎉 hello");
    let subset = &mixed[0..6];
    println!("{}", subset);