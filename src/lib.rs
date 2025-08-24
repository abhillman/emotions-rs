/// A very small, self-contained module you can drop into any project.
#[derive(Debug)]
pub struct EmotionNode<'a> {
    pub name: &'a str,
    pub children: Vec<EmotionNode<'a>>,
}

/// Build the complete “feelings wheel” as a tree.
pub fn build_emotion_tree<'a>() -> EmotionNode<'a> {
    use EmotionNode as N;

    N {
        name: "Emotions",
        children: vec![
            // Fearful ──────────────────────────────────────────────────────────
            N {
                name: "Fearful",
                children: vec![
                    N {
                        name: "Scared",
                        children: vec![
                            N {
                                name: "Helpless",
                                children: vec![],
                            },
                            N {
                                name: "Frightened",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Anxious",
                        children: vec![
                            N {
                                name: "Overwhelmed",
                                children: vec![],
                            },
                            N {
                                name: "Worried",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Insecure",
                        children: vec![
                            N {
                                name: "Inadequate",
                                children: vec![],
                            },
                            N {
                                name: "Inferior",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Weak",
                        children: vec![
                            N {
                                name: "Worthless",
                                children: vec![],
                            },
                            N {
                                name: "Insignificant",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Rejected",
                        children: vec![
                            N {
                                name: "Excluded",
                                children: vec![],
                            },
                            N {
                                name: "Persecuted",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Threatened",
                        children: vec![
                            N {
                                name: "Nervous",
                                children: vec![],
                            },
                            N {
                                name: "Exposed",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            // Angry ────────────────────────────────────────────────────────────
            N {
                name: "Angry",
                children: vec![
                    N {
                        name: "Let down",
                        children: vec![
                            N {
                                name: "Betrayed",
                                children: vec![],
                            },
                            N {
                                name: "Resentful",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Humiliated",
                        children: vec![
                            N {
                                name: "Disrespected",
                                children: vec![],
                            },
                            N {
                                name: "Ridiculed",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Bitter",
                        children: vec![
                            N {
                                name: "Indignant",
                                children: vec![],
                            },
                            N {
                                name: "Violated",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Mad",
                        children: vec![
                            N {
                                name: "Furious",
                                children: vec![],
                            },
                            N {
                                name: "Jealous",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Aggressive",
                        children: vec![
                            N {
                                name: "Provoked",
                                children: vec![],
                            },
                            N {
                                name: "Hostile",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Frustrated",
                        children: vec![
                            N {
                                name: "Infuriated",
                                children: vec![],
                            },
                            N {
                                name: "Annoyed",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Distant",
                        children: vec![
                            N {
                                name: "Withdrawn",
                                children: vec![],
                            },
                            N {
                                name: "Numb",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Critical",
                        children: vec![
                            N {
                                name: "Skeptical",
                                children: vec![],
                            },
                            N {
                                name: "Dismissive",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            // Disgusted ────────────────────────────────────────────────────────
            N {
                name: "Disgusted",
                children: vec![
                    N {
                        name: "Disapproving",
                        children: vec![
                            N {
                                name: "Judgmental",
                                children: vec![],
                            },
                            N {
                                name: "Embarrassed",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Disappointed",
                        children: vec![
                            N {
                                name: "Appalled",
                                children: vec![],
                            },
                            N {
                                name: "Revolted",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Awful",
                        children: vec![
                            N {
                                name: "Nauseated",
                                children: vec![],
                            },
                            N {
                                name: "Detestable",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Repelled",
                        children: vec![
                            N {
                                name: "Horrified",
                                children: vec![],
                            },
                            N {
                                name: "Hesitant",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            // Sad ──────────────────────────────────────────────────────────────
            N {
                name: "Sad",
                children: vec![
                    N {
                        name: "Hurt",
                        children: vec![
                            N {
                                name: "Embarrassed",
                                children: vec![],
                            },
                            N {
                                name: "Disappointed",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Depressed",
                        children: vec![
                            N {
                                name: "Inferior",
                                children: vec![],
                            },
                            N {
                                name: "Empty",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Guilty",
                        children: vec![
                            N {
                                name: "Remorseful",
                                children: vec![],
                            },
                            N {
                                name: "Ashamed",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Despair",
                        children: vec![
                            N {
                                name: "Powerless",
                                children: vec![],
                            },
                            N {
                                name: "Grief",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Vulnerable",
                        children: vec![
                            N {
                                name: "Fragile",
                                children: vec![],
                            },
                            N {
                                name: "Victimized",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Lonely",
                        children: vec![
                            N {
                                name: "Abandoned",
                                children: vec![],
                            },
                            N {
                                name: "Isolated",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            // Happy ─────────────────────────────────────────────────────────────
            N {
                name: "Happy",
                children: vec![
                    N {
                        name: "Optimistic",
                        children: vec![
                            N {
                                name: "Inspired",
                                children: vec![],
                            },
                            N {
                                name: "Hopeful",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Trusting",
                        children: vec![
                            N {
                                name: "Intimate",
                                children: vec![],
                            },
                            N {
                                name: "Sensitive",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Peaceful",
                        children: vec![
                            N {
                                name: "Thankful",
                                children: vec![],
                            },
                            N {
                                name: "Loving",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Powerful",
                        children: vec![
                            N {
                                name: "Creative",
                                children: vec![],
                            },
                            N {
                                name: "Courageous",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Accepted",
                        children: vec![
                            N {
                                name: "Valued",
                                children: vec![],
                            },
                            N {
                                name: "Respected",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Proud",
                        children: vec![
                            N {
                                name: "Confident",
                                children: vec![],
                            },
                            N {
                                name: "Successful",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Interested",
                        children: vec![
                            N {
                                name: "Inquisitive",
                                children: vec![],
                            },
                            N {
                                name: "Curious",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Content",
                        children: vec![
                            N {
                                name: "Joyful",
                                children: vec![],
                            },
                            N {
                                name: "Free",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Playful",
                        children: vec![
                            N {
                                name: "Cheeky",
                                children: vec![],
                            },
                            N {
                                name: "Aroused",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            // Surprised ────────────────────────────────────────────────────────
            N {
                name: "Surprised",
                children: vec![
                    N {
                        name: "Excited",
                        children: vec![
                            N {
                                name: "Energetic",
                                children: vec![],
                            },
                            N {
                                name: "Eager",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Amazed",
                        children: vec![
                            N {
                                name: "Awe",
                                children: vec![],
                            },
                            N {
                                name: "Astonished",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Confused",
                        children: vec![
                            N {
                                name: "Perplexed",
                                children: vec![],
                            },
                            N {
                                name: "Disillusioned",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Startled",
                        children: vec![
                            N {
                                name: "Dismayed",
                                children: vec![],
                            },
                            N {
                                name: "Shocked",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            // Bad ──────────────────────────────────────────────────────────────
            N {
                name: "Bad",
                children: vec![
                    N {
                        name: "Tired",
                        children: vec![
                            N {
                                name: "Unfocused",
                                children: vec![],
                            },
                            N {
                                name: "Sleepy",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Stressed",
                        children: vec![
                            N {
                                name: "Out of control",
                                children: vec![],
                            },
                            N {
                                name: "Overwhelmed",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Busy",
                        children: vec![
                            N {
                                name: "Rushed",
                                children: vec![],
                            },
                            N {
                                name: "Pressured",
                                children: vec![],
                            },
                        ],
                    },
                    N {
                        name: "Bored",
                        children: vec![
                            N {
                                name: "Apathetic",
                                children: vec![],
                            },
                            N {
                                name: "Indifferent",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
        ],
    }
}

impl<'a> EmotionNode<'a> {
    /// Collects _immutable_ references to all nodes that sit `depth`
    /// layers below `self`.
    ///
    /// • `depth == 0` → returns a `Vec` containing only `self`
    /// • deeper than the tree’s height → returns an empty `Vec`
    pub fn nodes_at_depth(&self, depth: usize) -> Vec<&EmotionNode<'a>> {
        if depth == 0 {
            return vec![self];
        }

        // Recurse into every child, asking each for nodes `depth - 1` below it.
        let mut acc = Vec::new();
        for child in &self.children {
            acc.extend(child.nodes_at_depth(depth - 1));
        }
        acc
    }
}

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
