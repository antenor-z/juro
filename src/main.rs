pub mod juro;

fn main() {

    let calc: juro::Calculation = juro::Calculation{
        initial: 1230.0,
        interest: juro::Interest::from_month(0.01), 
        contribution_per_month: 2000.0};

    let result: Vec<juro::Result> = calc.by_month(juro::Time::from_year(19.0));

    println!("{:?}", serde_json::to_string(&result));
}

