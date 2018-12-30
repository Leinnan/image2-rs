pub trait Color: Sync + Send {
    fn name() -> &'static str;
    fn channels() -> usize;
    fn has_alpha() -> bool;
}

macro_rules! make_color {
    ($name:ident, $name_s:expr, $channels:expr, $alpha:expr) => {
        #[cfg_attr(
            feature = "ser",
            derive(serde_derive::Serialize, serde_derive::Deserialize)
        )]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name;

        impl Color for $name {
            fn channels() -> usize {
                $channels
            }
            fn has_alpha() -> bool {
                $alpha
            }
            fn name() -> &'static str {
                $name_s
            }
        }
    };
}

make_color!(Gray, "gray", 1, false);
make_color!(Rgb, "rgb", 3, false);
make_color!(Bgr, "bgr", 3, false);
make_color!(RgbPacked, "rgb_packed", 1, false);
make_color!(Rgba, "rgba", 4, true);
make_color!(Bgra, "bgra", 4, true);
make_color!(Cmyk, "cmyk", 4, false);
make_color!(Yuv, "yuv", 3, false);
