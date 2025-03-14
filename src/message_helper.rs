pub struct MessageHelper;

impl MessageHelper {
    pub fn convert_message_to_integers(message: &str) -> Vec<i64> {
        message.chars().map(|c| c as i64).collect()
    }

    pub fn convert_integers_to_message(integers: &[i64]) -> String {
        integers.iter().map(|&i| char::from_u32(i as u32).unwrap_or('?')).collect()
    }
}