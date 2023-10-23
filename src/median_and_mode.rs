use std::collections::HashMap;

pub struct MedianAndMode {
    pub median: f32,
    pub mode: i32,
}

pub fn calculate_median_and_mode(mut vec: Vec<i32>) -> MedianAndMode {
    let mut mode = vec[0];
    vec.sort();
    let len = vec.len();
    let mid = len / 2;
    let mut median: f32 = vec[mid] as f32;
    if len % 2 == 0 {
        median = (median + vec[mid - 1] as f32) / 2.0;
    }

    let mut scores = HashMap::new();
    for i in vec {
        let count = scores.entry(i).or_insert(0);
        *count += 1
    }

    let mut highest_count = 0;
    for (key, value) in scores {
        if value > highest_count {
            mode = key;
            highest_count = value;
        }
    }

    MedianAndMode {
        median: median,
        mode: mode,
    }
}
