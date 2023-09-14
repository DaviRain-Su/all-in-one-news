use crate::errors::Error;
use anyhow::Result;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns an instance of `SubscribeName` if the input satisfies all our validation constraints
    /// on subscriber name, `false` otherwise.
    pub fn parse(s: &str) -> Result<Self> {
        // `.trim()` returns a view over the input `s` without trailing
        // whiterspace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as "user-percived"
        // character: `a` is a single grapheme, but is tis composed of two characters
        // (`a` and ``).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grpheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;

        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        // Return `false` if any of our coditions have veen violated
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(Error::ValidateSubscriberNameError("Subscriber name is invalid".into()).into())
        } else {
            Ok(Self(s.to_owned()))
        }
    }

    pub fn inner(self) -> String {
        // The caller gets the inner string,
        // but they do not have a SubscriberName anymore!
        // That's because `inner` takes `self` by value,
        // consuming it according to move semantics
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        // The caller gets a mutable reference to the inner string.
        // This allows them to perform *arbitrary* changes to
        // value itself, potentially breaking our invariants!
        &mut self.0
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_eq!(SubscriberName::parse(&name).is_ok(), true);
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_eq!(SubscriberName::parse(&name).is_err(), true);
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_eq!(SubscriberName::parse(&name).is_err(), true);
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_eq!(SubscriberName::parse(&name).is_err(), true);
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_eq!(SubscriberName::parse(&name).is_err(), true);
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_eq!(SubscriberName::parse(&name).is_ok(), true);
    }
}
