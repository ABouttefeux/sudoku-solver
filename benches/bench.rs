use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use sudoku::grid::Sudoku;

fn criterion_benchmark(c: &mut Criterion) {
    let mut groupe = c.benchmark_group("solver");
    groupe.throughput(Throughput::Elements(1_u64));
    groupe.bench_function("deduction solve given", |b| {
        b.iter(|| {
            let mut sudoku = Sudoku::<3>::new([
                [3, 0, 0, 6, 0, 0, 0, 9, 0],
                [0, 4, 0, 0, 2, 0, 0, 5, 0],
                [0, 8, 0, 0, 7, 0, 1, 6, 0],
                [9, 0, 0, 3, 0, 4, 7, 0, 0],
                [0, 5, 0, 0, 8, 0, 0, 2, 0],
                [0, 0, 1, 9, 0, 0, 0, 0, 6],
                [0, 2, 7, 0, 3, 0, 0, 4, 0],
                [0, 9, 0, 0, 6, 0, 0, 1, 0],
                [0, 3, 0, 0, 0, 5, 0, 0, 8],
            ]);
            let _ = sudoku.try_solve().unwrap();
        })
    });

    groupe.bench_function("solve back trace given", |b| {
        b.iter(|| {
            let mut sudoku = Sudoku::<3>::new([
                [3, 0, 0, 6, 0, 0, 0, 9, 0],
                [0, 4, 0, 0, 2, 0, 0, 5, 0],
                [0, 8, 0, 0, 7, 0, 1, 6, 0],
                [9, 0, 0, 3, 0, 4, 7, 0, 0],
                [0, 5, 0, 0, 8, 0, 0, 2, 0],
                [0, 0, 1, 9, 0, 0, 0, 0, 6],
                [0, 2, 7, 0, 3, 0, 0, 4, 0],
                [0, 9, 0, 0, 6, 0, 0, 1, 0],
                [0, 3, 0, 0, 0, 5, 0, 0, 8],
            ]);
            let _ = sudoku.solve_back_trace().unwrap();
        })
    });
    groupe.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
