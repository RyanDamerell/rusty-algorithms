use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    iter,
    ops::Add,
};

//TODO: undocumented and not fully tested/implemented

#[derive(Clone, PartialEq, Eq, Hash)]
struct BinaryString {
    bits: Vec<u8>,
    endpos: u64,
    index: u64,
}

impl Default for BinaryString {
    fn default() -> Self {
        BinaryString {
            bits: Vec::new(),
            endpos: 0,
            index: 0,
        }
    }
}

impl BinaryString {
    fn new(b: bool) -> Self {
        BinaryString {
            bits: vec![0],
            endpos: 1,
            index: 0,
        }
    }
    fn append(&mut self, other: &mut Self) {
        if self.endpos & 0b111 == 0 {
            //if endpos is divisible by 8
            self.bits.append(&mut other.bits);
        } else {
            for b in other {
                self.push(b);
            }
        }
    }
    fn push(&mut self, bit: bool) {
        if self.endpos & 0b111 == 0 {
            self.endpos += 1;
            self.bits.push(if bit { 0x80 } else { 0x00 });
        } else {
            *self.bits.last_mut().unwrap() |= if bit { 1 << (self.endpos & 7) } else { 0 };
        }
    }
    fn with(&self, rhs: bool) -> Self {
        let mut out = self.clone();
        out.push(rhs);
        out
    }
}

impl Iterator for BinaryString {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.endpos {
            None
        } else {
            //first: get value at bits[index/8]
            //second: get the (index%8)th bit
            //return that bit as a boolean
            let out = self.bits[(self.index >> 3) as usize] & (1 << (self.index & 0b111)) != 0;
            self.index += 1;
            Some(out)
        }
    }
}

enum HuffTree<T: Eq + Hash + Copy> {
    Node(usize, Box<Self>, Box<Self>),
    Leaf(usize, T),
}

type HuffEncodingTable<T: Eq + Hash + Copy> = HashMap<BinaryString, T>;

impl<T: Eq + Hash + Copy> HuffTree<T> {
    fn gen_trees(list: Vec<T>) -> Vec<Box<Self>> {
        let mut map: HashMap<T, usize> = HashMap::new();
        for e in list {
            match map.get_mut(&e) {
                Some(i) => {
                    *i += 1;
                }
                None => {
                    map.insert(e, 1);
                }
            }
        }
        map.iter()
            .map(|(k, v)| Box::new(Self::Leaf(*v, *k)))
            .collect()
    }

    fn merge_all(list: &mut Vec<Box<Self>>) -> Box<Self> {
        while list.len() > 1 {
            list.sort_by_key(|x| x.count());
            let t1 = list.remove(1);
            let t2 = list.remove(0);
            list.push(Self::merge(t1, t2));
        }
        list.remove(0)
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

    fn create_encoding_table(&self) -> HuffEncodingTable<T> {
        let mut out = HashMap::new();
        match self {
            HuffTree::Node(_, left, right) => {
                left.encoding_recurse(BinaryString::new(false), &mut out);
                right.encoding_recurse(BinaryString::new(true), &mut out);
            }
            HuffTree::Leaf(_, elem) => {
                out.insert(BinaryString::new(false), *elem);
            }
        }
        out
    }

    fn encoding_recurse(&self, s: BinaryString, mut table: &mut HuffEncodingTable<T>) {
        match self {
            HuffTree::Node(_, left, right) => {
                left.encoding_recurse(s.with(false), &mut table);
                right.encoding_recurse(s.with(true), &mut table);
            }
            HuffTree::Leaf(_, elem) => {
                table.insert(s, *elem);
            }
        }
    }
}
