use rand::{thread_rng, Rng};

/// create an url of length `long` using characters from `alphabet`
fn create_url(long: usize, alphabet: &str) -> String {
    let alphabet: Vec<char> = alphabet.chars().collect();
    let alphabet_length = alphabet.len();

    let mut rng = thread_rng();
    (0..long)
        .map(|_| rng.gen_range(0..alphabet_length))
        .map(|idx| alphabet[idx])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::create_url;

    #[test]
    fn create_url_produce_valid_value() {
        let alphabet = "0123456789";
        let long = 3;

        let result = create_url(long, alphabet);

        assert_eq!(result.len(), long);
        assert!(result.chars().all(|c| alphabet.contains(c)));
    }
}
