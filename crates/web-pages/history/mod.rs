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

    let mut today = Vec::new();
    let mut yesterday = Vec::new();
    let mut last_week = Vec::new();
    let mut last_month = Vec::new();
    let mut monthly = HashMap::new();

    let total_count = histories.len();

    for history in histories {
        let created_date = history.created_at.date();

        if created_date == today_start {
            today.push(history);
        } else if created_date == yesterday_start {
            yesterday.push(history);
        } else if created_date >= last_week_start {
            last_week.push(history);
        } else if created_date >= last_month_start {
            last_month.push(history);
        } else {
            // Group into monthly buckets
            let month_key = format!(
                "{:04}-{:02}",
                created_date.year(),
                created_date.month() as u8
            );
            monthly
                .entry(month_key)
                .or_insert_with(Vec::new)
                .push(history);
        }
    }

    // Create a Vec<HistoryBucket> from the categorized histories.
    let mut buckets = vec![
        HistoryBucket {
            name: "Today".to_string(),
            histories: today,
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
