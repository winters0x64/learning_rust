fn main() {
    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number: Option<&str> = None;

    println!("{:#?}",absent_number);
}