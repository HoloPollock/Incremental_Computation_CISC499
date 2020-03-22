use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;
use incremental_tree::choice::Choice;
use incremental_tree::node::{Calculable, Node};
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let unmodfiedtree = Node::gen_node_of_depth(20);
    let mut calced_tree = unmodfiedtree.clone();
    calced_tree.calc();
    c.bench_function("initial calculation", |b| {
        b.iter_batched(
            || unmodfiedtree.clone(),
            |mut tree| tree.calc(),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("modified from scratch", |b| {
        b.iter_batched(
            || {
                let mut clean_tree = unmodfiedtree.clone();
                clean_tree.define_modify(vec![Choice::Op, Choice::Right, Choice::Left], "+");
                return clean_tree;
            },
            |mut clean_tree| clean_tree.calc(),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("incremental calc", |b| {
        b.iter_batched(
            || {
                let inc_tree = calced_tree.clone();
                return inc_tree;
            },
            |mut inc_tree| {
                inc_tree.define_modify_and_calc(vec![Choice::Op, Choice::Right, Choice::Left], "+");
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte).measurement_time(Duration::from_secs(100000));
    targets =  criterion_benchmark
);
criterion_main!(benches);
