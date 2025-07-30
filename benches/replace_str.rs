use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn find_all_indices(input: &str, target: &str) -> Vec<usize> {
    let mut occurrences: Vec<usize> = Vec::new();
    let mut last_occurrence = 0;

    // Find all occurrences
    while let Some(index) = input[last_occurrence..].find(target) {
        let absolute_index = last_occurrence + index;
        occurrences.push(absolute_index);
        last_occurrence = absolute_index + target.len();
    }

    occurrences
}

#[inline(never)]
pub fn replace_str(input: &str, target: &str, replacement: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let occurrences = find_all_indices(input, target);

    let replacement_enlargement = (replacement.len() as isize) - (target.len() as isize);
    let input_enlargement = replacement_enlargement * (occurrences.len() as isize);

    let mut replaced = String::with_capacity(((input.len() as isize) + input_enlargement) as usize);

    let mut after_occurrence: usize = 0;

    for start_of_occurrence in occurrences {
        // Push before occurrence
        replaced.push_str(&input[after_occurrence..start_of_occurrence]);

        replaced.push_str(replacement);
        after_occurrence = start_of_occurrence + target.len();
    }

    // Push after last occurrence
    replaced.push_str(&input[after_occurrence..]);

    replaced
}

pub fn replace_str_v2(input: &str, target: &str, replacement: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let occurrences: Vec<(usize, _)> = input.match_indices(target).collect();

    let replacement_enlargement = (replacement.len() as isize) - (target.len() as isize);
    let input_enlargement = replacement_enlargement * (occurrences.len() as isize);

    let mut replaced = String::with_capacity(((input.len() as isize) + input_enlargement) as usize);

    let mut after_occurrence: usize = 0;

    for (start_of_occurrence, _) in occurrences {
        // Push before occurrence
        replaced.push_str(&input[after_occurrence..start_of_occurrence]);

        replaced.push_str(replacement);
        after_occurrence = start_of_occurrence + target.len();
    }

    // Push after last occurrence
    replaced.push_str(&input[after_occurrence..]);

    replaced
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("replace_str");

    for i in 0..2 {
        group.bench_function(BenchmarkId::new("my replace", i), |b| {
            b.iter(|| {
                replace_str(
                    black_box("These crocodiles are cool. I love to look at crocodiles."),
                    black_box("crocodiles"),
                    black_box("cats"),
                )
            })
        });
        group.bench_function(
            BenchmarkId::new("my replace using std match_indices", i),
            |b| {
                b.iter(|| {
                    replace_str_v2(
                        black_box("These crocodiles are cool. I love to look at crocodiles."),
                        black_box("crocodiles"),
                        black_box("cats"),
                    )
                })
            },
        );
        group.bench_function(BenchmarkId::new("String replace", i), |b| {
            b.iter(|| {
                String::from(black_box(
                    "These crocodiles are cool. I love to look at crocodiles.",
                ))
                .replace(black_box("crocodiles"), black_box("cats"))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
