#![recursion_limit = "2048"]
use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkGroup, BenchmarkId, Criterion,
};
use criterion_cycles_per_byte::CyclesPerByte;
use incremental_tree::choice::Choice;
use incremental_tree::list::NodeList;
use std::time::Duration;

pub fn benchmark_nested(c: &mut Criterion<CyclesPerByte>) {
    let mut group: BenchmarkGroup<CyclesPerByte> = c.benchmark_group("nested_compare");
    for i in [100, 600, 1200, 2000].iter() {
        for j in 10..=20 {
            let parameter_string = format!("list of {} nodes with depth of {}", i, j);
            let n_list = NodeList::gen_random_of_depth(*i, j);
            let mut calced_and_sorted_list = Box::new(n_list.clone());
            calced_and_sorted_list.sort();
            group.bench_with_input(
                BenchmarkId::new("from_scatch", &parameter_string),
                &n_list,
                |b, node_list| {
                    b.iter_batched(
                        || node_list.clone(),
                        |mut list| {
                            list.sort();
                        },
                        BatchSize::LargeInput,
                    );
                },
            );
            group.bench_with_input(
                BenchmarkId::new("incremental sort", &parameter_string),
                &calced_and_sorted_list,
                |b, cs_list| {
                    b.iter_batched(
                        || cs_list.clone(),
                        |mut list| {
                            list.defined_modify_first_element(
                                vec![Choice::Left, Choice::Right, Choice::Op],
                                "+",
                            );
                        },
                        BatchSize::LargeInput,
                    )
                },
            );
        }
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte).measurement_time(Duration::from_secs(2000));
    targets = benchmark_nested
);
criterion_main!(benches);
