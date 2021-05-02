use reqwest;
use serde::Deserialize;
use crate::coffee_config::Gitlab;

#[derive(Debug, Deserialize, PartialEq)]
pub struct MergeRequest {
    pub title: String,
    pub author: Author,
    pub created_at: String,
    pub upvotes: i8,
    pub web_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
}

impl MergeRequest {
    pub async fn get_open(config: &Gitlab) -> Result<Self, Box<dyn std::error::Error>> {
        let response = reqwest::get(&config.base_url)
            .await?
            .json::<MergeRequest>()
            .await?;
        
        Ok(response)
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
        let data = r#"{
            "title": "Whatever",
            "author": {
                "name": "Author Name"
            },
            "created_at": "2021-05-01T00:00:00Z",
            "upvotes": 1,
            "web_url": "https://test.gitlab.com/projects/x/mrs/1",
            "other_field": "Shouldn't be in the DS"
        }"#;
        
        let mock = server.mock(|when, then| {
            when.method(GET);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(data);
        });
        let config = Gitlab::new(server.base_url().to_string(), "TOKEN".to_string());
        let response = MergeRequest::get_open(&config).await;
        // Exactly one HTTP method that matched the mock requirements
        mock.assert();
        assert_eq!(response.is_ok(), true);
        let result = response.unwrap();
        assert_eq!(result, MergeRequest {
            title: "Whatever".to_string(),
            author: Author { name: "Author Name".to_string() },
            created_at: "2021-05-01T00:00:00Z".to_string(),
            upvotes: 1,
            web_url: "https://test.gitlab.com/projects/x/mrs/1".to_string(),
        });
    }
}