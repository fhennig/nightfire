use crate::light::{Coordinate, Color, ColorsExt};
use crate::light::cmap::{ColorMap, StaticSolidMap};
use crate::light::mask::{Mask, ActivatableMask};
use palette::Mix;

/// A color map layer is similar to a color map.  However, a layer can
/// include "transparency".  When the color for the layer is
/// calculated, the layer receives the color for the position from the
/// layer below it.  This way the layer can transparently merge its
/// color value with the one below.

//pub trait ColorMapLayer {
//    fn get_color(&self, pos: &Coordinate, color: Color) -> Color;
//}

/// A Masked color map layer includes a color map and a mask.  The
/// mask defines how much of the color map is shown.  The mask defines
/// the transparency. If the mask is 0, the layer is not transparent
/// and ignores the given color value.  If the mask is 1 the layer is
/// fully transparent.
pub struct Layer<C, M> {
    pub map: C,
    pub mask: M,
}

impl<C, M> Layer<C, M>
where
    C: ColorMap,
    M: Mask,
{
    pub fn new(color_map: C, mask: M) -> Layer<C, M> {
        Layer {
            map: color_map,
            mask: mask,
        }
    }

    pub fn get_color(&self, pos: &Coordinate, color: Color) -> Color {
        // mask 0 -> other color ("transparent")
        // mask 1 - >this layer's color ("solid")
        self.map.get_color(&pos).mix(&color, 1. - self.mask.get_value(&pos))
    }
}

/// A solid layer is a convenience type for a static solid layer with
/// a mask.
pub type SolidLayer<M> = Layer<StaticSolidMap, M>;

pub type MaskLayer<M> = SolidLayer<ActivatableMask<M>>;

pub struct Layers;

impl Layers {
    /// Creates a layer with a solid color and a given mask.
    pub fn new_solid<M: Mask>(color: Color, mask: M) -> SolidLayer<M> {
        Layer::<StaticSolidMap, M> {
            map: StaticSolidMap::new(color),
            mask: mask,
        }
    }

    /// Creates a black layer with an activatable mask.  The mask is
    /// off by default.
    pub fn new_mask<M: Mask>(mask: M) -> MaskLayer<M> {
        Layer::<StaticSolidMap, ActivatableMask<M>> {
            map: StaticSolidMap::new(Color::black()),
            mask: ActivatableMask::<M>::new(mask, false),
        }
    }
}
