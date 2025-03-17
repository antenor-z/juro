use serde::{Deserialize, Serialize};

pub struct Interest {
    interest_month: f64
}
impl Interest {
    pub fn from_month(month: f64) -> Interest {
        return Interest{interest_month: month}
    }
    pub fn from_effective_year(year: f64) -> Interest {
        return Interest{interest_month: (1.0 + year).powf(1.0/12.0) - 1.0}
    }
    fn value(&self) -> f64 {
        return self.interest_month
    }
}
pub struct Time {
    time_month: f64
}
impl Time {
    pub fn from_month(month: f64) -> Time {
        return Time{time_month: month}
    }
    pub fn from_year(year: f64) -> Time {
        return Time{time_month: year * 12.0}
    }
    fn value(&self) -> f64 {
        return self.time_month
    }
}
pub struct Calculation {
    pub initial: f64,
    pub interest: Interest,
    pub contribution_per_month: f64
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Result {
    month: i32,
    acumulated: f64,
    invested: f64,
}
impl Calculation {
    pub fn compound_interest(&self, time: Time) -> f64 {
        let c: f64 = self.initial;
        let i: f64 = self.interest.value();
        let n: f64 = time.value();
        let p: f64 = self.contribution_per_month;

        return c * (1.0+i).powf(n) + p * ((1.0+i).powf(n)-1.0) / i;
    }
    pub fn by_month(&self, time: Time) -> Vec<Result>{

        let mut result: Vec<Result> = Vec::new();
        let mut t: f64 = 0.0;
        while t <= time.time_month {
            result.push(
                Result { 
                    month: t as i32, 
                    acumulated: self.compound_interest(Time{time_month: t}), 
                    invested: self.investment_only(Time{time_month: t}) });
            t += 1.0;
        }
        return result
    }
    pub fn investment_only(&self, time: Time) -> f64 {
        let c: f64 = self.initial;
        let p: f64 = self.contribution_per_month;

        return c + time.value() * p;
    }
}