use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut sum = 0;
    for result in rdr.records() {
        let record = result?;
        for value in record.iter() {
            let number = value.parse::<i32>()?;
            sum += number;
        }
    }
    println!("The sum is {}", sum);
    Ok(())
}
