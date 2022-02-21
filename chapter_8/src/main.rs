fn age() -> u32 {
    15
}

enum Foo {
    Bar,
}

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            // &"Ferris" is a string slice -> &str
            &"Ferris" => println!("There is a rustacean under us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    // Guards vs Bindings?
    // Couldn't find any information why the one would be used over the other
    match age() {
        x if x == 15 => println!("Age is {}", x),
        _ => println!("Some other age"),
    }

    match age() {
        x @ 15 => println!("Age is {}", x),
        _ => println!("Some other age"),
    }

    let a = Foo::Bar;

    // if let Foo::Bar = a {
    //     println!("a is foobar");
    // }
}
