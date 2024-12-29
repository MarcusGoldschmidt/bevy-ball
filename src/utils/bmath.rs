use std::f64::consts::PI;

pub fn normal_curve(x: f64, mean: f64, std_dev: f64) -> f64 {
    let coefficient = 1.0 / (std_dev * (2.0 * PI).sqrt());
    let exponent = -((x - mean).powi(2)) / (2.0 * std_dev.powi(2));
    coefficient * exponent.exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_curve_test() {
        let mut x = 0.0;
        let add_to_x = 0.05;

        while x <= 1. {
            let result = normal_curve(x, 0.5, 0.35);
            println!("{}", result);
            x += add_to_x;
        }
    }
}
