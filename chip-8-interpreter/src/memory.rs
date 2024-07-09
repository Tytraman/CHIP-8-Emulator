pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: u16) -> Self {
        Self {
            data: vec![0; size as usize],
        }
    }

    pub fn read8(&self, offset: u16) -> Result<u8, String> {
        if offset as usize >= self.data.len() {
            return Err(format!("trying to read offset {offset} of a {} length memory", self.data.len()));
        }

        Ok(self.data[offset as usize])
    }

    pub fn read16(&self, offset: u16) -> Result<u16, String> {
        if self.data.len() < 2 {
            return Err("this memory doesn't have enough space to read from".to_string());
        }

        if offset as usize >= self.data.len() - 1 {
            return Err(format!("trying to read offset {offset} of a {} length memory", self.data.len()));
        }

        let msb = self.data[offset as usize];
        let lsb = self.data[(offset + 1) as usize];

        let value = (lsb as u16) | ((msb as u16) << 8);

        Ok(value)
    }

    pub fn write8(&mut self, offset: u16, value: u8) -> Result<(), String> {
        if offset as usize >= self.data.len() {
            return Err(format!("trying to write a 8-bits value at offset {offset} of a {} length memory", self.data.len()));
        }

        self.data[offset as usize] = value;

        Ok(())
    }

    pub fn write16(&mut self, offset: u16, value: u16) -> Result<(), String> {
        if self.data.len() < 2 {
            return Err("this memory doesn't have enough space to store this data".to_string());
        }

        if offset as usize >= self.data.len() - 1 {
            return Err(format!("trying to write a 16-bits value at offset {offset} of a {} length memory", self.data.len()));
        }

        let msb = ((value >> 8) & 0xFF) as u8;
        let lsb = (value & 0xFF) as u8;

        self.data[offset as usize] = msb;
        self.data[(offset + 1) as usize] = lsb;

        Ok(())
    }

    pub fn write8_range(&mut self, start: u16, end: u16, content: &[u8]) -> Result<(), String> {
        if start == end {
            return Ok(());
        }

        if start > end {
            return Err("'start' parameter cannot be superior to 'end' parameter".to_string());
        }

        if end as usize >= self.data.len() {
            return Err("range is out of bound from memory length".to_string());
        }

        for (dest, from) in self.data[start as usize..end as usize].iter_mut().zip(content) {
            *dest = *from;
        }

        Ok(())
    }
}