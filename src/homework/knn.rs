// Author: Ricardo Arias.

use std::{cmp::Ordering, collections::HashMap, iter::zip};

/// This function assigns a class from `clases` for `muestra`, comparing the euclidean distances between `muestra` and all `conocidos` vectors.
///
/// This function haves a computational complexity that depends on:
///     - `conocidos`' shape (its rows and columns amount),
///     - the amount of `clases`
///     - the `k` neighbors to compare.
///
/// Having all of these factors, the analytical time that this function takes is of these form:
/// t1
pub fn clasificar(conocidos: &[&[f64]], clases: &[i32], muestra: &[f64], k: usize) -> Option<i32> {
    /* First of all, take these considerations:
    - N will be the amount of characteristics of `conocidos` matrix (it's columns).
    - M will be the amount of registries (the matrix's rows).
    - We'll name C for the amount of classes at `clases` (clases.len()) */
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
    // N^2 operations to be done.
    //          N(t6 + t7)                              t8
    for (index, vector) in conocidos.iter().enumerate() {
        let distance = euclidean_distance(vector, muestra);
        distances.push((index, distance.unwrap()))
    }

    distances.sort_by(
        // First, we create a lambda where a and b are references to tuples.
        //          t9            +       t10
        |a: &(usize, f64), b: &(usize, f64)| {
            /* a.1 and b.1 makes reference to the distance value.
            So, this operation takes the a distance value, compares
            with the b distance value, and then assign it if is greater,
            lesser or equal (or None, this possibility forces us to unwrap the result).

            The operations below are constant time (because basically they are memory-access operations), so
            t11 + 12 + t13 + t14 */
            a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)
        },
        /* Finally, sort_by applies this function that returns a comparator, and sorts the vector.
        All these operations repeats N(log(N)) times, so the total cost for this operation will be
        (t9 + t10 + t11 + t12 + t13 + t14)N(log(N)) */
    );
    // These yield and take operation are constant time (I guess), such the take function.

    // Ensuring that `k` is not longer than the available neighbors. This is just an if comparison.
    // t26
    let k = k.min(distances.len());

    // t14 + t15 + t16 + t17
    let nearest_neighbors = distances.iter().take(k);
    // t18
    let mut class_count = HashMap::new();

    // We iterate here over a k-length vector.
    nearest_neighbors.for_each(|&(index, _)| {
        /* All of these operations takes many other operations, but we can simplify some things:
            - We can assume that the hash function to hash clases[index] is time constant.
            - All lookup, insertion or update operations are constant time.

           Considering all those things, this operation just haves 4 constants.
           k(t19 + t20 + t21 + t22)
        */
        *class_count.entry(clases[index]).or_insert(0) += 1;
    });

    // This operation iterates this HashMap and finds the maximum element.
    // t23 + t24*k + t25
    class_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(class, _)| class)
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
    use crate::homework::knn::*;

    #[test]
    fn test_euclidean_distance_same_length() {
        let x: [f64; 2] = [0., 0.];
        let y: [f64; 2] = [1., 1.];
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

    #[test]
    fn test_clasificar_simple_case() {
        let conocidos: &[&[f64]] = &[
            &[1.0, 2.0], // Point A
            &[2.0, 3.0], // Point B
            &[3.0, 4.0], // Point C
        ];
        let clases: &[i32] = &[0, 1, 1]; // Classes A: 0, B: 1, C: 1
        let muestra: &[f64] = &[2.5, 3.5]; // Test point closer to B and C
        let k = 2;

        let predicted_class = clasificar(conocidos, clases, muestra, k);
        assert_eq!(predicted_class, Some(1), "The predicted class should be 1.");
    }

    #[test]
    fn test_clasificar_all_same_class() {
        let conocidos: &[&[f64]] = &[&[1.0, 2.0], &[2.0, 3.0], &[3.0, 4.0]];
        let clases: &[i32] = &[0, 0, 0]; // All points belong to class 0
        let muestra: &[f64] = &[2.5, 3.5];
        let k = 2;

        let predicted_class = clasificar(conocidos, clases, muestra, k);
        assert_eq!(predicted_class, Some(0), "The predicted class should be 0.");
    }

    #[test]
    fn test_clasificar_edge_case_k_larger_than_data() {
        let conocidos: &[&[f64]] = &[&[1.0, 2.0], &[2.0, 3.0]];
        let clases: &[i32] = &[0, 1];
        let muestra: &[f64] = &[2.5, 3.5];
        let k = 5; // k is larger than the number of known points

        let predicted_class = clasificar(conocidos, clases, muestra, k);
        // Behavior depends on your implementation, assuming it handles this by falling back to considering all points
        assert_eq!(predicted_class, Some(1), "The predicted class should be 1.");
    }

    #[test]
    fn test_clasificar_invalid_input() {
        let conocidos: &[&[f64]] = &[&[1.0, 2.0], &[2.0, 3.0]];
        let clases: &[i32] = &[0]; // Mismatched length
        let muestra: &[f64] = &[2.5, 2.5];
        let k = 2;

        let predicted_class = clasificar(conocidos, clases, muestra, k);
        assert_eq!(
            predicted_class, None,
            "The function should return None for mismatched input lengths."
        );
    }
}
