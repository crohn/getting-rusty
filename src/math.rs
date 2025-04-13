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
