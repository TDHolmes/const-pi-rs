# const-pi-rs

This repo was made because one day, I was curious to know how far compile time compute has come.
The result is two implementations of approximating pi, along with some custom const functions for
`f64`. The only unstable feature I had to enable was `const_fn_floating_point_arithmetic`, as
const time compute with floating point numbers are not stable, as different platforms can compute
in slightly different ways for floats.