use rand::{thread_rng, Rng};

/// create an url of length `long` using characters from `alphabet`
fn create_url(long: usize, alphabet: &str) -> Option<String> {
    if alphabet.is_empty() {
        return None;
    }

    let alphabet: Vec<char> = alphabet.chars().collect();
    let alphabet_length = alphabet.len();

    let mut rng = thread_rng();
    Some(
        (0..long)
            .map(|_| rng.gen_range(0..alphabet_length))
            .map(|idx| alphabet[idx])
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::create_url;

    #[test]
    fn create_url_produce_valid_value() {
        let alphabet = "0123456789";
        let long = 3;

        let result = create_url(long, alphabet);

        assert_eq!(result.as_ref().unwrap().len(), long);
        assert!(result
            .as_ref()
            .unwrap()
            .chars()
            .all(|c| alphabet.contains(c)));
    }

    #[test]
    fn create_url_with_length_zero_returns_empty_url() {
        let alphabet = "123";
        let long = 0;

        let result = create_url(long, alphabet);

        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn create_url_with_empty_alphabet_returns_none() {
        let alphabet = "";
        let long = 1;

        let result = create_url(long, alphabet);

        assert!(result.is_none());
    }
}
