use std::cmp::Reverse;
use std::collections::BTreeMap;

use chrono::prelude::*;
use chrono::Duration;
use tokio::io::{AsyncWriteExt, BufWriter, Stdout};

use crate::error::OplError;
use crate::opldate::OplDate;
use crate::rootlogs::{RootLog, RootLogs};

pub async fn print_root_by_date(
    stdout: tokio::io::Stdout,
    data: RootLogs,
    offset: &Option<u32>,
) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);
    if offset.is_none() {
        //println!("{:?}", data.logs);

        let map: BTreeMap<OplDate, Vec<RootLog>> = data
            .get_logs_by_date()
            .expect("Logdateien konnten nicht per Datum sortiert werden!");
        // for (k, v) in map {
        //     print_entry(&mut writer, k, &v).await?;
        // }
        for (k, v) in map.iter().rev() {
            print_entry(&mut writer, k, &v).await?;
        }
        writer.flush().await?;
    } else {
        let today = Utc::today();
        print_btree(data, offset.unwrap(), &mut writer, today).await?;
    }
    Ok(())
}

async fn print_btree(
    data: RootLogs,
    offset: u32,
    writer: &mut BufWriter<Stdout>,
    today: Date<Utc>,
) -> Result<(), OplError> {
    for i in 0..offset {
        let date = today - Duration::days(i as i64);
        let btree = data.get_logs_by_date()?;
        if let Some(v) = btree.get(&OplDate::from(date)) {
            print_entry(writer, &date, v).await?;
        }
    }
    writer.flush().await?;
    Ok(())
}

async fn print_entry(
    writer: &mut BufWriter<Stdout>,
    date: &Date<Utc>,
    v: &[RootLog],
) -> Result<(), OplError> {
    writer.write(date.to_string().as_bytes()).await?;
    writer.write(b": ").await?;
    for s in v {
        writer.write(s.name.as_bytes()).await?;
        writer.write(b"[").await?;
        writer.write(s.log_typ.to_string().as_bytes()).await?;
        writer.write(b"]").await?;
        writer.write(b", ").await?;
    }
    writer.write(b"\n").await?;
    Ok(())
}

pub async fn print_config(stdout: tokio::io::Stdout, config: String) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);
    writer.write(config.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn print_apps(stdout: tokio::io::Stdout) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);
    writer.write(b"DQM\n").await?;
    writer.write(b"FOMIS\n").await?;
    writer.flush().await?;
    Ok(())
}
