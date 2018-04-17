// Copyright (c) 2016-2017 Ministerio de Fomento
//                    Instituto de Ciencias de la Construcción Eduardo Torroja (IETcc-CSIC)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Author(s): Rafael Villar Burke <pachi@ietcc.csic.es>,
//            Daniel Jiménez González <dani@ietcc.csic.es>

// -----------------------------------------------------------------------------------
// Vector utilities
// -----------------------------------------------------------------------------------

//export const zip = (...rows: any[]): any[] => [...rows[0]].map((_, c) => rows.map(row => row[c]));

// Elementwise sum res[i] = vec1[i] + vec2[i] + ... + vecj[i]
pub fn veclistsum(veclist: &[Vec<f32>]) -> Vec<f32> {
    let maxlen: usize = veclist.iter().map(|lst| lst.len()).max().unwrap_or(0_usize);
    veclist.iter().fold(vec![0.0], |acc, ref x| {
        (0..maxlen)
            .map(|idx| acc.get(idx).unwrap_or(&0.0) + x.get(idx).unwrap_or(&0.0))
            .collect::<Vec<_>>()
    })
}

// // Elementwise minimum min res[i] = min(vec1[i], vec2[i])
pub fn vecvecmin(vec1: &[f32], vec2: &[f32]) -> Vec<f32> {
    vec1.iter()
        .enumerate()
        .map(|(ii, el)| el.min(*vec2.get(ii).unwrap_or(&0.0)))
        .collect()
}

// // Elementwise sum of arrays
pub fn vecvecsum(vec1: &[f32], vec2: &[f32]) -> Vec<f32> {
    vec1.iter()
        .enumerate()
        .map(|(ii, el)| el + vec2.get(ii).unwrap_or(&0.0))
        .collect()
}

// // Elementwise difference res[i] = vec1[i] - vec2[i]
pub fn vecvecdif(vec1: &[f32], vec2: &[f32]) -> Vec<f32> {
    vec1.iter()
        .enumerate()
        .map(|(ii, el)| el - vec2.get(ii).unwrap_or(&0.0))
        .collect()
}

// // Elementwise multiplication res[i] = vec1[i] * vec2[i]
pub fn vecvecmul(vec1: &[f32], vec2: &[f32]) -> Vec<f32> {
    vec1.iter()
        .enumerate()
        .map(|(ii, el)| el * vec2.get(ii).unwrap_or(&0.0))
        .collect()
}

// // Multiply vector by scalar
pub fn veckmul(vec1: &[f32], k: f32) -> Vec<f32> {
    vec1.iter().map(|el| el * k).collect()
}

// // Sum all elements in a vector
pub fn vecsum(vec: &[f32]) -> f32 {
    vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vecops_veclistsum() {
        let myvec = vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 2.0, 2.0],
            vec![3.0, 3.0, 3.0],
        ];
        assert_eq!(vec![6.0, 6.0, 6.0], veclistsum(&myvec));
    }

    #[test]
    fn vecops_vecvecmin() {
        assert_eq!(
            vec![2.0, 1.0, 2.0],
            vecvecmin(&[2.0, 2.0, 2.0], &[4.0, 1.0, 2.0])
        );
    }

    #[test]
    fn vecops_vecvecsum() {
        assert_eq!(
            vec![4.0, 4.0, 4.0],
            vecvecsum(&[2.0, 1.0, 3.0], &[2.0, 3.0, 1.0])
        );
    }

    #[test]
    fn vecops_vecvecdif() {
        assert_eq!(
            vec![1.0, 1.0, 1.0],
            vecvecdif(&[2.0, 3.0, 4.0], &[1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn vecops_vecvecmul() {
        assert_eq!(
            vec![1.0, 6.0, 4.0],
            vecvecmul(&[1.0, 3.0, 2.0], &[1.0, 2.0, 2.0])
        );
    }

    #[test]
    fn vecops_veckmul() {
        assert_eq!(vec![2.0, 4.0, 6.0], veckmul(&[1.0, 2.0, 3.0], 2.0));
    }

    #[test]
    fn vecops_vecsum() {
        assert_eq!(9.0, vecsum(&[2.0, 3.0, 4.0]));
    }
}
