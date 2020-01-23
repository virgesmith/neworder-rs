

#[cfg(test)]
mod test {

  //use super::*;
  use crate::timeline::Timeline;
  use crate::neworder as no;
  use crate::callback::Callback;
  
  use pyo3::prelude::*;
  use pyo3::{Python, PyResult};
  use pyo3::types::*; 
    
  #[test]
  fn timeline_statics() {
    assert_eq!(Timeline::DISTANT_PAST, Timeline::DISTANT_PAST);
    assert_eq!(Timeline::FAR_FUTURE, Timeline::FAR_FUTURE);
    assert!(Timeline::DISTANT_PAST < 0.0);
    assert!(0.0 < Timeline::FAR_FUTURE);
    assert!(!(Timeline::DISTANT_PAST < Timeline::NEVER));
    assert!(!(Timeline::DISTANT_PAST >= Timeline::NEVER));
    assert!(!(Timeline::FAR_FUTURE <= Timeline::NEVER));
    assert!(!(Timeline::FAR_FUTURE > Timeline::NEVER));

    let x = -1e10;
    assert!(Timeline::DISTANT_PAST < x);
    assert!(Timeline::FAR_FUTURE > x);
    let x = 1e10;
    assert!(Timeline::DISTANT_PAST < x);
    assert!(Timeline::FAR_FUTURE > x);

    // dreams never end
    assert_ne!(Timeline::NEVER, x);
    assert_ne!(Timeline::NEVER, Timeline::NEVER);
    assert!(!(Timeline::NEVER < x));
    assert!(!(Timeline::NEVER == x));
    assert!(!(Timeline::NEVER >= x));
    // no nay never
    assert!(!Timeline::isnever(x)); 
    // no nay never no more
    assert!(Timeline::isnever(Timeline::NEVER))  
  }

  #[test]
  fn test_no() {
    // test logging - use {, true} to make it look like function returning bool. If a problem, there will be an exception or worse
    assert!({no::log("neworder module test"); true}, "log message");
    assert!({no::log(&format!("test logging types: {} {} {} {} {:?} {}", false, 0, 0.0, "", vec![0; 10], true)); true}, "log data");

    // // test formatting
    // CHECK(format::decimal(3.14, 6, 6) == "     3.140000");
    // // ignores the 1 LHS padding as there are 6 digits
    // CHECK(format::decimal(1000000.0 / 7, 1, 6) == "142857.142857");
    // CHECK(format::pad(3, 4) == "   3");
    // CHECK(format::pad(3, 5, '0') == "00003");
    // // ignores 3 as number requires 4 chars
    // CHECK(format::pad(5000, 3, '0') == "5000");
    // CHECK(format::hex<int32_t>(24233) == "0x00005ea9");
    // CHECK(format::hex<size_t>(133, false) == "0000000000000085");
    // CHECK(format::boolean(false) == "false");
    
    // /*no::Environment& env =*/ no::getenv();
    let gil = Python::acquire_gil();
    let py = gil.python();
  
    let neworder = no::init_embedded(py).unwrap();
    let locals = [("neworder", neworder)].into_py_dict(py);

    // Check required (but defaulted) attrs visible from both rust and python
    let attrs = ["rank", "size"];

    //Callback::exec("neworder.log(dir(neworder))".to_string(), None, Some(locals)).run(py).unwrap();
    // This isn't quite the same as below 
    for a in &attrs {
      assert!(match neworder.get(a) {
        Ok(_) => true,
        Err(_) => false
      });
      assert!(Callback::eval(format!("'{}' in dir(neworder)", a), None, Some(locals)).run(py).unwrap().is_true(py).unwrap(), "attr seen by python");
    }
    // for (size_t i = 0; i < sizeof(attrs)/sizeof(attrs[0]); ++i)
    // {
    //   CHECK(pycpp::has_attr(module, attrs[i]));
    //   CHECK(no::Callback::eval("'%%' in locals()"_s % attrs[i])().cast<bool>());
    // }

    // check string conversion
    assert_eq!("<built-in function never>", format!("{}", neworder.get("never").unwrap()));
    assert_eq!(format!("{}", neworder.call0("never").unwrap()), "nan");

    // Check diagnostics consistent
    assert_eq!(
      format!("{}", Callback::eval("neworder.name()".to_string(), None, Some(locals)).run(py).unwrap().as_ref(py).str().unwrap()),
      no::name());
    // CHECK(no::Callback::eval("version() == '%%'"_s % no::module_version())().cast<bool>());
    // CHECK(no::Callback::eval("python() == '%%'"_s % no::python_version()/*.c_str()*/)().cast<bool>());

  }
}