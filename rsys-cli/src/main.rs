use rsys::Result;

fn main() -> Result<()> {
    println!("Hostname - {}", rsys::linux::hostname()?);

    Ok(())
}
