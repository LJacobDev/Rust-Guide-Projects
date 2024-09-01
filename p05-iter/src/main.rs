fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    //example of an Iterator struct being derived from a Vector (it can be used on other data structures as well)

    // color_iter needs to be mutable because it has a cursor value that changes when .next() is invoked

    let mut color_iter = colors.iter();

    println!("{:?}", color_iter.next()); //prints "red"
    println!("{:?}", color_iter.next()); //prints "green"
    println!("{:?}", color_iter.next()); //prints "blue"
    println!("{:?}", color_iter.next()); //prints None

    print_elements_for_in(&colors);

    //print all elements in the vector using an Iterator and Iterator consumer
    print_elements(&colors);

    //print each element but use an Iterator Adaptor to modify the data before printing it
    print_elements_adaptor(&colors);

    //print only "green" and "blue" by passing in a Vector Slice, but the function needs to know it will be receiving a Vector Slice in its function signature
    print_elements_vector_slice(&colors[1..]); //this version takes from index 1 to the end
    print_elements_vector_slice(&colors[1..3]); //this version takes from index 1 up to but not including 3 (there is no index 3, this is just a way to indicate to take 1 and 2)


    //print only the first letter of each string in the vector
    shorten_strings_first_try(&colors);

    println!("colors Vector before running shorten_strings: {:?}", colors);
    
    //print vector after modifying it in place to only have the first letter of each string
    //it has been updated to use a vector slice, so that it will now turn it into "red", "g", "b" 
    shorten_strings(&mut colors[1..3]);

    println!("colors Vector after running shorten_strings: {:?}", colors);

    //convert vector of strings to uppercase, returns a new vector and doesn't modify original one
    println!("{:#?}", to_uppercase_first_try(&colors));
    
    //returns a new vector of strings where they're uppercase, doesn't modify original vector, uses .collect() 
    println!("{:#?}", to_uppercase(&colors));


    let new_vector = move_elements_first_try(colors);
    println!("new_vector: {:?}", new_vector);

    // this println! of 'colors' has an error saying it's a moved value so it seems to have worked
    // println!("original colors vector: {:?}", colors);

    //try to move new_vector contents back to colors
    // move_elements(new_vector, colors);
    // println!("{:?}", colors);

    //it doesn't seem to allow that ince it was already move out of, so trying colors_new
    
    let mut colors_new = vec![];
    move_elements(new_vector, &mut colors_new);

    println!("colors_new: {:?}", colors_new);


    let colors_exploded = explode_first_try(&colors_new);
    println!("{:?}", colors_exploded);


    println!("{:?}", explode(&colors_new));


    println!("{:?}", find_color_or(&colors_new, "re", "notfound"));
    println!("{:?}", find_color_or(&colors_new, "bo", "notfound"));

}

///Example to demonstrate for..in loop which uses an Iterator behind the scenes
///It calls .next() on it, unwraps the Option type that the Iterator provides,
///And once a None is found it breaks the loop
fn print_elements_for_in(data: &Vec<String>) {
    println!("Running print_elements_for_in:");
    for element in data {
        println!("{}", element)
    }
}

///Example to demonstrate the .iter().for_each() Iterator Consumer method
fn print_elements(data: &Vec<String>) {
    println!("Running print_elements:");

    //the contents inside for_each() is called a "closure" which is an anonymous function
    //the input parameters are between the two pipe characters, and the return value is to the right of the pipes
    data.iter().for_each(|el| println!("{}", el));

    //example where the closure / anonymous function has multiple lines
    // data.iter().for_each(|el| {
    //     println!("{}", el);
    //     println!("extra line");
    // })
}

///Example of an Iterator Adaptor method (map)
fn print_elements_adaptor(data: &Vec<String>) {
    println!("Running print_elements_adaptor:");
    data.iter() //creates an Iterator
        .map(|el| format!("{} {}", el, el)) //creates a new Iterator that has the closure called on each element of the first Iterator
        .for_each(|el| println!("{}", el)) //performs a closure for each element of the Iterator returned by .map()
}

///Example using a Vector Slice so that just a part of the input vector can be used without having to duplicate it on the heap in a new vector
///Now that the input parameter is set to the type of a "Vector Slice", `&[String]`, 
///it can now accept BOTH a full vector reference like `&Vec<String>`, like &colors, as well as a slice of one, like &colors[1..]
fn print_elements_vector_slice(data: &[String]) {
    println!("Running print_elements_vector_slice");
    data.iter().for_each(|el| println!("{}", el))
}


