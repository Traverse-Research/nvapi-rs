use std::fmt::Debug;

use crate::{
    handles::{self, NvDRSProfileHandle},
    NvAPI_Status, MAKE_NVAPI_VERSION,
};

pub const NVAPI_UNICODE_STRING_MAX: usize = 2048;
pub const NVAPI_BINARY_DATA_MAX: usize = 4096;

nvenum! {
    pub enum NVDRS_SETTING_TYPE / DrsSettingType {
        NVDRS_DWORD_TYPE / Dword = 0,
        NVDRS_BINARY_TYPE / Binary = 1,
        NVDRS_STRING_TYPE / String = 2,
        NVDRS_WSTRING_TYPE / Wstring = 3,
    }
}

nvenum! {
    pub enum NVAPI_VSYNC_MODE / VsyncMode {
        VSYNCMODE_PASSIVE / Passive = 0x60925292,
        VSYNCMODE_FORCEOFF / ForceOff = 0x08416747,
        VSYNCMODE_FORCEON / ForceOn = 0x47814940,
        VSYNCMODE_FLIPINTERVAL2 / FlipInterval2 = 0x32610244,
        VSYNCMODE_FLIPINTERVAL3 / FlipInterval3 = 0x71271021,
        VSYNCMODE_FLIPINTERVAL4 / FlipInterval4 = 0x13245256,
        VSYNCMODE_VIRTUAL / Virtual = 0x18888888,
    }
}

impl TryFrom<u32> for VsyncMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x60925292 => Ok(Self::Passive),
            0x08416747 => Ok(Self::ForceOff),
            0x47814940 => Ok(Self::ForceOn),
            0x32610244 => Ok(Self::FlipInterval2),
            0x71271021 => Ok(Self::FlipInterval3),
            0x13245256 => Ok(Self::FlipInterval4),
            0x18888888 => Ok(Self::Virtual),
            _ => Err(()),
        }
    }
}

nvenum! {
    pub enum NVAPI_AA_MODE_REPLAY / AaModeReplay {
        AA_MODE_REPLAY_SAMPLES_MASK / AaModeReplaySamplesMask = 0x00000070,
        AA_MODE_REPLAY_SAMPLES_ONE / AaModeReplaySamplesOne = 0x00000000,
        AA_MODE_REPLAY_SAMPLES_TWO / AaModeReplaySamplesTwo = 0x00000010,
        AA_MODE_REPLAY_SAMPLES_FOUR / AaModeReplaySamplesFour = 0x00000020,
        AA_MODE_REPLAY_SAMPLES_EIGHT / AaModeReplaySamplesEight = 0x00000030,
        AA_MODE_REPLAY_MODE_MASK / AaModeReplayModeMask = 0x0000000f,
        AA_MODE_REPLAY_MODE_OFF / AaModeReplayModeOff = 0x00000000,
        AA_MODE_REPLAY_MODE_ALPHA_TEST / AaModeReplayModeAlphaTest = 0x00000001,
        AA_MODE_REPLAY_MODE_PIXEL_KILL / AaModeReplayModePixelKill = 0x00000002,
        AA_MODE_REPLAY_MODE_DYN_BRANCH / AaModeReplayModeDynBranch = 0x00000004,
        AA_MODE_REPLAY_MODE_OPTIMAL / AaModeReplayModeOptimal = 0x00000004,
        AA_MODE_REPLAY_MODE_ALL / AaModeReplayModeAll = 0x00000008,
        AA_MODE_REPLAY_MODE_MAX / AaModeReplayModeMax = 0x0000000f,
        AA_MODE_REPLAY_TRANSPARENCY / AaModeReplayTransparency = 0x00000023,
        AA_MODE_REPLAY_DISALLOW_TRAA / AaModeReplayDisallowTraa = 0x00000100,
        AA_MODE_REPLAY_TRANSPARENCY_DEFAULT / AaModeReplayTransparencyDefault = 0x00000000,
        AA_MODE_REPLAY_TRANSPARENCY_DEFAULT_TESLA / AaModeReplayTransparencyDefaultTesla = 0x00000000,
        AA_MODE_REPLAY_TRANSPARENCY_DEFAULT_FERMI / AaModeReplayTransparencyDefaultFermi = 0x00000000,
        AA_MODE_REPLAY_MASK / AaModeReplayMask = 0x0000017f,
    }
}

impl TryFrom<u32> for AaModeReplay {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let slf = match value {
            0x00000070 => Self::AaModeReplaySamplesMask,
            0x00000000 => Self::AaModeReplaySamplesOne,
            0x00000010 => Self::AaModeReplaySamplesTwo,
            0x00000020 => Self::AaModeReplaySamplesFour,
            0x00000030 => Self::AaModeReplaySamplesEight,
            0x0000000f => Self::AaModeReplayModeMask,
            0x00000001 => Self::AaModeReplayModeAlphaTest,
            0x00000002 => Self::AaModeReplayModePixelKill,
            0x00000004 => Self::AaModeReplayModeDynBranch,
            0x00000008 => Self::AaModeReplayModeAll,
            0x00000023 => Self::AaModeReplayTransparency,
            0x00000100 => Self::AaModeReplayDisallowTraa,
            0x0000017f => Self::AaModeReplayMask,
            _ => return Err(()),
        };

