use std::collections::HashSet;
use std::ops::Deref;

pub struct Import<'a>(pub &'a [&'a str]);

pub enum Order {
    Original,
    Sorted,
}

pub fn remove_duplicates(mut s: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    s.retain(move |c| seen.insert(c.clone()));
    s
}

pub fn format_flat(imports: &[Import], order: Order) -> Vec<String> {
    let mut result = imports
        .iter()
        .map(|x| {
            x.0.iter()
                .skip(1)
                .fold(String::from(format!("{}", x.0[0])), |acc, x| {
                    format!("{acc}::{x}")
                })
        })
        .collect::<Vec<_>>();

    result = remove_duplicates(result);

    match order {
        Order::Original => result,
        Order::Sorted => {
            result.sort();
            result
        }
    }
}

struct Tree {
    root: Node,
}

impl Tree {
    fn new(root_name: String) -> Self {
        Tree {
            root: Node::new(root_name),
        }
    }

    fn load_tree(&mut self, imports: &[Import]) {
        for import in imports.into_iter() {
            let mut tree_ref = &mut self.root;

            for import_level in import.0.iter().cloned() {
                let matching_child = { tree_ref.children.iter().any(|b| b.value == *import_level) };

                tree_ref = match matching_child {
                    true => tree_ref
                        .children
                        .iter_mut()
                        .find(|b| b.value == *import_level)
                        .unwrap(),
                    false => {
                        let node = Box::new(Node::new(String::from(import_level)));
                        tree_ref.children.push(node);
                        tree_ref.children.last_mut().unwrap()
                    }
                };
            }

            tree_ref
                .children
                .push(Box::new(Node::new(String::from("self"))));
        }
    }
}

struct Node {
    value: String,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(value: String) -> Self {
        Node {
            value,
            children: vec![],
        }
    }

    fn format_nested(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push_str(&" ".repeat(indent));
        result.push_str(&self.value);

        if !self.children.iter().all(|x| x.value == "self") {
            result.push_str("::{\n");

            for self_child in self.children.iter().filter(|x| x.value == "self")  {
                result.push_str(&self_child.format_nested(indent + 4));
            }

            for child in self.children.iter().filter(|x| x.value != "self") {
                result.push_str(&child.format_nested(indent + 4));
            }

            result.push_str(&" ".repeat(indent));
            result.push_str("}");
        }

        if indent == 0 {
            result.push('\n');
        } else {
            result.push_str(",\n");
        }

        result
    }
}

fn sort_children(node: &mut Node) {
    node.children.sort_by(|a, b| a.value.cmp(&b.value));
    for child in &mut node.children {
        sort_children(child);
    }
}

pub fn format_nested(imports: &[Import], order: Order) -> Vec<String> {
    let mut tree = Tree::new(String::from("use")); // just to have common starting point
    tree.load_tree(imports);

    if let Order::Sorted = order {
        sort_children(&mut tree.root);
    };

    let mut result = vec![];

    for node in tree.root.children.iter().map(|x| x.deref()) {
        result.push(node.format_nested(0))
    }

    result
}

fn main() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "a", "A2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a", "A1"]),
    ];

    let res = format_nested(imports, Order::Original);
    println!("{:?}", res);
}

#[test]
fn test_sorted_format_flat_basic() {
    let imports = &[
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "a"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Sorted),
        &[
            "my_crate::a",
            "my_crate::b::B1",
            "my_crate::b::B2",
            "my_crate::c",
        ]
    );
}

#[test]
fn test_sorted_format_flat_duplicate() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b", "B2"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Sorted),
        &[
            "my_crate::a",
            "my_crate::b::B1",
            "my_crate::b::B2",
            "my_crate::c",
        ]
    );
}

#[test]
fn test_original_format_flat_basic() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Original),
        &[
            "my_crate::a",
            "my_crate::b::B2",
            "my_crate::b::B1",
            "my_crate::c",
        ]
    );
}

#[test]
fn test_original_format_flat_duplicate() {
    let imports = &[
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "a"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Original),
        &["my_crate::b::B2", "my_crate::c", "my_crate::a",]
    );
}

#[test]
fn test_sorted_format_nested_basic() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        &[concat!(
            "my_crate::{\n",
            "    a,\n",
            "    b::{\n",
            "        B1,\n",
            "        B2,\n",
            "    },\n",
            "    c,\n",
            "}\n",
        )]
    );
}

#[test]
fn test_sorted_format_nested_duplicate() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        &[concat!(
        "my_crate::{\n",
        "    a,\n",
        "    b::{\n",
        "        B1,\n",
        "        B2,\n",
        "    },\n",
        "    c,\n",
        "}\n",
        )]
    );
}

#[test]
fn test_original_format_nested_basic() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        &[concat!(
        "my_crate::{\n",
        "    a,\n",
        "    b::{\n",
        "        B2,\n",
        "        B1,\n",
        "    },\n",
        "    c,\n",
        "}\n",
        )]
    );
}

#[test]
fn test_original_format_nested_duplicate() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        &[concat!(
        "my_crate::{\n",
        "    a,\n",
        "    b::{\n",
        "        B1,\n",
        "        B2,\n",
        "    },\n",
        "    c,\n",
        "}\n",
        )]
    );
}

#[test]
fn test_original_format_nested_self_first() {
    let imports = &[
        Import(&["my_crate", "b"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b", "B2"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        &[concat!(
        "my_crate::{\n",
        "    b::{\n",
        "        self,\n",
        "        B1,\n",
        "        B2,\n",
        "    },\n",
        "}\n",
        )]
    );
}

#[test]
fn test_original_format_nested_self_middle() {
    let imports = &[
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b"]),
        Import(&["my_crate", "b", "B1"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        &[concat!(
        "my_crate::{\n",
        "    b::{\n",
        "        self,\n",
        "        B2,\n",
        "        B1,\n",
        "    },\n",
        "}\n",
        )]
    );
}

#[test]
fn test_sorted_format_nested_self() {
    let imports = &[
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        &[concat!(
        "my_crate::{\n",
        "    b::{\n",
        "        self,\n",
        "        B1,\n",
        "        B2,\n",
        "    },\n",
        "}\n",
        )]
    );
}