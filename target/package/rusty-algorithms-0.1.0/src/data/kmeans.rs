use rand::prelude::*;
use rand_pcg::{Lcg128Xsl64, Pcg64};
use std::ops::Range;

//customizable values
//I would use generics but trait aliasing is still only available in nightly rust
type Scalar = f32; //could just as easily be f64, just change it here
const THRESHHOLD: Scalar = Scalar::EPSILON; //level of precision, can be set to any value but best left as epsilon

//begin code proper
type Point = (Scalar, Scalar);

pub struct Cluster {
    attatched: Vec<Box<Point>>,
    mean: Point,
}

impl Cluster {
    //recalculate the mean based on all associated points
    fn recalc(&mut self) -> bool {
        let old = self.mean;
        let mut new = (Scalar::default(), Scalar::default());
        for boxed in &self.attatched {
            let (a, b) = **boxed;
            new.0 += a;
            new.1 += b;
        }
        new.0 /= self.attatched.len() as Scalar;
        new.1 /= self.attatched.len() as Scalar;
        self.mean = new;
        distance(old, new) < THRESHHOLD
    }

    fn new(r: &Range<Scalar>, rng: &mut Lcg128Xsl64) -> Self {
        Self {
            attatched: Vec::new(),
            mean: (rng.gen_range(r.clone()), rng.gen_range(r.clone())),
        }
    }
    fn clear(&mut self) {
        self.attatched = Vec::new();
    }
}

fn distance(a: Point, b: Point) -> Scalar {
    let (dx, dy) = (a.0 - b.0, a.1 - b.1);
    Scalar::sqrt(dx * dx + dy * dy)
}

//attatch a point to the closest cluster
fn attatch(clusters: &mut Vec<Cluster>, p: Box<Point>) {
    let mut min = (0, distance(clusters[0].mean, *p));
    for i in 1..clusters.len() {
        let d = distance(clusters[i].mean, *p);
        if d < min.1 {
            min = (i, d);
        }
    }
    clusters[min.0].attatched.push(p);
}

//Kmeans is a data clustering algorithm. It will take in a series of points, and categorize them
// in clumps, or clusters. It relies on k being an accurate estimate of the number of clusters.
pub fn kmeans(points: &[Point], num_clusters: usize, seed: u64, range: Range<Scalar>) {
    let mut clusters: Vec<Cluster> = Vec::with_capacity(num_clusters);
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut masterlist = Vec::with_capacity(points.len());
    let mut flag = true;
    points.iter().for_each(|x| masterlist.push(Box::new(*x)));
    clusters
        .iter_mut()
        .for_each(|x| *x = Cluster::new(&range, &mut rng));
    while flag {
        flag = false;
        clusters.iter_mut().for_each(|x| x.clear());
        masterlist
            .iter()
            .for_each(|p| attatch(&mut clusters, p.clone()));
        clusters
            .iter_mut()
            .for_each(move |e| flag = flag || e.recalc());
    }
}
