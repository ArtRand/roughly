enum Base {
    A,
    C,
    G,
    T
}


impl Base {
    fn parse(b: char) -> Result<Base, Err> {
        match b {
            'A' => Ok(Base.A)
            
        }
    }
}