use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{collections::HashMap, hint::black_box};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` is a hash map with `String` keys and `Progress` values.
    // map = { "variables1": Complete, "from_str": None, … }
    map.values().fold(
        0,
        |sum, progress| {
            if *progress == value {
                sum + 1
            } else {
                sum
            }
        },
    )
}

fn count_iterator_filter(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` is a hash map with `String` keys and `Progress` values.
    // map = { "variables1": Complete, "from_str": None, … }
    map.values().filter(|progress| **progress == value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator_hybrid(
    collection: &[HashMap<String, Progress>],
    value: Progress,
) -> usize {
    // `collection` is a slice of hash maps.
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
    let mut count = 0;
    for map in collection {
        count += map.values().filter(|progress| **progress == value).count();
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().fold(0, |sum_of_collection, exercises| {
        sum_of_collection
            + exercises
                .values()
                .filter(|progress| **progress == value)
                .count()
    })
}

fn get_map() -> HashMap<String, Progress> {
    let mut map = HashMap::new();
    map.insert(String::from("variables1"), Progress::Complete);
    map.insert(String::from("functions1"), Progress::Complete);
    map.insert(String::from("hashmap1"), Progress::Complete);
    map.insert(String::from("arc1"), Progress::Some);
    map.insert(String::from("as_ref_mut"), Progress::None);
    map.insert(String::from("from_str"), Progress::None);

    map
}

fn get_vec_map() -> Vec<HashMap<String, Progress>> {
    let map = get_map();

    let mut other = HashMap::new();
    other.insert(String::from("variables2"), Progress::Complete);
    other.insert(String::from("functions2"), Progress::Complete);
    other.insert(String::from("if1"), Progress::Complete);
    other.insert(String::from("from_into"), Progress::None);
    other.insert(String::from("try_from_into"), Progress::None);

    vec![map, other]
}

fn criterion_benchmark(c: &mut Criterion) {
    let progress_states = [Progress::Complete, Progress::Some, Progress::None];
    let map = black_box(get_map());
    let vec_map = black_box(get_vec_map());

    let mut group = c.benchmark_group("Count specific elements in hashmap");

    for reps in (0..progress_states.len()).step_by(progress_states.len()) {
        for (i, progress_state) in progress_states.iter().enumerate() {
            group.bench_function(BenchmarkId::new("1D with for loop", i + reps), |b| {
                b.iter(|| count_for(&map, *progress_state))
            });

            group.bench_function(
                BenchmarkId::new("1D with iterators (fold)", i + reps),
                |b| b.iter(|| count_iterator(&map, *progress_state)),
            );

            group.bench_function(
                BenchmarkId::new("1D with iterators (filter)", i + reps),
                |b| b.iter(|| count_iterator_filter(&map, *progress_state)),
            );

            group.bench_function(BenchmarkId::new("2D with for loop", i + reps), |b| {
                b.iter(|| count_collection_for(&vec_map, *progress_state))
            });

            group.bench_function(
                BenchmarkId::new("2D with iterators and loops", i + reps),
                |b| b.iter(|| count_collection_iterator_hybrid(&vec_map, *progress_state)),
            );

            group.bench_function(BenchmarkId::new("2D with iterators", i + reps), |b| {
                b.iter(|| count_collection_iterator(&vec_map, *progress_state))
            });
        }
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
