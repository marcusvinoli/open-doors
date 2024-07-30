use serde::{Serialize, Deserialize};

#[derive( Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum DataType {
    #[default]
    Empty,
    Any(String), // Rendered as String.
    Boolean(bool), // Boolean value rendered as "Yes" for true and "No" for false
    Integer(isize), // Any integer number.
    IntegerPositive(usize), // Only positive number {}
    IntegerNegative(usize), // Only negative number. Rendered with "-"
    Float(f32), // Floating point number
    Option(Vec<String>), // Sigle option 
    Options(Vec<String>), // Multiple options
    NullableAny(Option<String>), // Rendered as String. Not obrigatory.
    NullableBoolean(Option<bool>), // Boolean value rendered as "Yes" for true and "No" for false. Not obrigatory.
    NullableInteger(Option<isize>), // Any integer number. Not obrigatory.
    NullableIntegerPositive(Option<usize>), // Only positive number. Not obrigatory.
    NullableIntegerNegative(Option<usize>), // Only negative number. Rendered with "-". Not obrigatory.
    NullableFloat(Option<f32>), // Floating point number. Not obrigatory.
    NullableOption(Option<Vec<String>>), // Sigle option. Not obrigatory.
    NullableOptions(Option<Vec<String>>), // Multiple options. Not obrigatory.
}

#[derive( Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Fields {
    attribute: String,
    kind: DataType,
}

#[derive( Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Template {
    pub fields: Vec<Fields>
}
