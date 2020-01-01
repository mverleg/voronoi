
pub fn crop<T: PartialOrd<T>>(val: T, upper: T, lower: T) -> T {
    if val > upper {
        return upper;
    }
    if val < lower {
        return lower;
    }
    val
}
