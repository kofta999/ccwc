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
        let chars_count = count_chars(&file);
        res.push(chars_count.to_string());
    };

    res.push(filename);
    println!("{}", res.join(" "));
    Ok(())
}

fn count_chars(content: &str) -> usize {
    // len() counts the number of bytes
    content.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bytes() {
        let s = "Hello world!";
        assert_eq!(12, count_chars(s));

        let s = "美味しいご飯だった";
        assert_eq!(27, count_chars(s));
    }
}
