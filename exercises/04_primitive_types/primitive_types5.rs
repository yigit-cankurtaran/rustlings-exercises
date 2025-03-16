fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    let (name, age) = cat; // destructuring, first is name, 2nd is age

    println!("{name} is {age} years old");
}
