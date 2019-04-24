/// https://exercism.io/my/solutions/fe22ce70d1784831bb8703d6812c0f9d
pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.contains(&0){
            return None;
        }
        //https://en.wikipedia.org/wiki/Triangle_inequality
        else if !(sides[0] + sides[1] > sides[2]) || !(sides[0] + sides[2] > sides[1]) || !(sides[1] + sides[2] > sides[0]) {
            return None;
        }

        Some(Triangle {
                a: sides[0], 
                b: sides[1], 
                c: sides[2]})
    }

    //https://www.cut-the-knot.org/triangle/Triangles.shtml
    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c && self.b == self.c 
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        (self.a == self.b) || (self.b == self.c) || (self.a == self.c)
    }
}
