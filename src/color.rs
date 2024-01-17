/// An RGB color with 16 bits per channel.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl Color {
    /// The perceived lightness of the color
    /// as a value between `0` (black) and `100` (white)
    /// where `50` is the perceptual "middle grey".
    /// ```
    /// # use term_color::Color;
    /// # let color = Color::default();
    /// let is_dark = color.perceived_lightness() <= 50;
    /// ```
    pub fn perceived_lightness(&self) -> u8 {
        luminance_to_perceived_lightness(luminance(self))
    }
}

impl Color {
    /// Parses an X11 color (see `man xparsecolor`).
    pub(crate) fn parse_x11(input: &str) -> Option<Self> {
        let raw_parts = input.strip_prefix("rgb:")?;
        let mut parts = raw_parts.split('/');
        let red = parse_channel(parts.next()?)?;
        let green = parse_channel(parts.next()?)?;
        let blue = parse_channel(parts.next()?)?;
        Some(Color { red, green, blue })
    }
}

fn parse_channel(input: &str) -> Option<u16> {
    let len = input.len();
    // From the xparsecolor man page:
    //   h indicates the value scaled in 4 bits,
    //   hh the value scaled in 8 bits,
    //   hhh the value scaled in 12 bits, and
    //   hhhh the value scaled in 16 bits, respectively.
    let shift = (1..=4).contains(&len).then_some(16 - 4 * len as u16)?;
    Some(u16::from_str_radix(input, 16).ok()? << shift)
}

// Implementation of determining the perceived lightness
// follows this excellent answer: https://stackoverflow.com/a/56678483

fn srgb_to_lin(channel: f64) -> f64 {
    if channel < 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

// Luminance (Y)
fn luminance(color: &Color) -> f64 {
    let r = color.red as f64 / u16::MAX as f64;
    let g = color.green as f64 / u16::MAX as f64;
    let b = color.blue as f64 / u16::MAX as f64;
    0.2126 * srgb_to_lin(r) + 0.7152 * srgb_to_lin(g) + 0.0722 * srgb_to_lin(b)
}

// Perceptual lightness (L*)
fn luminance_to_perceived_lightness(luminance: f64) -> u8 {
    if luminance < 216. / 24389. {
        (luminance * (24389. / 27.)) as u8
    } else {
        (luminance.powf(1. / 3.) * 116. - 16.) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_has_perceived_lightness_zero() {
        let black = Color::default();
        assert_eq!(0, black.perceived_lightness())
    }

    #[test]
    fn white_has_perceived_lightness_100() {
        let black = Color {
            red: u16::MAX,
            green: u16::MAX,
            blue: u16::MAX,
        };
        assert_eq!(100, black.perceived_lightness())
    }
}
