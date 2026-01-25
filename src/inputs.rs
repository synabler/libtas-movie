//! Module that defines an input sequence.

use core::{fmt::Display, str::FromStr};

/// An error while parsing inputs, containing the type and the string that caused the error.
#[derive(Debug)]
pub enum InvalidInputsError {
    Line(String),
    Keyboard(String),
    Mouse(String),
}

/// A keyboard input in a frame.
/// Each element is the Xlib KeySym value of a key.
///
/// # Syntax
/// `KeyboardInput` starts with `K`, followed by an unordered list of keys,
/// expressed in hexadecimal and separated by `:`.
///
/// For example, `K7a:ff53` means that the keys `0x7a (z)` and `0xff53 (right)`
/// were pressed (or held down) on that frame.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyboardInput(pub Vec<u32>);

impl FromStr for KeyboardInput {
    type Err = InvalidInputsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('K') else {
            return Err(InvalidInputsError::Keyboard(s.to_owned()));
        };
        let Ok(keys) = s
            .split(':')
            .map(|s| u32::from_str_radix(s, 16))
            .collect::<Result<Vec<u32>, _>>()
        else {
            return Err(InvalidInputsError::Keyboard(s.to_owned()));
        };
        Ok(Self(keys))
    }
}

impl Display for KeyboardInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

/// The reference mode of a mouse input.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReferenceMode {
    /// Absolute coordinates.
    #[default]
    Absolute,
    /// Relative coordinates.
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Absolute => write!(f, "A"),
            Self::Relative => write!(f, "R"),
        }
    }
}

/// A mouse input in a frame.
///
/// # Syntax
/// `MouseInput` starts with `M`, followed by the format `xpos:ypos:X:12345:0`.
/// - `xpos` and `ypos` are the coordinates of the pointer.
/// - `X` is [`ReferenceMode`], either `A` for absolute or `R` for relative.
/// - Each of `12345` is whether each mouse button is pressed (or held down).
///   Note that, according to [the documentation](https://clementgallet.github.io/libTAS/guides/format/),
///   The only thing that matters is whether the character is `.` or not.
///   i.e. a button is pressed if the character in the corresponding position is not `.`.
/// - TODO: what is `0` at the end?
///
/// For example, `M166:270:A:1....:0` means that the absolute coordinate `(166, 270)`
/// was clicked (or held down) with the left mouse button on that frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MouseInput {
    /// X-coordinate of the pointer (can be negative).
    pub xpos: i32,
    /// Y-coordinate of the pointer (can be negative).
    pub ypos: i32,
    /// Reference mode.
    pub reference_mode: ReferenceMode,
    /// Left mouse click (aka button 1).
    pub left_click: bool,
    /// Middle mouse click (aka button 2).
    pub middle_click: bool,
    /// Right mouse click (aka button 3).
    pub right_click: bool,
    /// Mouse button 4 click.
    pub button4: bool,
    /// Mouse button 5 click.
    pub button5: bool,
}

impl FromStr for MouseInput {
    type Err = InvalidInputsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('M') else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let mut tokens = s.split(':');

        let Some(xpos) = tokens.next() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let Ok(xpos) = xpos.parse::<i32>() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let Some(ypos) = tokens.next() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let Ok(ypos) = ypos.parse::<i32>() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let Some(reference_mode) = tokens.next() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let Ok(reference_mode) = reference_mode.parse::<ReferenceMode>() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let Some(clicks) = tokens.next() else {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
        };
        let clicks = clicks.as_bytes();
        if clicks.len() != 5 {
            return Err(InvalidInputsError::Mouse(s.to_owned()));
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

/// An input in a frame.
/// Controllers, flags, and variable framerates are not implemented yet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Input {
    /// Keyboard input.
    pub keyboard: Option<KeyboardInput>,
    /// Mouse input.
    pub mouse: Option<MouseInput>,
    pub controllers: (), // TODO
    pub flags: (),       // TODO
    pub framerate: (),   // TODO
}

impl FromStr for Input {
    type Err = InvalidInputsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "|" {
            return Ok(Self::default());
        }

        let Some(line) = s.strip_prefix('|') else {
            return Err(InvalidInputsError::Line(s.to_owned()));
        };
        let Some(line) = line.strip_suffix('|') else {
            return Err(InvalidInputsError::Line(line.to_owned()));
        };

        let mut input = Self::default();
        for section in line.split('|') {
            match section.chars().next() {
                Some('K') => {
                    input.keyboard = Some(section.parse()?);
                }
                Some('M') => {
                    input.mouse = Some(section.parse()?);
                }
                Some('C') => {
                    // TODO
                }
                Some('F') => {
                    // TODO
                }
                Some('T') => {
                    // TODO
                }
                _ => {
                    return Err(InvalidInputsError::Line(line.to_owned()));
                }
            }
        }
        Ok(input)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

/// A sequence of inputs, one per frame.
#[derive(Clone, Debug, Default)]
pub struct Inputs(pub Vec<Input>);

impl core::ops::Index<usize> for Inputs {
    type Output = Input;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl FromStr for Inputs {
    type Err = InvalidInputsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = vec![];

        for line in s.split('\n') {
            // "each line that starts with the character `|` is an input frame."
            if !line.starts_with('|') {
                continue;
            }
            inputs.push(line.parse::<Input>()?);
        }
        Ok(Self(inputs))
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for input in &self.0 {
            writeln!(f, "{input}")?;
        }
        Ok(())
    }
}
