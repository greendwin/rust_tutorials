use linked_hash_map::LinkedHashMap;
use serde::de::DeserializeOwned;
use serde_yaml::Value;
use std::collections::HashMap;
use std::error::Error;

pub trait ParserPlugin<'a>: 'a {
    fn init(&'a self, parser: &mut Parser<'a>);
}

#[derive(Debug)]
pub enum ParserError {
    Msg(String),
    Any(Box<dyn Error>),
}

pub type ParserResult = Result<(), ParserError>;

impl<T> From<T> for ParserError
where
    T: Error + 'static,
{
    fn from(other: T) -> Self {
        ParserError::Any(Box::new(other))
    }
}

pub struct Parser<'a> {
    cmds: HashMap<String, Box<dyn Fn(Value) -> ParserResult + 'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            cmds: HashMap::new(),
        }
    }

    pub fn register(&mut self, plugin: &'a impl ParserPlugin<'a>) {
        plugin.init(self);
    }

    pub fn add_cmd<Cfg, Cmd, Cb>(&mut self, cmd: Cmd, f: Cb)
    where
        Cfg: DeserializeOwned,
        Cmd: Into<String>,
        Cb: Fn(Cfg) -> ParserResult + 'a,
    {
        self.cmds.insert(
            cmd.into(),
            Box::new(move |val| {
                let cfg: Cfg = serde_yaml::from_value(val)?;
                f(cfg)
            }),
        );
    }

    pub fn parse(&self, text: &str) -> ParserResult {
        let commands: LinkedHashMap<String, Value> = serde_yaml::from_str(text)?;

        for (k, v) in commands {
            let callback = self
                .cmds
                .get(&k)
                .ok_or_else(|| ParserError::Msg(format!("unknown command: {:?}", k)))?;

            callback(v)?;
        }

        Ok(())
    }
}
