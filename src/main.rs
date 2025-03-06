use std::io;

#[derive(Debug)]
enum SatEntry {
    Comment(String),
    Parameters(usize, usize),
    OrLine(Vec<isize>),
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let variables = 9 * 9 * 9; // 9 squares, 9 entries each, 9 possible values each

    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut initialnums: Vec<SatEntry> = Vec::new();
    for row in 0..9 {
        stdin.read_line(&mut buffer)?;
        initialnums.extend(
            buffer
                .chars()
                .take(9)
                .enumerate()
                .filter_map(|(i, x)| {
                    if !x.is_ascii() {
                        return None;
                    };
                    match x {
                        '1'..='9' => Some(SatEntry::OrLine(vec![
                            (row * 81 + i * 9) as isize + x as isize - '0' as isize,
                        ])),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>(),
        );
        buffer.clear();
    }

    let single_num_constraints: Vec<_> = (0..81)
        .flat_map(|cell| single_constraint(cell * 9 + 1))
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

    for area in 0..9 {
        let (srow, scol) = (area / 3, area % 3);
        for num in 0..9 {
            let square_base = (srow * 27 + scol * 3) * 9 + num + 1;
            constraints.push(SatEntry::OrLine(vec![
                square_base,
                square_base + 9,
                square_base + 18,
                square_base + 81,
                square_base + 90,
                square_base + 99,
                square_base + 162,
                square_base + 171,
                square_base + 180,
            ]));

            let row_base = area * 9 * 9 + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..9).map(|cell| row_base + cell * 9).collect(),
            ));

            let column_base = area * 9 + num + 1;
            constraints.push(SatEntry::OrLine(
                (0..9).map(|cell| column_base + cell * 81).collect(),
            ));
        }
    }

    constraints
}

fn single_constraint(cell_start: isize) -> Vec<SatEntry> {
    let mut constraints = vec![SatEntry::OrLine(
        (0..9).map(|num| (cell_start + num)).collect(),
    )];

    for i in 0..8 {
        for j in i + 1..9 {
            constraints.push(SatEntry::OrLine(vec![-(cell_start + i), -(cell_start + j)]));
        }
    }

    constraints
}
