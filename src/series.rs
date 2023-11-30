use polars::prelude::*;


#[derive(Debug)]
#[allow(dead_code)]
pub struct CreateSeries {
    length: i32,
}


impl CreateSeries {
    pub fn create_new(arg1: &str, arg2: [i32; 5]) -> Series {
        // let series_len_prop = CreateSeries { length: 5 };
        let series = Series::new(arg1, arg2);
        series
    }
}
