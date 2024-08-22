use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    //this returns an OK variant of the Result enum that contains a string of the text contained in logs.txt
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);

    //This returns an Err variants of the Result enum and it contains an Os struct with more information
    //such as an error code number, an error 'kind' like 'NotFound', and an error message like "the system
    //cannot find the file specified"
    let text_not_there = fs::read_to_string("logsNotExisting.txt");
    println!("{:#?}", text_not_there);

    //I originally named the binding as 'textNotHere' and the rust-analyzer warned me that it should be
    //written in snake case, which I appreciate

    //getting the logs.txt file and using a match statement to get the OK value into a string

    // let logs_text = fs::read_to_string("logs.txt");
    // match logs_text {
    //     Ok(text) => println!("{}", text),
    //     Err(error_info) => println!("{}", error_info),
    // }



    /*

    // a long way to write it out using nested match statements

    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text_result) => {
            error_logs = extract_error_logs(text_result.as_str());

            match fs::write("error_logs.txt", error_logs.join("\n")) {
                // Ok(..) and Ok(_) are almost interchangeable here, but the Ok(_) is said to more intentionally communicate that the empty tuple value being received here is being ignored
                // Ok(_) => println!("Wrote error_logs.txt"),
                Ok(..) => println!("Wrote error_logs.txt"),
                Err(error) => println!("{:#?}", error),
            }
        }
        Err(what_went_wrong) => {
            println!("{:#?}", what_went_wrong)
        }
    }

    */


    // a quicker way to retrieve the values in the Result::Ok() enum using .expect()
    // but this doesn't have as robust of error handling and will be more prone to panics

    //this however now lacks a way to println! that the file was read or written successfully

    let text = fs::read_to_string("logs.txt")
        .expect("Could not read file, program crashing");

    let error_logs = extract_error_logs(text.as_str());

    fs::write("error_logs.txt", error_logs.join("\n"))
        .expect("Failed to write error_logs.txt, program crashing");




    //doing this same thing but with a Try operator instead


    //using the Try operator here will either assign a string to text_try if it succeeds,
    //or if it fails, it will provide an Error type, but in order for this to work, the calling
    //function of main() also needs to return a Result of Ok or Error, and the error will be printed to console 
    let text_try = fs::read_to_string("logs.txt")?;

    let error_logs_try = extract_error_logs(text_try.as_str());

    //adding a Try operator here will enable a brief way to get the value unwrapped in case of Ok,
    //and if Err, it will propagate an error message up to main() which will print the error to console
    //but for this to work, main() needs to have -> Result<(), Error>
    //putting an invalid path here results in an error being printed, but the program also panics and ends
    fs::write("error_logs_try.txt", error_logs_try.join("\n"))?;

    // FOR THESE TRY OPERATORS TO WORK, IT ALSO NEEDS A FINAL Ok(()) AT THE BOTTOM OF main() 



    /*

    //another way of doing file write with an error handling test on it using the if let construct

    println!("{:#?}", error_logs);

    //write error logs to an error log file
    //this one successfully writes the error_logs.txt file
    //but this doesn't give off any indication that it succeeded because it only checks for the Err outcome
    if let Err(e) = fs::write("error_logs.txt", error_logs.join("\n")) {
        println!("Error writing file: {:#?}", e)
    }

    //this one intentionally picks a non-existing directory so that the Err will fire to see its output
    if let Err(e) = fs::write("./nodirectory/error_logs.txt", error_logs.join("\n")) {
        println!("Error writing file: {:#?}", e)
    }

    */

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

    //this needs to be here as the final return value of main() in order to allow using the Try operator above here
    Ok(())

}

/// Takes a string of logs text, breaks it by newline separator, extracts any that starts with "ERROR",
/// returns a Vector of Strings of lines that meet that criterion
fn extract_error_logs(text_to_parse: &str) -> Vec<String> {
    let lines = text_to_parse.split("\n");

    let mut error_lines = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            error_lines.push(line.to_string());
        }
    }

    error_lines
}
