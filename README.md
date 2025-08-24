Emotions from [`Feelings Wheel`](https://feelingswheel.com).

Example usage:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_tree() {
        let tree = build_emotion_tree();
        assert_eq!(tree.name, "Emotions");
        assert_eq!(tree.children.len(), 7);
        assert_eq!(tree.nodes_at_depth(1).len(), 7);
    }
}
```