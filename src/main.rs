fn main() {
    //let names = vec!["Bob", "Frank", "Ferris"];

    //    // iter
    //    for name in names.iter() {
    //        match name {
    //            &"Ferris" => println!("There is a rustacean among us!"),
    //            _ => println!("Hello {}", name),
    //        }
    //    }
    //
    //    // into_iter
    //    // names move
    //    for name in names.into_iter() {
    //        match name {
    //            "Ferris" => println!("There is a rustacean among us!"),
    //            _ => println!("Hello {}", name),
    //        }
    //    }
    //
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("{:?}", names);
}
