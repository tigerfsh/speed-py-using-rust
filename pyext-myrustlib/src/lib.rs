
// use cpython::py_module_initializer;
// 
// py_module_initializer!(pyext_myrustlib, |py, m| {
//     m.add(py, "__doc__", "Module documentation string")?;
//     Ok(())
// });

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;
    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }
    Ok(total)
}

fn count_doubles_once(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;
    let mut chars = val.chars();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }
    Ok(total)
}

fn count_doubles_once_bytes(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;
    let mut chars = val.bytes();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }

    Ok(total)
}
py_module_initializer!(pyext_myrustlib, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str)))?;
    m.add(py, "count_doubles_once", py_fn!(py, count_doubles_once(val: &str)))?;
    m.add(py, "count_doubles_once_bytes", py_fn!(py, count_doubles_once_bytes(val: &str)))?;
    Ok(())
});
