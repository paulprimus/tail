use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::http::fetch_url;
use crate::opltyp::OplCmd;
use crate::parse::parse_root;
use crate::term::print_root;
use std::str::FromStr;

#[derive(Debug)]
pub struct ActionParam {
    pub env: Environment,
    pub opltype: OplCmd,
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

pub async fn do_action(action_param: ActionParam, config: Config) -> Result<(), OplError> {
    //    println!("{:?} - {:?}", action_param, config);
    let stdout = tokio::io::stdout();
    match &action_param.opltype {
        OplCmd::FOMIS(offset) => list_root(&action_param, config, stdout, offset).await?,
        OplCmd::DQM(offset) => list_root(&action_param, config, stdout, offset).await?,
        _ => unreachable!("Darf nicht passieren!"),
    };
    Ok(())
}

async fn list_root(
    action_param: &ActionParam,
    config: Config,
    stdout: tokio::io::Stdout,
    offset: &Option<u32>,
) -> Result<(), OplError> {
    //let sdf = action_param.env;
    let url = config.get_url_for(&action_param)?;
    let data = fetch_url(url).await?;
    let logs = parse_root(data.unwrap())?;
    print_root(stdout, logs, offset).await?;
    Ok(())
}
