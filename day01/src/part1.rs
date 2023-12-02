pub fn process(input: &str) -> anyhow::Result<String> {
    let result = input
        .lines()
        .map(|line| {
            let v: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap()).parse::<i32>()
                .expect("parse number")
        }).sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}