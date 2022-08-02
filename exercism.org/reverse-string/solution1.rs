pub fn reverse(input: &str) -> String {
    if input == "" {
        return "".to_string();
    } else {
        let mut vec_s1 = Vec::new();
        for token in input.chars() {
            vec_s1.push(token);
        }
        vec_s1.reverse();
        let str: String = vec_s1.into_iter().collect();
        return str
    }
}