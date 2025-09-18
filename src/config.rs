pub const PREBUILT_TARGET_BYTES: &[u8] = &[
    199, 177, 77, 85, 229, 154, 144, 124, 52, 76, 100, 34, 91, 125, 69, 62, 83, 231, 140, 136, 83,
    94, 96, 75, 50, 99, 114, 37, 35, 96, 74, 118, 120, 99, 94, 101, 39, 83, 227, 182, 139, 34, 118,
    80, 77, 10, 85, 83, 76, 9, 102, 72, 35, 105, 206, 185, 52, 9, 80, 100, 104, 107, 74, 39, 112,
    77, 104, 199, 178,
];

pub const SUSPEND_ATTEMPT: u8 = 5;
pub const SUSPEND_UNIQUE_SHOULD: usize = 6;

pub const SUSPEND_UNTIL: u64 = 1800; // second(s)
pub const SUSPEND_EACH: u64 = 3; // second(s)

pub const IDLE_AFTER_FINISH: u64 = 3;
