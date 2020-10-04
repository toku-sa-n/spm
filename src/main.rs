use tempfile::TempDir;

fn main() -> Result<(), std::io::Error> {
    let tmp_dir = TempDir::new()?;

    println!("Created: {:?}", tmp_dir);

    Ok(())
}
