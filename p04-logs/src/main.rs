use std::fs;

fn main() {

    //this returns an OK variant of the Result enum
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);

    //This returns an Err variants of the Result enum and it contains more information like an error code number,
    //an error 'kind' like 'NotFound', and an error message like "the system cannot find the file specified"
    let textNotThere = fs::read_to_string("logsNotExisting.txt");
    println!("{:#?}", textNotThere);
}


