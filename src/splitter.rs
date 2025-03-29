pub fn split_lines(code: String, line_end_char: char) -> Vec<String> {
    let lines_str: Vec<&str> = code.split(line_end_char).collect();
    let mut lines: Vec<String> = vec![];

    for line in lines_str {
        lines.push(line.to_string());
    }

    return lines;
}

pub fn split_func(line: String, arg_seperator: char) -> Vec<String> {
    let args_str: Vec<&str> = line.split(arg_seperator).collect();
    let mut args: Vec<String> = vec![];

    for line in args_str {
        args.push(line.to_string());
    }

    return args;
}