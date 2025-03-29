mod splitter;

#[cfg(test)]
mod tests {
    use super::*;
    use splitter::*;

    #[test]
    fn line_split_test() {
        let code_str: String = "arg arg;
arg arg;
        ".to_string();

        let lines_vec: Vec<String> = split_lines(code_str, ';');

        assert_eq!(lines_vec, vec!["arg arg;".to_string(), "arg arg;".to_string()]);
    }

    #[test]
    fn func_split_test() {
        let code_str: String = "arg arg;
arg arg;
        ".to_string();

        let lines_vec: Vec<String> = split_lines(code_str, ';');

        let args_vec: Vec<String> = split_func(lines_vec[0].clone(), ' ');

        assert_eq!(args_vec, vec!["arg".to_string(), "arg".to_string()]);
    }
}
