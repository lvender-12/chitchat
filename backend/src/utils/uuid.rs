use lav_seed::{Generator, LavError};

pub fn generate_uuid(count: u64) -> Result<String, LavError> {
    let mut generator = Generator::build(count)
        .min_seed(10_000_000)
        .max_seed(999_999_999)
        .build()?;

    let id = generator.generate()?;
    Ok(id.to_string())
}
