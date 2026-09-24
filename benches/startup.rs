//! Measures how long the `lithra` binary takes to start and answer `--version`.

use std::process::{Command, Stdio};

use criterion::{Criterion, criterion_group, criterion_main};

fn startup(c: &mut Criterion) {
    c.bench_function("lithra --version", |b| {
        b.iter(|| {
            let status = Command::new(env!("CARGO_BIN_EXE_lithra"))
                .arg("--version")
                .stdout(Stdio::null())
                .status();
            assert!(status.is_ok_and(|status| status.success()));
        });
    });
}

criterion_group!(benches, startup);
criterion_main!(benches);
