pub mod levenshtein;

/// # Struct: Correct Word
/// A struct used to represent the result of the correct function.
/// It has the word and the confidence of the correction.
/// The word is an optional string, because the function might not be able to correct the word, given the threshold.
/// The confidence is the distance between the input and the corrected word.
/// The lower the distance, the better the correction.
pub struct CorrectWord{
    pub word: Option<String>,
    pub confidence: u16,
}

/// # Enum: Algorithm
/// This enum defines the algorithms that can be used to correct a word.
///
/// Currently, the following algorithms are supported:
/// * Levenshtein: A simple algorithm that calculates the distance between two strings. The lower the distance, the better the correction.
///
/// # Example
/// This enum is used as an argument to the correct function.
/// ```
/// use correct_word::correct_word;
/// use correct_word::Algorithm;
///
/// let result = correct_word(Algorithm::Levenshtein, "hilo".to_string(), vec!["hello".to_string(), "world".to_string()], Some(5));
/// assert_eq!(result.word.unwrap(), "hello");
/// ```
pub enum Algorithm {
    Levenshtein,
}

/// Correct a word from a list of options.
/// Takes in a word and a list of options, and returns the best option.
///
/// # Arguments
/// * `algorithm` - The algorithm to use to correct the word. The algorithm is an enum, as defined in the [Algorithm](enum.Algorithm.html) enum.
/// * `input` - The word to correct.
/// * `options` - A list of options to correct the word to.
/// * `threshold` - The maximum distance between the input and the corrected word. If the distance is greater than the threshold, the function will return None.
///
/// # Returns
///
/// `[CorrectWord](type.CorrectWord.html) = (Option<String>, u16))`
///
/// This function returns a tuple of an optional string and a u16. The string is the corrected word, and the u16 is the distance between the input and the corrected word.
///
/// # Example
/// ```
/// use correct_word::correct_word;
/// use correct_word::Algorithm;
///
/// let result = correct_word(Algorithm::Levenshtein, "hilo".to_string(), vec!["hello".to_string(), "world".to_string()], Some(5));
/// assert_eq!(result.word.unwrap(), "hello");
/// ```
///
/// # Note
///
/// All the algorithms used in this crate return a distance between the input and the corrected word.
/// The lower the distance, the better the correction.
/// You can use the threshold to make sure that the function doesn't return a word that is too different from the input.
/// By default, the threshold is 0, which means that the function will return the best option, even if it is very different from the input.
/// So, it is up to you to choose the level of tolerance you want.
/// Usually, a threshold of 5 is a good value.
pub fn correct_word(
    algorithm: Algorithm,
    input: String,
    options: Vec<String>,
    threshold: Option<u8>,
) -> CorrectWord {
    let mut best = String::new();
    let mut best_distance = u16::MAX;
    options.iter().for_each(|option| {
        let distance = match algorithm {
            Algorithm::Levenshtein => {
                levenshtein::levenshtein_distance(input.to_string(), option.to_string())
            }
        };
        if distance < best_distance {
            best = option.to_string();
            best_distance = distance;
        }
    });

    if best_distance > threshold.unwrap_or(0) as u16 {
        CorrectWord {
            word: None,
            confidence: best_distance,
        }
    } else {
        CorrectWord {
            word: Some(best),
            confidence: best_distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_test() {
        let result = correct_word(
            Algorithm::Levenshtein,
            "he".to_string(),
            vec!["hello".to_string(), "world".to_string(), "hi".to_string()],
            Some(5),
        );
        assert_eq!(result.word.unwrap(), "hi");
    }
}
