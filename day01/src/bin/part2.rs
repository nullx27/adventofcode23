use std::collections::HashMap;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let out: i32 = input
        .lines()
        .map(replace)
        .map(|line| {
            let v: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap()).parse::<i32>()
                .expect("parse number")
        })
        .sum::<i32>();

    Ok(out.to_string())
}

pub fn replace(line: &str) -> String {
    let numbers: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut out = String::new();
    let _ = (0..line.len()).for_each(|i| {
        let substr = &line[i..];

        for (from, to) in &numbers {
            if substr.starts_with(from) {
                out.push_str(to)
            }
        }
        if let Some(num) = line.chars().nth(i).unwrap().to_digit(10) {
            out.push_str(num.to_string().as_str());
        }
    });

    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_cases() -> anyhow::Result<()> {
        assert_eq!("29", process("two1nine")?);
        assert_eq!("83", process("eightwothree")?);
        assert_eq!("13", process("abcone2threexyz")?);
        assert_eq!("24", process("xtwone3four")?);
        assert_eq!("42", process("4nineeightseven2")?);
        assert_eq!("14", process("zoneight234")?);
        assert_eq!("76", process("7pqrstsixteen")?);

        Ok(())
    }

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}