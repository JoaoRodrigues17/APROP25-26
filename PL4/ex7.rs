mod complex_number;
use complex_number::ComplexNumber;

pub struct ComplexVector {
    elements: Vec<ComplexNumber>,
}

impl complex_vector{
    pub fn new() -> ComplexVector {
        ComplexVector {
            elements: Vec::new(),
        }
    }

    pub fn from_vec(vec: Vec<ComplexNumber>) -> ComplexVector {
        ComplexVector {
            elements: vec,
        }
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

    pub fn add(&self, other: &ComplexVector) -> Option<ComplexVector> {
        if self.len() != other.len() {
            return None;
        }
        let result_vec = self.elements.iter().zip(other.elements.iter())
            .map(|(a, b)| a.add(b))
            .collect();
        Some(ComplexVector { elements: result_vec })
    }

    pub fn max_magnitude(&self) -> Option<&ComplexNumber> {
        self.elements.iter().max_by(|a, b| a.magnitude().partial_cmp(&b.magnitude()).unwrap())
    }

    pub fn min_magnitude(&self) -> Option<&ComplexNumber> {
        self.elements.iter().min_by(|a, b| a.magnitude().partial_cmp(&b.magnitude()).unwrap())
    }

    pub fn average(&self) -> Option<ComplexNumber> {
        let len = self.elements.len();
        if len == 0 {
            return None;
        }
        let sum = self.elements.iter().fold(
            ComplexNumber::init(0.0, 0.0),
            |acc, c| acc.add(c)
        );
        Some(ComplexNumber {
            real: sum.real / len as f64,
            imaginary: sum.imaginary / len as f64,
        })
    }

    // You can add more methods like sub, multiply, etc. similar to add.

    pub fn print(&self) {
        for c in &self.elements {
            c.print();
        }
    }
}