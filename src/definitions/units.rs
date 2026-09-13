pub struct ByteUnits;

impl ByteUnits {
    pub const KB: usize = 1024;
    pub const MB: usize = ByteUnits::KB * 1024;
    pub const GB: usize = ByteUnits::MB * 1024;
    #[cfg(target_pointer_width = "64")]
    pub const TB: usize = ByteUnits::GB * 1024;
    #[cfg(target_pointer_width = "64")]
    pub const PB: usize = ByteUnits::TB * 1024;
    #[cfg(target_pointer_width = "64")]
    pub const EB: usize = ByteUnits::PB * 1024;
    #[cfg(target_pointer_width = "128")]
    pub const ZB: usize = ByteUnits::EB * 1024;
    #[cfg(target_pointer_width = "128")]
    pub const YB: usize = ByteUnits::ZB * 1024;
    #[cfg(target_pointer_width = "128")]
    pub const RB: usize = ByteUnits::YB * 1024;
    #[cfg(target_pointer_width = "128")]
    pub const QB: usize = ByteUnits::RB * 1024;  // Yeah, this will for sure be used someday
}

