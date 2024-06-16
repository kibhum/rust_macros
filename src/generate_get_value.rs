// ident: an identifier or keyword. For example, the start of a function
// declaration (fn hello) has a keyword followed by an identifier, and we
// can capture them both by using ident twice

macro_rules! generate_get_value {
    ($struct_type:ident) => {
        generate_get_value!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}
