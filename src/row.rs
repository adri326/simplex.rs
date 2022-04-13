use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub coefficients: Vec<SuperReal>,
    pub minus_z: SuperReal,
}

#[allow(dead_code)]
impl Row {
    pub fn new(coefficients: Vec<SuperReal>, minus_z: SuperReal) -> Self {
        Self {
            coefficients,
            minus_z
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.coefficients.len() + 1
    }

    pub fn div(&mut self, by: SuperReal) {
        let inverted = by.conj() / (by.real() * by.real());

        for c in self.coefficients.iter_mut() {
            *c = *c * inverted;
        }

        self.minus_z = self.minus_z * inverted;
    }

    pub fn add(&mut self, row: &Row) {
        assert!(self.len() == row.len());

        for (c, o) in self.coefficients.iter_mut().zip(row.coefficients.iter()) {
            *c = *c + *o;
        }

        self.minus_z = self.minus_z + row.minus_z;
    }

    pub fn sub(&mut self, row: &Row) {
        assert!(self.len() == row.len());

        for (c, o) in self.coefficients.iter_mut().zip(row.coefficients.iter()) {
            *c = *c - *o;
        }

        self.minus_z = self.minus_z - row.minus_z;
    }

    pub fn sub_mul(&mut self, row: &Row, by: SuperReal) {
        assert!(self.len() == row.len());

        for (c, o) in self.coefficients.iter_mut().zip(row.coefficients.iter()) {
            *c = *c - *o * by;
        }

        self.minus_z = self.minus_z - row.minus_z * by;
    }

    pub fn to_printable(&self) -> prettytable::Row {
        let mut vec = Vec::with_capacity(self.len());

        for coefficient in self.coefficients.iter() {
            vec.push(prettytable::Cell::new(&format!("{}", coefficient)));
        }

        vec.push(prettytable::Cell::new(&format!("{}", self.minus_z)));

        prettytable::Row::new(vec)
    }
}

impl From<Vec<i64>> for Row {
    fn from(vec: Vec<i64>) -> Self {
        assert!(vec.len() >= 1);

        let mut coefficients = Vec::with_capacity(vec.len() - 1);
        for x in vec.iter().take(vec.len() - 1) {
            coefficients.push(SuperReal::from(*x));
        }

        Row {
            coefficients,
            minus_z: SuperReal::from(*vec.last().unwrap())
        }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for x in self.coefficients.iter() {
            write!(f, "| {} ", x)?;
        }
        write!(f, "| {} |", self.minus_z)
    }
}
