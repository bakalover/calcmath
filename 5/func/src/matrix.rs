pub struct MT<T> {
    data: Vec<Vec<T>>,
}

impl MT<f32> {
    pub fn new(row: usize, col: usize) -> Self {
        let m = MT {
            data: vec![vec![0.0; col]; row],
        };
        m
    }
    pub fn get_data(&self) -> &Vec<Vec<f32>> {
        &self.data
    }
}

impl std::ops::Index<usize> for MT<f32> {
    type Output = [f32];
    fn index(&self, index: usize) -> &Self::Output {
        assert!(self.data.len() > index, "bad index");
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for MT<f32> {
    fn index_mut(&mut self, index: usize) -> &mut [f32] {
        assert!(self.data.len() > index, "bad index");
        &mut self.data[index]
    }
}
