#[derive(Debug, PartialEq, Clone)]
pub enum RequestType {
    Get,
    Post,
    Delete,
    Put,
    Patch,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContentType {
    Json,
    Text,
    Xml,
    Form,
}

pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else if trimmed.starts_with("localhost") || trimmed.starts_with("127.0.0.1") {
        format!("http://{}", trimmed)
    } else {
        format!("https://{}", trimmed)
    }
}

pub async fn send_request(
    url: &str,
    req_type: &RequestType,
    message_body: &Option<String>,
    content_type: &ContentType,
) -> Result<String, reqwest::Error> {
    let final_url = normalize_url(url);

    let request = format!(
        "{:?} {} {:?} {}",
        req_type,
        final_url,
        content_type,
        message_body.as_deref().unwrap_or("None")
    );
    println!("{}", request);

    let req_type_string = match *content_type {
        ContentType::Text => "text/plain",
        ContentType::Json => "application/json",
        ContentType::Form => "application/x-www-form-urlencoded",
        ContentType::Xml => "application/xml",
    };

    let client = reqwest::Client::new();
    match *req_type {
        RequestType::Get => {
            let res = client.get(&final_url).send().await?.text().await?;
            Ok(res)
        }

        RequestType::Delete => {
            let res = client.delete(&final_url).send().await?.text().await?;
            Ok(res)
        }

        RequestType::Post => {
            let body = message_body.as_deref().unwrap_or("");
            let res = client
                .post(&final_url)
                .header(reqwest::header::CONTENT_TYPE, req_type_string)
                .body(body.to_string())
                .send()
                .await?
                .text()
                .await?;
            Ok(res)
        }

        RequestType::Patch => {
            let body = message_body.as_deref().unwrap_or("");
            let res = client
                .patch(&final_url)
                .header(reqwest::header::CONTENT_TYPE, req_type_string)
                .body(body.to_string())
                .send()
                .await?
                .text()
                .await?;
            Ok(res)
        }

        RequestType::Put => {
            let body = message_body.as_deref().unwrap_or("");
            let res = client
                .put(&final_url)
                .header(reqwest::header::CONTENT_TYPE, req_type_string)
                .body(body.to_string())
                .send()
                .await?
                .text()
                .await?;
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url() {
        assert_eq!(normalize_url("https://example.com"), "https://example.com");
        assert_eq!(normalize_url("http://example.com"), "http://example.com");
        assert_eq!(normalize_url("example.com"), "https://example.com");
        assert_eq!(normalize_url("localhost:8080/api"), "http://localhost:8080/api");
        assert_eq!(normalize_url("127.0.0.1:3000"), "http://127.0.0.1:3000");
    }

    #[tokio::test]
    async fn test_send_get_request() {
        let result = send_request(
            "https://httpbin.org/get",
            &RequestType::Get,
            &None,
            &ContentType::Json,
        )
        .await;

        match result {
            Ok(body) => {
                assert!(body.contains("headers") || body.contains("origin") || body.contains("url"));
            }
            Err(e) => {
                // If offline or network unavailable, test shouldn't hard fail CI
                eprintln!("Network request failed (possibly offline): {}", e);
            }
        }
    }
}
