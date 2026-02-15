use log::error;
use roux::util::RouxError;
use roux_stream::StreamError;

pub async fn log_roux_err(err: StreamError<RouxError>) {
    error!("Error getting submission: {}", err);
    match err {
        StreamError::TimeoutError(timeout) => {
            error!("is TimeoutError");
            error!("timeout: {}", timeout);
        }
        StreamError::SourceError(e) => {
            error!("is SourceError");
            error!("RouxError: {}", e);
            match e {
                RouxError::Network(net_e) => {
                    error!("is RouxError::Network");
                    error!("Network: {}", net_e);
                    net_e
                        .status()
                        .map(|status| error!("Status: {}", status))
                        .unwrap_or_else(|| error!("Couldn't get status from Network error"));
                }
                RouxError::Status(stat_e) => {
                    error!("is RouxError::Status");
                    error!(
                        "reddit submission handler received a response of status: {}",
                        stat_e.status()
                    );
                    error!(
                        "response: {}",
                        stat_e
                            .text()
                            .await
                            .unwrap_or("Couldnt read response.text()".to_string())
                    );
                }
                RouxError::Parse(parse_e) => {
                    error!("is RouxError::Parse");
                    error!("Parse: {}", parse_e);
                }
                RouxError::Auth(e) => {
                    error!("is RouxError::Auth");
                    error!("Auth: {}", e);
                }
                RouxError::CredentialsNotSet => {
                    error!("is RouxError::CredentialsNotSet");
                    error!("CredentialsNotSet: {}", e);
                }
                RouxError::OAuthClientRequired => {
                    error!("is RouxError::OAuthClientRequired");
                    error!("OAuthClientRequired: {}", e);
                }
            }
        }
    }
}
