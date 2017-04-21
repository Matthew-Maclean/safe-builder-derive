## You wouldn't call unwrap on a constructor!

A proc-macro for deriving a type-safe builder pattern on any struct*

example:

    extern crate safe_builder;
    #[macro_use]
    extern crate safe_builder_derive;

    use safe_builder::*;

    #[derive(SafeBuilder)]
    struct Person
    {
        name: String,
        age: usize,
        //        town    street  number
        address: (String, String, usize)
    }

    fn main()
    {
        let me: Person = Person::build()
            .name("Matthew M.".to_owned())
            .age(18)
            .address(("Toronto".to_owned(), "Younge st.".to_owned(), 0));
        // no need to call unwrap - the compiler knows that the type is complete
        
        let you = Person::build()
            .age(0) // hello, world!
            .address(("City".to_owned(), "Street".to_owned(), 0))
            .name("You!".to_owned());
        // build your structs in any order!
        
        /*
        Wont compile: the type is incomplete
        let no_name: Person = Person::build()
            .name("Unkown".to_owned())
            .address((String::new(), String::new(), 0));
        */
    }
    
Please use this crate on your most complicated structs, and open some issues when you find bugs!

*no generic struct or those with lifeties, yet
