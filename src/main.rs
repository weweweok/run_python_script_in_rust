use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use std::fs;
use std::path::Path;

fn create_ascii_art(path: &Path, py_app: String) -> PyResult<Vec<u8>> {
    Python::with_gil(|py| -> PyResult<PyObject> {
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

        Ok(result_bytes.as_bytes().to_vec())
    });
}

fn main() -> PyResult<()> {
    let path = Path::new("./python_app");
    let py_app = fs::read_to_string(path.join("creategif.py"))?;
    let result_bytes = create_ascii_art(&path, py_app)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    #[test]
    fn create_file_test() {
        if Path::new("./python_app/ascii_art.gif").exists() {
            fs::remove_file("./python_app/ascii_art.gif").unwrap();
        }
        let path = Path::new("./python_app");
        let py_app = fs::read_to_string(path.join("creategif.py"))?;
        let resut_bytes = create_ascii_art(&path, py_app).unwrap();
        let mut file = fs::OpenOptions::new() // ファイルを開く or 作成 or 上書き
            .append(true) // ファイルがなければ作成
            .create(true)
            .write(true)
            .open("./python_app/ascii_art.gif")?;
        file.write_all(resut_bytes.as_bytes())?;
        assert_eq!(Path::new("./python_app/ascii_art.gif").exists(), true);
    }
}
