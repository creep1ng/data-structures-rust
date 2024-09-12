// Author: Ricardo Arias.

use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

pub fn clasificar(conocidos: &[&[f64]], clases: &[i32], muestra: &[f64], k: usize) -> Option<i32> {
    if conocidos.len() != clases.len()
        || conocidos.is_empty()
        || muestra.len() != conocidos[0].len()
    {
        return None;
    }

    let mut distances: Vec<(usize, f64)> = Vec::new();
    for (index, vector) in conocidos.iter().enumerate() {
        let distance = euclidean_distance(vector, muestra);
        distances.push((index, distance.unwrap()));
    }

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    let k = k.min(distances.len());
    let nearest_neighbors = distances.iter().take(k);

    let mut class_count = HashMap::new();
    nearest_neighbors.for_each(|&(index, _)| {
        *class_count.entry(clases[index]).or_insert(0) += 1;
    });

    class_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(class, _)| class)
}

fn euclidean_distance(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() {
        return None;
    }

    let mut squared_sum: f64 = 0.0;
    for (x_i, y_i) in zip(x, y) {
        squared_sum += (y_i - x_i).powi(2);
    }
    Some(squared_sum.sqrt())
}

fn generate_random_sample_matrix(rng: &mut ThreadRng, n: usize) -> Vec<Vec<f64>> {
    let between = Uniform::from(0.0..10.0);
    (0..n)
        .map(|_| (0..2).map(|_| between.sample(rng)).collect())
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let sizes = [500, 1000, 2000, 4000, 8000, 16000, 32000]; // Different sizes for N

    for &n in &sizes {
        let sample_matrix: Vec<Vec<f64>> = generate_random_sample_matrix(&mut rng, n);
        let conocidos: Vec<&[f64]> = sample_matrix.iter().map(|v| v.as_slice()).collect();

        let clases: Vec<i32> = (0..n)
            .map(|_| if rng.gen::<bool>() { 0 } else { 1 })
            .collect();

        let muestra: Vec<f64> = (0..2).map(|_| rng.gen_range(0.0..10.0)).collect();

        let k = 5; // Number of neighbors to consider

        c.bench_function(&format!("clasificar_benchmark_n{}", n), |b| {
            b.iter(|| clasificar(&conocidos, &clases, &muestra, k))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
