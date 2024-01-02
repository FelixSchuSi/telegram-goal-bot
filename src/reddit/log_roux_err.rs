use log::error;
use roux::util::RouxError;
use roux_stream::StreamError;

pub async fn log_roux_err(err: StreamError<RouxError>) {
    match err {
        StreamError::TimeoutError(timeout) => {
            error!("is TimeoutError: {}", timeout);
        }
        StreamError::SourceError(e) => match e {
            RouxError::Network(net_e) => {
                error!("is RouxError::Network: {}", net_e);
                let status_message = net_e
                    .status()
                    .map_or_else(|| "No status code".to_string(), |v| v.to_string());
                error!("Status: {}", status_message);
                error!("is_body(): {}", net_e.is_body());
                error!("is_builder(): {}", net_e.is_builder());
                error!("is_connect(): {}", net_e.is_connect());
                error!("is_decode(): {}", net_e.is_decode());
                error!("is_redirect(): {}", net_e.is_redirect());
                error!("is_request(): {}", net_e.is_request());
                error!("is_status(): {}", net_e.is_status());
                error!("is_timeout(): {}", net_e.is_timeout());
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
            RouxError::Parse(parse_e) => error!("is RouxError::Parse: {}", parse_e),
            RouxError::Auth(e) => error!("is RouxError::Auth: {}", e),
        },
    }
}
