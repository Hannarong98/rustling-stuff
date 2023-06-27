use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

/* struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();

    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(middle) = &person.middle {
        full_name.push_str(&middle);
        full_name.push_str(" ");
    }

    full_name.push_str(&person.last);
    full_name
}
 */
fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    }

    Ok(string)
}

fn main() {
    /* let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };

    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };

    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };

    assert_eq!(build_full_name(&bob), "Robert Murdock Jones"); */

    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }

    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}
