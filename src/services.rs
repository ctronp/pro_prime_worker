use rug::Integer;
use tokio::join;
use crate::database::answer_db;
use crate::entities::{Input, Output};

// Return Output with char {Y, N, P} if is prime, is not, or probably
#[inline]
fn prime_b10(str_value: String) -> String {
    match str_value.parse::<Integer>() {
        Ok(n) => {
            match n.is_probably_prime(51) {
                rug::integer::IsPrime::Yes => "Yes".to_string(),
                rug::integer::IsPrime::Probably => "Probably".to_string(),
                rug::integer::IsPrime::No => "No".to_string()
            }
        }
        Err(_) => "Value Error".to_string()
    }
}

pub async fn process_value(value: String) -> String {
    let val_ref = &value[..];
    if let Some(query) = answer_db(val_ref).await {
        return query
    } else {
        prime_b10(value)
    }
}
