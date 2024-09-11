use std::iter::zip;

pub fn clasificar(conocidos: &[&[f64]], clases: &[i32], muestra: &[f64], k: usize) -> Option<i32> {
    // All of these operations at this if statement are comparisons or memory-access operations, so we can assume that all this operation has a t1 cost.
    if conocidos.len() != clases.len() // t1
        || conocidos.is_empty() // + t2
        || muestra.len() != conocidos[0].len()
    // + t3
    {
        return None; // + t4
    }

    // t5
    // This is storing the index of the vector
    let mut distances: Vec<(usize, f64)> = Vec::new();

    // Since euclidean_distance has 7*O(1) + 4*O(N), having a growth order of N, this part of the code takes
    for (index, vector) in conocidos.iter().enumerate() {
        let distance = euclidean_distance(vector, muestra);
        distances.push((index, distance.unwrap()))
    }

    // Here should occur the distance assign for each `conocidos` value, then sort them by distance, and select the most frequent from the bottom-k.

    Some(0)
}

/// Evaluates the euclidean distance between x and y.
///
/// Returns None if the vectors are not of the same length. Otherwise returns a f64.
///
/// This function has a runtime complexity equals to (t1+t2+t3+t9) + N(t4+t5+t6+t7+t8), and a growth order of N.
///
/// So, euclidean_distance~~O(N).
fn euclidean_distance(x: &[f64], y: &[f64]) -> Option<f64> {
    // Both x.len() and y.len() haves constant time execution.
    // t1
    if x.len() != y.len() {
        return None;
    }

    // Assignments are constant-time.
    // t2
    let mut squared_sum: f64 = 0.0;

    /*  zip creation is constant time (because it works by accessing to the iterators of the vectors).
    the assignation for x_i and y_i are constant time (because they read a reference).
    these assignations occurs n times, where n is the dimension of x and y vectors. */

    //   N(t4 + t5)                t3
    for (x_i, y_i) in zip(x, y) {
        // I'm not sure, but this seems that all these operations are constant time.
        //          t6  +   t7  +   t8
        squared_sum += (y_i - x_i).powi(2);
    }

    //              t9
    Some(squared_sum.sqrt())
}

#[cfg(test)]
mod test {
    use crate::homework::knn::euclidean_distance;

    #[test]
    fn test_euclidean_distance_same_length() {
        let x = [0., 0.];
        let y = [1., 1.];
        let expected = 2_f64.sqrt();
        assert_eq!(euclidean_distance(&x, &y).unwrap(), expected)
    }

    #[test]
    fn test_euclidean_distance_different_length() {
        let x = [0., 0.];
        let y = [0., 0., 0.];
        let expected = None;
        assert_eq!(euclidean_distance(&x, &y), expected)
    }
}
