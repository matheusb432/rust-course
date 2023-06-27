fn main() {
    // NOTE annotating numeric literals will type it
    let num = 15u8;
    let num = 999_usize;
    // * Converting between different types, data loss may happen if the
    // * converted type is smaller than the original type
    let num = 15u8 as u16;

    // * Operations must be done on the same type
    let num = 15u16 as u8 + 15u8;

    // * When a number is too small, it will be clamped to it's closest value
    let num = 800.5f32 as u8; // =255
    let num = -300f32 as u8; // =0
    let num = 800.5f32 as i8; // =127
    let num = -300f32 as i8; // =-128
}
