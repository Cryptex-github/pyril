use std::fmt::Display;

use pyo3::{prelude::*, types::PyType, pyclass::CompareOp};
use ril::Dynamic;

/// Represents a single-bit pixel that represents either a pixel that is on or off.
#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct BitPixel {
    #[pyo3(get, set)]
    value: bool,
}

#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct L {
    #[pyo3(get, set)]
    value: u8,
}

#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct Rgb {
    #[pyo3(get, set)]
    r: u8,
    #[pyo3(get, set)]
    g: u8,
    #[pyo3(get, set)]
    b: u8,
}

#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct Rgba {
    #[pyo3(get, set)]
    r: u8,
    #[pyo3(get, set)]
    g: u8,
    #[pyo3(get, set)]
    b: u8,
    #[pyo3(get, set)]
    a: u8,
}

#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct Pixel {
    pub inner: Dynamic,
}

impl From<Dynamic> for Pixel {
    fn from(inner: Dynamic) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Pixel {
    #[classmethod]
    fn from_bitpixel(_: &PyType, value: bool) -> Self {
        Self {
            inner: Dynamic::BitPixel(ril::BitPixel(value)),
        }
    }

    #[classmethod]
    fn from_l(_: &PyType, value: u8) -> Self {
        Self {
            inner: Dynamic::L(ril::L(value)),
        }
    }

    #[classmethod]
    fn from_rgb(_: &PyType, r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: Dynamic::Rgb(ril::Rgb { r, g, b }),
        }
    }

    #[classmethod]
    fn from_rgba(_: &PyType, r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: Dynamic::Rgba(ril::Rgba { r, g, b, a }),
        }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py)?;
                let val = self == &other;
                Ok(val.into_py(py))
            }
            CompareOp::Ne => {
                let other = other.extract::<Self>(py)?;
                let val = self != &other;
                Ok(val.into_py(py))
            }
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __repr__(&self) -> String {
        let out = match self.inner {
            Dynamic::BitPixel(v) => format!("BitPixel({})", v.value()),
            Dynamic::L(v) => format!("L({})", v.value()),
            Dynamic::Rgb(v) => format!("Rgb({}, {}, {})", v.r, v.g, v.b),
            Dynamic::Rgba(v) => format!("Rgba({}, {}, {}, {})", v.r, v.g, v.b, v.a),
        };

        format!("<Pixel {}>", out)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.__repr__())
    }
}

#[pymethods]
impl BitPixel {
    #[new]
    fn new(value: bool) -> Self {
        Self { value }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py)?;
                let val = self == &other;
                Ok(val.into_py(py))
            }
            CompareOp::Ne => {
                let other = other.extract::<Self>(py)?;
                let val = self != &other;
                Ok(val.into_py(py))
            }
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __repr__(&self) -> String {
        format!("<BitPixel value={}>", self.value)
    }
}

#[pymethods]
impl L {
    #[new]
    fn new(value: u8) -> Self {
        Self { value }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py)?;
                let val = self == &other;
                Ok(val.into_py(py))
            }
            CompareOp::Ne => {
                let other = other.extract::<Self>(py)?;
                let val = self != &other;
                Ok(val.into_py(py))
            }
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __repr__(&self) -> String {
        format!("<L value={}>", self.value)
    }
}

#[pymethods]
impl Rgb {
    #[new]
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py)?;
                let val = self == &other;
                Ok(val.into_py(py))
            }
            CompareOp::Ne => {
                let other = other.extract::<Self>(py)?;
                let val = self != &other;
                Ok(val.into_py(py))
            }
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __repr__(&self) -> String {
        format!("<Rgb r={} g={} b={}>", self.r, self.g, self.b)
    }
}

#[pymethods]
impl Rgba {
    #[new]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py)?;
                let val = self == &other;
                Ok(val.into_py(py))
            }
            CompareOp::Ne => {
                let other = other.extract::<Self>(py)?;
                let val = self != &other;
                Ok(val.into_py(py))
            }
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __repr__(&self) -> String {
        format!("<Rgb r={} g={} b={} a={}>", self.r, self.g, self.b, self.a)
    }
}

impl From<ril::BitPixel> for BitPixel {
    fn from(pixel: ril::BitPixel) -> Self {
        Self {
            value: pixel.value(),
        }
    }
}

impl From<ril::L> for L {
    fn from(pixel: ril::L) -> Self {
        Self {
            value: pixel.value(),
        }
    }
}

impl From<ril::Rgb> for Rgb {
    fn from(pixel: ril::Rgb) -> Self {
        Self {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
        }
    }
}

impl From<ril::Rgba> for Rgba {
    fn from(pixel: ril::Rgba) -> Self {
        Self {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
            a: pixel.a,
        }
    }
}
