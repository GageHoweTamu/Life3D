use std::collections::HashSet;

pub struct OctreeNode {
    center: [f32; 3],
    half_size: f32,
    organisms: HashSet<usize>,
    children: Option<Box<[OctreeNode; 8]>>,
}

impl OctreeNode {
    pub fn new(center: [f32; 3], half_size: f32) -> Self {
        OctreeNode {
            center,
            half_size,
            organisms: HashSet::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, organism_id: usize, position: [f32; 3], min_size: f32) {
        if self.half_size <= min_size {
            self.organisms.insert(organism_id);
            return;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        let index = self.get_octant(position);
        if let Some(children) = &mut self.children {
            children[index].insert(organism_id, position, min_size);
        }
    }

    pub fn subdivide(&mut self) {
        let new_half_size = self.half_size / 2.0;
        self.children = Some(Box::new([
            OctreeNode::new([self.center[0] - new_half_size, self.center[1] - new_half_size, self.center[2] - new_half_size], new_half_size),
            OctreeNode::new([self.center[0] + new_half_size, self.center[1] - new_half_size, self.center[2] - new_half_size], new_half_size),
            OctreeNode::new([self.center[0] - new_half_size, self.center[1] + new_half_size, self.center[2] - new_half_size], new_half_size),
            OctreeNode::new([self.center[0] + new_half_size, self.center[1] + new_half_size, self.center[2] - new_half_size], new_half_size),
            OctreeNode::new([self.center[0] - new_half_size, self.center[1] - new_half_size, self.center[2] + new_half_size], new_half_size),
            OctreeNode::new([self.center[0] + new_half_size, self.center[1] - new_half_size, self.center[2] + new_half_size], new_half_size),
            OctreeNode::new([self.center[0] - new_half_size, self.center[1] + new_half_size, self.center[2] + new_half_size], new_half_size),
            OctreeNode::new([self.center[0] + new_half_size, self.center[1] + new_half_size, self.center[2] + new_half_size], new_half_size),
        ]));
    }

    pub fn get_octant(&self, position: [f32; 3]) -> usize {
        let mut index = 0;
        if position[0] > self.center[0] { index |= 1; }
        if position[1] > self.center[1] { index |= 2; }
        if position[2] > self.center[2] { index |= 4; }
        index
    }

    pub fn query(&self, query_box: &[f32; 6]) -> Vec<usize> {
        let mut result = Vec::new();
        if !self.intersects(query_box) {
            return result;
        }

        result.extend(&self.organisms);

        if let Some(children) = &self.children {
            for child in children.iter() {
                result.extend(child.query(query_box));
            }
        }

        result
    }

    pub fn intersects(&self, query_box: &[f32; 6]) -> bool {
        !(query_box[0] > self.center[0] + self.half_size ||
          query_box[3] < self.center[0] - self.half_size ||
          query_box[1] > self.center[1] + self.half_size ||
          query_box[4] < self.center[1] - self.half_size ||
          query_box[2] > self.center[2] + self.half_size ||
          query_box[5] < self.center[2] - self.half_size)
    }
}

pub struct Octree {
    root: OctreeNode,
    min_size: f32,
}

impl Octree {
    pub fn new(center: [f32; 3], size: f32, min_size: f32) -> Self {
        Octree {
            root: OctreeNode::new(center, size / 2.0),
            min_size,
        }
    }

    pub fn insert(&mut self, organism_id: usize, position: [f32; 3]) {
        self.root.insert(organism_id, position, self.min_size);
    }

    pub fn query(&self, query_box: &[f32; 6]) -> Vec<usize> {
        self.root.query(query_box)
    }
}