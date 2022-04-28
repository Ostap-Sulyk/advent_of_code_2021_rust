fn main() {
    let data: Vec<i32> = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|x| x.iter().sum::<i32>())
        .collect();

    println!("{}", count_increase(&data));
}
fn count_increase(list: &[i32]) -> u32 {
    let mut count = 0;
    for i in 1..list.len() {
        if list[i] > list[i - 1] {
            count += 1
        }
    }
    count
}
