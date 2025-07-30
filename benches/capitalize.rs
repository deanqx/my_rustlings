use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();

    let mut capitalized = match chars.next() {
        Some(first) => first.to_uppercase().to_string(),
        None => String::new(),
    };

    capitalized.push_str(chars.as_str());
    capitalized
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut capitalized_words = Vec::with_capacity(words.len());

    for word in words {
        capitalized_words.push(capitalize_first(word));
    }

    capitalized_words
}

fn capitalize_words_vector_map(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Capitalize first");
    for i in 0..2 {
        group.bench_function(BenchmarkId::new("with map", i), |b| {
            b.iter(|| {
                capitalize_words_vector_map(black_box(&vec![
                    "these",
                    "crocodiles",
                    "are",
                    "cool",
                    "I",
                    "love",
                    "to",
                    "look",
                    "at",
                    "crocodiles",
                ]))
            })
        });
        group.bench_function(BenchmarkId::new("with for loop", i), |b| {
            b.iter(|| {
                capitalize_words_vector(black_box(&vec![
                    "these",
                    "crocodiles",
                    "are",
                    "cool",
                    "I",
                    "love",
                    "to",
                    "look",
                    "at",
                    "crocodiles",
                ]))
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
