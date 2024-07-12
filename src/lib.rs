extern crate nalgebra as na;
extern crate numpy;
extern crate pointpca2_rust;

use na::DMatrix;
use numpy::PyArray1;
use numpy::PyReadonlyArray2;
use pointpca2_rust::features;
use pointpca2_rust::knn_search;
use pointpca2_rust::pooling;
use pointpca2_rust::predictors;
use pointpca2_rust::preprocessing;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn print_if_verbose<'a>(string: &'a str, verbose: &'a bool) {
    if *verbose {
        println!("{}", string);
    }
}

fn as_dmatrix<'a, T>(x: &'a PyReadonlyArray2<T>) -> DMatrix<T>
where
    T: numpy::Element + na::Scalar,
{
    let data: Vec<T> = x.as_array().iter().cloned().collect();
    DMatrix::from_row_slice(x.shape()[0], x.shape()[1], &data)
}

#[pyfunction]
fn compute_pointpca2<'py>(
    _py: Python<'py>,
    points_a: PyReadonlyArray2<'py, f64>,
    colors_a: PyReadonlyArray2<'py, u8>,
    points_b: PyReadonlyArray2<'py, f64>,
    colors_b: PyReadonlyArray2<'py, u8>,
    search_size: usize,
    verbose: bool,
) -> &'py PyArray1<f64> {
    let points_a = as_dmatrix(&points_a);
    let colors_a = as_dmatrix(&colors_a);
    let points_b = as_dmatrix(&points_b);
    let colors_b = as_dmatrix(&colors_b);
    print_if_verbose("Preprocessing", &verbose);
    let (points_a, colors_a) = preprocessing::preprocess_point_cloud(&points_a, &colors_a);
    let (points_b, colors_b) = preprocessing::preprocess_point_cloud(&points_b, &colors_b);
    print_if_verbose("Performing knn search", &verbose);
    let knn_indices_a = knn_search::knn_search(&points_a, &points_a, search_size);
    let knn_indices_b = knn_search::knn_search(&points_b, &points_a, search_size);
    print_if_verbose("Computing local features", &verbose);
    let local_features = features::compute_features(
        points_a,
        colors_a,
        points_b,
        colors_b,
        knn_indices_a,
        knn_indices_b,
        search_size,
    );
    print_if_verbose("Computing predictors", &verbose);
    let predictors_result = predictors::compute_predictors(local_features);
    print_if_verbose("Pooling predictors", &verbose);
    let pooled_predictors = pooling::mean_pooling(predictors_result);
    print_if_verbose("Done", &verbose);
    let py_array = PyArray1::from_iter(_py, pooled_predictors.iter().cloned());
    py_array
}

#[pymodule]
fn pointpca2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_pointpca2, m)?)?;
    Ok(())
}
