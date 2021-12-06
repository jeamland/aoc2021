fn main() {
    let input = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();

    let mut timers: Vec<u32> = input
        .trim_end()
        .split(',')
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    for day in 1..=80 {
        let mut new_fish = 0;

        for timer in &mut timers {
            if *timer == 0 {
                *timer = 6;
                new_fish += 1;
            } else {
                *timer -= 1;
            }
        }

        timers.extend(std::iter::repeat(8).take(new_fish));

        if day == 18 {
            println!("Day 18: {}", timers.len());
        }
    }

    println!("Day 80: {}", timers.len());
}
