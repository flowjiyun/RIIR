use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,
    #[arg(short('n'), long("number"), conflicts_with("number_nonblack_lines"))]
    pub number_lines: bool,
    #[arg(short('b'), long("number-nonblank"))]
    pub number_nonblack_lines: bool,
}
