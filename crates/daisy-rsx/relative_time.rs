#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RelativeTimeFormat {
    Datetime,
    #[default]
    Relative,
    Duration,
    Auto,
    Micro,
    Elapsed,
}

impl RelativeTimeFormat {
    pub fn to_string(&self) -> &'static str {
        match self {
            RelativeTimeFormat::Datetime => "datetime",
            RelativeTimeFormat::Relative => "relative",
            RelativeTimeFormat::Duration => "duration",
            RelativeTimeFormat::Auto => "auto",
            RelativeTimeFormat::Micro => "micro",
            RelativeTimeFormat::Elapsed => "elapsed",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RelativeTimeProps {
    format: Option<RelativeTimeFormat>,
    datetime: String,
}

pub fn RelativeTime(props: RelativeTimeProps) -> Element {
    let format = if props.format.is_some() {
        props.format.unwrap()
    } else {
        Default::default()
    };

    rsx!(
        relative
            - time {
                datetime: props.datetime,
                format: format.to_string()
            }
    )
}
