use std::{env, process, time::Instant};

use merge_sort::{get_rand_vec, parallel_sort, serial_sort};

const TEST_COUNT: usize = 10;

fn main() {
    let mut args = env::args();
    args.next();

    let capacity: usize = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Capacidade inválida: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!("Capacidade não recebida");
            process::exit(1);
        }
    };

    let threads: Option<usize> = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => Some(num),
            Err(e) => {
                eprintln!("Número de threads inválido: {}", e);
                process::exit(1);
            }
        },
        None => None,
    };

    let durations: [u128; TEST_COUNT] = [0; TEST_COUNT];
    let durations = durations.map(|_| {
        let mut arr = get_rand_vec(capacity);
        let start = Instant::now();
        if let Some(threads) = threads {
            parallel_sort(&mut arr, threads);
        } else {
            serial_sort(&mut arr);
        }
        let duration = start.elapsed();
        duration.as_millis()
    });
    let duration: u128 = durations.iter().sum::<u128>() / durations.len() as u128;

    println!("{:?}", duration);
}
