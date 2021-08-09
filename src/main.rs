fn main() {
    //    let mut n = 1;
    //
    //    while n < 101 {
    //        if n % 15 == 0 {
    //            println!("fizzbuzz");
    //        } else if n % 3 == 0 {
    //            println!("fizz");
    //        } else if n % 5 == 0 {
    //            println!("buzz");
    //        } else {
    //            println!("{}", n);
    //        }
    //
    //        n += 1;
    //    }

    for i in 1..=100 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}
