mod re_unwrap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrapr() {
        let sampled = [1, 4, 54, 2344];

        assert_eq!(unwrapr!(sampled.binary_search(&1)), 0);

        assert_eq!(unwrapr!(sampled.binary_search(&2344)), 3);
    }
}

