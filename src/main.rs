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

    fn is_primal_realisable(constraints: &Vec<Row>) -> bool {
        constraints.iter().all(|row| row.minus_z >= SuperReal::from((0, 0, 0)))
    }

    fn is_dual_realisable(target: &Row) -> bool {
        target.coefficients.iter().all(|&c| c <= SuperReal::from((0, 0, 0)))
    }

    for _step in 0..max_steps {
        let dual_step = !is_primal_realisable(&constraints) && is_dual_realisable(&target);
        let (active_row, entrant_var, exit_var, exit_index) = if dual_step {
            // Dual step
            // Trouver la ligne k
            // Trouver la variable sortante, la variable de la base qui est active dans la ligne
            // Trouver la variable entrante, argmax(c_j/a_{kj})
            // Effectuer la transformation

            let exit_row = match argmin(
                constraints
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (i, row.minus_z))
                    .filter(|(_i, x)| *x < SuperReal::from(0)),
            ) {
                None => break,
                Some(x) => x,
            };

            let entrant_var = match argmax(
                constraints[exit_row]
                    .coefficients
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(i, x)| *x < SuperReal::from(0) && !basis.iter().find(|&j| j == i).is_some())
                    .map(|(i, x)| (i, -target.coefficients[i] / x)),
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

            (exit_row, entrant_var, exit_var, exit_index)
        } else {
            let entrant_var = match argmax(
                target
                    .coefficients
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(i, x)| *x >= SuperReal::from(0) && !basis.iter().find(|&j| j == i).is_some()),
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

            (exit_row, entrant_var, exit_var, exit_index)
        };

        basis[exit_index] = entrant_var;

        if hashset.contains(&basis) {
            break;
        } else {
            hashset.insert(basis.clone());
        }

        println!("Étape {}: {}", _step + 1, if dual_step {"duale"} else {"primale"});
        println!("Variable entrante: {}", entrant_var + 1);
        println!("Variable sortante: {}", exit_var + 1);
        println!("Base: {:?}", basis.iter().map(|x| x+1).collect::<Vec<_>>());

        let div_by = constraints[active_row].coefficients[entrant_var];
        constraints[active_row].div(div_by);

        let div_by = constraints[active_row].clone();

        for (y, row) in constraints.iter_mut().enumerate() {
            if y == active_row {
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
    // builder.push(vec![-2, 1], 2, Cond::Lte);
    // builder.push(vec![-1, 2], 5, Cond::Lte);
    // builder.push(vec![1, -4], 5, Cond::Lte);
    // builder.target(Row::from(vec![1, 2, 0]));

    builder.push(vec![-2, -2, -1], -3, Cond::Lte);
    builder.push(vec![-3, -1, -3], -4, Cond::Lte);
    builder.target(Row::from(vec![-180, -120, -150, 0]));

    {
        println!("== Algorithme simplexe dual ==");
        let (constraints, target, basis) = builder.transform().build();
        let (basis, target) = simplex(
            constraints,
            target,
            basis,
            10
        );

        println!("{:?}", basis);
        println!("{}", target);
    }

    println!("\n");

    println!("== Algorithme simplexe primal ==");

    let (constraints, target, basis) = builder.build();

    let (basis, target) = simplex(
        constraints,
        target,
        basis,
        10
    );

    println!("{:?}", basis);
    println!("{}", target);
}
