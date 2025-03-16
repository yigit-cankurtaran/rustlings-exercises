fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4]; // slices need a reference
                                   // we indicate that we're not taking ownership of data, just getting a read-only view
                                   // keeping the original "a" array intact and owned by its scope.

        assert_eq!([2, 3, 4], nice_slice);
    }
}
