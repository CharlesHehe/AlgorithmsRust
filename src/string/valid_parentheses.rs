use std::char;

pub fn is_valid(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let r = '(';
    for index in (0..chars.len() as usize).step_by(2) {
        match chars[index] {
            '(' => match chars[index + 1] {
                ')' => continue,
                _ => return false,
            },
            '[' => match chars[index + 1] {
                ']' => continue,
                _=>(),
            },
            '{' => match chars[index + 1] {
                '}' => continue,
                _=>(),
            },
            _=>return false,
        }
    }
    true
}

pub fn is_valid2(s: String) -> bool {
    let mut stack = Vec::new();
    for i in s.chars(){
        match i {
            '{'=>stack.push('}'),
            '('=>stack.push(')'),
            '['=>stack.push(']'),
            _=>if Some(i) != stack.pop(){return false}
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn works2() {
        assert_eq!(is_valid2(String::from("({})")), false);
    }
}
