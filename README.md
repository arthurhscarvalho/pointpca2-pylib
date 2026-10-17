# pointpca2-pylib

## Purpose

Python bindings for the [pointpca2-rs](https://github.com/arthurhscarvalho/pointpca2-rs) Rust backend. The package extracts 40 PointPCA2 predictors from a reference and a distorted colored point cloud. These predictors are intended for subsequent regression; the package does not train a model or return a perceptual quality score.

The PyPI distribution and Python import are both named `pointpca2`.

## Related projects

The numerical implementation is maintained in [pointpca2-rs](https://github.com/arthurhscarvalho/pointpca2-rs). [MAPPE](https://github.com/arthurhscarvalho/mappe) uses the Python package for mesh quality assessment. Refer to the backend repository for algorithm implementation details.

## Requirements

- Python `>=3.10,<3.13`; Python 3.11 or 3.12 is recommended for use with the MAPPE workspace.
- NumPy and the other runtime dependencies declared in [pyproject.toml](pyproject.toml).
- For source builds: a current stable Rust toolchain, Cargo, a native linker/C compiler, Git, and Maturin.

## Installation

### From PyPI

In a Python 3.11 or 3.12 environment:

```bash
python -m pip install pointpca2
```

A compatible wheel does not require a local Rust toolchain. To build from source, use the checkout instructions below, including any required backend directories.

### From source

```bash
git clone https://github.com/arthurhscarvalho/pointpca2-pylib.git
cd pointpca2-pylib
python3.12 -m venv .venv
source .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install "maturin>=1.0,<2.0"
python -m maturin develop --release
```

Cargo fetches `pointpca2-rs` from Git tag `v1.5.1`, as declared in [Cargo.toml](Cargo.toml). A sibling backend checkout is not required. The activation command above is for a POSIX shell. Re-run `python -m maturin develop --release` after changing the binding or its backend.

## Usage

```python
import numpy as np
from pointpca2 import compute_pointpca2

# Replace these files with your own coordinate and RGB arrays.
points_reference = np.load("reference_points.npy")
colors_reference = np.load("reference_colors.npy")
points_test = np.load("distorted_points.npy")
colors_test = np.load("distorted_colors.npy")

predictors = compute_pointpca2(
    points_reference,
    colors_reference,
    points_test,
    colors_test,
    search_size=81,
    max_workers=0,
    verbose=False,
)
print(predictors.shape)  # (40,)
```

| Argument | Description |
| --- | --- |
| `points_reference`, `points_test` | XYZ coordinate arrays of shape `(N, 3)` and `(M, 3)`, converted to `float64`. |
| `colors_reference`, `colors_test` | Matching RGB arrays, with the same shape as their cloud's coordinates. Supply integer values in `[0, 255]` or floating-point values in `[0, 1]`; the backend receives `uint8`. |
| `search_size` | Neighborhood size; defaults to `81`. Use a positive value no larger than the distinct point count of either cloud after duplicate merging. |
| `max_workers` | Rayon thread-pool size; `0` uses the default based on logical CPUs. |
| `verbose` | Enables processing messages; defaults to `False`. |

The clouds may contain different numbers of points, but both require colors. The result is a one-dimensional `float64` NumPy array of shape `(40,)`. Normalized floating-point color arrays are scaled in place by the Python wrapper; pass copies if you need to preserve them.

A complete PLY-loading example and sample clouds are included in [examples/example_compute_predictors.py](examples/example_compute_predictors.py). After installing the optional reader dependency, run it from the repository root:

```bash
python -m pip install open3d
python examples/example_compute_predictors.py
```

## Publications

The first publication describes this implementation. The second describes the original PointPCA+ method, referred to as PointPCA2 in this project.

1. Arthur Henrique Silva Carvalho and Pedro Garcia Freitas. “PointPCA2-RS: PointPCA2 with Resource Saving.” *XLIII Brazilian Symposium on Telecommunications and Signal Processing (SBrT)*, Natal, Brazil, 2025. [Paper](https://biblioteca.sbrt.org.br/articlefile/4834.pdf).
2. Xuemei Zhou, Evangelos Alexiou, Irene Viola, and Pablo Cesar. “PointPCA+: A full-reference Point Cloud Quality Assessment metric with PCA-based features.” *Signal Processing: Image Communication*, vol. 135, article 117262, 2025. [DOI: 10.1016/j.image.2025.117262](https://doi.org/10.1016/j.image.2025.117262).

```bibtex
@inproceedings{carvalho2025pointpca2rs,
  author    = {Carvalho, Arthur Henrique Silva and Freitas, Pedro Garcia},
  title     = {{PointPCA2-RS}: {PointPCA2} with Resource Saving},
  booktitle = {XLIII Brazilian Symposium on Telecommunications and Signal Processing (SBrT)},
  address   = {Natal, Brazil},
  year      = {2025},
  url       = {https://biblioteca.sbrt.org.br/articlefile/4834.pdf}
}

@article{zhou2025pointpcaplus,
  author  = {Zhou, Xuemei and Alexiou, Evangelos and Viola, Irene and Cesar, Pablo},
  title   = {{PointPCA+}: A full-reference Point Cloud Quality Assessment metric with {PCA}-based features},
  journal = {Signal Processing: Image Communication},
  volume  = {135},
  pages   = {117262},
  year    = {2025},
  doi     = {10.1016/j.image.2025.117262}
}
```

## License

[MIT License](LICENSE).
