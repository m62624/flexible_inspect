use super::*;
use log::trace;
use pyo3::PyTraverseError;
use pyo3::PyVisit;

impl TemplateValidator {
    /*
    Метод `__traverse__` используется для рекурсивного обхода объекта и уведомления `Python` о всех
    вложенных объектах, которые должны быть добавлены в механизм управления памятью `Python`.
    Это гарантирует, что объекты типа `PyObject` не будут освобождены Python'ом до тех пор,
    пока они не перестанут использоваться в `Rust`.
     */

    #[cfg(not(tarpaulin_include))]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        if let Some(slf) = &self.0 {
            for cart_wrapper in slf.cartridges.iter() {
                // ================= (LOG) =================
                trace!(
                    "the __traverse __ for `{}` is running",
                    cart_wrapper.get_cartridge().get_py_class()
                );
                // =========================================
                visit.call(cart_wrapper.get_cartridge().get_py_class())?;
            }
        }
        Ok(())
    }

    #[cfg(not(tarpaulin_include))]
    fn __clear__(&mut self) {
        // ================= (LOG) =================
        trace!("the `__clear__` process for `{:?}` is in progress", self.0);
        // =========================================
        self.0 = None;
    }
}
