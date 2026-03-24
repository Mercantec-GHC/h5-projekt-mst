use super::error::Error;


pub trait System {

    fn on_add(&self) -> Result<(), Error> {
        Ok(())
    }
    
    fn on_update(&self) -> Result<(), Error> {
        Ok(())
    }

    fn on_remove(&self) -> Result<(), Error> {
        Ok(())
    }
}