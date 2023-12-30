// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// impl Wrapper<T>{
//     pub fn just(){
//         println!("hi");
//     }
// }

// --> exercises/14_generics/generics2.rs:19:14
// |
// 19 | impl Wrapper<T>{
// |              ^ not found in this scope
// |

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn store_u32_in_wrapper() {
        
        //Wrapper::just();
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
