# p03-media

This project is designed to represent a simple catalog that can hold Books, Movies, and Audiobooks.  All three Media items have a title, but the other fields such as 'author' or 'director' vary across each type.

The purpose of this project is to demonstrate how Rust's Enums make it possible to group together items that have some similarities yet some differences in a way that can lead to less repeated code than if each were represented as a separate struct.

This project introduces the idea of using Enum variants such as Media::Book { title, author }, Media::Movie { title, director } and Media::Audiobook { title }, and shows how they can be handed to a single function instead of defining three separate functions.  This function uses Pattern Matching to check which variant is being processed, and this allows for custom handling for each variant to be performed accordingly.
