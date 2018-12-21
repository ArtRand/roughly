use std::collections;
use std::fs;

pub fn parse_fasta(file_path: &str) -> Vec<SequenceRecord> {
    let content = fs::read_to_string(file_path).expect("failed to get content");

    let mut sequence_accumulator: Vec<&str> = Vec::new();
    let mut record_accumulator: Vec<SequenceRecord> = Vec::new();
    let mut current_sequence_header: &str;
    let mut lines = content.lines();

    if let Some(h) = lines.next() {
        current_sequence_header = h;
    } else {
        panic!("first line must be fasta!")
    }

    let mut st: usize = 0;
    let mut ed: usize = 0;
    for line in lines {
        if line.starts_with('>') {
            let record: SequenceRecord;
            if let Ok(sequence_record) =
                SequenceRecord::create(current_sequence_header, &sequence_accumulator[st..ed])
            {
                record = sequence_record
            } else {
                continue;
            }
            record_accumulator.push(record);
            current_sequence_header = line;
            st = ed;
            continue;
        }
        sequence_accumulator.push(line);
        ed += 1;
    }

    if let Ok(sr) = SequenceRecord::create(current_sequence_header, &sequence_accumulator[st..ed]) {
        record_accumulator.push(sr);
    }
    record_accumulator
}

pub struct SequenceRecord {
    pub sequence: String,
    pub header: String,
    pub comment: String,
}

impl SequenceRecord {
    fn create(header: &str, sequences: &[&str]) -> Result<SequenceRecord, &'static str> {
        let joined = sequences.join("");

        let mut bases: collections::HashSet<char> = collections::HashSet::with_capacity(4);
        bases.insert('A');
        bases.insert('C');
        bases.insert('G');
        bases.insert('T');

        for b in joined.chars() {
            if !bases.contains(&b) {
                eprintln!("sequence contains illegal character {}", b);
                return Err("illegal character");
            }
        }

        let splitted: Vec<&str> = header
            .split(|c: char| c.is_whitespace())
            .filter(|s| s.len() > 0)
            .collect();
        let sequence_name: String;
        if let Some(name) = splitted.first() {
            sequence_name = String::from(*name);
        } else {
            return Err("failed to parse name");
        }
        let sequence_comment = splitted[1..].join(" ");

        Ok(SequenceRecord {
            sequence: joined,
            header: sequence_name[1..].to_string(),
            comment: sequence_comment,
        })
    }

    pub fn get_base(&self, i: usize) -> &str {
        self.sequence.get(i..(i + 1)).unwrap()
    }
}
