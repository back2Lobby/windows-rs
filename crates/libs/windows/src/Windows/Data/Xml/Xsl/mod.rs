#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXsltProcessor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXsltProcessor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b64703f_550c_48c6_a90f_93a5b964518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToString: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXsltProcessor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXsltProcessor2 {
    type Vtable = IXsltProcessor2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXsltProcessor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8da45c56_97a5_44cb_a8be_27d86280c70a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToDocument: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXsltProcessorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXsltProcessorFactory {
    type Vtable = IXsltProcessorFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXsltProcessorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x274146c0_9a51_4663_bf30_0ef742146f20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateInstance: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XsltProcessor(::windows_core::IUnknown);
impl XsltProcessor {
    #[doc = "Required features: `\"Data_Xml_Dom\"`"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToString<P0>(&self, inputnode: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::Dom::IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformToString)(::windows_core::Interface::as_raw(this), inputnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToDocument<P0>(&self, inputnode: P0) -> ::windows_core::Result<super::Dom::XmlDocument>
    where
        P0: ::windows_core::TryIntoParam<super::Dom::IXmlNode>,
    {
        let this = &::windows_core::ComInterface::cast::<IXsltProcessor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformToDocument)(::windows_core::Interface::as_raw(this), inputnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateInstance<P0>(document: P0) -> ::windows_core::Result<XsltProcessor>
    where
        P0: ::windows_core::IntoParam<super::Dom::XmlDocument>,
    {
        Self::IXsltProcessorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), document.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXsltProcessorFactory<R, F: FnOnce(&IXsltProcessorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XsltProcessor, IXsltProcessorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XsltProcessor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XsltProcessor {
    const IID: ::windows_core::GUID = <IXsltProcessor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.XsltProcessor";
}
::windows_core::imp::interface_hierarchy!(XsltProcessor, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for XsltProcessor {}
unsafe impl ::core::marker::Sync for XsltProcessor {}
