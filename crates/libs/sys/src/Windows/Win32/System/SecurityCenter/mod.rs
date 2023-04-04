::windows_targets::link!("wscapi.dll" "system" #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"] fn WscGetAntiMalwareUri(ppszuri : *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("wscapi.dll" "system" #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"] fn WscGetSecurityProviderHealth(providers : u32, phealth : *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("wscapi.dll" "system" #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"] fn WscQueryAntiMalwareUri() -> ::windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
::windows_targets::link!("wscapi.dll" "system" #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"] fn WscRegisterForChanges(reserved : *mut ::core::ffi::c_void, phcallbackregistration : *mut super::super::Foundation:: HANDLE, lpcallbackaddress : super::Threading:: LPTHREAD_START_ROUTINE, pcontext : *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("wscapi.dll" "system" #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"] fn WscRegisterForUserNotifications() -> ::windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("wscapi.dll" "system" #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"] fn WscUnRegisterChanges(hregistrationhandle : super::super::Foundation:: HANDLE) -> ::windows_sys::core::HRESULT);
pub type IWSCDefaultProduct = *mut ::core::ffi::c_void;
pub type IWSCProductList = *mut ::core::ffi::c_void;
pub type IWscProduct = *mut ::core::ffi::c_void;
pub type IWscProduct2 = *mut ::core::ffi::c_void;
pub type IWscProduct3 = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSCDefaultProduct: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2981a36e_f22d_11e5_9ce9_5e5517507c66);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSCProductList: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x17072f7b_9abe_4a74_a261_1eb76b55107a);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type SECURITY_PRODUCT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PRODUCT_STATE = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PRODUCT_SUBSTATUS = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PROVIDER = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = 4i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = 8i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = 16i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = 32i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = 64i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = 127i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PROVIDER_HEALTH = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = 3i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_SIGNATURE_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = 1i32;
