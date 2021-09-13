pub fn read_input() -> Vec<f64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let points = input
        .split_whitespace()
        .map(|x| x.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()
        .unwrap();

    return points;
}

pub fn read_input_rain_hours() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let rain_hours = input
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    return rain_hours[0];
}
