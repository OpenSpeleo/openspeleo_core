use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::io::Read;
use zip::ZipArchive;

// We need to make the functions public first, so we'll benchmark through Python interface
use pyo3::prelude::*;

fn benchmark_load_ariane_tml(c: &mut Criterion) {
    let test_files = vec![
        "tests/artifacts/hand_survey.tml",
        "tests/artifacts/test_simple.mini.tml",
        "tests/artifacts/test_simple.tml",
        "tests/artifacts/test_with_walls.tml",
        "tests/artifacts/test_large.tml",
    ];

    Python::with_gil(|py| {
        let module = PyModule::import(py, "openspeleo_core.ariane_core").unwrap();

        for filepath in test_files {
            let name = filepath.split('/').last().unwrap_or(filepath);

            c.bench_function(&format!("load_tml_{}", name), |b| {
                b.iter(|| {
                    let result = module
                        .getattr("load_ariane_tml_file_to_dict")
                        .unwrap()
                        .call1((filepath,))
                        .unwrap();
                    black_box(result);
                });
            });
        }
    });
}

fn benchmark_xml_parsing(c: &mut Criterion) {
    // Load XML content from a test file
    let file = fs::File::open("tests/artifacts/test_simple.tml").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut archive = ZipArchive::new(reader).unwrap();
    let mut xml_file = archive.by_name("Data.xml").unwrap();
    let mut xml_contents = String::new();
    xml_file.read_to_string(&mut xml_contents).unwrap();

    Python::with_gil(|py| {
        let module = PyModule::import(py, "openspeleo_core._rust_lib.ariane").unwrap();

        c.bench_function("xml_str_to_dict", |b| {
            b.iter(|| {
                let result = module
                    .getattr("xml_str_to_dict")
                    .unwrap()
                    .call1((&xml_contents, false))
                    .unwrap();
                black_box(result);
            });
        });
    });
}

criterion_group!(benches, benchmark_load_ariane_tml, benchmark_xml_parsing);
criterion_main!(benches);
