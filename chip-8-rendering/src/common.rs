use std::ffi::CString;

pub struct ChipString;

impl ChipString {
    pub fn i8_array_to_string(arr: &[i8]) -> String {
        // Créer un CString nécessite un tableau qui n'a pas d'octet valant 0,
        // il faut donc filtrer la collection.
        let message: Vec<u8> = arr
            .iter()
            .take_while(|&c| *c != 0)
            .map(|&c| c as u8)
            .collect();

        let message = CString::new(message).unwrap();

        message.into_string().unwrap()
    }
}
