use std::collections::HashSet;
pub mod superreal;
use superreal::*;

pub mod row;
use row::Row;

pub mod constraint;
#[allow(unused_imports)]
use constraint::*;

#[allow(dead_code)]
fn argmax<F: std::cmp::PartialOrd, I: Iterator<Item = (usize, F)>>(mut iter: I) -> Option<usize> {
    use std::cmp::Ordering;
    let mut best = iter.next()?;

    for item in iter {
        match item.1.partial_cmp(&best.1)? {
            Ordering::Greater => best = item,
            _ => {}
        }
    }

    Some(best.0)
}

#[allow(dead_code)]
fn argmin<F: std::cmp::PartialOrd, I: Iterator<Item = (usize, F)>>(mut iter: I) -> Option<usize> {
    use std::cmp::Ordering;
    let mut best = iter.next()?;

    for item in iter {
        match item.1.partial_cmp(&best.1)? {
            Ordering::Less => best = item,
            _ => {}
        }
    }

    Some(best.0)
}

#[allow(dead_code)]
fn simplex(
    mut constraints: Vec<Row>,
    mut target: Row,
    mut basis: Vec<usize>,
    max_steps: usize,
) -> (Vec<usize>, Row) {
    {
        let mut table = prettytable::Table::new();
        for row in constraints.iter() {
            table.add_row(row.to_printable());
        }
        table.add_row(target.to_printable());

        table.printstd();
    }

    let mut hashset = HashSet::new();
    hashset.insert(basis.clone());

    for _step in 0..max_steps {
        let entrant_var = match argmax(
            target
                .coefficients
                .iter()
                .copied()
                .enumerate()
                .filter(|(i, x)| *x >= SuperReal::from(0) && !basis.iter().find(|x| *x == i).is_some()),
        ) {
            None => break,
            Some(x) => x,
        };

        let exit_row = match argmin(
            constraints
                .iter()
                .enumerate()
                .map(|(i, row)| (i, row.minus_z / row.coefficients[entrant_var]))
                .filter(|(_i, x)| *x >= SuperReal::from(0)),
        ) {
            None => break,
            Some(x) => x,
        };

        let (exit_index, exit_var) = basis
            .iter()
            .copied()
            .enumerate()
            .find(|(_, b)| constraints[exit_row].coefficients[*b] != SuperReal::from(0))
            .expect("No basis coefficient in row");

        basis[exit_index] = entrant_var;

        if hashset.contains(&basis) {
            break;
        } else {
            hashset.insert(basis.clone());
        }

        println!("Step {}", _step);
        println!("Variable entrante: {}", entrant_var);
        println!("Variable sortante: {}", exit_var);
        println!("Base: {:?}", basis);

        let div_by = constraints[exit_row].coefficients[entrant_var];
        constraints[exit_row].div(div_by);

        let div_by = constraints[exit_row].clone();

        for (y, row) in constraints.iter_mut().enumerate() {
            if y == exit_row {
                continue
            }

            row.sub_mul(&div_by, row.coefficients[entrant_var]);
        }

        target.sub_mul(&div_by, target.coefficients[entrant_var]);

        let mut table = prettytable::Table::new();
        for row in constraints.iter() {
            table.add_row(row.to_printable());
        }
        table.add_row(target.to_printable());

        table.printstd();
    }

    (basis, target)
}

fn main() {
    let mut builder = ConstraintBuilder::new();
    builder.push(vec![-2, 1], 2, Cond::Lte);
    builder.push(vec![-1, 2], 5, Cond::Lte);
    builder.push(vec![1, -4], 5, Cond::Lte);

    let (constraints, target, basis) = builder.build(Row::from(vec![1, 2, 0]));

    let (basis, target) = simplex(
        constraints,
        target,
        basis,
        10
    );

    // println!("{:?}", basis);
    // println!("{}", target);
}
