use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = day01::part2::process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}