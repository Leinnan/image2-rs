use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Histogram {
    total: usize,
    bins: Box<[usize]>,
}

impl std::ops::Index<usize> for Histogram {
    type Output = usize;

    fn index(&self, i: usize) -> &usize {
        &self.bins[i]
    }
}

impl std::ops::IndexMut<usize> for Histogram {
    fn index_mut(&mut self, i: usize) -> &mut usize {
        &mut self.bins[i]
    }
}

impl AsRef<[usize]> for Histogram {
    fn as_ref(&self) -> &[usize] {
        self.bins.as_ref()
    }
}

impl Histogram {
    pub fn new(nbins: usize) -> Histogram {
        Histogram {
            total: 0,
            bins: vec![0; nbins].into_boxed_slice(),
        }
    }

    pub fn join<'a>(h: impl AsRef<[Histogram]>) -> Histogram {
        let h = h.as_ref();
        let mut hist = Histogram::new(h[0].len());

        for i in h {
            hist.total += i.total;
            for (index, value) in i.bins() {
                hist[index] = hist[index] + value
            }
        }

        hist
    }

    pub fn add_value<T: Type>(&mut self, value: T) {
        let x = value.to_norm() * (self.bins.len() - 1) as f64;
        self.incr_bin(x.round() as usize)
    }

    pub fn incr_bin(&mut self, index: usize) {
        self.bins[index] += 1;
        self.total += 1;
    }

    pub fn bin(&self, index: usize) -> usize {
        self.bins[index]
    }

    pub fn bins<'a>(&'a self) -> impl 'a + Iterator<Item = (usize, usize)> {
        self.bins.iter().enumerate().map(|(a, b)| (a, *b))
    }

    pub fn len(&self) -> usize {
        self.bins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the bin index of the minimum value. There may be other bins with the same value, which
    /// would not be reported by this function
    pub fn min_index(&self) -> usize {
        let mut min = usize::MAX;
        let mut index = 0;
        for (i, n) in self.bins.iter().enumerate() {
            if *n < min {
                min = *n;
                index = i;
            }
        }

        index
    }

    /// Get the bin index of the maximum value. There may be other bins with the same value, which
    /// would not be reported by this function
    pub fn max_index(&self) -> usize {
        let mut max = 0;
        let mut index = 0;
        for (i, n) in self.bins.iter().enumerate() {
            if *n > max {
                max = *n;
                index = i;
            }
        }
        index
    }

    /// Count the number of bins with the given value
    pub fn count(&self, v: usize) -> usize {
        self.bins.iter().map(|bin| (*bin == v) as usize).sum()
    }

    pub fn distribution(&self) -> Vec<f64> {
        let total: f64 = self.bins().map(|(_, x)| x as f64).sum();
        self.bins().map(|(_, x)| x as f64 / total).collect()
    }

    pub fn sum(&self) -> usize {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_histogram_basic() {
        let image = Image::<f32, Rgb>::new(100, 100);
        let hist = image.histogram(255);

        for h in hist {
            assert!(h.bins[0] == 100 * 100);
            assert!(h.min_index() == 1);
            assert!(h.max_index() == 0);
            assert!(h.distribution()[0] == 1.0);
            assert!(h.distribution().into_iter().skip(1).sum::<f64>() == 0.0);
        }
    }
}
