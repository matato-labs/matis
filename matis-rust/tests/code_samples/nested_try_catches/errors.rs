fn ordinary() -> Result<(), Error> {
    if ready() {
        work(work()?)?;
    }
    Ok(())
}
