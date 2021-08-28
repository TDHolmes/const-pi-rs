#![feature(const_fn_floating_point_arithmetic)]

/// Approximate sqrt at compile time using
/// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method
///
/// ```
/// # use const_pi::approx_sqrt;
/// assert_eq!(approx_sqrt(4.), 2.);
/// assert_eq!(approx_sqrt(9.), 3.);
/// ```
pub const fn approx_sqrt(input: f64) -> f64 {
    let mut x_n = input / 2.; // very naive initial guess
    let mut error = 1.;
    while error != 0. {
        let x_np1 = 0.5 * (x_n + input / x_n);
        error = x_np1 - x_n;
        x_n = x_np1;
    }

    x_n
}

/// floating point compile time factorial
///
/// ```
/// # use const_pi::factorial;
/// assert_eq!(factorial(3.), 6.);
/// assert_eq!(factorial(6.), 720.);
/// assert_eq!(factorial(0.), 1.);
/// assert_eq!(factorial(1.), 1.);
/// ```
pub const fn factorial(num: f64) -> f64 {
    if num == 0. || num == 1. {
        1.
    } else {
        factorial(num - 1.) * num
    }
}

/// compile time floating point power function
/// ```
/// # use const_pi::powf;
/// assert_eq!(powf(2., 5.), 32.);
/// assert_eq!(powf(-2., 5.), -32.);
/// assert_eq!(powf(-2., 4.), 16.);
/// assert_eq!(powf(0., 9999.), 1.)
/// assert_eq!(powf(9999., 0.), 1.)
/// ```
pub const fn powf(input: f64, mut exp: f64) -> f64 {
    if input == 0. || exp == 0. {
        return 1.;
    }

    let mut output = input;
    exp -= 1.;
    while exp > 0. {
        output *= input;
        exp -= 1.;
    }
    output
}
