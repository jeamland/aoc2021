fn main() {
    let input = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();

    let positions: Vec<u32> = input
        .trim_end()
        .split(',')
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut best = 0;
    let mut position_best = 0;
    let mut worst = 0;
    let mut position_worst = 0;

    for position in min..=max {
        let fuel_consumed = positions
            .iter()
            .map(|p| {
                let distance = (*p as i32 - position as i32).abs() as u32;
                (distance * (distance + 1)) / 2
            })
            .sum::<u32>();
        println!("{} -> {}", position, fuel_consumed);

        if fuel_consumed > worst {
            worst = fuel_consumed;
            position_worst = position;
        }
        if best == 0 || fuel_consumed < best {
            best = fuel_consumed;
            position_best = position;
        }
    }

    println!("best at {} ({})", position_best, best);
    println!("worst at {} ({})", position_worst, worst);
}
