# Coffee Rustviews

[Coffee Reviews](https://gitlab.com/raymundo.vr/coffee-reviews) in 🦀

__Get a notification about pending Merge Requests for your (team's) projects.__

Got time for a coffee? Why not reviewing some code while enjoying it?

## Requirements

- Rust >= 1.40

## Getting started

1. Clone this project
2. Build the application by executing `cargo build`

## Configuration

The presence of a configuration file is required. The [config.sample.json](config.sample.json) contains all possible configuration options needed to run the project.

### Configuring Gitlab settings

Before starting be sure to have obtained a [Gitlab API Token](https://docs.gitlab.com/ee/api/README.html#authentication) to authenticate the requests.

Once you have the authentication token include it in the configuration file in the `gitlab` section. Example:

```json
{
    "gitlab": {
        "base_url": "https://gitlab.com/api/v4",
        "token": "here-goes-your-token"
    },
    [...]
}
```
Other configuration options are available, please check the [sample attached](config.sample.json) to this project.


### Posting to Slack

An application webhook is required to receive messages in Slack. You can create one by registering an application. Please proceed as specified in the official [Slack documentation](https://api.slack.com/).

Once you have your webhook URL include it in the configuration file. Example:
```json
{
    [...]
    "publish": {
        "slack": {
            "webhook_url": "https://hooks.slack.com/services/yourappwebhook-url"
        }
    }
}
```

### Posting to Microsoft Teams

To receive the messages a connector for incoming webhook needs to be created in your channel. Proceed as specified in the [Teams documentation](https://docs.microsoft.com/en-us/microsoftteams/platform/webhooks-and-connectors/how-to/add-incoming-webhook).

Once you have registered the connector in your channel you can specify the url in the configuration settings.
```json
{
    [...]
    "publish": {
        "teams": {
            "webhook_url": "https://url.outlook.com/webhooks/your-webhook-url"
        }
    }
}
```

## Running the app

Create a `config.json` file that includes all your configuration settings.

Then execute:

`cargo run -- -c /path/to/config.json`

This command will print a message indicating the Merge Requests posted and a confirmation on the number of messages posted (all merge requests go in one message).

If all is configured correctly you should be able to see something like

```
Posted 4 Merge Requests. Successful posts: 1
```

And the corresponding message in your channel.

### Config file argument

If you don't specify a path to your `config.json` using `-c /path/to/config.json` the program will assume that it resides in the current path.

### Salutation

Currently the messages are preceeded by:

__Hi! There are some Merge Requests to review :)__

### Periodic executions

The best use case scenario is when this script is executed by a Cron job.

Can be quickly resumed as follows.

1. Build the release binary

```bash
$ cargo build --release
```
This creates a target/release folder inside the project's folder that includes the executable file `coffee-rustviews`.

2. Edit your cron jobs to execute the bash script.

For example
```bash
0 9 * * *  /path/to/coffee-rustviews -c /path/to/config.json
```
Will execute the script everyday at 09:00 hrs.