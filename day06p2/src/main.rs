fn main() {
    let input = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();
    let mut pending = [0usize; 9];

    for timer in input
        .trim_end()
        .split(',')
        .map(|t| t.parse::<usize>().unwrap())
    {
        pending[timer] += 1;
    }

    dbg!(pending);
    for day in 0..256 {
        pending[(day + 7) % 9] += pending[day % 9];

        println!(
            "Day {}: {} ({:?})",
            day + 1,
            pending.iter().sum::<usize>(),
            pending
        );
    }
}
