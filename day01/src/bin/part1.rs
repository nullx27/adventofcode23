use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = day01::part1::process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}