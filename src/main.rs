use std::fs;
use structopt::StructOpt;
use quotes::Quote;

// Parse command line arguments using structopt.
#[derive(Debug, StructOpt)]
struct Opt {

    /// Required: command to perform on the quotes.
    /// 
    /// `count`: Count number of quotes from each author.
    /// 
    /// `list`: List all possible quotes.
    /// 
    /// `write`: Write quotes to another file. Appends quotes to the end of the file.
    command: String,

    /// Required: input file containing quotes.
    input: String,

    /// Output file. Required for `quotes write`.
    #[structopt(default_value = "")]
    output: String,

    /// Include line numbers for `quotes list`.
    #[structopt(short, long)]
    line_number: bool,

    /// Filter quotes by author.
    /// 
    /// Case insensitive. Matches an author that contains the search term.
    #[structopt(short, long)]
    author: Option<String>,

    /// Filter quote text.
    /// 
    /// Case insensitive. Matches any text that contains the search term.
    #[structopt(short, long)]
    search: Option<String>,
}

// TODO: encrypt and decrypt quotes
fn main() {
    let opt = Opt::from_args();
    let contents = fs::read_to_string(&opt.input).expect(&format!("Could not find file {}.", opt.input));
    let mut quote_list = Quote::parse_file(&contents, false);
    
    // Filter quote text.
    if let Some(query) = opt.search {
        quote_list = Quote::search(quote_list, &query, false);
    }
    // Filter quote authors.
    if let Some(query) = opt.author {
        quote_list = Quote::search(quote_list, &query, true);
    }

    if opt.command == "list" {
        Quote::print_list(quote_list, opt.line_number);
    } else if opt.command == "count" {
        Quote::print_lengths(&quote_list);
    } else if opt.command == "write" {
        Quote::write_quotes(&quote_list, &opt.output).unwrap();
    } else {
        print!("Invalid command. Try --help for options.");
    }
}
