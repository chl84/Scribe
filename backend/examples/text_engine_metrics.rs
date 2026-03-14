use std::hint::black_box;
use std::time::{Duration, Instant};

use scribe_backend::domain::document::{Document, DocumentId, Edit, TextOffset, TextRange};

fn main() {
    println!("Scribe text engine performance baseline");
    println!("These measurements are local, indicative, and non-gating.");
    println!();

    let insert = measure_insert_workload();
    let delete = measure_delete_workload();
    let line_lookup = measure_line_lookup_workload();

    print_result("Insert workload", insert);
    print_result("Delete workload", delete);
    print_result("Line lookup workload", line_lookup);
}

fn measure_insert_workload() -> Measurement {
    let mut document = Document::open(DocumentId::new(1), make_fixture_text(20_000, 48));
    let iterations = 1_000usize;
    let inserted = "++instrumented-edit++";
    let start = Instant::now();

    for _ in 0..iterations {
        let offset = document
            .line_start_offset(document.line_count() / 2)
            .expect("middle line offset should exist");

        black_box(
            document
                .apply_edit(Edit::Insert {
                    offset,
                    text: inserted.to_string(),
                })
                .expect("insert workload should succeed"),
        );
    }

    Measurement::new("insert", iterations, start.elapsed())
}

fn measure_delete_workload() -> Measurement {
    let mut document = Document::open(DocumentId::new(2), make_fixture_text(20_000, 64));
    let iterations = 1_000usize;
    let delete_len = 8usize;
    let start = Instant::now();

    for _ in 0..iterations {
        let line_start = document
            .line_start_offset(document.line_count() / 2)
            .expect("middle line offset should exist");
        let range = TextRange::new(line_start, line_start.checked_add(delete_len))
            .expect("delete range should be valid");

        black_box(
            document
                .apply_edit(Edit::Delete { range })
                .expect("delete workload should succeed"),
        );
    }

    Measurement::new("delete", iterations, start.elapsed())
}

fn measure_line_lookup_workload() -> Measurement {
    let document = Document::open(DocumentId::new(3), make_fixture_text(50_000, 32));
    let offsets = build_lookup_offsets(&document, 10_000);
    let start = Instant::now();

    for offset in offsets {
        black_box(
            document
                .offset_to_position(offset)
                .expect("line lookup should succeed"),
        );
    }

    Measurement::new("line lookup", 10_000, start.elapsed())
}

fn build_lookup_offsets(document: &Document, iterations: usize) -> Vec<TextOffset> {
    let line_count = document.line_count();
    let mut offsets = Vec::with_capacity(iterations);

    for iteration in 0..iterations {
        let line = (iteration * 37) % line_count;
        let line_start = document
            .line_start_offset(line)
            .expect("line start should exist");
        offsets.push(line_start.checked_add(8));
    }

    offsets
}

fn make_fixture_text(line_count: usize, line_len: usize) -> String {
    let alphabet = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut text = String::with_capacity(line_count * (line_len + 1));

    for line in 0..line_count {
        for column in 0..line_len {
            let index = (line + column) % alphabet.len();
            text.push(alphabet[index] as char);
        }

        text.push('\n');
    }

    text
}

fn print_result(label: &str, measurement: Measurement) {
    println!(
        "{label}: total={:.2?}, iterations={}, avg={:.2?}",
        measurement.total,
        measurement.iterations,
        measurement.average()
    );
}

#[derive(Debug, Clone, Copy)]
struct Measurement {
    _name: &'static str,
    iterations: usize,
    total: Duration,
}

impl Measurement {
    const fn new(name: &'static str, iterations: usize, total: Duration) -> Self {
        Self {
            _name: name,
            iterations,
            total,
        }
    }

    fn average(self) -> Duration {
        Duration::from_secs_f64(self.total.as_secs_f64() / self.iterations as f64)
    }
}
