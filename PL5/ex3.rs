mod complex_number;
use complex_number::ComplexNumber;

use rayon::prelude::*; // <-- add this

pub struct ComplexVector {
    elements: Vec<ComplexNumber>,
}

impl ComplexVector {
    pub fn new() -> ComplexVector {
        ComplexVector {
            elements: Vec::new(),
        }
    }

    pub fn from_vec(vec: Vec<ComplexNumber>) -> ComplexVector {
        ComplexVector { elements: vec }
    }

    pub fn push(&mut self, element: ComplexNumber) {
        self.elements.push(element);
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, index: usize) -> Option<&ComplexNumber> {
        self.elements.get(index)
    }

    // Parallel vector addition
    pub fn add(&self, other: &ComplexVector) -> Option<ComplexVector> {
        if self.len() != other.len() {
            return None;
        }

        let result_vec: Vec<ComplexNumber> = self
            .elements
            .par_iter()
            .zip(other.elements.par_iter())
            .map(|(a, b)| a.add(b))   // assumes ComplexNumber::add(&self, &Self) -> Self
            .collect();

        Some(ComplexVector { elements: result_vec })
    }

    // Parallel max magnitude
    pub fn max_magnitude(&self) -> Option<&ComplexNumber> {
        self.elements
            .par_iter()
            .max_by(|a, b| {
                a.magnitude()
                    .partial_cmp(&b.magnitude())
                    .unwrap()
            })
    }

    // Parallel min magnitude
    pub fn min_magnitude(&self) -> Option<&ComplexNumber> {
        self.elements
            .par_iter()
            .min_by(|a, b| {
                a.magnitude()
                    .partial_cmp(&b.magnitude())
                    .unwrap()
            })
    }

    // Parallel average using reduce
    pub fn average(&self) -> Option<ComplexNumber> {
        let len = self.elements.len();
        if len == 0 {
            return None;
        }

        let sum = self
            .elements
            .par_iter()
            .map(|c| *c) // if ComplexNumber: Copy; otherwise clone
            .reduce(
                || ComplexNumber::init(0.0, 0.0),
                |acc, c| acc.add(&c),
            );

        Some(ComplexNumber {
            real: sum.real / len as f64,
            imaginary: sum.imaginary / len as f64,
        })
    }

    pub fn print(&self) {
        for c in &self.elements {
            c.print();
        }
    }
}
