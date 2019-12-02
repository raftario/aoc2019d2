use std::time::Instant;

static CODE: &[usize] = include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[inline(always)]
fn run(n: usize, v: usize) -> usize {
    let mut code = CODE.to_vec();
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

    for n in 0..100 {
        for v in 0..100 {
            if run(n, v) == 19690720 {
                println!("Result is {}", 100 * n + v);

                let elapsed = start.elapsed();
                println!(
                    "Done in {}μs ({}ms)",
                    elapsed.as_micros(),
                    elapsed.as_millis()
                );

                return;
            }
        }
    }
}
