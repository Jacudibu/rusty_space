use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::image::Image;
use std::path::Path;

#[must_use]
fn is_on_edge(a: u32, b: u32, a_max: u32, b_max: u32, offset_to_edge: u32, length: u32) -> bool {
    if a == offset_to_edge || a == a_max - 1 - offset_to_edge {
        if b <= length - offset_to_edge {
            return true;
        }
        if b >= b_max - 1 - length + offset_to_edge {
            return true;
        }
    }

    false
}

#[must_use]
fn is_in_corner(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offset_to_corner: u32,
    corner_length: u32,
) -> bool {
    is_on_edge(x, y, width, height, offset_to_corner, corner_length)
        || is_on_edge(y, x, height, width, offset_to_corner, corner_length)
}

/// Generates an [Image] with highlighted corners from an image file at the provided path and returns the strong [Handle] to it.
#[must_use]
pub fn generate_image_with_highlighted_corners<P>(
    path: P,
    assets: &mut Assets<Image>,
) -> Handle<Image>
where
    P: AsRef<Path>,
{
    const DISTANCE_TO_EDGE: u32 = 3;
    const LEN: u32 = 5;

    let original = image::open(path).unwrap().into_rgba8();

    let width = original.width() + DISTANCE_TO_EDGE * 2;
    let height = original.height() + DISTANCE_TO_EDGE * 2;

    let mut outlined_image = image::ImageBuffer::from_fn(width, height, |x, y| {
        if is_in_corner(x, y, width, height, 0, LEN) {
            // White corners right at the edge
            image::Rgba([255, 255, 255, 255])
        } else if is_in_corner(x, y, width, height, 1, LEN) {
            // Black corners inside the white corners for contrast
            image::Rgba([0, 0, 0, 255])
        } else {
            image::Rgba([0, 0, 0, 0])
        }
    });

    image::imageops::overlay(
        &mut outlined_image,
        &original,
        DISTANCE_TO_EDGE as i64,
        DISTANCE_TO_EDGE as i64,
    );

    assets.add(Image::from_dynamic(
        outlined_image.into(),
        false,
        RenderAssetUsages::RENDER_WORLD,
    ))
}
