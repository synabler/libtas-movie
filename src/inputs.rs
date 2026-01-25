use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum InvalidInputs {
    Line(String),
    Keyboard(String),
    Mouse(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyboardInput(pub Vec<u32>);

impl FromStr for KeyboardInput {
    type Err = InvalidInputs;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('K') else {
            return Err(InvalidInputs::Keyboard(s.to_owned()));
        };
        let Ok(keys) = s
            .split(':')
            .map(|s| u32::from_str_radix(s, 16))
            .collect::<Result<Vec<u32>, _>>()
        else {
            return Err(InvalidInputs::Keyboard(s.to_owned()));
        };
        Ok(KeyboardInput(keys))
    }
}

impl Display for KeyboardInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K")?;
        for (idx, key) in self.0.iter().enumerate() {
            if idx != 0 {
                write!(f, ":")?;
            }
            write!(f, "{key:x}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReferenceMode {
    #[default]
    Absolute,
    Relative,
}

impl FromStr for ReferenceMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Absolute),
            "R" => Ok(Self::Relative),
            _ => Err(()),
        }
    }
}

impl Display for ReferenceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute => write!(f, "A"),
            Self::Relative => write!(f, "R"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MouseInput {
    pub xpos: i32,
    pub ypos: i32,
    pub reference_mode: ReferenceMode,
    pub left_click: bool,
    pub middle_click: bool,
    pub right_click: bool,
    pub button4: bool,
    pub button5: bool,
}

impl FromStr for MouseInput {
    type Err = InvalidInputs;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('M') else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let mut tokens = s.split(':');

        let Some(xpos) = tokens.next() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let Ok(xpos) = xpos.parse::<i32>() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let Some(ypos) = tokens.next() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let Ok(ypos) = ypos.parse::<i32>() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let Some(reference_mode) = tokens.next() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let Ok(reference_mode) = reference_mode.parse::<ReferenceMode>() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let Some(clicks) = tokens.next() else {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        };
        let clicks = clicks.as_bytes();
        if clicks.len() != 5 {
            return Err(InvalidInputs::Mouse(s.to_owned()));
        }
        let left_click = clicks[0] != b'.';
        let middle_click = clicks[1] != b'.';
        let right_click = clicks[2] != b'.';
        let button4 = clicks[3] != b'.';
        let button5 = clicks[4] != b'.';

        Ok(Self {
            xpos,
            ypos,
            reference_mode,
            left_click,
            middle_click,
            right_click,
            button4,
            button5,
        })
    }
}

impl Display for MouseInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "M{}:{}:{}:{}{}{}{}{}:0",
            self.xpos,
            self.ypos,
            self.reference_mode,
            if self.left_click { '1' } else { '.' },
            if self.middle_click { '2' } else { '.' },
            if self.right_click { '3' } else { '.' },
            if self.button4 { '4' } else { '.' },
            if self.button5 { '5' } else { '.' },
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Input {
    pub keyboard: Option<KeyboardInput>,
    pub mouse: Option<MouseInput>,
    pub controllers: (), // TODO
    pub flags: (),       // TODO
    pub framerate: (),   // TODO
}

impl FromStr for Input {
    type Err = InvalidInputs;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "|" {
            return Ok(Self::default());
        }

        let Some(line) = s.strip_prefix('|') else {
            return Err(InvalidInputs::Line(s.to_owned()));
        };
        let Some(line) = line.strip_suffix('|') else {
            return Err(InvalidInputs::Line(line.to_owned()));
        };

        let mut input = Input::default();
        for section in line.split('|') {
            match section.chars().next() {
                Some('K') => {
                    input.keyboard = Some(section.parse()?);
                }
                Some('M') => {
                    input.mouse = Some(section.parse()?);
                }
                Some('C') => todo!(),
                Some('F') => todo!(),
                Some('T') => todo!(),
                _ => {
                    return Err(InvalidInputs::Line(line.to_owned()));
                }
            }
        }
        Ok(input)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")?;
        if let Some(keyboard) = &self.keyboard {
            write!(f, "{keyboard}|")?;
        }
        if let Some(mouse) = &self.mouse {
            write!(f, "{mouse}|")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Inputs(pub Vec<Input>);

impl std::ops::Index<usize> for Inputs {
    type Output = Input;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl FromStr for Inputs {
    type Err = InvalidInputs;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = vec![];

        for line in s.split('\n') {
            // "each line that starts with the character `|` is an input frame."
            if !line.starts_with('|') {
                continue;
            }
            inputs.push(line.parse::<Input>()?);
        }
        Ok(Inputs(inputs))
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for input in &self.0 {
            writeln!(f, "{input}")?;
        }
        Ok(())
    }
}
