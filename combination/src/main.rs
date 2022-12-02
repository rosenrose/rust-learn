fn main() {
    let mut c = combination(1..=4, 1);
    println!("{}\n{c:?}\n", c.len());

    c = combination(1..=4, 2);
    println!("{}\n{c:?}\n", c.len());

    c = combination(2..=5, 3);
    println!("{}\n{c:?}\n", c.len());

    c = combination(1..=5, 2);
    println!("{}\n{c:?}\n", c.len());

    c = combination(1..=5, 3);
    println!("{}\n{c:?}\n", c.len());

    c = combination(5..=10, 3);
    println!("{}\n{c:?}\n", c.len());

    c = combination(5..=10, 4);
    println!("{}\n{c:?}\n", c.len());

    c = combination(1..=10, 3);
    println!("{}\n{c:?}\n", c.len());

    c = combination(20..=29, 7);
    println!("{}\n{c:?}\n", c.len());
}

fn combination<I>(iter: I, n: i32) -> Vec<Vec<i32>>
where
    I: Iterator<Item = i32>,
{
    let mut result = Vec::new();
    let mut seed = vec![vec![]];

    for num in iter {
        for i in 0..seed.len() {
            let mut t = seed[i].clone();
            t.push(num);

            if t.len() == n as usize {
                result.push(t.clone());
            }

            if t.len() < n as usize {
                seed.push(t);
            }
        }
    }

    result
}
