struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u32, // unsigned ints because we expect positive values
    green: u32,
    blue: u32,
}

/* TODO: Add the fields that the test `tuple_structs` expects */
struct ColorTupleStruct(u32, u32, u32);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
    let red: ColorTupleStruct = ColorTupleStruct(255, 0, 0);
    println!("{}", red.0);
    println!("{}", red.1);
    println!("{}", red.2);

    let green: ColorRegularStruct = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };

    println!("{}", green.red);
    println!("{}", green.green);
    println!("{}", green.blue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =

        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // let green =

        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =

        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
