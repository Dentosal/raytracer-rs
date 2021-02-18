#[allow(non_camel_case_types)]
pub type float = f32;

pub fn approx_eq(a: float, b: float) -> bool {
    (a - b) < 0.000_01
}
