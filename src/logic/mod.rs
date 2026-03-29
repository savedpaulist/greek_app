pub mod diacritics;
pub mod paradigm;
pub mod sm2;

/// Returns a randomly shuffled list of indices `0..n` using Fisher-Yates.
/// Uses `js_sys::Math::random()` on WASM; returns in-order on other targets.
pub fn shuffled_indices(n: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..n).collect();
    #[cfg(target_arch = "wasm32")]
    {
        for i in (1..n).rev() {
            let j = (js_sys::Math::random() * (i + 1) as f64) as usize;
            v.swap(i, j);
        }
    }
    v
}
