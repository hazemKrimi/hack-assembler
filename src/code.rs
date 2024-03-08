pub fn decimal_to_binary(decimal: &i32) -> String {
    String::from(format!("{decimal:015b}"))
}

// fn translate_dest(dest: &String) -> String {
//     let cloned = dest.clone();

//     cloned
// }