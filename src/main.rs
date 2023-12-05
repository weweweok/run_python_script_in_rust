use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> PyResult<()> {
    let path = Path::new("./python_app");
    let py_app = fs::read_to_string(path.join("creategif.py"))?;
    Python::with_gil(|py| -> PyResult<()> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
        syspath.insert(0, path.to_str())?;
        let module = PyModule::from_code(py, &py_app, "creategif.py", "")?;
        let class = module.getattr("CreateAsciiArt")?;
        let instance = class.call0()?;
        let image_bytes = fs::read("./python_app/posted-image.gif")?;
        let py_image_bytes = PyBytes::new(py, &image_bytes);
        let arg = (py_image_bytes,);
        let result = instance.call_method1("create_binary_ascii_art", arg)?;
        let result_bytes: &PyBytes = result.extract()?;
        let mut file = fs::OpenOptions::new() // ファイルを開く or 作成 or 上書き
            .append(true) // ファイルがなければ作成
            .create(true)
            .write(true)
            .open("./python_app/ascii_art.gif")?;
        file.write_all(result_bytes.as_bytes())?;
        Ok(())
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_file_test() {
        if Path::new("./python_app/ascii_art.gif").exists() {
            fs::remove_file("./python_app/ascii_art.gif").unwrap();
        }
        main().unwrap();
        assert_eq!(Path::new("./python_app/ascii_art.gif").exists(), true);
    }
}
