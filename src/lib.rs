use uuid::Uuid;

/// Function to generate a compound UUID.
pub fn v7v4_xuid() -> String {

    // Create a v4 UUID and convert to string
         
    let v4 = Uuid::new_v4().simple();
    let v4_str = v4.to_string();

    // Create a v7 UUID and convert to string 
    let v7 = Uuid::now_v7().simple();
    let v7_str = v7.to_string();

    // Concatenate the strings with a separator
    format!("{}{}", v7_str, v4_str)
}
