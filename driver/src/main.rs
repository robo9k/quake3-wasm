fn main() -> Result<(), Box<dyn std::error::Error>> {
    module::dll_entry()?;

    Ok(())
}
