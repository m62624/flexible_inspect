use pyo3::exceptions::PyTypeError;

use crate::*;
pub fn pylist_to_vec_string(pylist: &PyList) -> Vec<String> {
    let mut vec_string = Vec::new();
    for item in pylist.iter() {
        if let Ok(s) = item.extract::<String>() {
            vec_string.push(s);
        }
    }
    vec_string
}
///Получаем `PyList` для конвертаций в `Iterator<PyType>` (проще работать с коллекций объектами)
pub fn py_list_to_py_types<'py>(pylist: &'py PyList) -> PyResult<Vec<&'py PyType>> {
    let mut result = Vec::with_capacity(pylist.len());
    for element in pylist.iter() {
        result.push(element.downcast::<PyType>()?);
    }
    Ok(result)
}

// !!! Примерные шаблоны будущих или же на них основанных функций конвертаций

// pub fn py_any_to_py_type<'py>(py: Python, pyobj: &'py Py<PyAny>) -> PyResult<PyType> {
//     match pyobj.downcast::<PyType>(py) {
//         Ok(value) => Ok(value),
//         Err(_) => Err(PyErr::new::<PyTypeError, _>("Expected a PyType object")),
//     }
// }
// pub fn get_hashmap_from_dict<'py>(
//     flags_with_errors: &'py PyDict,
// ) -> PyResult<HashMap<&'py str, &'py PyAny>> {
//     let (mut result, dict) = (HashMap::new(), flags_with_errors);
//     let keys = dict.keys();
//     for key in keys {
//         let key_str: &str = key.extract()?;
//         let value = match dict.get_item(&key) {
//             Some(value) => value,
//             None => return Err(PyErr::new::<exceptions::PyTypeError, _>("None value`")),
//         };
//         result.insert(key_str, value);
//     }
//     Ok(result)
// }

// pub fn (obj_class: &PyAny) -> PyResult<HashMap<String, &PyAny>> {
//     let mut result = HashMap::new();
//     for attr_name in obj_class.dir().into_iter() {
//         // Проверяем, есть ли атрибут
//         if obj_class.hasattr("extra")? {
//             let attr_val = obj_class.getattr("extra")?;
//             result.insert(attr_name.extract::<String>()?, attr_val);
//         }
//     }
//     dbg!(&result);
//     Ok(result)
// }

// pub fn get_attr_extra(obj_class: PyAny) -> PyResult<HashMap<String, String>> {
//     let mut result = HashMap::new();
//     for attr_name in obj_class.dir().into_iter() {
//         // Проверяем, есть ли атрибут
//         if obj_class.hasattr(&*attr_name.extract::<String>()?)? {
//             let attr_val = obj_class.getattr(&*attr_name.extract::<String>()?)?;
//             result.insert(
//                 attr_name.extract::<String>()?,
//                 attr_val.extract::<String>()?,
//             );
//         }
//     }
//     Ok(result)
// }
// for attr_name in obj.dir().into_iter() {
//     // Проверяем, есть ли атрибут
//     if obj.hasattr()? {
//         let attr_val = obj.getattr()?;
//         println!("{}: {:?}", attr_name, attr_val);
//     }
// }
