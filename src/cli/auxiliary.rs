#[derive(Clone, Copy)]
pub(crate) struct ImageSize {
    pub(crate) height: u32,
    pub(crate) width: u32,
}

pub(crate) fn overlay_centered<I, J>(bottom: &mut I, top: &J)
where
    I: image::GenericImage,
    J: image::GenericImageView<Pixel = I::Pixel> 
{
    let bw = bottom.width();
    let bh = bottom.height();
    let tw = top.width();
    let th = top.height();

    image::imageops::overlay(bottom, top, ((bw - tw) / 2) as i64, ((bh - th) / 2) as i64)
}