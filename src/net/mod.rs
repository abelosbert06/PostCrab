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

pub async fn send_request(
    url: &str,
    req_type: &RequestType,
    message_body: &Option<String>,
    content_type: &ContentType,
) -> Result<String, reqwest::Error> {
    let request = format!(
        "{:?} {} {:?} {}",
        req_type,
        url,
        content_type,
        message_body.as_deref().unwrap_or("None")
    );
    println!("{}", request);

    let req_type_string: String;
    match *content_type {
        ContentType::Text => req_type_string = "text/plain".to_string(),
        ContentType::Json => req_type_string = "application/json".to_string(),
        ContentType::Form => req_type_string = "application/x-www-form-urlencoded".to_string(),
        ContentType::Xml => req_type_string = "application/xml".to_string(),
    }

    let client = reqwest::Client::new();
    match *req_type {
        RequestType::Get => {
            let res = client.get(url).send().await?.text().await?;

            print!("{}", res);

            Ok(res)
        }

        RequestType::Delete => {
            let res = client.delete(url).send().await?.text().await?;
            print!("{}", res);
            Ok(res)
        }

        RequestType::Post => {
            let body = message_body.as_deref().unwrap_or("");
            let res = client
                .post(url)
                .header(reqwest::header::CONTENT_TYPE, req_type_string)
                .body(body.to_string())
                .send()
                .await?
                .text()
                .await?;
            print!("{}", res);
            Ok(res)
        }

        RequestType::Patch => {
            let body = message_body.as_deref().unwrap_or("");
            let res = client
                .patch(url)
                .header(reqwest::header::CONTENT_TYPE, req_type_string)
                .body(body.to_string())
                .send()
                .await?
                .text()
                .await?;
            print!("{}", res);
            Ok(res)
        }

        RequestType::Put => {
            let body = message_body.as_deref().unwrap_or("");
            let res = client
                .put(url)
                .header(reqwest::header::CONTENT_TYPE, req_type_string)
                .body(body.to_string())
                .send()
                .await?
                .text()
                .await?;
            print!("{}", res);
            Ok(res)
        }
    }
}
