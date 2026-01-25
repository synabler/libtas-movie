use std::str::FromStr;

pub struct InvalidConfig;

#[derive(Clone, Debug, Default)]
pub struct GeneralConfig {
    game_name: String,
}

impl FromStr for GeneralConfig {
    type Err = InvalidConfig;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();
        // TODO
        Ok(config)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TimetrackConfig {
    time: Option<usize>,
}

impl FromStr for TimetrackConfig {
    type Err = InvalidConfig;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();
        // TODO
        Ok(config)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Config {
    general: GeneralConfig,
    mainthread_timetrack: TimetrackConfig,
}

impl FromStr for Config {
    type Err = InvalidConfig;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((general, timetrack)) = s.split_once("\n\n[mainthread_timetrack]") else {
            return Err(InvalidConfig);
        };
        Ok(Self {
            general: general.parse()?,
            mainthread_timetrack: timetrack.parse()?,
        })
    }
}
