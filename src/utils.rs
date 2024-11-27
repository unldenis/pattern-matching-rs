pub trait StringExt {
    fn substring(&self, start : usize, end : usize) -> Self;
}

impl StringExt for String {
    fn substring(&self, start : usize, end : usize) -> Self {
        self.chars().skip(start).take(end - start).collect()
    }
}