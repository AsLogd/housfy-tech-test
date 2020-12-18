use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = "10, 10
N
FFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFRFFFFRFFFFRFFFF
";

pub fn benchmark(c: &mut Criterion) {
	let mut input = htt::parse_input(&INPUT).unwrap();
    c.bench_function("Simple", |b| b.iter(|| htt::solve(&mut input)));

}

criterion_group!(benches, benchmark);
criterion_main!(benches);

