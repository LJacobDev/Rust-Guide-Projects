# p04-logs

This project is made to demonstrate the Result Enum in Rust, and how to make use of the Ok() and Err() variants.

It is done by using std::fs::read_to_string() to read lines of text from a logs.txt file, and then seeing various ways to unwrap the resulting String from the Result::Ok() that is returned, as well as understanding what to do when a process fails and Err() is given instead.

A moment is also taken to demonstrate the difference between String and &str, and what must be done if &str values are needing to outlive the scope lifetime of a String that they are referencing, as shown in the way that extract_error_logs was made to return a Vec&ltString&gt and not a Vec<&str>

Some alternatives to using match statements and especially nested match statements is shown by demonstrating that the Result Enum also has .unwrap(), .expect(), .unwrap_or() methods.

Finally, the Try operator '?' was shown as a way to unwrap Result values or else propagate an error message up to main() which was modified to itself return a Result<(), Error>, which enables the message to appear on the console.
