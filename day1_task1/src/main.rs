fn main(){
    // if you want to play around use test.txt instead of input.txt
    let measures: Vec<i32> = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut count = 0;

    for i in 1..measures.len() {
        if measures[i] > measures[i - 1] {
            count += 1;
        }
    }
    println!("{count}")
}

