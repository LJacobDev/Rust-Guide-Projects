
//both mod basket and use basket::Basket are needed, it isn't enough to have just 'mod basket'
mod basket;
use basket::Basket;

mod container;
use container::Container;

mod stack;
use stack::Stack;

fn main() {

/*

    Objective:

    create a Struct called Basket that can hold one value that is of any possible type of data, whether number, string, vector, struct

    give it a .get() method that returns an Option if it has Some value or None

    give it a .put() method that assigns a new value to it, overwriting the prior value, unless it is having a number assigned while it already contains one, in which case, add the numbers together

    give it an .is_empty() method that returns a boolean about whether it contains anything currently


    Secondary goal:

    make a Stack Struct that can hold more than one value of various data types

    it can hold any kind of data, but its collections will be homogeneous 

    it has .put() method that adds an additional value to be stores

    it has .get() which returns an Option, with the most recently stored value or a None if Stack is empty

    it has .is_empty()



    Since these two structs have the same method names with the same method signatures, they can be implemented as a Trait

    This will also enable using a Stack or a Basket interchangeably in any place in the code due to their common Trait type

*/

let mut basket = Basket::new("starting value".to_string());

println!("{}", basket.is_empty());

basket.put(String::from("Stored value"));

println!("{}", basket.is_empty());

println!("{:?}", basket.get());

//after running the basket.get() it is taking the value out of self.item and replacing it with a None,
//so calling .get() doesn't just read the value, it retrieves it and takes it out of it
println!("{}", basket.is_empty());


//Originally Basket could only hold Option<String>, but it is being modified to hold Option<T> so it can be given numbers now
let mut b1 = Basket::new(10);

println!("b1.get contents: {:?}", b1.get());

b1.put(20);

println!("b1.get contents: {:?}", b1.get());

//Basket originally overwrites its old item with the new one in .put(), however it will be modified so that if it holds a number and is given a number, it will instead add the new number to the old number and store the result

//Update:  Rust makes it a little difficult to quickly check whether a value is numeric or not at runtime, so while initial attempts were made in the basket.rs file to give it this functionality, it has been abandoned for now so that I can go on to complete the Stack struct and move on to doing other things instead.  Interestingly, the instructor of the Rust course that this project was a part of did not complete this functionality either but also doesn't address that this had occurred.


let mut stack1 = Stack::new(vec![1,2,3]);

stack1.put(4);
println!("{:?}",stack1.get());
println!("{:?}", stack1.is_empty());

let mut stack2 = Stack::new(vec!["1".to_string(),"2".to_string(),"3".to_string()]);

//now the Structs and their functionality is complete up to the point at which they were made in the course videos



//using the Stack and Struct interchangeably:
add_value_to_container(&mut basket, String::from("new string"));
add_value_to_container(&mut stack2, String::from("new string"));

println!("{:?}", stack2.get());

}

//this is something that can add a string to either a Basket or a Struct, as long as it is a Basket or Struct that was made to hold strings.
fn add_value_to_container<T: Container<String>>(container: &mut T, value: String) {
    container.put(value);
}
