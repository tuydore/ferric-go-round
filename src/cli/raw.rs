#[derive(clap::Parser)]
#[command(author, version, about = include_str!("diagram.txt"))]
pub(crate) struct Cli {
    /// Target file.
    pub(crate) filepath: std::path::PathBuf,

    /// Number of panels to split image into.
    #[arg(short = 'n')]
    pub(crate) num_panels: u32,

    /// Target image height in pixels. If not given, will preserve current image height.
    #[arg(short = 'q')]
    pub(crate) panel_height_px: Option<u32>,

    /// Gaussian blur radius as fraction of panel height.
    #[arg(short = 's', default_value_t = 0.01)]
    pub(crate) sigma: f32,

    /// Radius of border around image.
    #[arg(short = 'b', default_value_t = 1)]
    pub(crate) thumbnail_border_radius: u32,

    /// Border color, given as "R,G,B" (e.g. 255,255,0). Defaults to black.
    #[arg(short = 'c')]
    pub(crate) thumbnail_border_color: Option<Vec<u8>>,

    /// Ratio of thumbnail width to panel width (including border).
    #[arg(short = 'f', default_value_t = 0.8)]
    pub(crate) thumbnail_frac_panel_width: f32,
}