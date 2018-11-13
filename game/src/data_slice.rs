pub struct DataSlice {
    pub slice_vector: Vec<f64>,
    pub name: String,
}

impl DataSlice {

    pub fn dumb_crossover(&self, slice: DataSlice) -> DataSlice {
        let mut output = Vec::new();
        for i in 0..slice.len() {
            output.push((self.get(i) + slice.get(i)) / 2.0);
        }
        DataSlice {
            slice_vector: output,
            name: format!("Child of {} and {}.", self.name, slice.name),
        }
    }

    pub fn copy(&self) -> DataSlice {
        let mut output = Vec::new();
        for i in 0..self.len() {
            output.push(self.get(i));
        }
        DataSlice {
            slice_vector: output,
            name: format!("{}", self.name),
        }
    }

    pub fn len(&self) -> usize {
        self.slice_vector.len()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.slice_vector[index]
    }

    pub fn greater(&self, slice: &DataSlice) -> bool {
        for i in 0..self.len() {
            if slice.get(i) > self.get(i) {
                return false
            }
        }
        true
    }

}
