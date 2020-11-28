use crate::config::Config;
use crate::error::OplError;
use crate::opltyp::OplCmd;
use crate::parse::RootLogs;
use chrono::prelude::*;
use chrono::Duration;
use tokio::io::{AsyncWriteExt, BufWriter, Stdout};

pub async fn print_root(
    stdout: tokio::io::Stdout,
    data: RootLogs,
    offset: &Option<u32>,
) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);
    if offset.is_none() {
        //println!("{:?}", data.logs);
        for (k, v) in data.logs {
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
    for i in 1..offset {
        let date = today - Duration::days(i as i64);
        if let Some(v) = data.logs.get(&date) {
            print_entry(writer, date, v).await?;
        }
    }
    writer.flush().await?;
    Ok(())
}

async fn print_entry(
    writer: &mut BufWriter<Stdout>,
    date: Date<Utc>,
    v: &[String],
) -> Result<(), OplError> {
    writer.write(date.to_string().as_bytes()).await?;
    writer.write(b": ").await?;
    for s in v {
        writer.write(s.as_bytes()).await?;
        writer.write(b", ").await?;
    }
    writer.write(b"\n").await?;
    Ok(())
}

pub async fn print_config(stdout: tokio::io::Stdout, config: Config) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);

    writer.write(config.to_string().as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn print_apps(stdout: tokio::io::Stdout, config: Config) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);
    writer.write(b"DQM\n").await?;
    writer.write(b"FOMIS\n").await?;
    writer.flush().await?;
    Ok(())
}
