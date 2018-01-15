pub fn match_abbrev(value: &str, query: &str) -> bool {

    if value.to_lowercase().starts_with(query.to_lowercase().as_str()) {
        return true;
    }

    let mut parts = Vec::new();
    let mut part = String::from("");
    for current_char in query.chars() {
        if current_char.is_uppercase() {
            if !part.is_empty() {
                parts.push(part);
            }
            part = String::from("");
        }
        part.push(current_char);
    }
    if !part.is_empty() {
        parts.push(part);
    }

    let mut index = 0;
    for part in parts {
        match value[index..].find(&part) {
            None => return false,
            Some(v) => {
                index = index + v;
            }
        }X
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_abbrev_should_pass() {
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "ProjSpLoc"));
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "PSL"));
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "ProjS"));
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "ProLoc"));
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "SLoc"));
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "P"));
        assert_eq!(true, match_abbrev("ProjectileSpawnLocation", "proj"));
    }

    #[test]
    fn match_abbrev_should_fail() {
        assert_eq!(false, match_abbrev("ProjectileSpawnLocation", "ProjSpLc"));
        assert_eq!(false, match_abbrev("ProjectileSpawnLocation", "PSLC"));
    }
}
