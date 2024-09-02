# p06-lifetimes

A simple example that demonstrates a lifetime annotation.  A function is made that takes in two references as inputs, and it returns a reference.

Rust assumes that the returned reference is going to point to the data in one of the two input references, and needs to have it specified by marking it with a lifetime annotation.
