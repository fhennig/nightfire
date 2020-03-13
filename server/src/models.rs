use palette::encoding::Srgb;
use palette::encoding::linear::Linear;
use palette::rgb::Rgb;


pub type PinValue = f64;
pub type Color = Rgb<Linear<Srgb>, PinValue>;
