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
    //vectors can grow and shrink in size, while arrays have fixed lengths
    cards: Vec<String>,
}

impl Deck {
    //the key word Self in this case is a reference to the type in the parenting inherent implementation block,
    //so Self here will return type Deck
    fn new() -> Self {
        /*
            Rather than typing out all the cards one at a time, it can be done as a nested for loop
        */

        //these are arrays and have a fixed length
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        //this is a vec![] because it will be given elements dynamically
        //cards.push() was giving an error about immutability so mut was added here to resolve that
        let mut cards = vec![];

        //these for loops work on arrays, but when I tried making suits and values vectors, there was an error given here
        //The error said: `values` moved due to this implicit call to `.into_iter()`, in previous iteration of looprustcE0382
        //It didn't seem to mind that suits was a vecotr, only values.
        //This was resolved anyway by using arrays instead.
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        //in Rust, variables are referred to as 'bindings'
        //an empty vector can be created using a macro like vec![]
        //another equivalent way to create the empty vector would be Deck { cards: Vec::new() }
        // let deck = Deck { cards: vec![] };

        //now that the cards vector exists it can be given to this struct literal instead of an empty vector
        // let deck = Deck { cards: cards };

        //but the rust-analyzer gives a hint that the shorthand can be used here because the field and binding have identical names
        // let deck = Deck { cards };
        // return deck;

        //this line is an equivalent result to the prior return statement above
        // return Deck { cards };

        //this is an IMPLICIT RETURN statement and is equivalent to the two prior return statements shown above
        //it is important that it does NOT have a semicolon on it for it to work as an implicit return
        //Rust will automatically return the last evaluated expression in a function IF IT DOESN'T HAVE A SEMICOLON on it
        Deck { cards }

    }
}

fn main() {
    //while a struct seemed to need to be assigned as a struct literal, like seen in 'let deck = Deck { cards },
    //an inherent implementation or 'impl Deck {fn new() -> Self{}}' was able to allow making a constructor to use like seen here
    let deck = Deck::new();

    //the :? inside the {} is the Debug formatter
    //which needs #[derive(Debug)] added to the struct Deck to make function without an error
    //but now it is able to print this struct to the console
    //adding the # to the formatter making it {:#?} makes it print in a 'pretty' way which is easier for a human to read
    println!("Here's your deck: {:#?}", deck);

    // another way of expressing that formatted string is to put the binding name in the {}
    // println!("Here's your deck: {deck:#?}");
}
