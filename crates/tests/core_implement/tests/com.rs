#![allow(non_snake_case)]

/// A custom declaration of implementation of `IPersist`
#[windows_core::interface("0000010c-0000-0000-C000-000000000046")]
unsafe trait ICustomPersist: windows_core::IUnknown {
    unsafe fn GetClassID(&self, clsid: *mut windows_core::GUID) -> windows_core::HRESULT;
}

/// A custom in-memory store
#[windows_core::implement(ICustomPersist)]
struct Persist;

impl ICustomPersist_Impl for Persist {
    unsafe fn GetClassID(&self, clsid: *mut windows_core::GUID) -> windows_core::HRESULT {
        *clsid = "117fb826-2155-483a-b50d-bc99a2c7cca3".into();
        windows_core::HRESULT(0)
    }
}

#[test]
fn test() -> windows_core::Result<()> {
    unsafe {
        let p: ICustomPersist = Persist.into();

        let mut b = windows_core::GUID::default();
        p.GetClassID(&mut b).ok()?;
        assert_eq!(b, "117fb826-2155-483a-b50d-bc99a2c7cca3".into());

        Ok(())
    }
}
