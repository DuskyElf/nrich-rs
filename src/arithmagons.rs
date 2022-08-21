// Arithmagons - https://nrich.maths.org/2670

// Three corners -> x, y, z
// Three edges -> p, q, r
// Edge = sum of it's corners
// Corner = (Sum of it's edges - third edge) / 2

pub fn calc_edges(x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    let p = x + y;
    let q = y + z;
    let r = z + x;
    (p, q, r)
}

pub fn calc_corners(p: i32, q: i32, r: i32) -> (i32, i32, i32) {
    let x = (p + r - q) / 2;
    let y = (p + q - r) / 2;
    let z = (q + r - p) / 2;
    (x, y, z)
}

#[test]
fn calc_edges_test() {
    assert_eq!(calc_edges(11, 2, 4), (13, 6, 15));
}

#[test]
fn calc_corners_test() {
    assert_eq!(calc_corners(22, 16, 18), (12, 10, 6));
}
