use crate::coffee_config::Publish;
use crate::gitlab_client::MergeRequest;
use reqwest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SlackMessage {
    text: String,
}

#[derive(Debug, Serialize)]
struct TeamsMessage {
    title: String,
    text: String,
}

async fn post_to_webhook<T: Serialize>(
    message: &T,
    webhook_url: &str
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(webhook_url).json(message).send().await?;

    match res.error_for_status() {
        Ok(_res) => Ok(true),
        Err(_err) => Ok(false),
    }
}

impl SlackMessage {
    fn new(title: &str, merge_requests: &Vec<MergeRequest>) -> Self {
        let messages: Vec<String> = merge_requests
            .into_iter()
            .map(|mr| format!(
                "<{}|{}> by {} opened on *{}*. Upvotes: {}",
                mr.web_url,
                mr.title,
                mr.author.name,
                mr.created_at,
                mr.upvotes
            ))
            .collect();
        SlackMessage {
            text: format!("{}\n{}", title, messages.join("\n"))
        }
    }

    // This method could be shared but async traits are currently not supported
    
}

impl TeamsMessage {
    fn new(title: &str, merge_requests: &Vec<MergeRequest>) -> Self {
        let messages: Vec<String> = merge_requests
            .into_iter()
            .map(|mr| format!(
                "[{}]({}) by {} opened on __{}__. Upvotes: {}",
                mr.web_url,
                mr.title,
                mr.author.name,
                mr.created_at,
                mr.upvotes
            ))
            .collect();
        TeamsMessage {
            title: title.to_string(),
            text: messages.join("\n")
        }
    }
}


pub async fn post_messages(
    merge_requests: &Vec<MergeRequest>,
    config: &Publish,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut success = 0;
    let salutation = "Hi! There are some Merge Requests to review :)";
    if let Some(slack_config) = config.slack.clone() {
        println!("Posting message to Slack");
        let message = SlackMessage::new(salutation, merge_requests);
        let _result = post_to_webhook(&message, &slack_config.webhook_url).await?;
        success += 1;
    }

    if let Some(teams_config) = config.teams.clone() {
        println!("Posting message to Teams");
        let message = TeamsMessage::new(salutation, merge_requests);
        let _result = post_to_webhook(&message, &teams_config.webhook_url).await?;
        success += 1;
    }

    Ok(success)
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::Method::POST;
    use crate::coffee_config::PublishChannel;
    use crate::gitlab_client::Author;

    #[tokio::test]
    async fn post_to_slack() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).header("Content-Type", "application/json");
            then.status(200);
        });

        let config = Publish {
            slack: Some(PublishChannel {
                webhook_url: server.base_url().to_string(),
            }),
            teams: None,
        };
        let merge_requests = vec![MergeRequest {
            title: "Whatever".to_string(),
            author: Author {
                name: "Author Name".to_string(),
            },
            created_at: "2021-05-01T00:00:00Z".to_string(),
            upvotes: 1,
            web_url: "https://test.gitlab.com/projects/x/mrs/1".to_string(),
        }];
        let result = post_messages(&merge_requests, &config).await.unwrap();
        mock.assert();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn post_to_teams() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).header("Content-Type", "application/json");
            then.status(200);
        });

        let config = Publish {
            teams: Some(PublishChannel {
                webhook_url: server.base_url().to_string(),
            }),
            slack: None,
        };
        let merge_requests = vec![MergeRequest {
            title: "Whatever".to_string(),
            author: Author {
                name: "Author Name".to_string(),
            },
            created_at: "2021-05-01T00:00:00Z".to_string(),
            upvotes: 1,
            web_url: "https://test.gitlab.com/projects/x/mrs/1".to_string(),
        }];
        let result = post_messages(&merge_requests, &config).await.unwrap();
        mock.assert();
        assert_eq!(result, 1);
    }
}
