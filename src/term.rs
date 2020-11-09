use crate::error::OplError;
use crate::parse::RootLogs;
use chrono::prelude::*;
use chrono::Duration;
use tokio::io::{AsyncWriteExt, BufWriter, Stdout};

pub async fn print_root(
    stdout: tokio::io::Stdout,
    data: RootLogs,
    offset: u32,
) -> Result<(), OplError> {
    let mut writer = BufWriter::new(stdout);
    if offset == 0 {
        for (k, v) in data.logs {
            print_entry(&mut writer, k, &v).await?;
            writer.flush();
        }
    } else {
        let today = Utc::today();
        print_btree(data, offset, &mut writer, today).await?;
    }
    Ok(())
}

async fn print_btree(
    data: RootLogs,
    offset: u32,
    writer: &mut BufWriter<&mut Stdout>,
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
    &mut  writer: BufWriter<&mut Stdout>,
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
