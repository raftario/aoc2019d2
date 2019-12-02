use std::{env, fs, sync::mpsc, thread, time::Instant};

#[inline(always)]
fn parse_args() -> (usize, String, usize) {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    let target_result = if args_len >= 2 {
        args[1].parse().expect("Invalid target result")
    } else {
        panic!("Missing target result");
    };
    let input_file = if args_len >= 3 {
        args[2].clone()
    } else {
        "input.txt".to_owned()
    };
    let thread_count = if args_len >= 4 {
        args[3].parse().expect("Invalid thread count")
    } else {
        1
    };

    (target_result, input_file, thread_count)
}

#[inline(always)]
fn parse_input_file(input_file: &str) -> Vec<usize> {
    let contents = fs::read_to_string(input_file).expect("Invalid input file");
    contents.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[inline(always)]
fn run(code: &[usize], n: usize, v: usize) -> usize {
    let mut code = code.to_vec();
    code[1] = n;
    code[2] = v;

    for i in (0..code.len()).step_by(4) {
        match code[i] {
            1 => {
                let idx = (code[i + 1], code[i + 2], code[i + 3]);
                code[idx.2] = code[idx.0] + code[idx.1];
            }
            2 => {
                let idx = (code[i + 1], code[i + 2], code[i + 3]);
                code[idx.2] = code[idx.0] * code[idx.1];
            }
            99 => break,
            _ => panic!("Invalid code"),
        }
    }

    code[0]
}

fn main() {
    let start = Instant::now();

    let (target_result, input_file, thread_count) = parse_args();
    let code = parse_input_file(&input_file);

    let chunk_size = 100 / thread_count;
    let range: Vec<usize> = (0..100).collect();
    let chunks = range.chunks(chunk_size);

    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let code = code.clone();
        let chunk = chunk.to_vec();
        let tx = tx.clone();

        thread::spawn(move || {
            for n in chunk {
                for v in 0..100 {
                    if run(&code, n, v) == target_result {
                        tx.send(100 * n + v).unwrap();
                    }
                }
            }
        });
    }

    let result = rx.recv().unwrap();
    let elapsed = start.elapsed();
    println!("Result is {}", result);
    println!(
        "Done in {}μs ({}ms)",
        elapsed.as_micros(),
        elapsed.as_millis()
    );
}
