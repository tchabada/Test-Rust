use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

#[allow(dead_code)]
fn assignment1(s: &str) -> bool {
    let mut letters = HashSet::new();

    for c in s.chars() {
        if letters.contains(&c) {
            return false;
        } else {
            letters.insert(c);
        }
    }

    true
}

#[allow(dead_code)]
fn assignment2(first: &str, second: &str) -> bool {
    if first.chars().count() != second.chars().count() {
        return false;
    }

    let mut first_letters = HashMap::new();
    let mut second_letters = HashMap::new();

    for c in first.chars() {
        *first_letters.entry(c).or_insert(0) += 1;
    }

    for c in second.chars() {
        *second_letters.entry(c).or_insert(0) += 1;
    }

    first_letters == second_letters
}

#[allow(dead_code)]
fn assignment3(s: &str) -> String {
    let count = s.chars().count() + s.matches(" ").count() * 2;
    let mut result = String::with_capacity(count);

    for c in s.chars() {
        if c == ' ' {
            result.push_str("%20");
        } else {
            result.push(c);
        }
    }

    result
}

#[allow(dead_code)]
fn assignment4(s: &str) -> bool {
    let mut letters = HashMap::new();

    for c in s.chars() {
        if c != ' ' {
            *letters.entry(c).or_insert(0) += 1;
        }
    }

    let mut found_odd = false;

    for (_, count) in letters {
        if count % 2 == 1 {
            if found_odd {
                return false;
            }

            found_odd = true;
        }
    }

    true
}

#[allow(dead_code)]
fn assignment5(first: &str, second: &str) -> bool {
    if (first.chars().count() as isize - second.chars().count() as isize).abs() > 1 {
        return false;
    }

    let first_chars = first.chars().collect::<Vec<_>>();
    let second_chars = second.chars().collect::<Vec<_>>();
    let mut difference = false;

    for i in 0..first_chars.len() {
        if first_chars[i] != second_chars[i] {
            if difference {
                return false;
            }

            difference = true;
        }
    }

    false
}

#[allow(unused_macros)]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
macro_rules! hashset {
    ($( $key: expr ),*) => {{
         let mut set = ::std::collections::HashSet::new();
         $( set.insert($key); )*
         set
    }}
}

fn main() {
    assert_eq!(assignment1("abc"), true);
    assert_eq!(assignment1("test"), false);

    assert_eq!(assignment2("abc", "cba"), true);
    assert_eq!(assignment2("a", "b"), false);

    assert_eq!(assignment3("a b"), "a%20b");

    assert_eq!(assignment4("tact coa"), true);
    assert_eq!(assignment4("tact"), false);

    assert_eq!(assignment5("abc", "abd"), true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        assert_eq!(assignment1("abc"), true);
        assert_eq!(assignment1("test"), false);

        assert_eq!(assignment2("abc", "cba"), true);
        assert_eq!(assignment2("a", "b"), false);

        assert_eq!(assignment3("a b"), "a%20b");

        assert_eq!(assignment4("tact coa"), true);
        assert_eq!(assignment4("tact"), false);

        assert_eq!(assignment5("abc", "abd"), true);
    }
}
