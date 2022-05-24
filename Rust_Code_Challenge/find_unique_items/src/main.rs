use std::vec::Vec;

fn unique(mut xs: Vec<i32>) ->Vec<i32> {
    let mut vec = Vec::new();
    let mut found = false;
    if xs.len() == 0 {
        return vec;
    }
    xs.sort();
    for j in 0..(xs.len()){
        for i in (j+1)..xs.len(){
            if xs[j] == xs[i] {
                println!("{}", xs[i]);
                found = true;
                break;
            }
        }
        if !found {
            vec.push(xs[j]);
        } else {
            found = false;
        }
    }
    vec
}


fn main() {
    let input = vec![1, 1, 2, 3, 4, 5, 6, 4,8];
    let answer = unique(input);
    println!("{:?}", answer);

}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output: Vec<i32> = unique(input);

    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}