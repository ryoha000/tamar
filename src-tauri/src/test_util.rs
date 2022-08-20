#[cfg(test)]
pub fn random_string() -> String {
    use ulid::Ulid;

    Ulid::new().to_string()
}
