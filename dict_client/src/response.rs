use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum StatusCode {
    // * 110 n databases present - text follows
    DatabasePresend = 110,
    // * 111 n strategies available - text follows
    StrategiesAvailable = 111,
    //   112 database information follows
    DatabaseInfo = 112,
    //   113 help text follows
    HelpText = 113,
    //   114 server information follows
    ServerInfo = 114,
    //   130 challenge follows
    ChallengeFollows = 130,
    // * 150 n definitions retrieved - definitions follow
    DefinitionsRetrieved = 150,
    // * 151 word database name - text follows
    DefinitionFollows = 151,
    // * 152 n matches found - text follows
    MatchesFound = 152,

    //   210 (optional timing and statistical information here)
    TimingInfo = 210,
    // * 220 text msg-id
    TextMsgId = 220,
    //   221 Closing Connection
    ClosingConnection = 221,
    //   230 Authentication successful
    AuthenticationSuccessful = 230,
    //   250 ok (optional timing information here)
    Ok = 250,

    //   330 send response
    SendResponse = 330,

    //   420 Server temporarily unavailable
    ServerTemporarilyUnavailable = 420,
    //   421 Server shutting down at operator request
    ServerShuttingDown = 421,

    //   500 Syntax error, command not recognized
    SyntaxErrorCommandNotRecognized = 500,
    //   501 Syntax error, illegal parameters
    SyntaxErrorIllegalParameters = 501,
    //   502 Command not implemented
    CommandNotImplemented = 502,
    //   503 Command parameter not implemented
    CommandParameterNotImplemented = 503,
    //   530 Access denied
    AccessDenied = 530,
    //   531 Access denied, use "SHOW INFO" for server information
    AccessDeniedShowInfo = 531,
    //   532 Access denied, unknown mechanism
    AccessDeniedUnknownMechanism = 532,
    //   550 Invalid database, use "SHOW DB" for list of databases
    InvalidDatabase = 550,
    //   551 Invalid strategy, use "SHOW STRAT" for a list of strategies
    InvalidStrategy = 551,
    //   552 No match
    NoMatch = 552,
    //   554 No databases present
    NoDatabasesPresent = 554,
    //   555 No strategies available
    NoStrategiesAvailable = 555,
}

impl StatusCode {
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            110 => Some(StatusCode::DatabasePresend),
            111 => Some(StatusCode::StrategiesAvailable),
            112 => Some(StatusCode::DatabaseInfo),
            113 => Some(StatusCode::HelpText),
            114 => Some(StatusCode::ServerInfo),
            130 => Some(StatusCode::ChallengeFollows),
            150 => Some(StatusCode::DefinitionsRetrieved),
            151 => Some(StatusCode::DefinitionFollows),
            152 => Some(StatusCode::MatchesFound),
            210 => Some(StatusCode::TimingInfo),
            220 => Some(StatusCode::TextMsgId),
            221 => Some(StatusCode::ClosingConnection),
            230 => Some(StatusCode::AuthenticationSuccessful),
            250 => Some(StatusCode::Ok),
            330 => Some(StatusCode::SendResponse),
            420 => Some(StatusCode::ServerTemporarilyUnavailable),
            421 => Some(StatusCode::ServerShuttingDown),
            500 => Some(StatusCode::SyntaxErrorCommandNotRecognized),
            501 => Some(StatusCode::SyntaxErrorIllegalParameters),
            502 => Some(StatusCode::CommandNotImplemented),
            503 => Some(StatusCode::CommandParameterNotImplemented),
            530 => Some(StatusCode::AccessDenied),
            531 => Some(StatusCode::AccessDeniedShowInfo),
            532 => Some(StatusCode::AccessDeniedUnknownMechanism),
            550 => Some(StatusCode::InvalidDatabase),
            551 => Some(StatusCode::InvalidStrategy),
            552 => Some(StatusCode::NoMatch),
            554 => Some(StatusCode::NoDatabasesPresent),
            555 => Some(StatusCode::NoStrategiesAvailable),
            _ => None,
        }
    }

    pub fn from_str(code: &str) -> Option<Self> {
        let code = code.parse::<u16>().ok()?;
        StatusCode::from_u16(code)
    }

    /// response data has multiple data, need to read until the line is "."
    pub fn is_multple_data(self) -> bool {
        matches!(
            self,
            Self::DatabasePresend
                | Self::StrategiesAvailable
                | Self::DefinitionsRetrieved
                | Self::DefinitionFollows
                | Self::MatchesFound
                | Self::TextMsgId
        )
    }

    pub fn is_error(self) -> bool {
        (500..=599).contains(&(self as u16))
            || matches!(
                self,
                Self::ServerTemporarilyUnavailable | Self::ServerShuttingDown
            )
    }
}

impl FromStr for StatusCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.parse::<u16>()?;
        StatusCode::from_u16(code).ok_or_else(|| "未知状态码".parse::<u16>().unwrap_err())
    }
}

#[derive(Debug)]
pub struct Response {
    /// status code from first line
    code: StatusCode,
    /// about the metadata of the response
    first_line: String,
    /// content of the response
    pub content: Vec<String>,
}

impl Response {
    /// parse the response from the server
    pub fn parse(content: impl Into<String>) -> Self {
        let line = content.into();
        let (first_line, content) = line.split_once('\n').unwrap_or(("", ""));

        let code = first_line
            .split_once(' ')
            .map(|(code, _)| StatusCode::from_str(code).unwrap())
            .unwrap();
        Response {
            code,
            first_line: first_line.to_string(),
            content: vec![content.to_string()],
        }
    }

    pub fn from_line(line: impl Into<String>) -> Self {
        let line = line.into();
        let (code, metadata) = line
            .split_once(' ')
            .map(|(code, metadata)| (StatusCode::from_str(code).unwrap(), metadata))
            .unwrap();

        Response {
            code,
            first_line: metadata.to_string(),
            content: vec![],
        }
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn first_line(&self) -> &str {
        &self.first_line
    }

    pub fn is_multple_data(&self) -> bool {
        self.code.is_multple_data()
    }

    /// return the count of the response
    /// if the response is not a count response, will return zero
    pub fn count(&self) -> usize {
        self.first_line
            .split_once(' ')
            .map(|(count, _)| count.parse().unwrap())
            .unwrap_or(0)
    }
}
