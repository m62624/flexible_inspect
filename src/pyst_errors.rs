use crate::*;
// lazy_static необходим, чтобы убедиться, что регулярные выражения компилируются ровно один раз.
use lazy_static::lazy_static;
use pyo3::exceptions::PyTypeError;
use pyo3::types::PyType;

// create_exception!(validator, BaseError, pyo3::exceptions::PyException);
// #[pyfunction]
// pub fn throw_error(py: Python, obj: &PyAny,) -> PyResult<PyErr> {
//     // get_attr_extra(obj)?;
//     // Создаем объект класса ошибки с переданными параметрами
//     let kwargs = PyDict::new(py);
//     let obj = obj.call(PyTuple::empty(py), Some(kwargs))?;
//     // Создаем объект класса & Возвращаем ошибку
//     let shared_obj = obj.into_py(py);
//     let pyerr = PyErr::new::<PyException, _>(shared_obj);
//     Ok(pyerr)
// }

// pub fn throw_error(py: Python, obj: &PyType) -> PyResult<()> {
//     // let cls = class.downcast::<PyType>()?;
//     // let extra = [("width", 1), ("height", 2)].into_py_dict(py);
//     // let error_instance = cls.call((), Some(extra))?;
//     //xx error_instance.setattr(py, "message", message)?;
//     //xx error_instance.setattr(py, "extra", extra)?;
//     // let shared_obj = error_instance.into_py(py);
//     // let py_error_instance = PyException::new_err(shared_obj);
//     // Err(py_error_instance)
//     todo!()
// }

// #[pyfunction]
// pub fn create_error(py: Python, class: PyObject) -> PyResult<PyObject> {
//     let cls = class.downcast::<PyType>(py)?;
//     let message = "error message";
//     let extra = [("width", 1), ("height", 2)].into_py_dict(py);
//     let error_instance = cls.call((), Some(extra))?;
//     dbg!(error_instance.getattr("__dir__"));
//     // error_instance.setattr("message", message)?;
//     // error_instance.setattr("extra", extra)?;
//     // Err(PyValueError::new_err(error_instance))
//     let shared_obj = error_instance.into_py(py);
//     // let pyerr = PyErr::new::<PyException, _>(shared_obj);
//     // dbg!(&pyerr);
//     Ok(shared_obj)
// }
