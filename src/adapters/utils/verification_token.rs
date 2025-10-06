use rand::Rng;

pub fn generate_verification_token() -> String {
    let mut bytes = [0u8; 128];
    rand::rng().fill(&mut bytes);
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_verification_token_length() {
        let token = generate_verification_token();
        // 128 bytes = 256 hex characters
        assert_eq!(token.len(), 256);
    }

    #[test]
    fn test_generate_verification_token_is_hex() {
        let token = generate_verification_token();
        // Verify all characters are valid hexadecimal
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_verification_token_uniqueness() {
        let token1 = generate_verification_token();
        let token2 = generate_verification_token();
        // Two tokens should be different
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_generate_verification_token_lowercase() {
        let token = generate_verification_token();
        // Hex encoding should be lowercase
        assert_eq!(token, token.to_lowercase());
    }

    #[test]
    fn test_generate_multiple_tokens() {
        // Generate multiple tokens to ensure consistency
        let tokens: Vec<String> = (0..10).map(|_| generate_verification_token()).collect();

        // All should have correct length
        assert!(tokens.iter().all(|t| t.len() == 256));

        // All should be unique
        let unique_count = tokens
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 10);
    }
}
