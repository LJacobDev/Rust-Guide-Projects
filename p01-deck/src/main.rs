/*
    objective:

    make a data object that can represent a deck of cards

    give it a new() method that creates a set of 52 cards
    give it a shuffle() method that randomizes the order of the cards
    give it a deal() method that returns one card

    this can be represented in a struct, which in rust is similar to classes in other languages
*/

#[derive(Debug)]
struct Deck {
    cards: Vec<String>, //vectors can grow and shrink in size, while arrays have fixed lengths
}

fn main() {

    /*
        Rather than typing out all the cards one at a time, it can be done as a nested for loop
     */

    //these are arrays and have a fixed length
    let suits = ["Hearts","Diamonds","Clubs","Spades"];
    let values = ["Ace","Two","Three","Four","Five","Six","Seven","Eight","Nine","Ten","Jack","Queen","King"];

    //this is a vec![] because it will be given elements dynamically
    //cards.push() was giving an error about immutability so mut was added here to resolve that
    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }


    //in Rust, variables are referred to as 'bindings'
    //another equivalent way to create the empty vector would be Deck { cards: Vec::new() }
    let deck = Deck { cards: vec![] }; 

    //the :? inside the {} is the Debug formatter
    //which needs #[derive(Debug)] added to the struct Deck to make function without an error
    //but now it is able to print this struct to the console
    println!("Here's your deck: {:?}", deck);

    println!("Here's the cards vector: {:?}", cards)
    
    // another way of expressing that formatted string is to put the binding name in the {}
    // println!("Here's your deck: {deck}");
}
