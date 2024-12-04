use na::DMatrix;
use numpy::PyReadonlyArray2;
use rayon::ThreadPoolBuilder;

pub fn as_dmatrix<T>(x: PyReadonlyArray2<T>) -> DMatrix<T>
where
    T: numpy::Element + na::Scalar,
{
    let data: Vec<T> = x.as_array().iter().cloned().collect();
    DMatrix::from_row_slice(x.shape()[0], x.shape()[1], &data)
}

pub fn set_max_workers(max_workers: usize) {
    if max_workers == 0 {
        return;
    }
    ThreadPoolBuilder::new()
        .num_threads(max_workers)
        .build_global()
        .expect("Failed to build global thread pool");
}
