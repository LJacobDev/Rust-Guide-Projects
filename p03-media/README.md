# p03-media

This project was not made to be readable or cleanly written, but rather to build up an understanding of how Enums and the Option type works in Rust, so the code has lots of incremental steps that were performed and then commented out without deleting them so that the steps in the thinking process could be better remembered.

It represents a simple catalog that can hold Books, Movies, and Audiobooks.  However, the project is less about making a useful catalog and more about demonstrating several important concepts in Rust regarding Enums, Options, and Pattern Matching.

All three Media items have a title, but the other fields such as 'author' or 'director' vary across each type.  This project is meant to demonstrate how Rust's Enums make it possible to group together items that have some similarities yet some differences in a way that can lead to less repeated code than if each were represented as a separate struct.

This project introduces the idea of using Enum variants such as Media::Book { title, author }, Media::Movie { title, director } and Media::Audiobook { title }, and shows how they can be handed to a single function instead of defining three separate functions.  This function uses Pattern Matching to check which variant is being processed, and this allows for custom handling for each variant to be performed accordingly.

The lessons of this project go on to demonstrate the concept of Option::Some() and Option::None in Rust by highlighting Vec::get which returns these Options as output, and how these are used in place of having a concept of 'null' values like many other programming languages have.

In order to understand Option::Some() and Option::None more thoroughly, custom methods and enums are made to mimic the behaviour of Vec::get and of Option::Some() and Option::None, showing what they work like behind the scenes.

This project also has many commented out sections of code that were written just to experiment with these and other Rust features to understand them better.
