use crate::coffee_config::Publish;
use crate::gitlab_client::MergeRequest;
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
    webhook_url: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(webhook_url).json(message).send().await?;

    match res.error_for_status() {
        Ok(_res) => Ok(true),
        Err(_err) => { println!("Error while posting to {:?} {:?}", webhook_url, _err); Ok(false) },
    }
}

impl SlackMessage {
    fn new(title: &str, merge_requests: &[MergeRequest]) -> Self {
        let messages: Vec<String> = merge_requests
            .iter()
            .map(|mr| {
                format!(
                    "<{}|{}> by {} opened on *{}*. Upvotes: {}",
                    mr.web_url, mr.title, mr.author.name, mr.created_at, mr.upvotes
                )
            })
            .collect();
        SlackMessage {
            text: format!("{}\n{}", title, messages.join("\n")),
        }
    }
}

impl TeamsMessage {
    fn new(title: &str, merge_requests: &[MergeRequest]) -> Self {
        let messages: Vec<String> = merge_requests
            .iter()
            .map(|mr| {
                format!(
                    "[{}]({}) by {} opened on __{}__. Upvotes: {}",
                    mr.web_url, mr.title, mr.author.name, mr.created_at, mr.upvotes
                )
            })
            .collect();
        TeamsMessage {
            title: title.to_string(),
            text: messages.join("\n"),
        }
    }
}

pub fn get_salutation(config: &Publish) -> &str {
    match &config.salutation {
        Some(message) => message,
        None => "Hi! There are some Merge Requests to review :)",
    }
}

pub async fn post_messages(
    merge_requests: &[MergeRequest],
    config: &Publish,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut success = 0;
    let salutation = get_salutation(config);

    if let Some(slack_config) = config.slack.clone() {
        println!("Posting message to Slack");
        let message = SlackMessage::new(salutation, merge_requests);
        let result = post_to_webhook(&message, &slack_config.webhook_url).await?;
        if result {
            success += 1;
        }
    }

    if let Some(teams_config) = config.teams.clone() {
        println!("Posting message to Teams");
        let message = TeamsMessage::new(salutation, merge_requests);
        let result = post_to_webhook(&message, &teams_config.webhook_url).await?;
        if result {
            success += 1;
        }
    }

    Ok(success)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coffee_config::PublishChannel;
    use crate::gitlab_client::Author;
    use httpmock::Method::POST;
    use httpmock::MockServer;

    #[tokio::test]
    async fn posts_to_slack() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).header("Content-Type", "application/json");
            then.status(200);
        });

        let config = Publish {
            salutation: None,
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
            work_in_progress: false
        }];
        let result = post_messages(&merge_requests, &config).await.unwrap();
        mock.assert();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn posts_to_teams() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).header("Content-Type", "application/json");
            then.status(200);
        });

        let config = Publish {
            salutation: None,
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
            work_in_progress: false
        }];
        let result = post_messages(&merge_requests, &config).await.unwrap();
        mock.assert();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn post_to_webhook_is_false() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).header("Content-Type", "application/json");
            then.status(500);
        });

        let config = Publish {
            salutation: None,
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
            work_in_progress: false
        }];
        let result = post_messages(&merge_requests, &config).await.unwrap();
        mock.assert();
        assert_eq!(result, 0);
    }

    #[test]
    fn gets_salutation_when_defined_in_config() {
        let config = Publish {
            salutation: Some("Hey tests!".into()),
            slack: None,
            teams: None,
        };

        assert_eq!(get_salutation(&config), "Hey tests!");
    }

    #[test]
    fn gets_salutation_when_not_defined_in_config() {
        let config = Publish {
            salutation: None,
            slack: None,
            teams: None,
        };

        assert_eq!(get_salutation(&config), "Hi! There are some Merge Requests to review :)");
    }
}
