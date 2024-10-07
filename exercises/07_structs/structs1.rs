struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
    // Experimenting with the ColorRegularStruct
    let red_color = ColorRegularStruct { red: 255, green: 0, blue: 0 };
    println!("Red Color - R: {}, G: {}, B: {}", red_color.red, red_color.green, red_color.blue);

    // Experimenting with the ColorTupleStruct
    let blue_color = ColorTupleStruct(0, 0, 255);
    println!("Blue Color - R: {}, G: {}, B: {}", blue_color.0, blue_color.1, blue_color.2);

    // Experimenting with the UnitStruct
    let unit = UnitStruct;
    println!("{:?} is a unit struct!", unit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =
        let green: ColorRegularStruct = ColorRegularStruct { red: 0, green: 255, blue: 0 };
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
