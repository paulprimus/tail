use crate::config::{Config, PrintableApp};
use crate::error::{OplError, OplErrorKind};
use crate::http::fetch_url;
use crate::oplcmd::{OplAppCmd, OplCmd};
use crate::parse::parse_root;
use crate::term::{print_apps, print_config, print_root_by_date};
use std::str::FromStr;
use crate::logtyp::LogTyp;

#[derive(Debug)]
pub struct ActionParam {
    pub env: Environment,
    pub oplcmd: OplCmd,
}

#[derive(Debug, PartialEq)]
pub enum Environment {
    ENTW,
    TEST,
    PROD,
}

impl FromStr for Environment {
    type Err = OplError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("Umgebung wird verglichen {}", s);
        match s.trim().to_lowercase().as_str() {
            "entw" => Ok(Environment::ENTW),
            "test" => Ok(Environment::TEST),
            "prod" => Ok(Environment::PROD),
            _ => Err(OplError::new(OplErrorKind::EnvironmentNotFoundError)),
        }
    }
}

pub async fn dispatch(action_param: ActionParam, config: Config) -> Result<(), OplError> {
    //    println!("{:?} - {:?}", action_param, config);
    let stdout = tokio::io::stdout();
    match &action_param.oplcmd {
        //  OplCmd::FOMIS(offset) => list_root(&action_param, config, stdout, offset).await?,
        OplCmd::FOMIS(oplappcmd) => match &oplappcmd {
            OplAppCmd::LIST(offset, typ) => {
                list_root(&action_param, config, stdout, offset, typ).await?
            }
            OplAppCmd::CONFIG => list_app_config(config.fomis, stdout).await?,
        },
        OplCmd::DQM(oplappcmd) => match &oplappcmd {
            OplAppCmd::LIST(offset, typ) => {
                list_root(&action_param, config, stdout, offset, typ).await?
            }
            OplAppCmd::CONFIG => list_app_config(config.dqm, stdout).await?,
        },
        OplCmd::CONFIG => list_config(config, stdout).await?,
        OplCmd::LIST => list_apps(stdout).await?,
        //_ => unreachable!("Darf nicht passieren!"),
    };
    Ok(())
}

async fn list_root(
    action_param: &ActionParam,
    config: Config,
    stdout: tokio::io::Stdout,
    offset: &Option<u32>,
    typ: &LogTyp
) -> Result<(), OplError> {
    //let sdf = action_param.env;
    let url = config.get_url_for(&action_param)?;
    let data = fetch_url(url).await?;
    let logs = parse_root(data.unwrap())?;
    print_root_by_date(stdout, logs, offset).await?;
    Ok(())
}

async fn list_config(config: Config, stdout: tokio::io::Stdout) -> Result<(), OplError> {
    let value = config.to_string();
    print_config(stdout, value).await?;
    Ok(())
}

async fn list_app_config<T: PrintableApp>(
    config: T,
    stdout: tokio::io::Stdout,
) -> Result<(), OplError> {
    let value = config.stringify();
    print_config(stdout, value).await?;
    Ok(())
}

async fn list_apps(stdout: tokio::io::Stdout) -> Result<(), OplError> {
    print_apps(stdout).await?;
    Ok(())
}
