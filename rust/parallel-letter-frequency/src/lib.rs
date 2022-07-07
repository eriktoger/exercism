use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut results = HashMap::new();

    if input.is_empty() {
        return results;
    }

    let mut threads = Vec::with_capacity(worker_count);
    let chunk_size = std::cmp::max(input.len() / worker_count, 1);

    for chunk in input.chunks(chunk_size) {
        let vec_chunk = Vec::from(chunk);

        let thread = thread::spawn(move || {
            let mut work = HashMap::new();

            for sentence in vec_chunk {
                for c in sentence
                    .chars()
                    .map(|c| c.to_ascii_lowercase())
                    .filter(|c| c.is_alphabetic())
                {
                    *work.entry(c).or_insert(0) += 1;
                }
            }

            return work;
        });

        threads.push(thread);
    }

    for thread in threads {
        let result = thread.join().expect("thread panicked");

        for (k, v) in result.iter() {
            *results.entry(*k).or_insert(0) += v;
        }
    }

    return results;
}