        Ok(slf)
    }
}

nvenum! {
    pub enum NVDRS_SETTING_LOCATION / DrsSettingLocation {
        NVDRS_CURRENT_PROFILE_LOCATION / Current = 0,
        NVDRS_GLOBAL_PROFILE_LOCATION / Global = 1,
        NVDRS_BASE_PROFILE_LOCATION / Base = 2,
        NVDRS_DEFAULT_PROFILE_LOCATION / Default = 3,
    }
}

nvstruct! {
    pub struct NVDRS_BINARY_SETTING {
        pub valueLength: u32,
        pub valueData: [u8; NVAPI_BINARY_DATA_MAX],
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union NvSettingValue {
    pub value_u32: u32,
    pub value_binary: NVDRS_BINARY_SETTING,
    pub value_unicode: [u16; NVAPI_UNICODE_STRING_MAX],
}

impl Debug for NvSettingValue {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}

impl Default for NvSettingValue {
    fn default() -> Self {
        Self { value_u32: 0u32 }
    }
}

nvstruct! {
    pub struct NVDRS_SETTING {
        pub version: u32,
        pub settingName: [u16; NVAPI_UNICODE_STRING_MAX],
        pub settingId: u32,
        pub settingType: NVDRS_SETTING_TYPE,
        pub settingLocation: NVDRS_SETTING_LOCATION,
        pub isCurrentPredefined: u32,
        pub isPredefinedValid: u32,
        pub predefinedValue: NvSettingValue,
        pub currentValue: NvSettingValue,
    }
}

impl Default for NVDRS_SETTING {
    fn default() -> Self {
        Self {
            version: MAKE_NVAPI_VERSION::<NVDRS_SETTING>(1),
            settingName: [0u16; NVAPI_UNICODE_STRING_MAX],
            settingId: 0,
            settingType: 0i32,
            settingLocation: 0i32,
            isCurrentPredefined: 0u32,
            isPredefinedValid: 0u32,
            predefinedValue: Default::default(),
            currentValue: Default::default(),
        }
    }
}

nvstruct! {
    pub struct NVDRS_APPLICATION {
        pub version: u32,
        pub isPredefined: u32,
        pub appName: [u16; NVAPI_UNICODE_STRING_MAX],
        pub userFriendlyName: [u16; NVAPI_UNICODE_STRING_MAX],
        pub launcher: [u16; NVAPI_UNICODE_STRING_MAX],
        pub fileInFolder: [u16; NVAPI_UNICODE_STRING_MAX],
        // This single u32 holds `isMetro` (bit 0), `isCommandLine` (bit 1), and
        // `reserved` (bits 2–31).
        pub flags: u32,
        pub commandLine:  [u16; NVAPI_UNICODE_STRING_MAX],
    }
}

impl Default for NVDRS_APPLICATION {
    fn default() -> Self {
        Self {
            version: MAKE_NVAPI_VERSION::<NVDRS_APPLICATION>(4),
            isPredefined: 0,
            appName: [0; NVAPI_UNICODE_STRING_MAX],
            userFriendlyName: [0; NVAPI_UNICODE_STRING_MAX],
            launcher: [0; NVAPI_UNICODE_STRING_MAX],
            fileInFolder: [0; NVAPI_UNICODE_STRING_MAX],
            flags: 0,
            commandLine: [0; NVAPI_UNICODE_STRING_MAX],
        }
    }
}

nvapi! {
    pub type DRS_CreateSessionFn = extern "C" fn(pNvDRSSessionHandle: *mut handles::NvDRSSessionHandle) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_CreateSession;
}

nvapi! {
    pub type DRS_DestroySessionFn = extern "C" fn(hNvDRSSessionHandle: handles::NvDRSSessionHandle) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_DestroySession;
}

nvapi! {
    pub type DRS_LoadSettingsFn = extern "C" fn(nvDRSSessionHandle: handles::NvDRSSessionHandle) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_LoadSettings;
}

nvapi! {
    pub type DRS_FindApplicationByNameFn = extern "C" fn(nvDRSSessionHandle: handles::NvDRSSessionHandle, appName: [u16; NVAPI_UNICODE_STRING_MAX], pNvDRSProfileHandle: *mut NvDRSProfileHandle, pNvDRSApplication: *mut NVDRS_APPLICATION) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_FindApplicationByName;
}

nvapi! {
    pub type DRS_GetCurrentGlobalProfileFn = extern "C" fn(nvDRSSessionHandle: handles::NvDRSSessionHandle, pNvDRSProfileHandle: *mut NvDRSProfileHandle) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_GetCurrentGlobalProfile;
}

nvapi! {
    pub type DRS_GetBaseProfileFn = extern "C" fn(nvDRSSessionHandle: handles::NvDRSSessionHandle, pNvDRSProfileHandle: *mut NvDRSProfileHandle) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_GetBaseProfile;
}

nvapi! {
    pub type DRS_GetSettingFn = extern "C" fn(nvDRSSessionHandle: handles::NvDRSSessionHandle, nvDRSProfileHandle: NvDRSProfileHandle, settingId: u32, pSetting: *mut NVDRS_SETTING) -> NvAPI_Status;
    pub unsafe fn NvAPI_DRS_GetSetting;
}
