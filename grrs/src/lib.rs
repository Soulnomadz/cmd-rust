use std::io::{BufRead, Write};
use anyhow::{Result, Context};

pub fn find_matches(content: impl BufRead, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        let line = line?;
        if line.contains(pattern) {
            // println!("{}", line);
            writeln!(writer, "{}", line)
                .with_context(|| "output fail")?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use assert_fs::prelude::*;

    #[test]
    fn find_a_match() {
        let mut ret = Vec::new();

        let temp = assert_fs::NamedTempFile::new("foo.txt").unwrap();

        temp.write_str("lorem ipsum\ndolor sit amet")
            .unwrap();

        let _ = find_matches(
            BufReader::new(File::open(temp.path()).unwrap()), 
            "lorem",
            &mut ret,
        );

        assert_eq!(ret, b"lorem ipsum\n");
    }
}