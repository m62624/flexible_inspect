mod bytes_convert;
mod next_step;
mod runners;

//=====================
use super::rule::Rule;
use super::*;
use bytes_convert::bytes_to_string_utf8;
#[pymethods]
impl TemplateValidator {
    fn validate_single_sync(&self, py: Python, text: &types::PyBytes) -> PyResult<()> {
        let mut errors = Vec::new();
        let text = bytes_to_string_utf8(text.as_bytes())?;
        self.exceptions
            .iter()
            .map(|exception_container| {
                runners::single_work::step_by_step_in_the_class(
                    py,
                    &mut errors,
                    exception_container,
                    &text,
                )
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}

#[test]
fn aboba() {
    let x = vec![Ok(1), Ok(2), Err("x"), Ok(3)];
    // let x = x.iter().map(|el| el.ok()?).collect::<PyResult<Vec<_>>>();
    dbg!();
}
