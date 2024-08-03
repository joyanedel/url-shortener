use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

/// create an url of length `long` using base62 characters
fn create_url(long: usize, alphabet: &str) -> String {
    let alphabet: Vec<char> = alphabet.chars().collect();
    let alphabet_length = alphabet.len();
    let mut shortened_url = String::with_capacity(long);

    let between = Uniform::from(0..alphabet_length);
    let mut rng = thread_rng();

    while shortened_url.len() < long {
        let alphabet_index = between.sample(&mut rng);
        let letter = alphabet[alphabet_index];
        shortened_url.push(letter);
    }

    shortened_url
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
