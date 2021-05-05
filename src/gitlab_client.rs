use crate::coffee_config::Gitlab;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct MergeRequest {
    pub title: String,
    pub author: Author,
    pub created_at: String,
    pub upvotes: i8,
    pub web_url: String,
    pub work_in_progress: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
}

impl MergeRequest {
    pub async fn get_open(config: &Gitlab) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        match &config.projects {
            None => {
                let url = format!("{}/merge_requests?state=opened", config.base_url);
                MergeRequest::get_merge_requests(&url, &config.token, config.include_wip).await
            },
            Some(project_ids) => {
                let mut merge_requests: Vec<Self> = vec![];
                for id in project_ids {
                    let url = format!("{}/projects/{}/merge_requests?state=opened", config.base_url, id);
                    let result = MergeRequest::get_merge_requests(&url, &config.token, config.include_wip).await;
                    if let Ok(mut mrs) = result {
                        merge_requests.append(&mut mrs);
                    }
                }

                Ok(merge_requests)
            }
        }
    }

    async fn get_merge_requests(url: &str, token: &str, include_wip: Option<bool>) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("PRIVATE-TOKEN", token)
            .send()
            .await?
            .json::<Vec<MergeRequest>>()
            .await?;
        let include_wip = include_wip == Some(true);
        let merge_requests: Vec<MergeRequest> = response
            .into_iter()
            .filter(|mr| !mr.work_in_progress || mr.work_in_progress && include_wip)
            .collect();
        Ok(merge_requests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[tokio::test]
    async fn it_loads_single_merge_request() {
        let server = MockServer::start();
        let data = r#"[{
            "title": "Whatever",
            "author": {
                "name": "Author Name"
            },
            "created_at": "2021-05-01T00:00:00Z",
            "upvotes": 1,
            "web_url": "https://test.gitlab.com/projects/x/mrs/1",
            "other_field": "Shouldn't be in the DS",
            "work_in_progress": false
        }]"#;

        let mock = server.mock(|when, then| {
            when.method(GET).header("PRIVATE-TOKEN", "TOKEN");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(data);
        });
        let config = Gitlab {
            base_url: server.base_url().to_string(),
            token: "TOKEN".to_string(),
            include_wip: None,
            projects: None,
        };
        let response = MergeRequest::get_open(&config).await;
        // Exactly one HTTP method that matched the mock requirements
        mock.assert();
        assert_eq!(response.is_ok(), true);
        let result = response.unwrap();
        assert_eq!(
            result,
            vec![MergeRequest {
                title: "Whatever".to_string(),
                author: Author {
                    name: "Author Name".to_string()
                },
                created_at: "2021-05-01T00:00:00Z".to_string(),
                upvotes: 1,
                web_url: "https://test.gitlab.com/projects/x/mrs/1".to_string(),
                work_in_progress: false,
            }]
        );
    }

    #[tokio::test]
    async fn it_loads_empty_response() {
        let server = MockServer::start();
        let data = r#"[]"#;

        let mock = server.mock(|when, then| {
            when.method(GET).header("PRIVATE-TOKEN", "TOKEN");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(data);
        });
        let config = Gitlab {
            base_url: server.base_url().to_string(),
            token: "TOKEN".to_string(),
            include_wip: None,
            projects: None,
        };
        let response = MergeRequest::get_open(&config).await;
        // Exactly one HTTP method that matched the mock requirements
        mock.assert();
        assert_eq!(response.is_ok(), true);
        let result = response.unwrap();
        assert_eq!(result, vec![]);
    }

    #[tokio::test]
    async fn excludes_wip() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).header("PRIVATE-TOKEN", "TOKEN");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(
                    r#"[{
                        "title": "WIP: Whatever",
                        "author": {
                            "name": "Author Name"
                        },
                        "created_at": "2021-05-01T00:00:00Z",
                        "upvotes": 1,
                        "web_url": "https://test.gitlab.com/projects/x/mrs/1",
                        "work_in_progress": true
                    },
                    {
                        "title": "Active",
                        "author": {
                            "name": "Author Name"
                        },
                        "created_at": "2021-05-01T00:00:00Z",
                        "upvotes": 1,
                        "web_url": "https://test.gitlab.com/projects/x/mrs/2",
                        "work_in_progress": false
                    }]"#,
                );
        });

        let config = Gitlab {
            base_url: server.base_url().to_string(),
            token: "TOKEN".to_string(),
            include_wip: Some(false),
            projects: None,
        };

        let response = MergeRequest::get_open(&config).await;
        assert_eq!(response.is_ok(), true);
        let response = response.unwrap();
        assert_eq!(response, vec![
            MergeRequest {
                title: "Active".to_string(),
                author: Author {
                    name: "Author Name".to_string()
                },
                created_at: "2021-05-01T00:00:00Z".to_string(),
                upvotes: 1,
                web_url: "https://test.gitlab.com/projects/x/mrs/2".to_string(),
                work_in_progress: false
            }
        ]);
        mock.assert();
    }

    #[tokio::test]
    async fn includes_wip() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).header("PRIVATE-TOKEN", "TOKEN");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(
                    r#"[{
                        "title": "WIP: Whatever",
                        "author": {
                            "name": "Author Name"
                        },
                        "created_at": "2021-05-01T00:00:00Z",
                        "upvotes": 1,
                        "web_url": "https://test.gitlab.com/projects/x/mrs/1",
                        "work_in_progress": true
                    },
                    {
                        "title": "Active",
                        "author": {
                            "name": "Author Name"
                        },
                        "created_at": "2021-05-01T00:00:00Z",
                        "upvotes": 1,
                        "web_url": "https://test.gitlab.com/projects/x/mrs/2",
                        "work_in_progress": false
                    }]"#,
                );
        });

        let config = Gitlab {
            base_url: server.base_url().to_string(),
            token: "TOKEN".to_string(),
            include_wip: Some(true),
            projects: None,
        };

        let response = MergeRequest::get_open(&config).await;
        assert_eq!(response.is_ok(), true);
        let response = response.unwrap();
        assert_eq!(response, vec![
            MergeRequest {
                title: "WIP: Whatever".to_string(),
                author: Author {
                    name: "Author Name".to_string()
                },
                created_at: "2021-05-01T00:00:00Z".to_string(),
                upvotes: 1,
                web_url: "https://test.gitlab.com/projects/x/mrs/1".to_string(),
                work_in_progress: true
            },
            MergeRequest {
                title: "Active".to_string(),
                author: Author {
                    name: "Author Name".to_string()
                },
                created_at: "2021-05-01T00:00:00Z".to_string(),
                upvotes: 1,
                web_url: "https://test.gitlab.com/projects/x/mrs/2".to_string(),
                work_in_progress: false
            }
        ]);
        mock.assert();
    }

    #[tokio::test]
    async fn gets_all_projects() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).header("PRIVATE-TOKEN", "TOKEN");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(
                    r#"[]"#,
                );
        });

        let config = Gitlab {
            base_url: server.base_url().to_string(),
            token: "TOKEN".to_string(),
            include_wip: Some(true),
            projects: Some(vec![
                "one".to_string(),
                "two".to_string(),
            ]),
        };

        let response = MergeRequest::get_open(&config).await;
        
        assert_eq!(response.is_ok(), true);
        mock.assert_hits(2);
    }
}
