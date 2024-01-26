#[cfg(test)]
pub mod some_test_mod {
    #[test]
    fn test_fn() {
        println!("Strale was here");
    }
}

#[cfg(test)]
use bencher::{
    Bencher,
    benchmark_group,
    benchmark_main,
};

#[cfg(test)]
fn bench_fn(bench: &mut Bencher) {
    bench.iter(|| {
        println!("Strale benched here");
    });
}

#[cfg(test)]
benchmark_group!(benches, bench_fn);

#[cfg(test)]
benchmark_main!(benches);