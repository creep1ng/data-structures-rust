use std::iter::zip;

pub fn clasificar(conocidos: &[&[f64]], clases: &[i32], muestra: &f64, k: usize) -> Option<i32> {
    if conocidos.len() != clases.len() {
        return None;
    }

    // Have to implement the method.
    Some(0)
}

fn euclidean_distance(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() {
        return None;
    }

    let mut squared_sum: f64 = 0.0;
    for (x_i, y_i) in zip(x, y) {
        squared_sum += (y_i - x_i).powi(2);
    }

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
