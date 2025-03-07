use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Define {
        database: String,
        word: String,
    },
    Match {
        database: String,
        strategy: String,
        word: String,
    },
    Client {
        text: String,
    },
    Auth {
        username: String,
        authentication_string: String,
    },
    ShowDb,
    ShowStrat,
    ShowInfo,
    ShowServer,
    Status,
    Help,
    Quit,
}

impl Command {
    ///  Look up the specified word in the specified database.
    ///  database name is "!" for all databases searched until a match is found.
    ///  database name is "*" for all databases matches will be displayed
    pub fn define(database: impl Into<String>, word: impl Into<String>) -> Self {
        Command::Define {
            database: database.into(),
            word: word.into(),
        }
    }

    /// Look up the specified word in the specified database.
    /// strategy have two options: "prefix" and "exact"
    pub fn matches(
        database: impl Into<String>,
        strategy: impl Into<String>,
        word: impl Into<String>,
    ) -> Self {
        Command::Match {
            database: database.into(),
            strategy: strategy.into(),
            word: word.into(),
        }
    }

    /// Send a message to the server.
    pub fn client(text: impl Into<String>) -> Self {
        Command::Client { text: text.into() }
    }

    pub fn show_db() -> Self {
        Command::ShowDb
    }

    pub fn show_strat() -> Self {
        Command::ShowStrat
    }

    pub fn show_info() -> Self {
        Command::ShowInfo
    }

    pub fn to_message(&self) -> String {
        let mut command = match self {
            Command::Define { database, word } => {
                format!("DEFINE {} {}", database, word)
            }
            Command::Match {
                database,
                strategy,
                word,
            } => {
                format!("MATCH {} {} {}", database, strategy, word)
            }
            Command::Client { text } => {
                format!("CLIENT {}", text)
            }
            Command::Auth {
                username,
                authentication_string,
            } => format!("AUTH {} {}", username, authentication_string),
            Command::ShowDb => "SHOW DB".to_string(),
            Command::ShowStrat => "SHOW STRAT".to_string(),
            Command::ShowInfo => "SHOW INFO".to_string(),
            Command::ShowServer => "SHOW SERVER".to_string(),
            Command::Status | Command::Help | Command::Quit => stringify!(self).to_uppercase(),
        };

        command.push_str("\r\n");
        command
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(core::format_args!("{}", self.to_message()))
    }
}

#[cfg(test)]
#[test]
pub fn builder() {
    let cmd = Command::define("test", "test");
    assert_eq!(cmd.to_message(), "DEFINE test test\r\n");

    let mat = Command::matches("test", "exact", "test");
    assert_eq!(mat.to_message(), "MATCH test exact test\r\n");

    let cmd = Command::show_db();
    assert_eq!(cmd.to_message(), "SHOW DB\r\n");

    let cmd = Command::client(String::from("value"));
    assert_eq!(cmd.to_message(), "CLIENT value\r\n");
}
