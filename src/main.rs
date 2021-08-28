#![feature(const_fn_floating_point_arithmetic)]
//! Approximate pi only using compile time computation

use const_pi::*;

const fn calculate_pi_madhava_leibniz(iterations: usize) -> f64 {
    let my_pi = approx_sqrt(12.);
    let mut sum = 0.;

    // using a for loop requires const_for & const_mut_refs features
    //   and still didn't seem to work with a range
    let mut current_iteration = 0;
    while current_iteration != iterations {
        let cur_itr = current_iteration as f64;
        let numerator = powf(-1_f64, cur_itr) / powf(3., cur_itr);
        let denominator = 2_f64 * cur_itr + 1.;
        sum += numerator / denominator;
        current_iteration += 1;
    }

    my_pi * sum
}

const fn calculate_pi_ramanujan(iterations: usize) -> f64 {
    let mult: f64 = (2. * approx_sqrt(2.)) / 9801.;
    let mut sum = 0.;

    // using a for loop requires const_for & const_mut_refs features
    //   and still didn't seem to work with a range
    let mut current_iteration = 0;
    while current_iteration != iterations {
        let k = current_iteration as f64;
        let numerator = factorial(4. * k) * (1103. + 26390. * k);
        let denominator = powf(factorial(k), 4.) * powf(396., 4. * k);

        sum += numerator / denominator;
        current_iteration += 1;
    }

    1. / (sum * mult)
}

const PI_APPROXIMATION_ML: f64 = calculate_pi_madhava_leibniz(32);
const PI_APPROXIMATION_R: f64 = calculate_pi_ramanujan(3);

fn main() {
    println!(" real: {:1.35}", std::f64::consts::PI);
    println!("   ML: {:1.35}", PI_APPROXIMATION_ML);
    println!("  ΔML: {:1.35}", PI_APPROXIMATION_ML - std::f64::consts::PI);
    println!("    R: {:1.35}", PI_APPROXIMATION_R);
    println!("   ΔR: {:1.35}", PI_APPROXIMATION_R - std::f64::consts::PI);

    println!("\napproximations from 1-1000 iterations:");
    for iterations in 1..1000 {
        let pi_approximation = calculate_pi_ramanujan(iterations);
        let pi_delta: f64 = std::f64::consts::PI - pi_approximation;

        println!(
            "{:5}: {:1.35} - {:1.35}",
            iterations, pi_approximation, pi_delta
        );
    }
}
