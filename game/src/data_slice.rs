pub struct DataSlice {
    pub slice_vector: Vec<f64>,
    pub name: String,
}

impl DataSlice {

    /// Perform a uniform crossover of two DataSlices.
    ///
    /// # Arguments
    /// * `slice` - The DataSlice to crossover with.
    ///
    /// # Remarks
    /// The resultant DataSlice is new, and therefore isn't in the memory location of either of
    /// the two that constructed it. This allows the reuse of the DataSlices that construct this
    /// crossover.
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

    /// Create a new DataSlice struct that is functionally identical to the current one, but
    /// doesn't share memory location.
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

    /// Returns the length of the DataSlice
    pub fn len(&self) -> usize {
        self.slice_vector.len()
    }

    /// Gets a specified indexed element of the DataSlice.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> f64 {
        self.slice_vector[index]
    }

    /// Returns a boolean representing whether the argument DataSlice is piecewise strictly
    /// larger than the current DataSlice.
    ///
    /// # Arguments
    /// * `slice` - The DataSlice to compare against.
    pub fn greater(&self, slice: &DataSlice) -> bool {
        for i in 0..self.len() {
            if slice.get(i) > self.get(i) {
                return false
            }
        }
        true
    }

}
