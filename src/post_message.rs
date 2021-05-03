use crate::coffee_config::{Publish, PublishChannel};
use crate::gitlab_client::{Author, MergeRequest};
use reqwest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SlackMessage {
    text: String,
}

fn format_mrkdwn(merge_request: &MergeRequest) -> String {
    format!(
        "<{}|{}> by {} opened on *{}*. Upvotes: {}",
        merge_request.web_url,
        merge_request.title,
        merge_request.author.name,
        merge_request.created_at,
        merge_request.upvotes
    )
}

fn format_markdown(merge_request: &MergeRequest) -> String {
    format!(
        "[{}]({}) by {} opened on __{}__. Upvotes: {}",
        merge_request.web_url,
        merge_request.title,
        merge_request.author.name,
        merge_request.created_at,
        merge_request.upvotes
    )
}

impl SlackMessage {
    fn new(merge_requests: &Vec<MergeRequest>) -> Self {
        let messages: Vec<String> = merge_requests
            .into_iter()
            .map(|mr| format_mrkdwn(&mr))
            .collect();
        println!("{:?}", messages);
        SlackMessage {
            text: messages.join("\n")
        }
    }

    async fn post_to_webhook(
        &self,
        webhook_url: &str
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client.post(webhook_url).json(self).send().await?;
    
        match res.error_for_status() {
            Ok(_res) => Ok(true),
            Err(_err) => Ok(false),
        }
    }
}

pub async fn post_messages(
    merge_requests: &Vec<MergeRequest>,
    config: &Publish,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut success = 0;
    if let Some(slack_config) = config.slack.clone() {
        println!("Posting message to Slack");
        let message = SlackMessage::new(merge_requests);
        let result = message.post_to_webhook(&slack_config.webhook_url).await?;
        success += 1;
    }

    if let Some(teams_config) = config.teams.clone() {
        println!("Posting message to Teams");
        let messages: Vec<String> = merge_requests
            .into_iter()
            .map(|mr| format_markdown(&mr))
            .collect();
        println!("{:?}", messages);
        success += messages.len();
    }

    Ok(success)
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::Method::POST;

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
        let config = Publish {
            teams: Some(PublishChannel {
                webhook_url: "https://teams.webhook.com/channel".to_string(),
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
        assert_eq!(result, 1);
    }
}
