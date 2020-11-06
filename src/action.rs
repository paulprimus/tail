use crate::error::{OplError, OplErrorKind};
use crate::opltyp::{OplCmdTyp, FomisCmdTyp};
use std::str::FromStr;
use crate::http::fetch_url;
use std::process;
use crate::config::Config;

pub struct ActionParam {
    pub env: Environment,
    pub opltype: OplCmdTyp
}

#[derive(Debug)]
pub enum Environment {
    ENTW,
    TEST,
    PROD,
}

impl FromStr for Environment {
    type Err = OplError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "entw" => Ok(Environment::ENTW),
            "test" => Ok(Environment::TEST),
            "prod" => Ok(Environment::PROD),
            _ => Err(OplError::new(OplErrorKind::EnvironmentNotFoundError)),
        }
    }
}

async fn doAction(cmd: &OplCmdTyp, config: Config) {
    match cmd {
        OplCmdTyp::FOMIS(fomisCmd) => {
            fetch_url(cmd, &config)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    process::exit(1);
                });
        }
        _ => {}
    }
}
