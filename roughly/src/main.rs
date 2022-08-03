use clap::{App, Arg, ArgMatches};
use roughlylib::aligner;
use roughlylib::dna;

fn arg_to_float(matches: &ArgMatches, name: &str) -> Result<f64, String> {
    let arg = matches.value_of(name).unwrap_or_default();
    if let Ok(s) = arg.parse::<f64>() {
        Ok(s)
    } else {
        Err(format!("failed to parse {}", arg))
    }
}

fn main() -> Result<(), String> {
    let matches = App::new("roughly: Smith-Waterman in Rust")
        .version("0.1.0")
        .author("Art Rand")
        .about("aligns DNA sequences from a FASTA file")
        .arg(
            Arg::with_name("FASTA")
                .short("f")
                .long("fasta")
                .help("input file to use")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("GAP")
                .short("g")
                .long("gap")
                .help("gap score, default -2")
                .default_value("-2")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("MATCH")
                .short("m")
                .long("match")
                .help("match score, default 3")
                .default_value("3")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("MISMATCH")
                .short("x")
                .long("mismatch")
                .help("mismatch score, default -3")
                .default_value("-3")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let file_path = matches.value_of("FASTA").unwrap();

    let records = dna::parse_fasta(&file_path);

    let a = &records[0];
    let b = &records[1];

    // let gap_cost: f64;
    // if let Ok(cost) = matches.value_of("GAP").unwrap_or_default().parse::<f64>() {
    //     gap_cost = cost;
    // } else {
    //     panic!("failed to parse gap cost");
    // }

    let sub: aligner::SubstitutionMatrix = aligner::SubstitutionMatrix {
        match_score: arg_to_float(&matches, "MATCH")?,
        mismatch_score: arg_to_float(&matches, "MISMATCH")?,
        gap_score: arg_to_float(&matches, "GAP")?,
    };

    let aln = aligner::align(&a, &b, &sub);
    let trace_back = aligner::do_traceback(a, b, &aln);
    println!("\n{}\n", trace_back);
    Ok(())
}
