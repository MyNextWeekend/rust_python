use pyo3::create_exception;


// 定义父异常类
create_exception!(rust_python, MyError, pyo3::exceptions::PyException);

// 定义异常子类
create_exception!(rust_python, ChildErrorA, MyError);
create_exception!(rust_python, ChildErrorB, MyError);
create_exception!(rust_python, ChildErrorC, MyError);




// 内部可能出现的异常，转换为python的异常
// pub fn err_to_py(e: Error) -> PyErr {
//     match e {
//         Error::Io(err) => PyIOError::new_err(err.to_string()),
//         Error::Ods(ref err) => match err {
//             OdsError::Io(error) => PyIOError::new_err(error.to_string()),
//             OdsError::Zip(error) => ZipError::new_err(error.to_string()),
//             OdsError::Xml(error) => XmlError::new_err(error.to_string()),
//             OdsError::XmlAttr(error) => XmlError::new_err(error.to_string()),
//             OdsError::Password => PasswordError::new_err(err.to_string()),
//             OdsError::WorksheetNotFound(error) => WorksheetNotFound::new_err(error.to_string()),
//             _ => CalamineError::new_err(err.to_string()),
//         },
//         Error::Xls(ref err) => match err {
//             XlsError::Io(error) => PyIOError::new_err(error.to_string()),
//             XlsError::Password => PasswordError::new_err(err.to_string()),
//             XlsError::WorksheetNotFound(error) => WorksheetNotFound::new_err(error.to_string()),
//             _ => CalamineError::new_err(err.to_string()),
//         },
//         Error::Xlsx(ref err) => match err {
//             XlsxError::Io(error) => PyIOError::new_err(error.to_string()),
//             XlsxError::Zip(error) => ZipError::new_err(error.to_string()),
//             XlsxError::Xml(error) => XmlError::new_err(error.to_string()),
//             XlsxError::XmlAttr(error) => XmlError::new_err(error.to_string()),
//             XlsxError::XmlEof(error) => XmlError::new_err(error.to_string()),
//             XlsxError::Password => PasswordError::new_err(err.to_string()),
//             XlsxError::WorksheetNotFound(error) => WorksheetNotFound::new_err(error.to_string()),
//             _ => CalamineError::new_err(err.to_string()),
//         },
//         Error::Xlsb(ref err) => match err {
//             XlsbError::Io(error) => PyIOError::new_err(error.to_string()),
//             XlsbError::Zip(error) => ZipError::new_err(error.to_string()),
//             XlsbError::Xml(error) => XmlError::new_err(error.to_string()),
//             XlsbError::XmlAttr(error) => XmlError::new_err(error.to_string()),
//             XlsbError::Password => PasswordError::new_err(err.to_string()),
//             XlsbError::WorksheetNotFound(error) => WorksheetNotFound::new_err(error.to_string()),
//             _ => CalamineError::new_err(err.to_string()),
//         },
//         _ => CalamineError::new_err(e.to_string()),
//     }
// }