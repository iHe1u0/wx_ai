use std::str::FromStr;

pub fn get_env<T: ToString>(key: &str, default_value: T) -> String {
    dotenv::var(key).unwrap_or(default_value.to_string())
}

#[allow(dead_code)]
pub fn get_env_exact<T: FromStr>(key: &str, default_value: T) -> T
where
    T::Err: std::fmt::Debug, // This ensures the `FromStr` implementation can fail and we can debug it
{
    dotenv::var(key)
        .ok()
        .and_then(|val| val.parse::<T>().ok()) // Try to parse the environment variable
        .unwrap_or(default_value) // If parsing fails, return the default value
}
