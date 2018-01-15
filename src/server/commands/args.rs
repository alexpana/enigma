pub fn split_args(v: &str) -> Vec<&str> {
    let trimmed = v.trim();

    let mut splits = Vec::new();    
    let mut prev_split = 0 as usize;
    let mut quoted = false;
    let mut was_quote = false;
    
    for (i, c) in trimmed.chars().enumerate() {
        if c.is_whitespace() && !quoted {
            if prev_split < i - 1 {
                splits.push(get_split(trimmed, prev_split, i, was_quote));
                was_quote = false;
            }
            prev_split = i+1
        } else if c == '"' {
            quoted = !quoted;
            was_quote = true;
        }
    }

    splits.push(get_split(trimmed, prev_split, trimmed.chars().count(), was_quote));

    splits
}

fn get_split(val: &str, from: usize, to: usize, was_quote: bool) -> &str {
    if was_quote {
        &val[from+1..to-1]
    } else {
        &val[from..to]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_simple_words() {
        assert_eq!(vec!("foo", "bar"), split_args("foo bar"));
    }

    #[test]
    fn split_skip_spaces() {
        assert_eq!(vec!("foo", "bar", "zmosh"), split_args("foo    bar    zmosh"));
    }
    
    #[test]
    fn split_skip_leading_and_trailing_spaces() {
        assert_eq!(vec!("foo", "bar"), split_args("    foo    bar"));
        assert_eq!(vec!("foo", "bar"), split_args("foo    bar     "));
        assert_eq!(vec!("foo", "bar"), split_args("    foo    bar     "));
    }
    
    #[test]
    fn split_skip_whitespace_characters() {
        assert_eq!(vec!("foo", "bar"), split_args("foo\t \tbar"));
    }
    
    #[test]
    fn split_preserve_quotes() {
        assert_eq!(vec!("foo", "bar is quoted"), split_args("foo   \"bar is quoted\"  "));
        assert_eq!(vec!("foo is quoted", "bar is quoted"),
                   split_args("\"foo is quoted\"   \"bar is quoted\"  "));
    }
}
