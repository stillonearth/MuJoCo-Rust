use lazy_static::lazy_static;
use mujoco_rs_sys::*;
// use mujoco_rs_sys::*;
use std::ffi::{CStr, CString};

// const XML: &str = r#"
// <mujoco>
//    <worldbody>
//       <light diffuse=".5 .5 .5" pos="0 0 3" dir="0 0 -1"/>
//       <geom type="plane" size="1 1 0.1" rgba=".9 0 0 1"/>
//       <body pos="0 0 1">
//          <joint type="free"/>
//          <geom type="box" size=".1 .2 .3" rgba="0 .9 0 1"/>
//       </body>
//    </worldbody>
// </mujoco>
// "#;

lazy_static! {
    static ref MJ_HOME: CString = CString::new(
        dirs::home_dir()
            .expect("Could not locate home directory!")
            .join(".mujoco")
            .to_str()
            .unwrap()
    )
    .unwrap();
    static ref XML_NAME: &'static CStr =
        CStr::from_bytes_with_nul(b"tests/res/simple.xml\0").unwrap();
}

fn load_model() -> *mut mjModel {
    // use std::mem::MaybeUninit;
    let mut vfs: Box<mjVFS> = {
        let mut vfs_uninit: Box<mjVFS> = unsafe {
            Box::from_raw(std::alloc::alloc(std::alloc::Layout::new::<
                mujoco_rs_sys::no_render::mjVFS_,
            >()) as *mut _)
        }; // MaybeUninit<mjVFS> = MaybeUninit::uninit();
        unsafe {
            mj_defaultVFS(&mut *vfs_uninit);
            vfs_uninit
        }
    };

    let model;
    unsafe {
        let mut err: Box<[std::os::raw::c_uchar; 1000]> = Box::new([b'\0'; 1000]);
        model = mj_loadXML(
            XML_NAME.as_ptr(),
            &*vfs,
            err.as_mut_ptr() as *mut std::os::raw::c_char,
            1000,
        );
        assert_ne!(
            model,
            std::ptr::null_mut(),
            "Error when loading XML: {}",
            String::from_utf8(err.to_vec()).unwrap()
        );
        mj_deleteVFS(&mut *vfs);
    }
    model
}

#[test]
fn test_load_model() {
    let model = load_model();
    unsafe { mj_deleteModel(model) };
}

#[test]
fn test_make_data() {
    let model = load_model();

    let data = unsafe { mj_makeData(model) };
    assert_ne!(data, std::ptr::null_mut());
    unsafe {
        mj_deleteData(data);
        mj_deleteModel(model);
    }
}

#[test]
fn test_simulate() {
    let model = load_model();
    let data = unsafe { mj_makeData(model) };
    assert_ne!(data, std::ptr::null_mut());

    for _ in 0..10 {
        println!("Doing step");
        unsafe { mj_step(model, data) };
    }
    unsafe {
        mj_deleteData(data);
        mj_deleteModel(model);
    }
}
