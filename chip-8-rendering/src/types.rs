use std::any::Any;

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }
}

pub struct UserData {
    pub(crate) data: Option<Box<dyn Any>>,
}

impl UserData {
    pub fn new(value: Box<dyn Any>) -> Self {
        Self { data: Some(value) }
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        match &self.data {
            Some(t) => t.downcast_ref(),
            None => None,
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        // Le `mut` dans '&mut self.data' est nécessaire pour pouvoir récupérer le cast mutable
        // juste après.
        if let Some(t) = &mut self.data {
            return t.downcast_mut();
        }

        None
    }
}
