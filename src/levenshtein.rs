/// Uses the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm to calculate the distance between two strings.
/// The lower the distance, the better the correction.
///
/// The Levenshtein Distance as a number is not a measure of string similarity.
/// For similarity of string, use levenshtein_similarity.
///
/// The algorithm is implemented using a dynamic programming approach.
/// In this, the distance between two strings is calculated by calculating the distance between the prefixes of the strings.
/// Look at the wiki page for more information.
///
/// This function is used by the Levenshtein algorithm in the Algorithm enum.
/// You are free to use it in your own code, however, the most common use case is to use the correct function.
///
/// # Arguments
///
/// * `string1` - The first string to compare.
/// * `string2` - The second string to compare.
///
/// # Returns
///
/// `u16` - The distance between the two strings.
///
/// # Example
/// ```
/// use correct_word::levenshtein::levenshtein_distance;
///
/// let distance = levenshtein_distance("hilo".to_string(), "hello".to_string());
/// assert_eq!(distance, 2);
/// ```
pub fn levenshtein_distance(string1: String, string2: String) -> u16 {
    if string1.len() < string2.len() {
        return levenshtein_distance(string2, string1);
    }

    if string2.is_empty() {
        return string1.len() as u16;
    }

    let mut previous_row: Vec<usize> = (0..string2.len() + 1).collect();
    for (i, c1) in string1.chars().enumerate() {
        let mut current_row = vec![i + 1];
        for (j, c2) in string2.chars().enumerate() {
            let insertions = previous_row[j + 1] + 1;
            let deletions = current_row[j] + 1;
            let substitutions = previous_row[j] + if c1 == c2 { 0 } else { 1 };
            current_row.push(insertions.min(deletions).min(substitutions));
        }
        previous_row = current_row;
    }

    previous_row[string2.len()] as u16
}

/// Uses the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm
/// as a measure to calculate the similarity beyween two strings.
/// Returns a f64 from 0 to 1: 1 being equal strings and 0 being completely different strings.
///
/// If you want to know if two strings are nearly equal; 0.5 is a good measure.
/// This is what you want to use in most cases instead of calcuating distance.
///
/// # Arguments
///
/// * `string1` - The first string to compare.
/// * `string2` - The second string to compare.
///
/// # Returns
///
/// `f64` - The distance between the two strings.
pub fn levenshtein_similarity(string1: String, string2: String) -> f64 {
    let distance = levenshtein_distance(string1.clone(), string2.clone());
    1.0 - (distance as f64 / std::cmp::max(string1.len(), string2.len()) as f64)
}
