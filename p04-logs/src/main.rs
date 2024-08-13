use std::fs;
use std::io::Error;

fn main() {
    //this returns an OK variant of the Result enum
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);

    //This returns an Err variants of the Result enum and it contains more information like an error code number,
    //an error 'kind' like 'NotFound', and an error message like "the system cannot find the file specified"
    let text_not_there = fs::read_to_string("logsNotExisting.txt");
    println!("{:#?}", text_not_there);

    //I originally named the binding as 'textNotHere' and the rust-analyzer warned me that it should be
    //written in snake case, which I appreciate


    
    println!("{:#?}", divide(10.0, 0.0));
}


//this function returns a Result with an Ok() that wraps an f64, or else an Error that wraps a message
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        //uh oh division by 0
        Err(Error::other("Can't divide by zero"))
    } else {
        Ok(a / b)
    }

}