pub fn split_at_first_occurrence<T: PartialEq + Clone>(
    original_vec: Vec<T>,
    split_value: &T,
) -> (Vec<T>, Vec<T>) {
    if let Some(index) = original_vec.iter().position(|x| x == split_value) {
        let (first_part, second_part) = original_vec.split_at(index);
        (first_part.to_vec(), second_part[1..].to_vec())
    } else {
        (original_vec, Vec::new())
    }
}
