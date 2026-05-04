
pub struct DSU{
    parent: Vec<usize>,
    pub size: Vec<usize>,
}

impl DSU{

    pub fn new(n: usize) -> Self {
        Self{
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find_parent(&self, num: usize) -> usize {
        let mut curr = num;
        while self.parent[curr] != curr {
            curr = self.parent[curr];
        }
        curr
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool{
        let parent_i = self.find_parent(i);
        let parent_j = self.find_parent(j);

        if parent_i != parent_j {
            self.parent[parent_j] = parent_i;
            self.size[parent_i] += self.size[parent_j];
            self.size[parent_j] = 0;
            return true;
        }
        false

    }


}