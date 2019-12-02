//! # aoc2019d2
//!
//! Part 2 of the day 2 challenge for Advent of Code 2019
//! but it's pretty fast
//!
//! ## Usage
//!
//! ```
//! $ cargo run --release -- <TARGET_RESULT> <INPUT_FILE> <THREAD_COUNT>
//! ```

use std::{env, fs, sync::mpsc, thread, time::Instant};

/// Parses the target result, input file and thread count from the arguments
#[inline(always)]
fn parse_args() -> (usize, String, usize) {
    // Get the passed args
    let args: Vec<String> = env::args().collect();

    // Parse the args
    let target_result = args
        .get(1)
        .expect("Missing target result")
        .parse()
        .expect("Invalid target result");
    let input_file = args.get(2).map_or("input.txt".to_owned(), |s| s.clone());
    let thread_count = args
        .get(3)
        .map_or(1, |s| s.parse().expect("Invalid thread count"));

    (target_result, input_file, thread_count)
}

/// Parses the code from the input file
#[inline(always)]
fn parse_input_file(input_file: &str) -> Vec<usize> {
    // Read the input file
    let contents = fs::read_to_string(input_file).expect("Invalid input file");
    // Split the contents at ',' and only keep valid digits
    contents.split(',').filter_map(|n| n.parse().ok()).collect()
}

/// Runs the code given a noun and a verb
#[inline(always)]
fn run(code: &[usize], n: usize, v: usize) -> usize {
    // Create a mutable copy of the code
    let mut code = code.to_vec();
    // Set the noun and verb
    code[1] = n;
    code[2] = v;

    // Iterate through the code by steps of 4
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
    // Start the timer
    let start = Instant::now();

    // Parse the input
    let (target_result, input_file, thread_count) = parse_args();
    let code = parse_input_file(&input_file);

    // Get a chunk out of 100 nouns to test for each thread
    let chunk_size = 100 / thread_count;
    let range: Vec<usize> = (0..100).collect();
    let chunks = range.chunks(chunk_size);

    // Create a MPSC channel to let threads send the answer
    let (tx, rx) = mpsc::channel();

    // Iterate through the noun chunks
    for chunk in chunks {
        // Copy variables needed locally
        let code = code.clone();
        let chunk = chunk.to_vec();
        let tx = tx.clone();

        // Spawn a new thread
        thread::spawn(move || {
            // Iterate through the nouns in the current chunks
            for n in chunk {
                // Iterate through the possible verbs
                for v in 0..100 {
                    // Send the result to the main thread if it's the target one
                    if run(&code, n, v) == target_result {
                        tx.send(100 * n + v).unwrap();
                    }
                }
            }
        });
    }

    // Wait for on of the threads to send the answer
    let result = rx.recv().unwrap();
    // Stop the timer and get the elapsed time
    let elapsed = start.elapsed();
    // Print results and exit
    println!("Result is {}", result);
    println!(
        "Done in {}μs ({}ms)",
        elapsed.as_micros(),
        elapsed.as_millis()
    );
}
