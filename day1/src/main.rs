fn part_1() {
    // read lines from input.txt
    let lines = std::fs::read_to_string("input.txt").unwrap();

    // split lines into a vector of strings
    let lines: Vec<&str> = lines.split("\n").collect();

    // first numbers in each line
    let mut first: Vec<u32> = Vec::new();

    // last numbers in each line
    let mut last: Vec<u32> = Vec::new();

    // iterate over each line and find the first and last numbers
    for line in &lines {
        // split line into a vector of strings
        let line: Vec<&str> = line.split("").collect();

        for number in &line {
            let number = number.parse();
            if let Ok(num) = number {
                first.push(num);
                break;
            }
        }

        let mut last_num: u32 = 0;

        for number in &line {
            let number = number.parse();
            if let Ok(num) = number {
                last_num = num;
            }
        }
        last.push(last_num);
    }

    // combine first and last numbers into a list of numbers
    let mut numbers: Vec<u32> = Vec::new();
    for i in 0..first.len() {
        numbers.push(first[i] * 10 + last[i]);
    }

    // sum all numbers
    let mut sum: u32 = 0;
    for number in numbers {
        sum += number;
    }

    // print sum
    println!("Solution to part 1: {}", sum);
}

fn part_2() {
    let digits: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // read lines from input.txt
    let lines = std::fs::read_to_string("input.txt").unwrap();

    // split lines into a vector of strings
    let lines: Vec<&str> = lines.split("\n").collect();

    // first numbers in each line
    let mut first: Vec<u32> = Vec::new();

    // last numbers in each line
    let mut last: Vec<u32> = Vec::new();

    // iterate over each line and find the first and last numbers
    for line in &lines {
        // split line into a vector of strings
        let line: Vec<&str> = line.split("").collect();

        'a: for (i, number) in line.iter().enumerate() {
            let number = number.parse();
            if let Ok(num) = number {
                first.push(num);
                break;
            }

            // check if index to index + size is a digit word
            for (j, digit) in digits.iter().enumerate() {
                if i + digit.len() >= line.len() {
                    continue;
                }
                if line[i..i + digit.len()].join("") == *digit {
                    first.push((j + 1) as u32);
                    break 'a;
                }
            }
        }

        let mut last_num: u32 = 0;

        for (i, number) in line.iter().enumerate() {
            let number = number.parse();
            if let Ok(num) = number {
                last_num = num;
            }

            // check if index to index + size is a digit word
            for (j, digit) in digits.iter().enumerate() {
                if i + digit.len() >= line.len() {
                    continue;
                }
                if line[i..i + digit.len()].join("") == *digit {
                    last_num = (j + 1) as u32;
                }
            }
        }
        last.push(last_num);
    }

    // combine first and last numbers into a list of numbers
    let mut numbers: Vec<u32> = Vec::new();
    for i in 0..first.len() {
        numbers.push(first[i] * 10 + last[i]);
    }

    // sum all numbers
    let mut sum: u32 = 0;
    for number in numbers {
        sum += number;
    }

    // print sum
    println!("Solution to part 2: {}", sum);
}

fn main() {
    part_1();
    part_2();
}
