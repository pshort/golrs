
pub struct t1 {
    truth: bool
}

impl t1 {
    pub fn new(input: bool) -> t1 {
        t1 {
            truth: input
        }
    }

    pub fn get(&self) -> bool {
        self.truth
    }

    
}