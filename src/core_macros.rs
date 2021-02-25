/// The "dec!" macro for Decimal.

#[macro_export]

macro_rules! dec {
    ($e: expr) => {
        Decimal::from_str($e.to_string().as_str()).unwrap()
    };
}
