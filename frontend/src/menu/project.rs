use chrono::{DateTime, Utc};
use leptos::*;

#[derive(Debug, Clone)]
pub struct RecentProject {
    pub name: String,
    pub path: String,
    pub last_opened: DateTime<Utc>,
}

#[component]
pub fn Project(project: RecentProject) -> impl IntoView {
    view! {
        <button class="w-full flex flex-col py-2">
            <h3 class="text-primary text-md">{project.name}</h3>
            <p class="text-neutral text-sm">{calculate_time_since(&project.last_opened)}</p>
        </button>
    }
}

// TODO fix this to properly return time since
fn calculate_time_since(time: &DateTime<Utc>) -> String {
    let time = Utc::now().signed_duration_since(time).abs();
    let unit;
    let diff;
    if time.num_weeks() > 0 {
        diff = time.num_weeks();
        unit = if diff > 1 {
            t!("weeks_title")
        } else {
            t!("week_title")
        };
    } else if time.num_days() > 0 {
        diff = time.num_days();
        unit = if diff > 1 {
            t!("days_title")
        } else {
            t!("day_title")
        };
    } else if time.num_hours() > 0 {
        diff = time.num_hours();
        unit = if diff > 1 {
            t!("hours_title")
        } else {
            t!("hour_title")
        };
    } else if time.num_minutes() > 0 {
        diff = time.num_minutes();
        unit = if diff > 1 {
            t!("minutes_title")
        } else {
            t!("minute_title")
        };
    } else {
        diff = time.num_seconds();
        unit = if diff > 1 {
            t!("seconds_title")
        } else {
            t!("second_title")
        };
    };

    format!("{diff} {unit} {}", t!("ago_title"))
}
