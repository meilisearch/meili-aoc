use std::fmt::Debug;
use std::{cmp::Ordering, collections::HashSet, str::FromStr};

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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Trie {
    pub directions: Vec<Dir>,
    pub childrens: Vec<Trie>,
    pub parent: Option<&'static Trie>,

    pub whole_path: Vec<Dir>,
    pub terminate: Vec<String>,
}

impl Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trie")
            .field(
                "directions",
                &self
                    .directions
                    .iter()
                    .map(|d| match d {
                        Dir::Left => 'L',
                        Dir::Right => 'R',
                    })
                    .collect::<String>(),
            )
            .field(
                "whole_path",
                &self
                    .whole_path
                    .iter()
                    .map(|d| match d {
                        Dir::Left => 'L',
                        Dir::Right => 'R',
                    })
                    .collect::<String>(),
            )
            .field("childrens", &self.childrens)
            .field("terminate", &self.terminate)
            // we must ignore the parents or else we're going to run in an inifinite loop
            .finish_non_exhaustive()
    }
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
            panic!("shouldn't be called");
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

                if self.childrens.len() == 2 {
                    panic!("error while inserting {}", name);
                }

                // if we reach this point it means there was not children anywhere
                // thus we're going to create a new node just for this string.
                self.childrens.push(Trie {
                    directions: directions_to_insert.to_vec(),
                    childrens: Vec::new(),
                    parent: None,
                    whole_path: Vec::new(),
                    terminate: vec![name],
                });
            }
        } else {
            // we'll need to split ourselves into two nodes
            let prefix = &self.directions[0..common_prefix];
            let suffix = &self.directions[common_prefix..];

            // good noice
            let suffix = Trie {
                directions: suffix.to_vec(),
                childrens: self.childrens.clone(),
                parent: None,
                whole_path: Vec::new(),
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
                    parent: None,
                    whole_path: Vec::new(),
                    terminate: vec![name],
                };
                self.directions = prefix.to_vec();
                self.childrens = vec![suffix, new];
                self.terminate = Vec::new();
            }
        }
    }

    /// Sort everything + set the parent parts of the trie.
    pub fn finish(&mut self) {
        self._finish(None, &[])
    }

    fn _finish(&mut self, r: Option<&'static Self>, whole_path: &[Dir]) {
        // We MUST sort everything BEFORE setting the parent
        self.childrens.sort();
        self.terminate.sort();
        // probably useless.
        self.whole_path.clear();
        self.whole_path
            .extend(whole_path.iter().chain(&self.directions).copied());
        let this: &'static Self = unsafe { (self as *const Self).as_ref().unwrap() };

        for c in &mut self.childrens {
            c._finish(Some(this), &self.whole_path);
        }

        self.parent = r;
    }

    pub fn first(&self) -> &str {
        self.terminate
            .first()
            .map(|s| s.as_ref())
            .unwrap_or_else(|| self.childrens[0].first())
    }

    /// ignored nodes were already explored in another iteration and can be traversed but should
    /// not be used as a terminal node in this iteration.
    pub fn fastest_access(&self, ignored: &HashSet<*const Trie>) -> Option<(&Trie, usize)> {
        // explored_nodes are the nodes we've already explored in THIS iteration.
        let mut explored_nodes = HashSet::<*const Trie>::new();
        explored_nodes.insert(self as *const Trie);
        // a pair of the node to explore + their distance from the beginning.
        let mut to_explore: Vec<(&Trie, usize)> = self
            .childrens
            .iter()
            .chain(self.parent)
            .map(|node| (node, 1))
            .collect();

        // the best node we've found so far
        let mut best: Option<(&Trie, usize)> = None;

        loop {
            to_explore.sort_unstable_by(|(_, l), (_, r)| l.cmp(r));

            let (current_node, dist) = match to_explore.pop() {
                Some(to_explore) => to_explore,
                // if there is no node left it means we've explored everything
                None if best.is_none() => return None,
                None => return best,
            };
            let current_node_addr = current_node as *const Trie;

            // if this is a terminal node AND it hasn't been encountered in a previous step.
            if !current_node.terminate.is_empty() && !ignored.contains(&current_node_addr) {
                if best.is_none() {
                    best = Some((current_node, dist));
                    // we found a better node, we can delete everything worse than that
                    to_explore.retain(|(_, d)| *d <= dist);
                } else {
                    let (b, d) = best.as_ref().unwrap();
                    if *d > dist {
                        best = Some((current_node, dist));
                        // we found a better node, we can delete everything worse than that
                        to_explore.retain(|(_, d)| *d <= dist);
                    } else if *d == dist {
                        if b.whole_path > current_node.whole_path {
                            best = Some((current_node, dist));
                        }
                    }
                    // else we can ignore this entry
                }
            }
            explored_nodes.insert(current_node_addr);
            to_explore.extend(
                current_node
                    .childrens
                    .iter()
                    .chain(current_node.parent)
                    .filter(|node| !explored_nodes.contains(&(*node as *const Trie)))
                    .map(|node| (node, dist + 1)),
            );
        }
    }

    pub fn nb_nodes(&self) -> usize {
        1 + self.childrens.iter().map(Self::nb_nodes).sum::<usize>()
    }

    pub fn depth(&self) -> usize {
        1 + self
            .childrens
            .iter()
            .map(Self::depth)
            .max()
            .unwrap_or_default()
    }

    pub fn to_graph(&self) {
        println!("digraph lol {{");
        self._to_graph(&mut 0, 0);
        println!("}}");
    }

    fn _to_graph(&self, node_id: &mut usize, last_node: usize) {
        *node_id += 1;
        let myself = *node_id;
        println!(
            "\t\"{}\" [label = \"{}\"]",
            myself,
            self.directions
                .iter()
                .map(|d| match d {
                    Dir::Left => 'L',
                    Dir::Right => 'R',
                })
                .collect::<String>()
        );
        println!("\t\"{}\" -> \"{}\"", last_node, myself);
        for child in &self.terminate {
            println!("\t\"{}\" -> \"{}\"", myself, child);
        }
        for child in &self.childrens {
            child._to_graph(node_id, myself);
        }
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
    fn test_basic_trie() {
        use Dir::*;

        let mut trie = Trie::default();

        // simple insert
        trie.insert(&vec![Left, Left, Left, Left], String::from("bob"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: "",
            whole_path: "",
            childrens: [
                Trie {
                    directions: "LLLL",
                    whole_path: "",
                    childrens: [],
                    terminate: [
                        "bob",
                    ],
                    ..
                },
            ],
            terminate: [],
            ..
        }
        "###);

        // insert with no relation
        trie.insert(&vec![Right, Right, Right, Right], String::from("alice"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: "",
            whole_path: "",
            childrens: [
                Trie {
                    directions: "LLLL",
                    whole_path: "",
                    childrens: [],
                    terminate: [
                        "bob",
                    ],
                    ..
                },
                Trie {
                    directions: "RRRR",
                    whole_path: "",
                    childrens: [],
                    terminate: [
                        "alice",
                    ],
                    ..
                },
            ],
            terminate: [],
            ..
        }
        "###);

        // splitting + terminate
        trie.insert(&vec![Left, Left], String::from("jean"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: "",
            whole_path: "",
            childrens: [
                Trie {
                    directions: "LL",
                    whole_path: "",
                    childrens: [
                        Trie {
                            directions: "LL",
                            whole_path: "",
                            childrens: [],
                            terminate: [
                                "bob",
                            ],
                            ..
                        },
                    ],
                    terminate: [
                        "jean",
                    ],
                    ..
                },
                Trie {
                    directions: "RRRR",
                    whole_path: "",
                    childrens: [],
                    terminate: [
                        "alice",
                    ],
                    ..
                },
            ],
            terminate: [],
            ..
        }
        "###);

        // appending at the end
        trie.insert(&vec![Left, Left, Left, Left, Left], String::from("michel"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: "",
            whole_path: "",
            childrens: [
                Trie {
                    directions: "LL",
                    whole_path: "",
                    childrens: [
                        Trie {
                            directions: "LL",
                            whole_path: "",
                            childrens: [
                                Trie {
                                    directions: "L",
                                    whole_path: "",
                                    childrens: [],
                                    terminate: [
                                        "michel",
                                    ],
                                    ..
                                },
                            ],
                            terminate: [
                                "bob",
                            ],
                            ..
                        },
                    ],
                    terminate: [
                        "jean",
                    ],
                    ..
                },
                Trie {
                    directions: "RRRR",
                    whole_path: "",
                    childrens: [],
                    terminate: [
                        "alice",
                    ],
                    ..
                },
            ],
            terminate: [],
            ..
        }
        "###);

        // splitting + appending
        trie.insert(&vec![Right, Right, Left, Left], String::from("tamo"));
        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: "",
            whole_path: "",
            childrens: [
                Trie {
                    directions: "LL",
                    whole_path: "",
                    childrens: [
                        Trie {
                            directions: "LL",
                            whole_path: "",
                            childrens: [
                                Trie {
                                    directions: "L",
                                    whole_path: "",
                                    childrens: [],
                                    terminate: [
                                        "michel",
                                    ],
                                    ..
                                },
                            ],
                            terminate: [
                                "bob",
                            ],
                            ..
                        },
                    ],
                    terminate: [
                        "jean",
                    ],
                    ..
                },
                Trie {
                    directions: "RR",
                    whole_path: "",
                    childrens: [
                        Trie {
                            directions: "RR",
                            whole_path: "",
                            childrens: [],
                            terminate: [
                                "alice",
                            ],
                            ..
                        },
                        Trie {
                            directions: "LL",
                            whole_path: "",
                            childrens: [],
                            terminate: [
                                "tamo",
                            ],
                            ..
                        },
                    ],
                    terminate: [],
                    ..
                },
            ],
            terminate: [],
            ..
        }
        "###);
    }

    #[test]
    fn test_trie() {
        use Dir::*;

        let mut trie = Trie::default();

        trie.insert(&vec![Left, Left, Left, Left], String::from("a"));
        trie.insert(&vec![Left, Left, Right], String::from("b"));
        trie.insert(&vec![Left, Left, Left], String::from("c"));
        trie.insert(&vec![Left, Right, Left], String::from("d"));
        trie.insert(&vec![Left, Right, Right], String::from("e"));
        trie.insert(&vec![Right, Right, Right], String::from("f"));

        trie.finish();

        insta::assert_debug_snapshot!(trie, @r###"
        Trie {
            directions: "",
            whole_path: "",
            childrens: [
                Trie {
                    directions: "L",
                    whole_path: "L",
                    childrens: [
                        Trie {
                            directions: "L",
                            whole_path: "LL",
                            childrens: [
                                Trie {
                                    directions: "L",
                                    whole_path: "LLL",
                                    childrens: [
                                        Trie {
                                            directions: "L",
                                            whole_path: "LLLL",
                                            childrens: [],
                                            terminate: [
                                                "a",
                                            ],
                                            ..
                                        },
                                    ],
                                    terminate: [
                                        "c",
                                    ],
                                    ..
                                },
                                Trie {
                                    directions: "R",
                                    whole_path: "LLR",
                                    childrens: [],
                                    terminate: [
                                        "b",
                                    ],
                                    ..
                                },
                            ],
                            terminate: [],
                            ..
                        },
                        Trie {
                            directions: "R",
                            whole_path: "LR",
                            childrens: [
                                Trie {
                                    directions: "L",
                                    whole_path: "LRL",
                                    childrens: [],
                                    terminate: [
                                        "d",
                                    ],
                                    ..
                                },
                                Trie {
                                    directions: "R",
                                    whole_path: "LRR",
                                    childrens: [],
                                    terminate: [
                                        "e",
                                    ],
                                    ..
                                },
                            ],
                            terminate: [],
                            ..
                        },
                    ],
                    terminate: [],
                    ..
                },
                Trie {
                    directions: "RRR",
                    whole_path: "RRR",
                    childrens: [],
                    terminate: [
                        "f",
                    ],
                    ..
                },
            ],
            terminate: [],
            ..
        }
        "###);
    }
}
