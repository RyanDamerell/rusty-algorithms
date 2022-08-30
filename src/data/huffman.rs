use std::{collections::HashMap, hash::Hash};
// A lot of the code here is spent making efficient binary strings.
// Essentially mimicking C++'s vector<bool> type
#[derive(Clone, PartialEq, Eq, Hash)]
struct BinaryString {
    bits: Vec<u8>,
    endpos: usize,
    index: usize,
}

impl Default for BinaryString {
    fn default() -> Self {
        Self::empty()
    }
}

impl BinaryString {
    fn empty() -> Self {
        BinaryString {
            bits: Vec::new(),
            endpos: 0,
            index: 0,
        }
    }

    fn new(b: bool) -> Self {
        BinaryString {
            bits: vec![0],
            endpos: 1,
            index: 0,
        }
    }
    // Modifies self to append another BinaryString to the end of this one.
    fn append(&mut self, other: &mut Self) {
        if self.endpos & 0b111 == 0 {
            //if endpos is divisible by 8, we can just append the bytes and add the length
            self.bits.append(&mut other.bits);
            self.endpos += other.endpos;
        } else {
            //if we're not so lucky, we have to add them each to fix the alignment
            for b in other {
                self.push(b);
            }
        }
    }
    fn push(&mut self, bit: bool) {
        self.endpos += 1;
        if self.endpos & 0b111 == 0 {
            self.bits.push(if bit { 1 } else { 0 });
        } else {
            *self.bits.last_mut().unwrap() |= if bit { 1 << (self.endpos & 7) } else { 0 };
        }
    }
    // Create a new BinaryString from cloning ourself and appending one bit.
    fn with(&self, rhs: bool) -> Self {
        let mut out = self.clone();
        out.push(rhs);
        out
    }
}

impl Iterator for BinaryString {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.endpos {
            None
        } else {
            //first: get value at bits[index/8]
            //second: get the (index%8)th bit
            //final: convert that to a boolean
            let out = (self.bits[(self.index >> 3) as usize]) & (1 << (self.index & 0b111)) != 0;
            self.index += 1;
            Some(out)
        }
    }
}

enum HuffTree<T: Eq + Hash + Copy> {
    Node(usize, Box<Self>, Box<Self>),
    Leaf(usize, T),
}

type HuffEncodingTable<T: Eq + Hash + Copy> = HashMap<T, BinaryString>;

impl<T: Eq + Hash + Copy> HuffTree<T> {
    fn gen_trees(list: Vec<T>) -> Vec<Box<Self>> {
        let mut map: HashMap<T, usize> = HashMap::new();
        for elem in list {
            match map.get_mut(&elem) {
                Some(i) => {
                    *i += 1;
                }
                None => {
                    map.insert(elem, 1);
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
        self.encoding_recurse(BinaryString::empty(), &mut out);
        out
    }

    fn encoding_recurse(&self, s: BinaryString, mut table: &mut HuffEncodingTable<T>) {
        match self {
            HuffTree::Node(_, left, right) => {
                left.encoding_recurse(s.with(false), &mut table);
                right.encoding_recurse(s.with(true), &mut table);
            }
            HuffTree::Leaf(_, elem) => {
                table.insert(*elem, s);
            }
        }
    }
}

fn HuffmanEncodeFull<T: Eq + Hash + Copy>(data: Vec<T>) -> (HuffTree<T>, BinaryString) {
    let root = *HuffTree::merge_all(&mut HuffTree::gen_trees(data));
    let table = root.create_encoding_table();
    let mut bin_str = BinaryString::empty();
    for elem in data.iter() {
        let temp = table.get(&elem).unwrap();
        bin_str.append(&mut temp);
    }
    (root, bin_str)
}
