// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{ImageRef, ImageRefMut, FuzzyZero, BGRA8, f64_bound};

/// Performs an arithmetic composition.
///
/// - `src1` and `src2` image pixels should have a **premultiplied alpha**.
/// - `dest` image pixels will have a **premultiplied alpha**.
///
/// # Panics
///
/// When `src1`, `src2` and `dest` have different sizes.
pub fn arithmetic_composite(
    k1: f64,
    k2: f64,
    k3: f64,
    k4: f64,
    src1: ImageRef,
    src2: ImageRef,
    dest: ImageRefMut,
) {
    assert!(src1.width == src2.width && src1.width == dest.width);
    assert!(src1.height == src2.height && src1.height == dest.height);

    let calc = |i1, i2, max| {
        let i1 = i1 as f64 / 255.0;
        let i2 = i2 as f64 / 255.0;
        let result = k1 * i1 * i2 + k2 * i1 + k3 * i2 + k4;
        f64_bound(0.0, result, max)
    };

    let mut i = 0;
    for (c1, c2) in src1.data.iter().zip(src2.data.iter()) {
        let a = calc(c1.a, c2.a, 1.0);
        if a.is_fuzzy_zero() {
            i += 1;
            continue;
        }

        let r = (calc(c1.r, c2.r, a) * 255.0) as u8;
        let g = (calc(c1.g, c2.g, a) * 255.0) as u8;
        let b = (calc(c1.b, c2.b, a) * 255.0) as u8;
        let a = (a * 255.0) as u8;

        dest.data[i] = BGRA8 { r, g, b, a };

        i += 1;
    }
}
