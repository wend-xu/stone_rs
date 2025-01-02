/// 字符串按行拼接
///
pub fn lines_concat(str_vec: Vec<String>) -> String {
    lines_concat_with_divide(str_vec, None)
}
pub fn lines_concat_with_divide(str_vec: Vec<String>, op_divide: Option<&str>) -> String {
    let mut concat_vec: Vec<String> = Vec::new();
    let mut max_width: usize = 0;
    let divide = if let Some(divide) = op_divide { divide } else { "" };

    str_vec.iter().for_each(|right| {
        let right_lines = right.lines();
        let mut line_index = 0;
        let mut max_width_inner = max_width;

        for right_line in right_lines {
            match concat_vec.get_mut(line_index) {
                Some(left_line) => {
                    if left_line.len() < max_width {
                        let full = repeat_char(' ', max_width - left_line.len());
                        left_line.push_str(full.as_str());
                    }
                    let line_concat = format!("{}{}{}", left_line, divide, right_line);
                    max_width_inner = max_width_inner.max(line_concat.len());
                    concat_vec[line_index] = line_concat;
                }
                None => {
                    let mut line_concat = right_line.to_string();
                    if max_width > 0 {
                        line_concat = format!("{}{}{}", repeat_char(' ', max_width), divide, right_line);
                    }
                    max_width_inner = max_width_inner.max(line_concat.len());
                    concat_vec.push(line_concat);
                }
            };
            line_index += 1;
        }
        max_width = max_width_inner;
    });

    concat_vec.iter_mut().for_each(|line| if line.len() < max_width {
        line.push_str(repeat_char(' ', max_width - line.len()).as_str());
    });
    concat_vec.join("\n")
}


pub fn wrapper_node_name(node_name: String) -> String {
    // 保证包装后名称长度是单数
    let node_name = if node_name.len() % 2 == 0 { format!("| {}  |", node_name) } else { format!("| {} |", node_name) };

    let node_len = node_name.len();
    let left_space = node_len / 2;

    let boundary = "-".repeat(node_len);
    let point1 = format!("{}{}{}", " ".repeat(left_space), "^", " ".repeat(node_len / 2));
    let point2 = format!("{}{}{}", " ".repeat(left_space), "|", " ".repeat(node_len / 2));

    format!("{}\n{}\n{}\n{}\n{}", point1, point2, boundary, node_name, boundary)
}

pub fn wrapper_sub_block(root: String, sub_block: String) -> String {
    let mut res = sub_block;

    let mut sub_block_lines = res.lines();
    if let Some(block_first_line) = sub_block_lines.next() {
        res = format!("{}\n{}", _gen_root_join(root.as_str(), block_first_line), res);
    }
    res
}

fn _gen_root_join(root: &str, line: &str) -> String {
    let width = line.len();
    let first_point = line.find("^").unwrap_or(0);
    let mut root_link_point = first_point.clone();
    let last_point = line.rfind("^").unwrap_or(0);
    let left_space = repeat_char(' ', width / 2);
    let right_space = repeat_char(' ', width - width / 2 - 1);

    let join_area =
        if first_point != 0 && first_point != last_point {
            root_link_point = width / 2;
            format!("{}{}{}\n{}{}{}\n{}{}{}",
                    left_space, "^", right_space,
                    left_space, "|", right_space,
                    repeat_char(' ', first_point),
                    repeat_char('-', last_point - first_point),
                    repeat_char(' ', width - last_point))
        } else {
            String::new()
        };

    let root_width_half = root.lines().next().unwrap().len() / 2;
    let left_space = repeat_char(' ', root_link_point - root_width_half);
    let right_space = repeat_char(' ', width - (root_link_point + root_width_half));

    let root_with_space = lines_concat(vec![left_space, root.to_string(), right_space]);
    if join_area.len() > 0 {
        format!("{}\n{}", root_with_space, join_area)
    } else {
        root_with_space
    }
}

pub fn repeat_char(char: char, count: usize) -> String {
    char.to_string().repeat(count)
}