pub mod form;
pub mod history_table;
pub mod index;
pub mod results;
use db::History;
use std::collections::HashMap;
use time::{Duration, OffsetDateTime};

#[derive(PartialEq, Clone)]
pub struct HistoryBucket {
    pub name: String,
    pub histories: Vec<History>,
}

pub fn bucket_history(histories: Vec<History>) -> (Vec<HistoryBucket>, usize) {
    let now = OffsetDateTime::now_utc();
    let today_start = now.date();
    let yesterday_start = today_start.previous_day().unwrap();
    let last_week_start = today_start - Duration::days(7);
    let last_month_start = today_start - Duration::days(30);

    let one_hour_ago = now - Duration::hours(1);
    let eight_hours_ago = now - Duration::hours(8);
    let twenty_four_hours_ago = now - Duration::hours(24);

    let mut last_hour = Vec::new();
    let mut last_8_hours = Vec::new();
    let mut last_24_hours = Vec::new();
    let mut yesterday = Vec::new();
    let mut last_week = Vec::new();
    let mut last_month = Vec::new();
    let mut monthly = HashMap::new();

    let total_count = histories.len();

    for history in histories {
        let created_at = history.created_at;

        if created_at >= one_hour_ago {
            last_hour.push(history);
        } else if created_at >= eight_hours_ago {
            last_8_hours.push(history);
        } else if created_at >= twenty_four_hours_ago {
            last_24_hours.push(history);
        } else if created_at.date() == yesterday_start {
            yesterday.push(history);
        } else if created_at.date() >= last_week_start {
            last_week.push(history);
        } else if created_at.date() >= last_month_start {
            last_month.push(history);
        } else {
            // Group into monthly buckets
            let month_key = format!("{:04}-{:02}", created_at.year(), created_at.month() as u8);
            monthly
                .entry(month_key)
                .or_insert_with(Vec::new)
                .push(history);
        }
    }

    // Create a Vec<HistoryBucket> from the categorized histories.
    let mut buckets = vec![
        HistoryBucket {
            name: "Last Hour".to_string(),
            histories: last_hour,
        },
        HistoryBucket {
            name: "Last 8 Hours".to_string(),
            histories: last_8_hours,
        },
        HistoryBucket {
            name: "Last 24 Hours".to_string(),
            histories: last_24_hours,
        },
        HistoryBucket {
            name: "Yesterday".to_string(),
            histories: yesterday,
        },
        HistoryBucket {
            name: "Last Week".to_string(),
            histories: last_week,
        },
        HistoryBucket {
            name: "Last Month".to_string(),
            histories: last_month,
        },
    ];

    // Add monthly buckets.
    for (month, histories) in monthly {
        buckets.push(HistoryBucket {
            name: format!("Month: {}", month),
            histories,
        });
    }

    (buckets, total_count)
}
