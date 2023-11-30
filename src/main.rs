use crate::series::CreateSeries;

mod series;


fn main() {
    let s = CreateSeries::create_new("a", [91, 2, 63, 4, 25]);

    println!("{:?}", s);
    println!("Min value: {}", s.min::<u64>().unwrap());
    println!("Max value: {}", s.max::<u64>().unwrap());
}
