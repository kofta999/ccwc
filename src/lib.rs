use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub bytes_arg: bool,
    pub lines_arg: bool,
    pub words_arg: bool,
    pub chars_arg: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut config = Config {
            file_path: String::from(""),
            bytes_arg: false,
            chars_arg: false,
            lines_arg: false,
            words_arg: false,
        };

        args.for_each(|arg| {
            match arg.as_str() {
                "-c" => config.bytes_arg = true,
                "-l" => config.lines_arg = true,
                "-w" => config.words_arg = true,
                x => config.file_path = x.to_string(),
            };
        });

        if config.file_path.len() == 0 {
            return Err("File path required");
        }

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = config.file_path.clone();
    let file = fs::read_to_string(config.file_path)?;
    let mut res: Vec<String> = Vec::new();

    // should do something else when im using stdin

    if config.bytes_arg {
        let chars_count = count_bytes(&file);
        res.push(chars_count.to_string());
    };

    if config.lines_arg {
        let lines_count = count_lines(&file);
        res.push(lines_count.to_string())
    }

    if config.words_arg {
        let words_count = count_words(&file);
        res.push(words_count.to_string())
    }

    res.push(filename);
    println!("{}", res.join(" "));
    Ok(())
}

fn count_bytes(content: &str) -> usize {
    // len() counts the number of bytes
    content.len()
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_words(content: &str) -> usize {
    content
        .lines()
        .fold(0, |acc, line| acc + line.trim().split_whitespace().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes() {
        let s = "Hello world!";
        assert_eq!(12, count_bytes(s));

        let s = "美味しいご飯だった";
        assert_eq!(27, count_bytes(s));
    }

    #[test]
    fn test_count_lines() {
        let s = "\
Hello world!
rust is cool
but typescript is cooler
";

        assert_eq!(3, count_lines(s));
    }

    #[test]
    fn test_count_words() {
        let s = "Title: The Art of War";
        assert_eq!(5, count_words(s));

        let s = "\
            The Project Gutenberg eBook of The Art of War

            This ebook is for the use of anyone anywhere in the United States and
            most other parts of the world at no cost and with almost no restrictions
            whatsoever. You may copy it, give it away or re-use it under the terms
            of the Project Gutenberg License included with this ebook or online
            at www.gutenberg.org. If you are not located in the United States,
            you will have to check the laws of the country where you are located
            before using this eBook.

            Title: The Art of War


            Author: active 6th century B.C. Sunzi
";
        assert_eq!(102, count_words(s));
    }
}
