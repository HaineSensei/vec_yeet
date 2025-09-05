pub trait YeetExtn {
    unsafe fn yeet(&mut self) -> Option<()>;

    fn careful_yeet(&mut self) -> Option<()>;
}

impl<T> YeetExtn for Vec<T> {
    unsafe fn yeet(&mut self) -> Option<()> {
        match self.len() {
            0 => None,
            l => {
                unsafe {
                    self.set_len(l-1);
                }
                Some(())
            },
        }
    }

    fn careful_yeet(&mut self) -> Option<()>{
        self.pop()?;
        Some(())
    }
}
