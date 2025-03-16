fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3; // shadowing, this isn't the old number just another variable also named number
    println!("Number plus two is: {}", number + 2); // shadowing enables this line to run properly as well
}
