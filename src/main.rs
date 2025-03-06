use std::io;

#[derive(Debug)]
enum SatEntry {
    Comment(String),
    Parameters(usize, usize),
    OrLine(Vec<isize>),
}

const ORDER: usize = 3;
const ORDER_2: usize = ORDER * ORDER;
const ORDER_3: usize = ORDER_2 * ORDER;
const ORDER_4: usize = ORDER_3 * ORDER;

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

fn parse_input() -> Result<Vec<SatEntry>, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut initialnums: Vec<SatEntry> = Vec::new();
    for row in 0..ORDER_2 {
        stdin.read_line(&mut buffer)?;
        initialnums.extend(
            buffer
                .chars()
                .take(ORDER_2)
                .enumerate()
                .filter_map(|(i, x)| {
                    if !x.is_ascii() {
                        return None;
                    };
                    match x {
                        '1'..='9' => Some(SatEntry::OrLine(vec![
                            (row * ORDER_4 + i * ORDER_2) as isize + x as isize - '0' as isize,
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
    // ROW_SIZE squares, ROW_SIZE entries each, ROW_SIZE possible values each
    let variables = ORDER_4 * ORDER_2;

    let initialnums = parse_input()?;

    let single_num_constraints: Vec<_> = (0..ORDER_4 as isize)
        .flat_map(|cell| single_constraint(cell * ORDER_2 as isize + 1))
        .collect();
    let constraints = sudoku_constraints();
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

fn sudoku_constraints() -> Vec<SatEntry> {
    let mut constraints = Vec::new();

    for area in 0..ORDER_2 {
        let (srow, scol) = (area / ORDER, area % ORDER);
        for num in 0..ORDER_2 as isize {
            let square_base = ((srow * ORDER_2 + scol) * ORDER_3) as isize + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..ORDER as isize)
                    .flat_map(|row| {
                        (0..ORDER as isize)
                            .map(|col| {
                                square_base + col * ORDER_2 as isize + row * ORDER_4 as isize
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect(),
            ));

            let row_base = (area * ORDER_4) as isize + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..ORDER_2)
                    .map(|cell| row_base + (cell * ORDER_2) as isize)
                    .collect(),
            ));

            let column_base = (area * ORDER_2) as isize + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..ORDER_2)
                    .map(|cell| column_base + (cell * ORDER_4) as isize)
                    .collect(),
            ));
        }
    }

    constraints
}

fn single_constraint(cell_start: isize) -> Vec<SatEntry> {
    let mut constraints = vec![SatEntry::OrLine(
        (0..ORDER_2 as isize)
            .map(|num| (cell_start + num))
            .collect(),
    )];

    for i in 0..(ORDER_2 - 1) as isize {
        for j in i + 1..ORDER_2 as isize {
            constraints.push(SatEntry::OrLine(vec![-(cell_start + i), -(cell_start + j)]));
        }
    }

    constraints
}
