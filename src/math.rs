use std::{cmp, collections::HashMap};

/// Returns the median of the provided list of integers.
///
/// The provided list of integers must be non-empty and sorted.
///
/// In case the list contains an even number of elements, the median will be the
/// mean of the two middle elements, otherwise it will be the middle element.
///
pub fn get_median(list: &[i32]) -> Result<f32, String> {
    if list.is_empty() {
        return Err("Error: Cannot compute the median of an empty list.".to_string());
    }

    if !list.is_sorted() {
        return Err("Error: Cannot compute the median of an unsorted list.".to_string());
    }

    let len = list.len();

    if len % 2 == 0 {
        Ok(get_median_len_even(list, len))
    } else {
        Ok(get_median_len_odd(list, len))
    }
}

/// Example
/// ```txt
/// [1, 2, 3, 4] -> 2.5
///     ^. ^----.
///       `-------` Returns the mean of these two
/// ```
fn get_median_len_even(list: &[i32], len: usize) -> f32 {
    let p = (len - 1) / 2;
    // cast before division, to preserve precision
    (list[p] as f32 + list[p + 1] as f32) / 2.0
}

/// Example
/// ```txt
/// [1, 2, 3] -> 2
///     ^--- Returns this
/// ```
fn get_median_len_odd(list: &[i32], len: usize) -> f32 {
    let p = (len - 1) / 2;
    list[p] as f32
}

/// Returns the mode of the provided list.
///
/// The mode is the value (or the values) that occurs more frequently in the list.
///
/// Examples
/// ```txt
/// [1, 1, 2, 3, 3, 3] -> 3
///           ^  ^  ^
///
/// [1, 1, 2, 3, 3] -> [1, 3]
///  ^  ^     ^  ^
/// ```
pub fn get_mode(list: &[i32]) -> Vec<i32> {
    let (frequencies, max_frequency): (HashMap<i32, u32>, u32) = list.iter().fold(
        (HashMap::new(), 0),
        |(mut frequencies, mut max_frequency), n| {
            let count = frequencies.entry(*n).or_default();
            *count += 1;

            max_frequency = cmp::max(max_frequency, *count);

            (frequencies, max_frequency)
        },
    );

    frequencies
        .into_iter()
        .filter_map(|(n, frequency)| {
            if frequency == max_frequency {
                Some(n)
            } else {
                None
            }
        })
        .collect()
}
