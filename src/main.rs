
#[derive(Debug)]
pub enum DivisonError {
    DivideByZero
}

fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, DivisonError> {
    if denominator == 0.0 {
        Err(DivisonError::DivideByZero)
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let val1 = safe_divide(10.0, 2.0);
    let val2 = safe_divide(0.0, 0.0);

    match val1 {
        Ok(res) => println!("Result: {}", res),
        Err(err) => println!("Error: {:?}", err)
    }

    match val2 {
        Ok(res) => println!("Result: {}", res),
        Err(err) => println!("Error: {:?}", err)
    }
}