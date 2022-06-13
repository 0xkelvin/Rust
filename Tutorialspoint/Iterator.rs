fn main() {
    let a = [10, 20, 30];

    let mut iter = a.iter();
    println!("{:?}", iter);

    for data in iter {
        println!("{}\t", data);
    }
// the following 3 methods return an iterator object from a collection,
// where T represents the elements in a collection
// 1. iter() gives an iterator over &T
// 2. into_iter() gives an iterator over T
// 3. iter_mut() gives an iterator over &mut T


    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
}