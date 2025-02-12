use vault::Env;

use crate::TvlMaker;

pub trait TvlCommand {
    fn name(&self) -> String;
    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()>;
    fn suite_env_json(&self, env: &Env) -> String {
        serde_json::to_string(env).unwrap()
    }
}
