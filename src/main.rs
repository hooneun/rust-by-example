fn main() {
    let x = 5u32;

    let y = {
        //25
        let x_squared = x * x;
        // 25 * 5 = 125
        let x_cube = x_squared * x;

        // 25 * 125 + 5
        x_cube * x_squared + x
    };

    let z = { 2 * x };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
