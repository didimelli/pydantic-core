use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::tools::safe_repr;

#[pyclass(module = "pydantic_core._pydantic_core", get_all, frozen, freelist = 100)]
#[derive(Debug, Clone)]
pub struct ArgsKwargs {
    pub(crate) args: Py<PyTuple>,
    pub(crate) kwargs: Option<Py<PyDict>>,
}

impl ArgsKwargs {
    fn eq(&self, py: Python, other: &Self) -> PyResult<bool> {
        if self.args.as_ref(py).eq(other.args.as_ref(py))? {
            match (&self.kwargs, &other.kwargs) {
                (Some(d1), Some(d2)) => d1.as_ref(py).eq(d2.as_ref(py)),
                (None, None) => Ok(true),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}

#[pymethods]
impl ArgsKwargs {
    #[new]
    fn py_new(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> Self {
        Self {
            args: args.into_py(py),
            kwargs: match kwargs {
                Some(d) if !d.is_empty() => Some(d.into_py(py)),
                _ => None,
            },
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => match self.eq(py, other) {
                Ok(b) => b.into_py(py),
                Err(e) => e.into_py(py),
            },
            CompareOp::Ne => match self.eq(py, other) {
                Ok(b) => (!b).into_py(py),
                Err(e) => e.into_py(py),
            },
            _ => py.NotImplemented(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        let args = safe_repr(self.args.as_ref(py));
        match self.kwargs {
            Some(ref d) => format!("ArgsKwargs({args}, {})", safe_repr(d.as_ref(py))),
            None => format!("ArgsKwargs({args})"),
        }
    }
}
