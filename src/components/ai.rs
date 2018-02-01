
struct BasicMonster {
    owner: &Entity<'a>
}

impl BasicMonster {
    pub fn take_turn(&self) {
        println!("The {} wonders when it will get to move.");
    }
}