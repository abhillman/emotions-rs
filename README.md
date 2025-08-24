# `emotions-rs`

Emotions from [`Emotion Wheel`](https://observablehq.com/@mbostock/emotion-wheel) available as a rust crate.

>This is a recreation of Geoffrey Roberts’s Emotion Wheel using D3’s partition layout. Robert’s 2015 work appears to be based on a vocabulary wheel by Kaitlin Robbs from 2014, which in turn appears to be based on The Feeling Wheel published by Gloria Willcox in 1982.

## Example usage

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