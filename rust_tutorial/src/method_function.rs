

pub struct Coordinate{
    pub x_axis: u8,
    pub y_axis: u8,
}

impl Coordinate{
    // self => object reference (Method)
    pub fn sum(&self) -> u8 {
        return self.x_axis + self.y_axis;
    }

    // No reference => Function 
    pub fn new(x_axis: u8, y_axis: u8) -> Coordinate {
        return Coordinate {
            x_axis,
            y_axis,
        };
    }

}


