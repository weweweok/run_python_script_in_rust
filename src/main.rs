use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use std::fs;
use std::path::Path;

fn create_ascii_art_from_binary(image_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let path = Path::new("./python_app");
    let py_app = fs::read_to_string(path.join("creategif.py")).unwrap();
    Python::with_gil(|py| -> PyResult<Vec<u8>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
        syspath.insert(0, path.to_str())?;
        let module = PyModule::from_code(py, &py_app, "creategif.py", "")?;
        let class = module.getattr("CreateAsciiArt")?;
        let instance = class.call0()?;
        let py_image_bytes = PyBytes::new(py, &image_bytes);
        let arg = (py_image_bytes,);
        let result = instance.call_method1("create_binary_ascii_art", arg)?;
        let result_bytes: &PyBytes = result.extract()?;

        Ok(result_bytes.as_bytes().to_vec())
    })
}

fn create_ascii_art(path: &Path, py_app: String) -> PyResult<Vec<u8>> {
    Python::with_gil(|py| -> PyResult<Vec<u8>> {
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
    })
}

fn main() -> PyResult<()> {
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
        let py_app = fs::read_to_string(path.join("creategif.py")).unwrap();
        let resut_bytes = create_ascii_art(&path, py_app).unwrap();
        let mut file = fs::OpenOptions::new() // ファイルを開く or 作成 or 上書き
            .append(true) // ファイルがなければ作成
            .create(true)
            .write(true)
            .open("./python_app/ascii_art.gif")
            .unwrap();
        file.write_all(resut_bytes.as_slice()).unwrap();
        assert_eq!(Path::new("./python_app/ascii_art.gif").exists(), true);
    }
    // #[test]
    // fn create_ascii_art_only_bytes() {
    //     let image_bytes = fs::read("./python_app/posted-image.gif").unwrap();
    //     let result = create_ascii_art_from_binary(image_bytes).unwrap();
    //     assert_eq!(result.len() > 0, true);
    // }
}
