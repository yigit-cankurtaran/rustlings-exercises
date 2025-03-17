// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // shadow vec, take ownership of its memory, make it mutable

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let my_vec: Vec<i32> = vec![1, 2, 3];
    let new_vec = fill_vec(my_vec); // new variable, take ownership of my_vec, push it out of scope
    for element in new_vec {
        println!("{element}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
