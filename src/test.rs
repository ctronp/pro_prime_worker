#[cfg(test)]
mod integration_tests {
    use std::collections::HashMap;

    use actix_web::http;
    use reqwest::header::HeaderMap;

    use crate::entities::{Input, Output};
    use crate::statics::get_port_u16;

    #[actix_web::test]
    async fn test_index() {
        crate::statics::debug_initialize().await;

        let resp = reqwest::get(format!("http://0.0.0.0:{}/", get_port_u16()))
            .await
            .unwrap();
        pretty_assertions::assert_eq!(http::StatusCode::OK, resp.status());
    }

    #[actix_web::test]
    async fn small_primes() {
        crate::statics::debug_initialize().await;

        let mut header = HeaderMap::new();
        header.insert("Secret", crate::statics::get_secret().parse().unwrap());

        let res = reqwest::Client::new()
            .post(format!("http://0.0.0.0:{}/primes", get_port_u16()))
            .json(&Input {
                values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            })
            .headers(header)
            .send()
            .await
            .unwrap();

        let output = res.json::<Output>().await.unwrap();
        assert_eq!(
            output.values,
            HashMap::from([
                ("1".to_string(), "No".to_string()),
                ("2".to_string(), "Yes".to_string()),
                ("3".to_string(), "Yes".to_string()),
            ])
        )
    }
}
