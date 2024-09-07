use super::container::Container;
use num::Num;

enum Numeric {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Float32(f32),
    Float64(f64),
}

fn is_number(input: Numeric) -> bool {
    match input {
        Numeric::Int8(_) => true,
        Numeric::Int16(_) => true,
        Numeric::Int32(_) => true,
        Numeric::Int64(_) => true,
        Numeric::Int128(_) => true,
        Numeric::Uint8(_) => true,
        Numeric::Uint16(_) => true,
        Numeric::Uint32(_) => true,
        Numeric::Uint64(_) => true,
        Numeric::Uint128(_) => true,
        Numeric::Float32(_) => true,
        Numeric::Float64(_) => true,
        _ => false,
    }
}


pub struct Basket<T> {
    pub item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { item: Some(item) }
    }

}

impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        //this .take() is capable of moving the value out of self.item, and leaving a None in its place,
        //where just returning self.item would generate a moving error by itself
        //if there is no value in self.item, .take() will return a None, so this method's implementation is complete with this.
        //.take() is a method of the Option enum, and since item is an Option<String>, it is available to use here
        self.item.take()
    }

    fn put(&mut self, item: T) {

        //in the starting case of using strings only, just assign what is given to put into the item
        //when numbers are included, additional functionality will be added to be able to add the numbers together
        // if is_number(item) && is_number(self.item) {
        //     self.item = Some(self.item + item);
        // } else {
        //     self.item = Some(item);
        // }

        self.item = Some(item);

    }

    fn is_empty(&self) -> bool {
        // match &self.item {
        //     Some(value) => false,
        //     None => true
        // }

        //whereas the match statement above does work, there is a built in method of Option.is_none() that does this
        self.item.is_none()
    }
}
