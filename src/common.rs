/* Common PLC functions, SCL for example */
pub fn scale (in_val: u32, in_low: u32, in_high: u32, out_low: u32, out_high: u32) -> u32 {
    if in_val > in_low { // If in value in lower than the low scale value, return 0 as signal is bad
        let in_scale = in_high - in_low;
        let out_scale = out_high - out_low;
        return (((in_val - in_low) * out_scale) / in_scale) + out_low;
    } else {
        return 0;
    }
}