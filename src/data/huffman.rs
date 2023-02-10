use bit_vec::BitVec;
use std::{collections::HashMap, hash::Hash};

enum HuffTree<T: Eq + Hash + Copy> {
    Node(usize, Box<Self>, Box<Self>),
    Leaf(usize, T),
}

type HuffEncodingTable<T: Eq + Hash + Copy> = HashMap<T, BitVec>;

impl<T: Eq + Hash + Copy> HuffTree<T> {
    fn gen_trees(list: &Vec<T>) -> Vec<Box<Self>> {
        let mut map: HashMap<T, usize> = HashMap::new();
        for elem in list {
            match map.get_mut(&elem) {
                Some(i) => {
                    *i += 1;
                }
                None => {
                    map.insert(*elem, 1);
                }
            }
        }
        map.iter()
            .map(|(k, v)| Box::new(Self::Leaf(*v, *k)))
            .collect()
    }

    fn merge_all(list: &mut Vec<Box<Self>>) -> Box<Self> {
        while list.len() > 1 {
            list.sort_unstable_by_key(|x| x.count());
            let t1 = list.pop().unwrap();
            let t2 = list.pop().unwrap();
            list.push(Self::merge(t1, t2));
        }
        list.swap_remove(0)
    }

    fn merge(first: Box<Self>, second: Box<Self>) -> Box<Self> {
        Box::new(Self::Node(first.count() + second.count(), first, second))
    }

    fn count(&self) -> usize {
        match self {
            Self::Node(i, _, _) => *i,
            Self::Leaf(i, _) => *i,
        }
    }

    // A wrapper for encoding_recurse where you don't have to allocate the hashmap yourself.
    // Just a stylistic choice.
    fn create_encoding_table(&self) -> HuffEncodingTable<T> {
        let mut out = HashMap::new();
        self.encoding_recurse(BitVec::new(), &mut out);
        out
    }

    fn encoding_recurse(&self, s: BitVec, mut table: &mut HuffEncodingTable<T>) {
        match self {
            HuffTree::Node(_, left, right) => {
                let mut tmp = s.clone();
                tmp.push(false);
                left.encoding_recurse(tmp, &mut table);
                let mut tmp = s.clone();
                tmp.push(true);
                right.encoding_recurse(tmp, &mut table);
            }
            HuffTree::Leaf(_, elem) => {
                table.insert(*elem, s);
            }
        }
    }
}

fn huffman_encode_full<T: Eq + Hash + Copy>(data: Vec<T>) -> (HuffTree<T>, BitVec) {
    let root = *HuffTree::merge_all(&mut HuffTree::gen_trees(&data));
    let table = root.create_encoding_table();
    let mut bin_str = BitVec::new();
    for elem in data {
        let mut temp = table.get(&elem).unwrap().clone();
        bin_str.append(&mut temp);
    }
    (root, bin_str)
}


