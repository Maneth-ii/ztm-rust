// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
   let v = true;
   let result;
   
   match v{
    true => result = "it's true",
    false => result = "it's false"
   }
   println!("{:?}", result);

}
