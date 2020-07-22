#![allow(dead_code)]

struct StringReverser {}

impl StringReverser {
    fn reverse(input: &str) -> String {
        input.chars().rev().collect()
    }

    fn reverse_method(&self, input: &str) -> String {
        StringReverser::reverse(input)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn reverse_method__succeeds_with_healthy_input() {
        // Given 
        let input = "abcd";
        let obj = StringReverser{};

        // When 
        let result = obj.reverse_method(input);

        // Then - should not error
        assert_eq!(result, "dcba");
    }

    #[test]
    fn reverse__succeeds_with_healthy_input() {
        // Given 
        let input = "abcd";

        // When 
        let result = StringReverser::reverse(input);

        // Then - should not error
        assert_eq!(result, "dcba");
    }
    
    #[test]
    fn reverse__succeeds_with_second_input() {
        // Given
        let input = "1234";

        // When
        let result = StringReverser::reverse(input);

        // Then
        assert_eq!(result, "4321");
    }

    #[test]
    fn reverse__succeeds_with_space() {
        // Given
        let input = "x yz";

        // When
        let result = StringReverser::reverse(input);

        // Then
        assert_eq!(result, "zy x");
    }
}