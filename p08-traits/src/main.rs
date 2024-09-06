use basket::Basket;

mod basket;

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

}
