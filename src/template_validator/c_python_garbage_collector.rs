use super::*;
use pyo3::gc::PyVisit;
use pyo3::PyTraverseError;

// РАБОТАЕТ ТОЛЬКО С `C API` (CPYTHON)
#[pymethods]
impl TemplateValidator {
    /*
    Метод `__traverse__` используется для рекурсивного обхода объекта и уведомления `Python` о всех
    вложенных объектах, которые должны быть добавлены в механизм управления памятью `Python`. В этой
    реализации метод `__traverse__` проходится по всем значениям в хэш-таблице `python_classes` и вызывает
    `visit.call()` для каждого объекта типа `PyObject` в свойстве `error` объекта типа `RuleStatus`.
    Это гарантирует, что объекты типа `PyObject` не будут освобождены Python'ом до тех пор,
    пока они не перестанут использоваться в `Rust`.
     */
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for class_py in self.py_classes.values() {
            visit.call(class_py)?;
        }
        Ok(())
    }

    /*
    Метод `__clear__` используется для освобождения памяти, занятой объектом. В этой реализации метод
    `__clear__` очищает хэш-таблицу `py_classes`, что приводит к уменьшению счетчика ссылок на каждый
    объект типа PyObject в свойстве error объекта типа `RuleStatus`. Если количество ссылок на объект
    достигнет нуля, он будет автоматически удален Python'ом.
     */
    fn __clear__(&mut self) {
        // Удаляем все значения из HashMap, тем самым снижая счетчик ссылок на PyObject
        self.py_classes.clear();
    }
}
