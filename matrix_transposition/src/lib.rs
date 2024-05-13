// Define a struct named Matrix as a tuple of 2 tuples. The nested tuple will contain 2 i32s.
#[derive(Debug, PartialEq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

// ( a b )   __ transposition __>   ( a c )
// ( c d )                          ( b d )


pub fn transpose(m: Matrix) -> Matrix {
    // Return a new Matrix with the
    // rows and columns transposed.
    Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
}
