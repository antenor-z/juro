use std::f64;

use serde::{Serialize};

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
    time_month: i32
}
impl Time {
    pub fn from_month(month: i32) -> Time {
        return Time{time_month: month}
    }
    pub fn from_year(year: i32) -> Time {
        return Time{time_month: year * 12}
    }
    fn value(&self) -> i32 {
        return self.time_month
    }
}
pub struct Calculation {
    pub initial: f64,
    pub interest: Interest,
    pub contribution_per_month: f64
}
#[derive(Serialize)]
pub struct Result {
    month: i32,
    acumulated: f64,
    invested: f64,
    increase: f64,
    increase_interest: f64
}
impl Calculation {
    pub fn compound_interest(&self, time: Time) -> f64 {
        let c: f64 = self.initial;
        let i: f64 = self.interest.value();
        let n: i32 = time.value();
        let p: f64 = self.contribution_per_month;

        return c * (1.0+i).powi(n) + p * ((1.0+i).powi(n)-1.0) / i;
    }
    pub fn by_month(&self, time: Time) -> Vec<Result>{

        let mut result: Vec<Result> = Vec::new();
        for t in 0 ..= time.time_month {
            let acumulated = self.compound_interest(Time{time_month: t});
            let invested = self.investment_only(Time{time_month: t});
            let increse: f64;
            let increase_interest: f64;
            if t > 0 {
                increse = acumulated - result.last().unwrap().acumulated;
                increase_interest = increse - self.contribution_per_month;
            } else {
                increse = 0.0;
                increase_interest = 0.0;
            }
            result.push(
                Result { 
                    month: t as i32, 
                    acumulated: acumulated, 
                    invested: invested,
                    increase: increse,
                    increase_interest: increase_interest});
        }
        return result
    }
    pub fn investment_only(&self, time: Time) -> f64 {
        let c: f64 = self.initial;
        let p: f64 = self.contribution_per_month;

        return c + time.value() as f64 * p;
    }
}