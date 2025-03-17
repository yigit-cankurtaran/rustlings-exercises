fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42); // push to y and move y out of scope
        let z = &mut x; // moving z here because we can't have both active
                        // they're both references to x, only one mutable reference can exist at any time
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
