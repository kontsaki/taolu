use day1::{find_pair, find_triplet, get_expenses};

fn main() {
    let expenses = get_expenses("input");
    let pair = find_pair(&expenses, 2020);
    if let Some((x, y)) = pair {
        println!("{}", x * y);
    }

    let triplet = find_triplet(&expenses, 2020);
    if let Some((x, y, z)) = triplet {
        println!("{}", x * y * z);
    }
}
