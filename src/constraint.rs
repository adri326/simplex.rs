use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cond {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq
}

pub struct ConstraintBuilder {
    constraints: Vec<Row>,
    conditions: Vec<Cond>,

    target: Option<Row>,
}

impl ConstraintBuilder {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            conditions: Vec::new(),
            target: None
        }
    }

    pub fn push(&mut self, coefficients: Vec<i64>, minus_z: i64, condition: Cond) {
        self.constraints.push(Row::new(
            coefficients.into_iter().map(|x| x.into()).collect(),
            minus_z.into()
        ));
        self.conditions.push(condition);
    }

    pub fn push_row(&mut self, row: Row, condition: Cond) {
        self.constraints.push(row);
        self.conditions.push(condition);
    }

    pub fn target(&mut self, target: Row) {
        self.target = Some(target);
    }

    // TODO: implement the dual algorithm into the solver itself
    pub fn transform(&self) -> Self {
        let self_target = self.target.as_ref().unwrap();
        let mut self_constraints = self.constraints.clone();

        // Transform constraints to only contain Gt, Gte and Eq
        for (constraint, cond) in self_constraints.iter_mut().zip(self.conditions.iter().copied()) {
            match cond {
                Cond::Gt | Cond::Gte => constraint.mul(SuperReal::from((0, -1, 0))),
                _ => {}
            }
        }

        let self_conditions = self.conditions.iter().map(|c| {
            match c {
                Cond::Gt | Cond::Lt => Cond::Lt,
                Cond::Gte | Cond::Lte => Cond::Lte,
                Cond::Eq => Cond::Eq
            }
        }).collect::<Vec<_>>();

        // Transform Lt into Lte and Eq into two Gte
        let mut new_self_constraints = Vec::with_capacity(self_constraints.len());
        for (mut constraint, cond) in self_constraints.into_iter().zip(self_conditions.into_iter()) {
            match cond {
                Cond::Lt => {
                    constraint.minus_z = constraint.minus_z - SuperReal::from((0, 0, 1));
                    new_self_constraints.push(constraint);
                },
                Cond::Lte => new_self_constraints.push(constraint),
                Cond::Eq => {
                    new_self_constraints.push(constraint.clone());
                    constraint.mul(SuperReal::from((0, -1, 0)));
                    new_self_constraints.push(constraint);
                }
                _ => {}
            }
        }
        let self_constraints = new_self_constraints;

        let mut constraints = Vec::with_capacity(self_target.coefficients.len());
        for x in 0..self_target.coefficients.len() {
            let mut coefficients = Vec::with_capacity(self.constraints.len());
            for y in 0..self.constraints.len() {
                coefficients.push(self.constraints[y].coefficients[x]);
            }

            constraints.push(Row::new(coefficients, self_target.coefficients[x]));
        }

        let conditions = vec![Cond::Gte; constraints.len()];

        let target = self_constraints.iter().map(|row| -row.minus_z).collect::<Vec<_>>();
        let target = Row::new(target, -self_target.minus_z);

        Self {
            constraints,
            conditions,
            target: Some(target)
        }
    }

    pub fn build(mut self) -> (Vec<Row>, Row, Vec<usize>) {
        let mut target = self.target.unwrap();
        let mut indices = vec![None; self.conditions.len()];
        let mut n_indices = 0;
        let offset = self.constraints.get(0).map(|x| x.coefficients.len()).unwrap_or(0);

        for (i, cond) in self.conditions.iter().enumerate() {
            match cond {
                Cond::Eq => {}
                _ => {
                    indices[i] = Some(n_indices);
                    n_indices += 1;
                }
            }
        }

        // Extend rows
        for row in self.constraints.iter_mut() {
            for _n in 0..n_indices {
                row.coefficients.push(SuperReal::from(0));
            }
        }

        for _n in 0..n_indices {
            target.coefficients.push(SuperReal::from(0));
        }

        // Add coefficients
        for ((row, index), cond) in self.constraints.iter_mut().zip(indices).zip(self.conditions.iter()) {
            if let Some(index) = index {
                match cond {
                    Cond::Lte | Cond::Lt => {
                        row.coefficients[index + offset] = SuperReal::from(1);
                    }
                    Cond::Gte | Cond::Gt => {
                        row.coefficients[index + offset] = SuperReal::from(-1);
                    }
                    _ => {}
                }
            }
        }

        // Add epsilons
        for (row, cond) in self.constraints.iter_mut().zip(self.conditions.iter()) {
            match cond {
                Cond::Lt => {
                    row.minus_z = row.minus_z - SuperReal::from((0, 0, -1));
                }
                Cond::Gt => {
                    row.minus_z = row.minus_z + SuperReal::from((0, 0, -1));
                }
                _ => {}
            }
        }

        let mut basis = Vec::with_capacity(self.conditions.len());
        for n in 0..n_indices {
            basis.push(n + offset);
        }
        let mut n = 0;
        while basis.len() < self.conditions.len() {
            basis.push(n);
            n += 1;
        }

        for b in 0..basis.len() {
            for c in 0..b {
                assert!(basis[b] != basis[c]);
            }
        }

        return (self.constraints, target, basis);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build() {
        let mut builder = ConstraintBuilder::new();
        builder.push(vec![-2, 1], 2, Cond::Lte);
        builder.push(vec![-1, 2], 5, Cond::Lte);
        builder.push(vec![1, -4], 5, Cond::Lte);

        let constraints = vec![
            Row::from(vec![-2, 1, 1, 0, 0, 2]),
            Row::from(vec![-1, 2, 0, 1, 0, 5]),
            Row::from(vec![1, -4, 0, 0, 1, 5]),
        ];
        let target = Row::from(vec![1, 2, 0, 0, 0, 0]);
        let basis = vec![2, 3, 4];

        assert_eq!(builder.build(Row::from(
            vec![1, 2, 0]
        )), (constraints, target, basis));
    }
}
