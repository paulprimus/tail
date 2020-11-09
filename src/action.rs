use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::http::fetch_url;
use crate::opltyp::OplCmdTyp;
use crate::term::print_root;
use std::str::FromStr;
use std::io::{stdout};
use crate::parse::parse_root;

pub struct ActionParam {
    pub env: Environment,
    pub opltype: OplCmdTyp,
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

pub async fn do_action(action_param: ActionParam, config: Config) -> Result<(), OplError> {
    let opl_cmd = action_param.opltype;
    let mut stdout = tokio::io::stdout();
    match opl_cmd {
        OplCmdTyp::FOMIS(_fomis_cmd) => {
            let data = fetch_url(opl_cmd, &config).await?;
            let logs = parse_root(data.unwrap())?;
            print_root(stdout, logs, 10);
            // .unwrap_or_else(|err| {
            //     eprintln!("{}", err);
            //     process::exit(1);
            // });
        }
        _ => {}
    };
    Ok(())
}
