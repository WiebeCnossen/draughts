#[macro_use]
extern crate criterion;

use criterion::Criterion;
use draughts::board::generator::Generator;
use draughts::board::position::Position;

fn generator_benchmark(c: &mut Criterion) {
    let position = Position::parse("wBckkk55rrrWt").unwrap();
    c.bench_function("black king", move |b| b.iter(|| position.piece_at(0)));
    c.bench_function("black piece", move |b| b.iter(|| position.piece_at(5)));
    c.bench_function("empty", move |b| b.iter(|| position.piece_at(20)));
    c.bench_function("white piece", move |b| b.iter(|| position.piece_at(40)));
    c.bench_function("white king", move |b| b.iter(|| position.piece_at(45)));

    macro_rules! bench_generator {
        ($id: ident, $position: literal) => {
            let generator = Generator::create();
            let $id = Position::parse($position).unwrap();
            let mut list = Vec::with_capacity(63);
            c.bench_function(stringify!($id), move |b| {
                b.iter(|| generator.legal_moves2(&$id, &mut list))
            });
        };
    }

    bench_generator!(study, "w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww");
    bench_generator!(initial, "wkkkk55rrrr");
    bench_generator!(multi_long_capture, "w 5/5/3b1/5/5/5/5/1b3/5/W4");
    bench_generator!(coup_turc, "b 5/el2/5/Bebew/2w2/5/eh2/3we/ew3/5");
    bench_generator!(goerres_bayar, "wcebeaka22b25rreteie");
}

criterion_group!(benches, generator_benchmark);
criterion_main!(benches);
