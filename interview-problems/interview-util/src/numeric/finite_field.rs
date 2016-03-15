use numeric::Field;

pub trait FiniteField: Field {
    fn max() -> Self;
    fn min() -> Self;
}
