pub fn parse_range(range: &str) -> Vec<usize> {
    let mut values = Vec::new();
    for range in range.split(",").collect::<Vec<&str>>() {
        let parts = range.split("-").collect::<Vec<&str>>();
        // Should basically always be 2 for a range of values 1 (default branch) for a single value
        match parts.len() {
            2 => {
                let start: usize = parts[0].parse().unwrap();
                let end: usize = parts[1].parse().unwrap();
                (start..=end).for_each(|value| values.push(value));
            }
            _ => {
                values.push(parts[0].parse::<usize>().unwrap());
            }
        };
    }
    values
}