///I went ahead to try to make the 'shorten_strings' function myself before seeing how the instructor implements it,
///but I think that this isn't what they had in mind, because this lesson was about showing ownership and borrowing,
///so I expect that their shorten_strings will probably try to change the data given to it or something else besides what this does
fn shorten_strings_first_try(data: &Vec<String>) {
    println!("Running shorten_strings_first_try:");
    data.iter().for_each(|el| println!("{:?}", el.chars().nth(0)))
}


///Shorten the strings to just their first letter but don't return a new vector, modify the existing one passed to the function
///This one needs to use data.iter_mut() instead of data.iter() because it needs to be able to change the data and .iter() is read only
fn shorten_strings(data: &mut [String]) {
    println!("Running shorten_strings:");
    data.iter_mut().for_each(|el| el.truncate(1));
}

///Returns a new vector where the strings are converted to uppercase
///This was my first guess at how to perform the procedure, however the instructor wanted to teach the .collect() method so
///his implementation will use that instead of what is done here
fn to_uppercase_first_try(data: &[String]) -> Vec<String>{
    println!("Running to_uppercase_first_try:");
    let mut output = vec![];
    data.iter().for_each(|el| output.push(el.to_ascii_uppercase()));
    output
}

///This version uses .collect(), which allows returning a Vec<String> without having to add extra lines of
///initializing an empty vector and pushing on to it
fn to_uppercase(data: &[String]) -> Vec<String> {
    println!("Running to_uppercase:");
    data.iter()
        .map(|el| el.to_uppercase())
        .collect()
}


///Trying to use the into_iter version to move ownership of a vector into a new vector, before seeing the instructor's way of doing it
///My version appears to work as it is, but I want to see whether the instructor's version differs from this somehow
///Yes, the instructor's version differs from this, however this version here still does move the ownership as intended
fn move_elements_first_try(data: Vec<String>) -> Vec<String> {
    println!("Running move_elements_first_try:");
    data.into_iter().collect()
}


///The instructor's version is to be handed two vectors that are initialized in the higher scope and this will move ownership of the
/// contents of the first one into the second one
fn move_elements(vector_from: Vec<String>, vector_to: &mut Vec<String>) {
    println!("Running move_elements:");
    vector_from.into_iter().for_each(|el| vector_to.push(el))
}


/// my imagined version for explode(), turning a Vec<String> into Vec<Vec<String>> but it will likely differ from the instructor's version
/// because he is introducing 'inner maps' here
fn explode_first_try(vec_original: &[String]) -> Vec<Vec<String>> {
    println!("Running explode_first_try:");
    let mut vec_new: Vec<Vec<String>> = vec![];
    vec_original.iter().for_each(|el| {
        let mut vec_build = vec![];
        el.chars().for_each(|letter| vec_build.push(letter.to_string()));
        vec_new.push(vec_build);
    });

    vec_new
}

///Making it in the way that the instructor showed:
/// This is taking a reference to a vector of strings,
/// it iterates it,
/// runs it through map, where for each string, it calls string.chars(),
/// which is itself a string iterator that gives each letter
/// then each letter is run through a new inner map method call to convert it to a string
/// then those are all collected and it makes it into a vector of strings
/// then the outer map is collected into a vector of vectors of strings and is returned
fn explode(data: &[String]) -> Vec<Vec<String>> {
    println!("Running explode:");
    data.iter()
        .map(|el| el.chars().map(|char| char.to_string()).collect()
        ).collect()
}


///Doing this from memory after seeing the instructor do it a few days ago:
/// takes in a vector, a string slice to search for and return element that contains it, a string slice of what to return as a fallback if first string isn't found
/// returns a string of the element found or the fallback if not found
fn find_color_or(data: &[String], findstr: &str, fallbackstr: &str) -> String {
    println!("Running find_color_or: ");
    data.iter()
    //.find() returns the first element for which the closure function returns true and it is wrapped in an Option enum
    .find(|el| el.contains(findstr))
    //.map_or() unwraps the Option enum and returns the fallback (first argument) if it is a None, and returns the second argument if it is a Some, where the value can be obtained from the some like |value| value like how a closure function works
    //documentation of map_or says: Returns the provided default result (if none), or applies a function to the contained value (if any).
    .map_or(String::from(fallbackstr), |val| String::from(val))
}