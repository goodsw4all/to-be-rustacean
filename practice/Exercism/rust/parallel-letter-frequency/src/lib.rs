use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn count_freq(letters: &str) -> HashMap<char, usize> {
    let letters = letters.to_lowercase();
    let mut hashmap = HashMap::new();
    for ch in letters.chars().filter(|x| x.is_alphabetic()) {
        let count = hashmap.entry(ch).or_insert(0);
        *count += 1;
    }

    hashmap
}

// test bench_large_parallel   ... bench:     194,546 ns/iter (+/- 22,155)
// test bench_large_sequential ... bench:     497,254 ns/iter (+/- 18,826)
// test bench_small_parallel   ... bench:      60,001 ns/iter (+/- 16,239)
// test bench_small_sequential ... bench:      17,501 ns/iter (+/- 1,100)
// test bench_tiny_parallel    ... bench:      33,182 ns/iter (+/- 6,993)
// test bench_tiny_sequential  ... bench:         100 ns/iter (+/- 4)
pub fn frequency_(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let all_mutex: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let input = input.join("");

    let full_len = input.len() / worker_count;
    let mut chunks: Vec<&str> = Vec::new();
    let mut end = 0;

    if full_len > 0 {
        for i in 0..worker_count - 1 {
            let begin = full_len * i;
            end = begin + full_len;
            chunks.push(&input[begin..end]);
        }
    }
    chunks.push(&input[end..]);

    let mut handles = vec![];

    for chunk in chunks.iter() {
        let all_mutex = Arc::clone(&all_mutex);
        let sub_string = chunk.to_string();

        let handle = thread::spawn(move || {
            let hashmap = count_freq(&sub_string);
            let mut all = all_mutex.lock().unwrap();

            for (k, v) in hashmap {
                *all.entry(k).or_insert(0) += v;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = all_mutex.lock().unwrap().clone();
    x
}

// first try without thread
// test bench_large_parallel   ... bench:     983,592 ns/iter (+/- 27,569)
// test bench_large_sequential ... bench:     498,581 ns/iter (+/- 11,922)
// test bench_small_parallel   ... bench:      34,581 ns/iter (+/- 20,868)
// test bench_small_sequential ... bench:      17,576 ns/iter (+/- 856)
// test bench_tiny_parallel    ... bench:         354 ns/iter (+/- 25)
// test bench_tiny_sequential  ... bench:         102 ns/iter (+/- 4)
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut all = HashMap::new();

    for letters in input.iter() {
        let hashmap = count_freq(letters);

        for (k, v) in hashmap {
            *all.entry(k).or_insert(0) += v;
        }
    }

    all
}

// I couldn't understand what worker count is for the problem for now
pub fn frequency_no_use_worker_count(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let all_mutex: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for letters in input.iter() {
        let all_mutex = Arc::clone(&all_mutex);
        let word = letters.to_string();

        let handle = thread::spawn(move || {
            let hashmap = count_freq(&word);
            let mut all = all_mutex.lock().unwrap();

            for (k, v) in hashmap {
                *all.entry(k).or_insert(0) += v;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = all_mutex.lock().unwrap().clone();
    x
}
