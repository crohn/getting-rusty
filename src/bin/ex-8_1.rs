// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use getting_rusty::math::{get_median, get_mode};

fn main() {
    let list = vec![1, 3, 5, 7];
    let median = get_median(&list);
    assert_eq!(median, Ok(4.0));

    let list = vec![3, 5, 7];
    let median = get_median(&list);
    assert_eq!(median, Ok(5.0));

    let list = vec![5, 3, 7];
    let median = get_median(&list);
    assert_eq!(
        median,
        Err("Error: Cannot compute the median of an unsorted list.".to_string())
    );

    let list = vec![];
    let median = get_median(&list);
    assert_eq!(
        median,
        Err("Error: Cannot compute the median of an empty list.".to_string())
    );

    let list = vec![];
    let mode = get_mode(&list);
    assert_eq!(mode, []);

    let list = vec![1, 3, 3, 5, 7];
    let mode = get_mode(&list);
    assert_eq!(mode, vec![3]);

    let list = vec![1, 3, 3, 5, 7, 7];
    let mode = get_mode(&list);
    assert_eq!(mode, vec![3, 7]);
}
