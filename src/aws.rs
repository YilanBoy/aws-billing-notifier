use aws_sdk_costexplorer::types::{DateInterval, Granularity, ResultByTime};
use chrono::Datelike;

pub struct Aws {}

impl Aws {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_account_id(&self) -> String {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_sts::Client::new(&config);

        let response = client.get_caller_identity().send().await;

        let response = match response {
            Ok(response) => response,
            Err(error) =>
                panic!("There was a problem getting the caller_identity: {:?}", error),
        };

        if let Some(account_id) = response.account() {
            account_id.to_string()
        } else {
            panic!("There was a problem getting the caller_identity");
        }
    }

    pub async fn get_account_cost_in_this_month(&self) -> Vec<ResultByTime> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_costexplorer::Client::new(&config);

        let now = chrono::Utc::now().naive_utc();
        let start_of_month = now.with_day(1).unwrap();

        let start_date = start_of_month.format("%Y-%m-%d").to_string();
        let end_date = now.format("%Y-%m-%d").to_string();

        let date_interval = DateInterval::builder()
            .start(start_date)
            .end(end_date)
            .build()
            .unwrap();

        let response = client
            .get_cost_and_usage()
            .time_period(date_interval)
            .metrics("UnblendedCost")
            .granularity(Granularity::Monthly)
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(error) =>
                panic!("There was a error getting the cost and usage: {:?}", error),
        };

        let mut result_by_time: Vec<ResultByTime> = Vec::new();

        for result in response.results_by_time() {
            result_by_time.push(result.clone());
        }

        result_by_time
    }
}