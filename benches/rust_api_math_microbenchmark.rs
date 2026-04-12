mod math_microbenchmark_support;

#[divan::bench(sample_count = 10)]
fn rust_rule_math_microbenchmark(bencher: divan::Bencher) {
    bencher
        .with_inputs(math_microbenchmark_support::math_microbenchmark_setup)
        .bench_local_refs(|input| {
            math_microbenchmark_support::run_math_microbenchmark(input);
        });
}

fn main() {
    divan::main();
}
