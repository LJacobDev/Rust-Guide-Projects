use rand::{seq::SliceRandom, thread_rng};
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
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();

        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, mut num_cards: usize) -> Vec<String> {
        if num_cards > self.cards.len() {
            num_cards = self.cards.len();
        }
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Here's your deck: {:#?}", deck);

    //enabled error handling for asking for too many cards,
    //but asking for a negative number of cards gives an error
    //it is left as-is for now because error handling will be addressed in a coming project
    let number_of_cards = 5;

    let some_cards = deck.deal(number_of_cards);

    println!("Dealt {} cards: {:#?}", some_cards.len(), some_cards);
    println!(
        "There are {} cards remaining in the deck: {:#?}",
        deck.cards.len(),
        deck
    );
}
