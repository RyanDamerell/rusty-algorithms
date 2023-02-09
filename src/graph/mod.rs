//TODO: astar, djikstra,

pub trait Graph {
    type Node: Eq;
    fn get_connections(&mut self, n: &Self::Node) -> Vec<Box<Self::Node>>;
}

mod example_graphs {
    pub struct AdjMatrix {
        size: (u32, u32),
        matrix: Box<[bool]>,
    }

    impl AdjMatrix {
        fn get(&self, r: u32, c: u32) -> bool {
            self.matrix[((r % self.size.0) + c) as usize]
        }
    }

    impl super::Graph for AdjMatrix {
        type Node = (u32, u32);
        fn get_connections(&mut self, n: &Self::Node) -> Vec<Box<Self::Node>> {
            let mut out = Vec::new();
            for i in 0..self.size.0 {
                if self.get(i, n.1) {
                    out.push(Box::new((i, n.1)));
                }
            }
            for i in 0..self.size.1 {
                if self.get(n.0, i) {
                    out.push(Box::new((n.0, i)));
                }
            }
            out
        }
    }

    pub struct AdjList {
        mainlist: Vec<Vec<Box<usize>>>,
    }

    impl super::Graph for AdjList {
        type Node = usize;

        fn get_connections(&mut self, n: &Self::Node) -> Vec<Box<Self::Node>> {
            self.mainlist[*n].clone()
        }
    }
}
