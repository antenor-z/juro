
fn main() {

    let c = Calculation{
        initial: 1230.0,
        interest: Interest::from_month(0.01), 
        contribution_per_month: 2000.0};

    println!("{}", c.compound_interest(Time::from_year(19.0)));
    println!("{}", c.investment_only(Time::from_year(19.0)));
}

struct Interest {
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
struct Time {
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
struct Calculation {
    initial: f64,
    interest: Interest,
    contribution_per_month: f64
}
impl Calculation {
    fn compound_interest(&self, time: Time) -> f64 {
        let c: f64 = self.initial;
        let i: f64 = self.interest.value();
        let n: f64 = time.value();
        let p: f64 = self.contribution_per_month;

        return c * (1.0+i).powf(n) + p * ((1.0+i).powf(n)-1.0) / i;
    }
    fn investment_only(&self, time: Time) -> f64 {
        let c: f64 = self.initial;
        let p: f64 = self.contribution_per_month;

        return c + time.value() * p;
    }
}






