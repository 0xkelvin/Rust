let a = [1, 2, 3];

let mut iter = a.iter().skip(2);

assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);