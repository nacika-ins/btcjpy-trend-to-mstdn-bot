pub trait AddMean {
    fn mean(&self) -> i64;
}
impl AddMean for Vec<i64> {
    fn mean(&self) -> i64 {
        let sum: i64 = self.iter().sum();
        let mean = sum as f64 / self.len() as f64;
        mean as i64
    }
}
