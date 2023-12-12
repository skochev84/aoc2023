fn main() {
    println!("Hello, world!");

    let mut input = r"Time:        48938595
Distance:   296192812361391"
        .lines();

    let times: Vec<usize> = input
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap_or(0))
        .skip(1)
        .collect();
    println!("{:#?}", times);

    let distances: Vec<usize> = input
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap_or(0))
        .skip(1)
        .collect();
    println!("{:#?}", distances);
    let mut product = 1;

    for j in 0..5 {
        let current_time = times[j];
        let current_distance = distances[j];
        let mut wins = 0;
        for i in 2..current_time {
            if i * (current_time - i) > current_distance {
                wins += 1;
            }
        }
        //println!("{:#?}", wins);
        product *= wins;
        println!("product: {:#?}", product);
    }
}
