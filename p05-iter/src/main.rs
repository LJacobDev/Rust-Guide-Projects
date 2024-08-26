fn main() {
    let colors = vec![
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
