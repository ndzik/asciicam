pub struct ASCIITable {
    pub table: Vec<u8>,
    pub init_len: usize,
    pub is_reversed: bool,
}

impl ASCIITable {
    pub fn inc_threshold(&mut self) {
        if self.is_reversed {
            self.table.insert(0, ' ' as u8);
        } else {
            self.table.push(' ' as u8);
        }
    }

    pub fn dec_threshold(&mut self) {
        if self.table.len() == self.init_len {
            return;
        }

        if self.is_reversed {
            self.table.remove(0);
        } else {
            self.table.pop();
        }
    }

    pub fn reset_threshold(&mut self) {
        if self.is_reversed {
            let r = self.table.len() - self.init_len;
            self.table.drain(0..r);
        } else {
            self.table.truncate(self.init_len);
        }
    }

    pub fn threshold(&self) -> usize {
        self.table.len() - self.init_len
    }
}
