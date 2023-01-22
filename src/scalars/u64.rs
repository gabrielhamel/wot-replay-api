use juniper::{ParseScalarValue, ParseScalarResult, Value};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct U64(pub u64);

#[juniper::graphql_scalar(description = "U64")]
impl<S> GraphQLScalar for U64
    where
        S: ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    fn from_input_value(v: &InputValue) -> Option<U64> {
        v
            .as_string_value()
            .and_then(|string| string.parse::<u64>()
                .ok()
                .and_then(|res|
                    Some(U64(res))))
    }

    fn from_str(value: ScalarToken) -> ParseScalarResult<S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}
