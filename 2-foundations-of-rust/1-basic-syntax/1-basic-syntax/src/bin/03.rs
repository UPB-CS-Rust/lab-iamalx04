fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    /*let mut max = 0;
    let mut min = 999;
    for x in input {
        if x > max {
            max = x;
        }
        if x < min {
            min = x;
        }
    }*/
    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();
    println!("{max} is largest and {min} is smallest");
}
