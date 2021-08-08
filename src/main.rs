fn main() {
    let decimal = 65.4321_f32;

    //let integer: u8 = decimal;
    //
    let integer = decimal as u8;
    let character = integer as char;

    //let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    //println!("1000 as a u8 is : {}", 1000 as u8);

    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);
    //println!(" 232 as a i8 is : {}", 232 as i8);
    //
    println!("300.0 is {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    println!("nan as u8 is {}", f32::NAN as u8);
}
