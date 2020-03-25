use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkGroup, BenchmarkId, Criterion,
};
use criterion_cycles_per_byte::CyclesPerByte;
use incremental_tree::choice::Choice;
use incremental_tree::node::{Calculable, Node};
use std::time::Duration;

pub fn benchmark_inital(c: &mut Criterion<CyclesPerByte>) {
    let mut group: BenchmarkGroup<CyclesPerByte> = c.benchmark_group("non_nested_compare");

    for i in 5..=20 {
        let unmodfiedtree = Node::gen_node_of_depth(i);
        let mut calced_tree = unmodfiedtree.clone();
        calced_tree.calc();
        group.bench_with_input(
            BenchmarkId::new("initial calculation", i),
            &unmodfiedtree,
            |b, tree| {
                b.iter_batched(
                    || tree.clone(),
                    |mut tree| tree.calc(),
                    BatchSize::LargeInput,
                );
            },
        );
        group.bench_with_input(
            BenchmarkId::new("modified from scratch", i),
            &unmodfiedtree,
            |b, tree| {
                b.iter_batched(
                    || {
                        let mut clean_tree = tree.clone();
                        clean_tree
                            .define_modify(vec![Choice::Op, Choice::Right, Choice::Left], "+");
                        return clean_tree;
                    },
                    |mut clean_tree| clean_tree.calc(),
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new("incremental calc", i),
            &calced_tree,
            |b, tree| {
                b.iter_batched(
                    || {
                        let inc_tree = tree.clone();
                        return inc_tree;
                    },
                    |mut inc_tree| {
                        inc_tree.define_modify_and_calc(
                            vec![Choice::Op, Choice::Right, Choice::Left],
                            "+",
                        );
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte).measurement_time(Duration::from_secs(30000));
    targets = benchmark_inital
);
criterion_main!(benches);
