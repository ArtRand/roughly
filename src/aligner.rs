use crate::dna::SequenceRecord;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Traceback {
    top: String,
    middle: String,
    bottom: String,
}

impl std::fmt::Display for Traceback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}\n", self.top, self.middle, self.bottom)
    }
}

pub struct Alignment {
    pub dp_matrix: HashMap<Coordinate, Cell>,
}

impl Alignment {
    fn get_cell(&self, i: i8, j: i8) -> Option<&Cell> {
        self.dp_matrix.get(&Coordinate { i: i, j: j })
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Coordinate {
    pub i: i8,
    pub j: i8,
}

#[derive(Debug)]
enum Pointer {
    Middle,
    Lower,
    Upper,
}

#[derive(Debug, Default)]
pub struct Cell {
    score: f64,
    pointer: Option<Pointer>,
}

impl Cell {
    fn new(score: f64, p: Option<Pointer>) -> Cell {
        Cell {
            score: score,
            pointer: p,
        }
    }
}

// https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats/28248065#28248065
// Score is not actually used, but keeping here for reference.
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Score(f64);

impl Score {
    fn new(score: f64) -> Option<Score> {
        if score.is_nan() {
            None
        } else {
            Some(Score(score))
        }
    }

    fn to_float(&self) -> f64 {
        self.0
    }
}

impl Eq for Score {}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Default for Score {
    fn default() -> Score {
        Score(0.0)
    }
}

#[derive(Debug)]
pub struct SubstitutionMatrix {
    pub match_score: f64,
    pub mismatch_score: f64,
    pub gap_score: f64,
}

pub fn align(A: &SequenceRecord, B: &SequenceRecord, sub: &SubstitutionMatrix) -> Alignment {
    fn initialize_first_row(a: &SequenceRecord, mat: &mut HashMap<Coordinate, Cell>) {
        for _i in -1..(a.sequence.len() as i8) {
            let cell: Cell = Default::default();
            let coord = Coordinate { i: _i, j: -1 };
            mat.insert(coord, cell);
        }
    }

    fn initialize_first_column(b: &SequenceRecord, mat: &mut HashMap<Coordinate, Cell>) {
        for _j in -1..(b.sequence.len() as i8) {
            let cell = Default::default();
            let coord = Coordinate { i: -1, j: _j };
            mat.insert(coord, cell);
        }
    }

    fn cell_calculate(
        upper: &Cell,
        middle: &Cell,
        lower: &Cell,
        a_i: &str,
        b_j: &str,
        sub: &SubstitutionMatrix,
    ) -> Cell {
        // debug
        //println!(
        //    "cell calculate\nupper {:?}\nmiddle {:?}\nlower {:?}",
        //    upper, middle, lower
        //);

        let from_upper_score = upper.score + sub.gap_score;
        let from_lower_score = lower.score + sub.gap_score;
        let from_middle_score = if a_i == b_j {
            middle.score + sub.match_score
        } else {
            middle.score + sub.mismatch_score
        };

        // there is no max in Rust for f64, so we resort to this nonsense - mostly to
        // appease the borrow checker. We cannot allocate a cell for each option, because
        // it won't be known which can be deallocated at run time.
        let (max_score, p) = if from_middle_score >= from_upper_score {
            if from_middle_score >= 0.0 {
                (from_middle_score, Some(Pointer::Middle))
            } else {
                (0.0, None)
            }
        } else if from_upper_score >= from_lower_score {
            if from_upper_score >= 0.0 {
                (from_upper_score, Some(Pointer::Upper))
            } else {
                (0.0, None)
            }
        } else {
            if from_lower_score >= 0.0 {
                (from_lower_score, Some(Pointer::Lower))
            } else {
                (0.0, None)
            }
        };

        // println!("max score by tree thing {} {:?}", max_score, p);

        // this was a different attempt, the idea was to find the max first then get the
        // index second. Once I know the index I can make the correct pointer.
        //let choices = [
        //    from_upper_score, from_middle_score, from_lower_score
        //];
        //let max = choices.iter().cloned().fold(0./0., f64::max);
        //println!("max is {:?}", max);

        Cell::new(max_score, p)
    }

    let mut dp_mat: HashMap<Coordinate, Cell> = HashMap::new();

    initialize_first_row(A, &mut dp_mat);
    initialize_first_column(B, &mut dp_mat);

    // println!("mat {:?}", dp_mat);

    for _j in 0..(B.sequence.len() as i8) {
        for _i in 0..(A.sequence.len() as i8) {
            let get_cell = |i_offset: i8, j_offset: i8| {
                let _coord = Coordinate {
                    i: _i - i_offset,
                    j: _j - j_offset,
                };
                if let Some(cell) = dp_mat.get(&_coord) {
                    return cell;
                } else {
                    panic!(
                        "cell not found for coordinate {:?}
                        i:{} j:{} (i_offset: {}, j_offset: {})",
                        _coord, _i, _j, i_offset, j_offset
                    );
                };
            };

            // debug
            //println!(
            //    "{} j={}: {} -- {} :i={} {}",
            //    B.header,
            //    _j,
            //    B.get_base(_j as usize),
            //    A.get_base(_i as usize),
            //    _i,
            //    A.header
            //);

            //println!("upper {:?}", upper);
            //println!("middle{:?}", middle);
            //println!("left {:?}", left);

            let upper = get_cell(0, 1);
            let middle = get_cell(1, 1);
            let lower = get_cell(1, 0);

            let this_cell: Cell = cell_calculate(
                &upper,
                &middle,
                &lower,
                A.get_base(_i as usize),
                B.get_base(_j as usize),
                sub,
            );

            dp_mat.insert(Coordinate { i: _i, j: _j }, this_cell);
        }
    }

    Alignment { dp_matrix: dp_mat }
}

pub fn do_traceback(a: &SequenceRecord, b: &SequenceRecord, alignment: &Alignment) -> Traceback {
    let mut max_score = 0.0;
    let mut i: i8 = -1;
    let mut j: i8 = -1;
    for (coord, cell) in &alignment.dp_matrix {
        if cell.score >= max_score {
            max_score = cell.score;
            i = coord.i;
            j = coord.j
        } else {
            continue;
        }
    }

    // println!("minimum {}, i {}, j {}",max_score, i, j);

    let mut current_cell = alignment
        .get_cell(i, j)
        .expect(&format!("failed to get cell at i {} j {}", i, j));

    let mut top = String::new();
    let mut middle = String::new();
    let mut bottom = String::new();

    loop {
        match current_cell.pointer {
            Some(Pointer::Middle) => {
                let _a = a.get_base(i as usize);
                let _b = b.get_base(j as usize);
                top.push_str(_a);
                middle.push_str("|");
                bottom.push_str(_b);
                i -= 1;
                j -= 1;
            }
            Some(Pointer::Lower) => {
                let _a = a.get_base(i as usize);
                top.push_str(_a);
                middle.push_str(" ");
                bottom.push_str("-");
                i -= 1;
            }
            Some(Pointer::Upper) => {
                let _b = b.get_base(j as usize);
                top.push_str("-");
                middle.push_str(" ");
                bottom.push_str(_b);
                j -= 1;
            }
            None => break,
        };
        current_cell = alignment
            .get_cell(i, j)
            .expect(&format!("failed to get cell at i {} j {}", i, j));
    }

    let mut top_line = top.chars().rev().collect::<String>();
    top_line.push_str(&format!(" {}", a.header));
    let mut botton_line = bottom.chars().rev().collect::<String>();
    botton_line.push_str(&format!(" {}", b.header));

    Traceback {
        top: top_line,
        middle: middle.chars().rev().collect::<String>(),
        bottom: botton_line,
    }
}
