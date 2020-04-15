use crate::light::{Coordinate, Color};
use crate::light::cprov::{ColorMap, StaticSolidMap};
use crate::light::mask::Mask;
use palette::Mix;

/// A color map layer is similar to a color map.  However, a layer can
/// include "transparency".  When the color for the layer is
/// calculated, the layer receives the color for the position from the
/// layer below it.  This way the layer can transparently merge its
/// color value with the one below.
pub trait ColorMapLayer {
    fn get_color(&self, pos: &Coordinate, color: Color) -> Color;
}

/// A Masked color map layer includes a color map and a mask.  The
/// mask defines how much of the color map is shown.  The mask defines
/// the transparency. If the mask is 0, the layer is not transparent
/// and ignores the given color value.  If the mask is 1 the layer is
/// fully transparent.
pub struct MaskedColorMapLayer<C, M> {
    map: C,
    mask: M,
}

impl<C, M> ColorMapLayer for MaskedColorMapLayer<C, M>
where
    C: ColorMap,
    M: Mask,
{
    fn get_color(&self, pos: &Coordinate, color: Color) -> Color {
        self.map.get_color(&pos).mix(&color, self.mask.get_value(&pos))
    }
}

/// A solid layer is a convenience type for a static solid layer with
/// a mask.
pub type SolidLayer<M> = MaskedColorMapLayer<StaticSolidMap, M>;

pub struct Layers;

impl Layers {
    pub fn new_solid<M: Mask>(color: Color, mask: M) -> SolidLayer<M> {
        MaskedColorMapLayer::<StaticSolidMap, M> {
            map: StaticSolidMap::new(color),
            mask: mask,
        }
    }
}
