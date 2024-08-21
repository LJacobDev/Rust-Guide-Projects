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

    //getting the logs.txt file and using a match statement to get the OK value into a string

    let logs_text = fs::read_to_string("logs.txt");

    let mut error_logs = vec![];

    // match logs_text {
    //     Ok(text) => println!("{}", text),
    //     Err(error_info) => println!("{}", error_info),
    // }

    match logs_text {
        Ok(text_result) => {
            error_logs = extract_error_logs(text_result)
        }
        Err(what_went_wrong) => {
            println!("{:#?}", what_went_wrong)
        }
    }


    println!("{:#?}", error_logs);


    /*

    //Demonstration of the Result Enum

    // println!("{:#?}", divide(10.0, 0.0));

    match divide(3.5, 10.0) {
        Ok(value) => println!("{:#?}", value),
        Err(what_went_wrong) => println!("{:#?}", what_went_wrong),
    }

    match divide(3.5, 0.0) {
        Ok(value) => println!("{:#?}", value),
        Err(what_went_wrong) => println!("{:#?}", what_went_wrong),
    }

    //this function can be defined below where it is invoked, so it seems to have bootstrapping in Rust

    //functions can also be defined either within the main() or outside the main() function and be called from main()

    //this function returns a Result with an Ok() that wraps an f64, or else an Error that wraps a message
    fn divide(a: f64, b: f64) -> Result<f64, Error> {
        if b == 0.0 {
            //uh oh division by 0
            Err(Error::other("Can't divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    */
}


///Takes a string of logs text, breaks it by newline separator, extracts any that starts with "ERROR", and returns a Vector of Strings of lines that meet that criterion
fn extract_error_logs(text_to_parse: String) -> Vec<String> {

    let lines = text_to_parse.split("\n");

    let mut error_lines = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            error_lines.push(line.to_string());
        }
    }

    error_lines

}
