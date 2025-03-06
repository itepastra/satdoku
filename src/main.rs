use std::{io, ops::Index};

#[derive(Debug)]
enum SatEntry {
    Comment(String),
    Parameters(usize, usize),
    OrLine(Vec<isize>),
}

#[derive(Clone, Copy)]
struct Ordah {
    order_1: usize,
    order_2: usize,
    order_3: usize,
    order_4: usize,
}

impl Ordah {
    fn new(order: usize) -> Self {
        Ordah {
            order_1: order,
            order_2: order * order,
            order_3: order * order * order,
            order_4: order * order * order * order,
        }
    }
}

impl Index<usize> for Ordah {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            1 => &self.order_1,
            2 => &self.order_2,
            3 => &self.order_3,
            4 => &self.order_4,
            _ => panic!("can't index to this order"),
        }
    }
}

impl std::fmt::Display for SatEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SatEntry::Comment(comment) => write!(f, "c {comment}")?,
            SatEntry::Parameters(variables, constraints) => {
                write!(f, "p cnf {variables} {constraints}")?
            }
            SatEntry::OrLine(items) => {
                for item in items {
                    write!(f, "{item} ")?;
                }
                write!(f, "0")?
            }
        }
        Ok(())
    }
}

fn parse_input(order: Ordah) -> Result<Vec<SatEntry>, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut initialnums: Vec<SatEntry> = Vec::new();
    for row in 0..order[2] {
        stdin.read_line(&mut buffer)?;
        initialnums.extend(
            buffer
                .chars()
                .take(order[2])
                .enumerate()
                .filter_map(|(i, x)| {
                    if !x.is_ascii() {
                        return None;
                    };
                    match x {
                        '1'..='9' => Some(SatEntry::OrLine(vec![
                            (row * order[4] + i * order[2]) as isize + x as isize - '0' as isize,
                        ])),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>(),
        );
        buffer.clear();
    }

    Ok(initialnums)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order = Ordah::new(3);

    // ROW_SIZE squares, ROW_SIZE entries each, ROW_SIZE possible values each
    let variables = order[4] * order[2];

    let initialnums = parse_input(order)?;

    let single_num_constraints: Vec<_> = (0..order[4] as isize)
        .flat_map(|cell| single_constraint(order, cell * order[2] as isize + 1))
        .collect();
    let constraints = sudoku_constraints(order);
    println!(
        "{}",
        SatEntry::Parameters(
            variables,
            constraints.len() + single_num_constraints.len() + initialnums.len()
        )
    );
    for initial_number in initialnums {
        println!("{}", initial_number)
    }
    for constraint in constraints {
        println!("{}", constraint)
    }
    for single_num in single_num_constraints {
        println!("{}", single_num)
    }

    Ok(())
}

fn sudoku_constraints(order: Ordah) -> Vec<SatEntry> {
    let mut constraints = Vec::new();

    for area in 0..order[2] {
        let (srow, scol) = (area / order[1], area % order[1]);
        for num in 0..order[2] as isize {
            let square_base = ((srow * order[2] + scol) * order[3]) as isize + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..order[1] as isize)
                    .flat_map(|row| {
                        (0..order[1] as isize)
                            .map(|col| {
                                square_base + col * order[2] as isize + row * order[4] as isize
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect(),
            ));

            let row_base = (area * order[4]) as isize + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..order[2])
                    .map(|cell| row_base + (cell * order[2]) as isize)
                    .collect(),
            ));

            let column_base = (area * order[2]) as isize + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..order[2])
                    .map(|cell| column_base + (cell * order[2]) as isize)
                    .collect(),
            ));
        }
    }

    constraints
}

fn single_constraint(order: Ordah, cell_start: isize) -> Vec<SatEntry> {
    let mut constraints = vec![SatEntry::OrLine(
        (0..order[2] as isize)
            .map(|num| (cell_start + num))
            .collect(),
    )];

    for i in 0..(order[2] - 1) as isize {
        for j in i + 1..order[2] as isize {
            constraints.push(SatEntry::OrLine(vec![-(cell_start + i), -(cell_start + j)]));
        }
    }

    constraints
}
