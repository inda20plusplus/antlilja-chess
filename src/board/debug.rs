use super::Board;

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..8 {
            let start = y * 8;
            for x in 0..8 {
                write!(f, "{:?}", self.0[start + x])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
