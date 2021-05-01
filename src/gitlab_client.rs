use reqwest;
use crate::coffee_config::Gitlab;

#[derive(Debug)]
pub struct MergeRequest {
    pub title: String,
    pub author: String,
    pub date: String,
    pub upvotes: i8,
    pub url: String,
}

impl MergeRequest {
    pub async fn get_open(config: &Gitlab) -> Result<Self, Box<dyn std::error::Error>> {
        let response = reqwest::get(&config.base_url)
            .await?;
        println!("Settings {:?}", config);
        println!("Response {:?}", response);

        Ok(MergeRequest {
            title: "self".to_string(),
            author: "test".to_string(),
            date: "today".to_string(),
            upvotes: 2,
            url: "https://gitlab.com/epqp".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use super::*;
    
    #[tokio::test]
    async fn it_works() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET);
            then.status(200)
                .header("Content-Type", "text/html; charset=UTF-8")
                .body("pong");
        });
        let config = Gitlab::new(server.base_url().to_string(), "TOKEN".to_string());
        let response = MergeRequest::get_open(&config).await;
        println!("Hello {:?}", response);
        mock.assert();
        assert_eq!(2, 2);
    }
}