// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let value:i128 = 4382392323829323923923203923923092309;
    let result:&str;
    match value {
        1 => result = "one",
        2 => result = "two",
        3 => result = "three",
        4 => result = "four",
        _ => result = "something else"
    }
    println!("{:?}" , result);
}
