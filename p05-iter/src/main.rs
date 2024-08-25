fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    //example of an Iterator struct being derived from a Vector (it can be used on other data structures as well)

    // color_iter needs to be mutable because it has a cursor value that changes when .next() is invoked

    let mut color_iter = colors.iter();

    println!("{:?}", color_iter.next());    //prints "red"
    println!("{:?}", color_iter.next());    //prints "green"
    println!("{:?}", color_iter.next());    //prints "blue"
    println!("{:?}", color_iter.next());    //prints None


    //print all elements in the vector using an Iterator and Iterator consumer
    print_elements(colors);
}

///Example to demonstrate the .iter().for_each() Iterator Consumer method
fn print_elements(data: Vec<String>) {
    data.iter().for_each(|el| println!("{}", el))
}
