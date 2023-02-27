fn main() {
    let mut result: i128 = 1;
    let mut squares: u32 = 1;
    
    result *= 2;
    while squares != 64 {
        print!(" Grains {result} ");
        if squares % 8 == 0 {
            println!("");
        }
        result *= 2;
        squares += 1;
    }


}
