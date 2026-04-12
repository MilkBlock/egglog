#[path = "../benches/math_microbenchmark_support.rs"]
mod math_microbenchmark_support;

#[test]
fn math_microbenchmark_smoke() {
    let mut input = math_microbenchmark_support::math_microbenchmark_setup();
    math_microbenchmark_support::run_math_microbenchmark_iters(&mut input, 1);
    math_microbenchmark_support::assert_mul_one_rewrites(&mut input);
}
