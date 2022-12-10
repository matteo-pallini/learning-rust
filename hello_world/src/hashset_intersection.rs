use std::collections::HashSet;

pub fn run() {
    let mut a = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    let mut b = HashSet::new();
    b.insert(2);
    b.insert(3);
    b.insert(4);
    let mut c = HashSet::new();
    c.insert(3);
    c.insert(4);
    c.insert(5);
    let bar = vec![&a, &b, &c];
    let first: HashSet<u32> = a.clone();
    let foo = bar.iter().fold(first, |acc, item| {
        HashSet::from_iter(item.iter().filter(|e| acc.contains(e)).map(|v| *v))
    });
    let v: Vec<_> = foo.into_iter().collect();
    println!("hashsets intersection is {:?}", v)
}
