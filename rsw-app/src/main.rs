use chrono::{DateTime, Utc};
use std::{env, io};
use uuid::Uuid;

#[derive(Debug)]
struct Note {
    id: Uuid,
    text: String,
    category: Category,
    created: DateTime<Utc>,
}

impl Note {
    fn new(text: &str, category: Category) -> Note {
        Note {
            id: Uuid::new_v4(),
            text: text.to_string(),
            category,
            created: Utc::now(),
        }
    }
}
#[derive(Debug)]
enum Category {
    LowPriority,
    Normal,
    HighPriority,
}

fn main() -> io::Result<()> {
    let homevar = env::var("HOME").expect("Not Found Again");

    match env::var("home") {
        Ok(val) => println!("{}", val),
        Err(_) => eprintln!("Not found"),
    };

    let note = Note::new("Test Note", Category::HighPriority);

    println!("{:?}", note);

    Ok(())
}
