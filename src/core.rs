use rational::Rational;

/// Finds an fractional approximation of `target` with an absolute error below `max_error` and a denominator below `max_denominator`.
pub fn find_ratio(target: f64, max_error: f64, max_denominator: u32) -> Option<Rational> {
    for denominator in 1..=max_denominator {
        let numerator = ((target * f64::from(denominator)).round()) as u32;
        let rational = Rational::new(numerator, denominator);
        if compute_error(rational, target) < max_error {
            return Option::Some(rational);
        }
    }
    return Option::None;
}

/// Computes the absolute error between `rational` and `reference`.
pub fn compute_error(rational: Rational, reference: f64) -> f64 {
    (f64::from(rational) - reference).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        assert_eq!(compute_error(Rational::new(5, 2), 3.0), 0.5);
    }

    #[test]
    fn test_find_ratio() {
        assert_eq!(find_ratio(2.499, 1e-3, 10000).unwrap(), Rational::new(5, 2));

        // Two historical approximations of pi, see https://en.wikipedia.org/wiki/Approximations_of_%CF%80
        assert_eq!(
            find_ratio(std::f64::consts::PI, 0.005, 10000).unwrap(),
            Rational::new(22, 7)
        );
        assert_eq!(
            find_ratio(std::f64::consts::PI, 1e-6, 10000).unwrap(),
            Rational::new(355, 113)
        );

        assert_eq!(
            find_ratio(std::f64::consts::PI, 1e-6, 100),
            Option::None
        );
    }
}
