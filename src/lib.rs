use std::fmt;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::process;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Phrase {
    text: String,
    author: String,
}

impl fmt::Display for Phrase {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" - {}", self.text, self.author)
    }
}

impl PartialEq for Phrase {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.author == other.author
    }
}

#[derive(Debug, Clone)]
pub struct Quote {
    line_number: usize,
    dialogue: Vec<Phrase>,
}

impl Quote {
    fn new(line: &str, line_number: usize) -> Quote {
        let mut quote_parts: Vec<&str> = Vec::new();

        let delimiters = Regex::new(r#"("-|”-|"|“|”| -|;;|
)"#).expect("Invalid regex");
        let splits = delimiters.split(line);
        for part in splits {
            let part_trim = part.trim();
            if !part_trim.is_empty() {
                quote_parts.push(part_trim);
            }
        }

        // Convert to a vector of Phrase.
        let num_phrase: usize = quote_parts.len() / 2;
        let mut dialogue = Vec::new();
        for i in 0..num_phrase {
            let text = quote_parts[i*2].to_owned();
            let author = quote_parts[i*2+1].to_owned();
            dialogue.push(Phrase { text, author });
        }

        Quote { dialogue, line_number}
    }

    // Write quotes to file.
    pub fn write_quotes(vec: &Vec<Quote>, filename: &str) -> io::Result<()> {
        if filename == "" {
            eprintln!("ERROR: `quote write` must specify output file.");
            process::exit(1);
        }
        let mut file = OpenOptions::new().create(true).append(true).open(filename)?;
        for quote in vec {
            let mut next = quote.to_string();
            next.push_str("\n");
            file.write_all(next.as_bytes())?;
        }
        Ok(())
    }

    // Take a file and convert it to a vector of quotes.
    // TODO: take argument as number of spaces between lines.
    pub fn parse_file(contents: &str, single_line_break: bool) -> Vec<Quote> {

        // Data structures to be output.
        let mut quotes_vec = Vec::new();

        // Stores information about current quote.
        let mut previous_line_empty = false;
        let mut line_number_option = None;
        let mut quote_text = String::new();

        for (line_number, line) in contents.lines().enumerate(){

            // Detects if there are two blank lines in a row.
            if line.is_empty() {
                previous_line_empty = true;

            } else {

                // If there are two blank lines in a row, create a new quote.
                if previous_line_empty || single_line_break{
                    if let Some(first_line_number) = line_number_option {
                        let quote = Quote::new(&quote_text, 1+first_line_number);
                        quotes_vec.push(quote);
                        previous_line_empty = false;
                        line_number_option = None;
                        quote_text = String::new();
                    }
                }

                // If this is the start of a new quote, set the line_number_option.
                if line_number_option == None {
                        line_number_option = Some(line_number);
                }
                quote_text.push_str(line);
                quote_text.push_str(";;"); // Delimiter between different lines in the quote.
            }
        }

        // If file does not end in a blank line, we need to make one more quote.
        if let Some(first_line_number) = line_number_option {
            let quote = Quote::new(&quote_text, 1+first_line_number);
            quotes_vec.push(quote);
        }

        quotes_vec
    }

    fn contains(&self, search: &str, author: bool) -> bool {
        for line in &self.dialogue {
            if author {
                if line.author.to_lowercase().contains(&search.to_lowercase()) {
                    return true;
                }
            } else {
                if line.text.to_lowercase().contains(&search.to_lowercase()) {
                    return true;
                }
            }
        }
        false
    }

    // Search for an author's name in a vector of quotes. 
    // Return a vec of all quotes with that name.
    pub fn search(quote_list: Vec<Quote>, name: &str, author: bool) -> Vec<Quote> {
        quote_list.into_iter().filter(|q| q.contains(name, author)).collect()
    }

    fn author_counts(quote_list: &Vec<Quote>) -> HashMap<&String, i32> {
        let mut count_map = HashMap::new();
        for quote in quote_list {
            let mut single_dialogue = HashSet::new();
            for line in &quote.dialogue {
                single_dialogue.insert(&line.author);
            }
            for author in single_dialogue {
                let current_opt: Option<&mut i32> = count_map.get_mut(&author);
                if let Some(current) = current_opt {
                    *current += 1;
                } else {
                    count_map.insert(author, 1);
                }
            }
        }
        count_map
    }

    // Print the key along with the number of quotes with each key.
    pub fn print_lengths(quote_list: &Vec<Quote>) {
        let count_map = Quote::author_counts(quote_list);
        let mut total = 0;
        for author in count_map.keys().sorted() {
            let count = count_map[author];
            println!("{}: {}", author, count);
            total += count;
        }
        println!("Total quotes: {}", total);
    }

    // Print all the quotes from a vector. Optionally include line numbers.
    pub fn print_list(quote_list: Vec<Quote>, line_numbers: bool) {
        for quote in quote_list {
            if line_numbers {
                println!("{}{}\n", quote, quote.line_number);
            } else {
                println!("{}", quote);
            }
        }
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "Quote number {}:", self.line_number)?;
        for phrase in &self.dialogue {
            writeln!(f, "{}", phrase)?;
        }
        write!(f, "")
    }
}

impl PartialEq for Quote {
    fn eq(&self, other: &Self) -> bool {
        self.line_number == other.line_number && self.dialogue == other.dialogue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "If you set your goals ridiculously high and it's a failure, you will fail above everyone else's success. -James Cameron

Life is what happens when you're busy making other plans. -John Lennon

“Real leaders must be ready to sacrifice all for the freedom of their people.” - Nelson Mandela

“A fundamental concern for others in our individual and community lives would go a long way in making the world the better place we so passionately dreamt of.” - Nelson Mandela

He didn’t fall? Inconceivable! - Vizzini 
You keep using that word. I do not think it means what you think it means - Inigo Montoya";
        let quote_list = Quote::parse_file(input, false);
        assert_eq!(quote_list[0], Quote::new("If you set your goals ridiculously high and it's a failure, you will fail above everyone else's success. -James Cameron", 1));
        assert_eq!(quote_list[4], Quote::new("He didn’t fall? Inconceivable! - Vizzini 
You keep using that word. I do not think it means what you think it means - Inigo Montoya", 9));
    
        let input = "If you set your goals ridiculously high and it's a failure, you will fail above everyone else's success. -James Cameron
Life is what happens when you're busy making other plans. -John Lennon
“Real leaders must be ready to sacrifice all for the freedom of their people.” - Nelson Mandela
“A fundamental concern for others in our individual and community lives would go a long way in making the world the better place we so passionately dreamt of.” - Nelson Mandela
“He didn’t fall? Inconceivable!” - Vizzini “You keep using that word. I do not think it means what you think it means” - Inigo Montoya";
        let quote_list = Quote::parse_file(input, true);
        assert_eq!(quote_list[0], Quote::new("If you set your goals ridiculously high and it's a failure, you will fail above everyone else's success. -James Cameron", 1));
        assert_eq!(quote_list[4], Quote::new("“He didn’t fall? Inconceivable!” - Vizzini “You keep using that word. I do not think it means what you think it means” - Inigo Montoya", 5));
    }
}