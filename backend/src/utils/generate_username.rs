pub fn generate_username(restaurant_name: &str) -> String {
    restaurant_name
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        + "_InitialUser"
}