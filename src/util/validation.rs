pub trait Validatable {
    fn validate(&self) -> Result<(), crate::error::Error> {
        Ok(())
    }
}
