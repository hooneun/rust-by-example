fn main() {
    let mut mutable_integer = 7i32;

    {
        let mutable_integer = mutable_integer;

        println!("{}", mutable_integer);
    }

    println!("{}", mutable_integer);

    mutable_integer = 3;

    println!("{}", mutable_integer);
}
