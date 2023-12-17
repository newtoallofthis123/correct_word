# Correct Word

A no brain "did you mean" suggestions generator written in Rust.
Plans to use a weird array of algorithms to get the best results.
But for now, it only uses the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance).

## Usage

Complete docs can be found [here](https://docs.rs/correct_word/latest/).

```rust
use correct_word::correct_word;
use correct_word::Algorithm;

fn main() {
    let word = "helo";
    let dictionary = vec!["hello", "world", "hell", "help", "helo", "hola"];
    let suggestion = correct_word(Algorithm::Levenshtein, word, dictionary, None);
    println!("{} with confidence {}", suggestion.0, suggestion.1);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
