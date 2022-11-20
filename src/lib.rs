use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

impl PartialOrd for Dir {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Dir::Left, Dir::Right) => Some(Ordering::Less),
            (Dir::Right, Dir::Left) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for Dir {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => panic!("unexpected direction"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Trie {
    pub directions: Vec<Dir>,
    pub childrens: Vec<Trie>,
    pub terminate: Vec<String>,
}

impl PartialOrd for Trie {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.directions.cmp(&other.directions))
    }
}

impl Ord for Trie {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Trie {
    pub fn insert(&mut self, directions: &[Dir], name: String) {
        if directions.is_empty() {
            // nice, we finished inserting our string
            return;
        }

        let common_prefix = longuest_prefix(&self.directions, directions);
        if common_prefix == self.directions.len() {
            // the inserted element matches 100% of our direction.
            if directions.len() == common_prefix {
                // if it also matches 100% of the inserted element then it's a terminal node
                // and we can stop.
                self.terminate.push(name);
            } else {
                // we need to insert this node in one of our child
                let directions_to_insert = &directions[common_prefix..];
                for children in self.childrens.iter_mut() {
                    if children.directions[0] == directions_to_insert[0] {
                        children.insert(directions_to_insert, name);
                        return;
                    }
                }
                // if we reach this point it means there was not children anywhere
                // thus we're going to create a new node just for this string.
                self.childrens.push(Trie {
                    directions: directions_to_insert.to_vec(),
                    childrens: Vec::new(),
                    terminate: vec![name],
                });
            }
        } else {
            // we'll need to split ourselves into two node
            let prefix = &self.directions[common_prefix..];
            let suffix = &self.directions[0..common_prefix];

            let suffix = Trie {
                directions: suffix.to_vec(),
                childrens: self.childrens.clone(),
                terminate: self.terminate.clone(),
            };

            if directions.len() == common_prefix {
                // It's a terminal node we can stop right there.
                self.directions = prefix.to_vec();
                self.childrens = vec![suffix];
                self.terminate = vec![name];
            } else {
                let new = Trie {
                    directions: directions[common_prefix..].to_vec(),
                    childrens: Vec::new(),
                    terminate: vec![name],
                };
                self.directions = prefix.to_vec();
                self.childrens = vec![suffix, new];
                self.terminate = Vec::new();
            }
        }
    }

    pub fn finish(&mut self) {
        self.childrens.iter_mut().for_each(Self::finish);
        self.childrens.sort();
        self.terminate.sort();
    }

    pub fn first(&self) -> &str {
        self.terminate
            .first()
            .map(|s| s.as_ref())
            .unwrap_or_else(|| self.childrens[0].first())
    }
}

fn longuest_prefix(left: &[Dir], right: &[Dir]) -> usize {
    left.iter()
        .zip(right)
        .take_while(|(l, r)| **l == **r)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        use Dir::*;

        assert_eq!(
            longuest_prefix(&vec![Left, Left, Left, Left], &vec![Left, Left, Left, Left]),
            4
        );

        assert_eq!(
            longuest_prefix(
                &vec![Left, Left, Right, Right],
                &vec![Left, Left, Left, Left]
            ),
            2
        );

        assert_eq!(longuest_prefix(&vec![Left, Left, Right, Right], &vec![]), 0);
    }

    #[test]
    fn test_trie() {
        use Dir::*;

        let mut trie = Trie::default();

        // simple insert
        trie.insert(&vec![Left, Left, Left, Left], String::from("bob"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: [],
            childrens: [
                Trie {
                    directions: [
                        Left,
                        Left,
                        Left,
                        Left,
                    ],
                    childrens: [],
                    terminate: [
                        "bob",
                    ],
                },
            ],
            terminate: [],
        }
        "###);

        // insert with no relation
        trie.insert(&vec![Right, Right, Right, Right], String::from("alice"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: [],
            childrens: [
                Trie {
                    directions: [
                        Left,
                        Left,
                        Left,
                        Left,
                    ],
                    childrens: [],
                    terminate: [
                        "bob",
                    ],
                },
                Trie {
                    directions: [
                        Right,
                        Right,
                        Right,
                        Right,
                    ],
                    childrens: [],
                    terminate: [
                        "alice",
                    ],
                },
            ],
            terminate: [],
        }
        "###);

        // splitting + terminate
        trie.insert(&vec![Left, Left], String::from("jean"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: [],
            childrens: [
                Trie {
                    directions: [
                        Left,
                        Left,
                    ],
                    childrens: [
                        Trie {
                            directions: [
                                Left,
                                Left,
                            ],
                            childrens: [],
                            terminate: [
                                "bob",
                            ],
                        },
                    ],
                    terminate: [
                        "jean",
                    ],
                },
                Trie {
                    directions: [
                        Right,
                        Right,
                        Right,
                        Right,
                    ],
                    childrens: [],
                    terminate: [
                        "alice",
                    ],
                },
            ],
            terminate: [],
        }
        "###);

        // appending at the end
        trie.insert(&vec![Left, Left, Left, Left, Left], String::from("michel"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: [],
            childrens: [
                Trie {
                    directions: [
                        Left,
                        Left,
                    ],
                    childrens: [
                        Trie {
                            directions: [
                                Left,
                                Left,
                            ],
                            childrens: [
                                Trie {
                                    directions: [
                                        Left,
                                    ],
                                    childrens: [],
                                    terminate: [
                                        "michel",
                                    ],
                                },
                            ],
                            terminate: [
                                "bob",
                            ],
                        },
                    ],
                    terminate: [
                        "jean",
                    ],
                },
                Trie {
                    directions: [
                        Right,
                        Right,
                        Right,
                        Right,
                    ],
                    childrens: [],
                    terminate: [
                        "alice",
                    ],
                },
            ],
            terminate: [],
        }
        "###);

        // splitting + appending
        trie.insert(&vec![Right, Right, Left, Left], String::from("tamo"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: [],
            childrens: [
                Trie {
                    directions: [
                        Left,
                        Left,
                    ],
                    childrens: [
                        Trie {
                            directions: [
                                Left,
                                Left,
                            ],
                            childrens: [
                                Trie {
                                    directions: [
                                        Left,
                                    ],
                                    childrens: [],
                                    terminate: [
                                        "michel",
                                    ],
                                },
                            ],
                            terminate: [
                                "bob",
                            ],
                        },
                    ],
                    terminate: [
                        "jean",
                    ],
                },
                Trie {
                    directions: [
                        Right,
                        Right,
                    ],
                    childrens: [
                        Trie {
                            directions: [
                                Right,
                                Right,
                            ],
                            childrens: [],
                            terminate: [
                                "alice",
                            ],
                        },
                        Trie {
                            directions: [
                                Left,
                                Left,
                            ],
                            childrens: [],
                            terminate: [
                                "tamo",
                            ],
                        },
                    ],
                    terminate: [],
                },
            ],
            terminate: [],
        }
        "###);
    }
}
