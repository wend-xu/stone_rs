pub fn get_from_captures<'a>(captures: &'a regex::Captures, names: &'a [&'a str]) -> Option<(&'a str, &'a str, usize, usize)> {
    let mut str_op: Option<(&'a str, &'a str, usize, usize)> = None;

    for name in names {
        if let Some(cap) = captures.name(name) {
            str_op = Some((name, cap.as_str(), cap.start(), cap.end()));
            break;
        }
    }
    if str_op.is_none() { None } else { str_op }
}