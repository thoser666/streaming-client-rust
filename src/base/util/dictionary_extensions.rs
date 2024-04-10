mod dictionary_extensins {
    use std::collections::HashMap;
    use std::hash::Hash;

    /// Gets the value at the specified key or the default of the value type.
    ///
    /// # Arguments
    /// * `map` - A reference to the hashmap to search through.
    /// * `key` - The key to look up.
    ///
    /// # Returns
    /// The value at the specified key or the default of the value type.
    pub fn get_or_default<K, V>(map: &HashMap<K, V>, key: K) -> V
    where
        K: Eq + Hash,
        V: Default + Clone,
    {
        match map.get(&key) {
            Some(value) => value.clone(),
            None => V::default(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_or_default_existing_key() {
            let mut map = HashMap::new();
            map.insert("apple", 3);
            let count = get_or_default(&map, "apple");
            assert_eq!(count, 3, "Should return the value for existing key.");
        }

        #[test]
        fn test_get_or_default_non_existing_key() {
            let mut map = HashMap::new();
            map.insert("apple", 3);
            let count = get_or_default(&map, "orange");
            assert_eq!(
                count, 0,
                "Should return the default value for non-existing key."
            );
        }

        #[test]
        fn test_get_or_default_with_custom_type() {
            #[derive(Debug, Clone, PartialEq, Default)]
            struct Fruit {
                name: String,
                quantity: usize,
            }

            let mut map = HashMap::new();
            map.insert(
                "apple",
                Fruit {
                    name: "Apple".to_string(),
                    quantity: 10,
                },
            );

            let result = get_or_default(&map, "apple");
            assert_eq!(
                result,
                Fruit {
                    name: "Apple".to_string(),
                    quantity: 10
                },
                "Should return the correct fruit object."
            );

            let default_fruit = get_or_default(&map, "banana");
            assert_eq!(
                default_fruit,
                Fruit::default(),
                "Should return the default fruit object."
            );
        }
    }

    fn main() {
        // The main function remains empty, as we use this file mainly for the library functionality and tests.
    }
}
