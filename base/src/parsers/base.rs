use regex::Regex;

use crate::pattern::RegexPattern;

use super::interpolated_str::InterpolatedStr;

/// Perform a basic replacement of the interpolation items based on the given pattern
/// example:
/// ```
/// let p = Regex::new(r"\{(\w+)\}").unwrap();
/// let mut my_string = "Hello {name}, there are {count} items in your cart".to_string();
/// let parsed = basic_parser(&mut my_string, p);
/// assert_eq!(parsed.txt, "Hello {0}, there are {1} items in your cart");
/// assert_eq!(parsed.items, vec!["{name}", "{count}"]);
/// ```
pub fn basic_parser(s: &mut String, p: RegexPattern) -> InterpolatedStr {
    let mut find_from = 0;
    let p: Regex = p.into();
    let total_count = p.captures_iter(s).count();
    let mut items = Vec::with_capacity(total_count);

    for i in 0..total_count {
        let match_ = p.find_at(s, find_from).unwrap();
        find_from = if items.len() > 10 {
            // after 10 items, the index will be 4 characters long { + num + } = 4
            match_.end() - 4
        } else {
            match_.end() - 3
        };
        let item = match_.as_str().to_owned();
        items.push(item);
        s.replace_range(match_.range(), &format!("{{{}}}", i));
    }

    InterpolatedStr { txt: s, items }
}

mod tests {

    #[test]
    fn basic_parser_test() {
        let p = super::RegexPattern::Default;
        let mut my_string = "Hello {name}, there are {count} items in your cart".to_string();
        let parsed = super::basic_parser(&mut my_string, p);
        assert_eq!(parsed.txt, "Hello {0}, there are {1} items in your cart");
        assert_eq!(parsed.items, vec!["{name}", "{count}"]);

        let mut my_string = "Hello {name}, {there} are {count} {items} in your {cart}. Please {check} page {for} more {details}. Also {other} contains {notification}. Additional info @{location} available at {here}".to_string();
        let parsed = super::basic_parser(&mut my_string, p);
        assert_eq!(parsed.txt, "Hello {0}, {1} are {2} {3} in your {4}. Please {5} page {6} more {7}. Also {8} contains {9}. Additional info @{10} available at {11}");
        assert_eq!(
            parsed.items,
            vec![
                "{name}",
                "{there}",
                "{count}",
                "{items}",
                "{cart}",
                "{check}",
                "{for}",
                "{details}",
                "{other}",
                "{notification}",
                "{location}",
                "{here}"
            ]
        );
    }
}
