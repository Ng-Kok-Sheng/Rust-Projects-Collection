use clap::Parser;

#[derive(Parser)]
struct EchoArgs {
    #[clap(short, long, default_value = "Echo")]
    echo_value: String,

    #[clap(short, long)]
    count: Option<bool>,
}

fn main() {
    let EchoArgs { echo_value, count } = EchoArgs::parse();
    if count == Some(true) {
        run_count(echo_value);
        return;
    }
    run_print(echo_value);
}

fn run_count(echo_string: String) {
    let trimmed_string = echo_string.trim().replace(' ', "");
    let splitted_string = trimmed_string.split(',');
    let mut numbers_from_string: Vec<i32> = Vec::new();

    for substring in splitted_string {
        let parsed_number = substring.parse::<i32>();

        if let Ok(number) = parsed_number {
            numbers_from_string.push(number);
        }
    }

    if numbers_from_string.is_empty() {
        println!("No numbers found");
        return;
    }

    let (mut count, mut sum, mut min, mut max) =
        (0, 0, numbers_from_string[0], numbers_from_string[0]);
    for number in numbers_from_string {
        count += 1;
        sum += number;
        min = if number < min { number } else { min };
        max = if number > max { number } else { max };
    }
    let average = sum / count;

    println!(
        "Count: {}\nSum: {}\nMin: {}\nMax: {}\nAverage: {}",
        count, sum, min, max, average
    );
}

fn run_print(echo_string: String) {
    println!("{}", echo_string);
}
