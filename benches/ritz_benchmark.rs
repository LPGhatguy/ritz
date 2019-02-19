use criterion::{Criterion, criterion_group, criterion_main};

use ritz::html;

fn simple(c: &mut Criterion) {
    c.bench_function("simple", |b| b.iter_with_large_drop(|| {
        let page = html! {
            <div>"Hello, world!"</div>
        };

        page.to_string()
    }));
}

fn nested(c: &mut Criterion) {
    c.bench_function("nested", |b| b.iter_with_large_drop(|| {
        let page = html! {
            <div>
                <span>
                    <label>
                        "Hi!"
                    </label>
                </span>
            </div>
        };

        page.to_string()
    }));
}

fn adjacent(c: &mut Criterion) {
    c.bench_function("adjacent", |b| b.iter_with_large_drop(|| {
        let page = html! {
            <div>
                <span />
                <span />
                <span />
                <span />
                <span />
                <span />
            </div>
        };

        page.to_string()
    }));
}

criterion_group!(benches, simple, nested, adjacent);
criterion_main!(benches);