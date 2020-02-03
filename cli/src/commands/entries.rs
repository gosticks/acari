use super::OutputFormat;
use acari_lib::{AcariError, Client, DateSpan, Minutes, TimeEntry};
use chrono::NaiveDate;
use itertools::Itertools;
use prettytable::{cell, format, row, Table};

pub fn entries(client: &dyn Client, output_format: OutputFormat, date_span: DateSpan) -> Result<(), AcariError> {
  let mut time_entries = client.get_time_entries(date_span)?;

  time_entries.sort_by(|t1, t2| t1.date_at.cmp(&t2.date_at));

  let grouped: Vec<(&NaiveDate, Vec<&TimeEntry>)> = time_entries
    .iter()
    .group_by(|e| &e.date_at)
    .into_iter()
    .map(|(customer_name, group)| (customer_name, group.collect()))
    .collect();

  match output_format {
    OutputFormat::Pretty => print_pretty(grouped),
    OutputFormat::Json => print_json(time_entries)?,
    OutputFormat::Flat => print_flat(grouped),
  }

  Ok(())
}

fn print_pretty(entries: Vec<(&NaiveDate, Vec<&TimeEntry>)>) {
  let mut entries_table = Table::new();
  entries_table.set_titles(row!["Day", "Time", "Customer", "Project", "Service"]);
  entries_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

  for (day, group) in entries {
    let sum = group.iter().map(|e| e.minutes).sum::<Minutes>();
    entries_table.add_row(row![day, sum, H3 -> " " ]);
    for entry in group {
      entries_table.add_row(row!["", entry.minutes, entry.customer_name, entry.project_name, entry.service_name]);
    }
  }
  entries_table.printstd();
}

fn print_json(entries: Vec<TimeEntry>) -> Result<(), AcariError> {
  println!("{}", serde_json::to_string_pretty(&entries)?);

  Ok(())
}

fn print_flat(entries: Vec<(&NaiveDate, Vec<&TimeEntry>)>) {
  for (date, group) in entries {
    for entry in group {
      println!(
        "{}\t{}\t{}\t{}\t{}",
        date, entry.customer_name, entry.project_name, entry.service_name, entry.minutes,
      );
    }
  }
}
