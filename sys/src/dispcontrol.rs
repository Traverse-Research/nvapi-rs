use std::os::raw::c_char;
use crate::status::NvAPI_Status;
use crate::handles;

nvapi! {
    pub type EnumNvidiaDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns NVAPI_END_ENUMERATION.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaDisplayHandle;
}

nvapi! {
    pub type EnumNvidiaUnAttachedDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvUnAttachedDispHandle: *mut handles::NvUnAttachedDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA unattached display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns error.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaUnAttachedDisplayHandle;
}

nvapi! {
    pub type GetAssociatedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display that is associated
    /// with the given display "name" (such as "\\.\DISPLAY1").
    pub unsafe fn NvAPI_GetAssociatedNvidiaDisplayHandle;
}

nvapi! {
    pub type DISP_GetAssociatedUnAttachedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvUnAttachedDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of an unattached NVIDIA display that is
    /// associated with the given display name (such as "\\DISPLAY1").
    pub unsafe fn NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle;
}


nvapi! {
    pub type DISP_GetDisplayIdByDisplayNameFn = extern "C" fn(displayName: *const c_char, displayId: *mut u32) -> NvAPI_Status;

    /// This API retrieves the Display Id of a given display by display name. The display must be
    /// active to retrieve the displayId. In the case of clone mode or Surround gaming, the
    /// primary or top-left display will be returned.
    ///
    /// `displayName` is the GDI name of the display, such as `\\.\DISPLAY1`.
    pub unsafe fn NvAPI_DISP_GetDisplayIdByDisplayName;
}

nvbits! {
    /// Bitfield in `NV_GET_VRR_INFO`
    pub enum NV_GET_VRR_INFO_FLAGS / VrrInfoFlags {
        /// Set if VRR Mode is currently active on given display.
        NV_GET_VRR_INFO_IS_VRR_ENABLED / VRR_ENABLED = 0x01,
        /// Set to true if VRR Mode is possible on the given display.
        NV_GET_VRR_INFO_IS_VRR_POSSIBLE / VRR_POSSIBLE = 0x02,
        /// Set to true if VRR Mode is requested for the given display.
        NV_GET_VRR_INFO_IS_VRR_REQUESTED / VRR_REQUESTED = 0x04,
        /// Set to true if the VRR indicator is enabled. This can be done through the NVIDIA
        /// Control Panel.
        NV_GET_VRR_INFO_IS_VRR_INDICATOR_ENABLED / VRR_INDICATOR_ENABLED = 0x08,
        /// Set to true if the display is in VRR Mode.
        NV_GET_VRR_INFO_IS_DISPLAY_IN_VRR_MODE / DISPLAY_IN_VRR_MODE = 0x10,
    }
}

nvstruct! {
    pub struct NV_GET_VRR_INFO_V1 {
        /// Structure version
        pub version: u32,
        /// The `bIs*` bitfields of the C struct
        pub flags: NV_GET_VRR_INFO_FLAGS,
        /// Reserved for future use
        pub reservedEx: [u32; 4],
    }
}

pub type NV_GET_VRR_INFO = NV_GET_VRR_INFO_V1;

nvversion! { NV_GET_VRR_INFO_VER1(NV_GET_VRR_INFO_V1 = 4 * 6, 1) }
nvversion! { NV_GET_VRR_INFO_VER = NV_GET_VRR_INFO_VER1 }

nvapi! {
    pub type Disp_GetVRRInfoFn = extern "C" fn(displayId: u32, pVrrInfo: *mut NV_GET_VRR_INFO) -> NvAPI_Status;

    /// This API returns the VRR (G-SYNC/Adaptive-Sync) state of a display.
    ///
    /// # Returns
    ///
    /// - `NVAPI_OK`: completed request
    /// - `NVAPI_INVALID_ARGUMENT`: Invalid input parameter
    /// - `NVAPI_INCOMPATIBLE_STRUCT_VERSION`: The version of `NV_GET_VRR_INFO` is invalid
    pub unsafe fn NvAPI_Disp_GetVRRInfo;
}
