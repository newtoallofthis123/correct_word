pub mod levenshtein;

/// # Struct: Correct Word
/// A struct used to represent the result of the correct function.
/// It has the word and the confidence of the correction.
/// The word is an optional string, because the function might not be able to correct the word, given the threshold.
/// The confidence is the similarity between the input and the corrected word between 0 to 1.
/// The higher the similarity, the better the correction.
pub struct CorrectWord {
    pub word: Option<String>,
    pub confidence: f64,
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
/// let result = correct_word(Algorithm::Levenshtein, "hilo".to_string(), vec!["hello".to_string(), "world".to_string()], None);
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
/// `[CorrectWord](type.CorrectWord.html) = (Option<String>, f64))`
///
/// This function returns a tuple of an optional string and a f64. The string is the corrected word, and the f64 is the similary between the input and the corrected word in the range from 0 to 1
///
/// # Example
/// ```
/// use correct_word::correct_word;
/// use correct_word::Algorithm;
///
/// let result = correct_word(Algorithm::Levenshtein, "hilo".to_string(), vec!["hello".to_string(), "world".to_string()], Some(0.4));
/// assert_eq!(result.word.unwrap(), "hello");
/// ```
///
/// # Note
///
/// All the algorithms used in this crate return a distance between the input and the corrected word.
/// The lower the distance, the better the correction.
/// You can use the threshold to make sure that the function doesn't return a word that is too different from the input.
/// By default, the threshold is 0.5.
/// So, it is up to you to choose the level of tolerance you want.
/// Usually, a threshold of 0.5 is a good value which is the default.
pub fn correct_word(
    algorithm: Algorithm,
    input: String,
    options: Vec<String>,
    threshold: Option<f64>,
) -> CorrectWord {
    let mut best = String::new();
    let mut best_now = 0.0;
    options.iter().for_each(|option| {
        let distance = match algorithm {
            Algorithm::Levenshtein => {
                levenshtein::levenshtein_similarity(input.to_string(), option.to_string())
            }
        };
        if distance > best_now {
            best = option.to_string();
            best_now = distance;
        }
    });

    if best_now < threshold.unwrap_or(0.5) {
        CorrectWord {
            word: None,
            confidence: best_now,
        }
    } else {
        CorrectWord {
            word: Some(best),
            confidence: best_now,
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
            None,
        );
        assert_eq!(result.word.unwrap(), "hi");
    }
}
