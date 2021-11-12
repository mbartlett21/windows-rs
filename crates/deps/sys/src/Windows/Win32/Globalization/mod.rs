#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn CompareStringA(locale: u32, dwcmpflags: u32, lpstring1: *const i8, cchcount1: i32, lpstring2: *const i8, cchcount2: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringEx(lplocalename: super::Foundation::PWSTR, dwcmpflags: COMPARE_STRING_FLAGS, lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32, lpversioninformation: *mut NLSVERSIONINFO, lpreserved: *mut ::core::ffi::c_void, lparam: super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringOrdinal(lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32, bignorecase: super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringW(locale: u32, dwcmpflags: u32, lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32) -> i32;
    pub fn ConvertDefaultLocale(locale: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoA(lpcalinfoenumproc: CALINFO_ENUMPROCA, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExA(lpcalinfoenumprocex: CALINFO_ENUMPROCEXA, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExEx(pcalinfoenumprocexex: CALINFO_ENUMPROCEXEX, lplocalename: super::Foundation::PWSTR, calendar: u32, lpreserved: super::Foundation::PWSTR, caltype: u32, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExW(lpcalinfoenumprocex: CALINFO_ENUMPROCEXW, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoW(lpcalinfoenumproc: CALINFO_ENUMPROCW, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsA(lpdatefmtenumproc: DATEFMT_ENUMPROCA, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExA(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXA, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExEx(lpdatefmtenumprocexex: DATEFMT_ENUMPROCEXEX, lplocalename: super::Foundation::PWSTR, dwflags: ENUM_DATE_FORMATS_FLAGS, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExW(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXW, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsW(lpdatefmtenumproc: DATEFMT_ENUMPROCW, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCA, languagegroup: u32, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCW, languagegroup: u32, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesA(lpcodepageenumproc: CODEPAGE_ENUMPROCA, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesW(lpcodepageenumproc: CODEPAGE_ENUMPROCW, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoID(geoclass: u32, parentgeoid: i32, lpgeoenumproc: GEO_ENUMPROC) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoNames(geoclass: u32, geoenumproc: GEO_ENUMNAMEPROC, data: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCA, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCW, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesA(lplocaleenumproc: LOCALE_ENUMPROCA, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesEx(lplocaleenumprocex: LOCALE_ENUMPROCEX, dwflags: u32, lparam: super::Foundation::LPARAM, lpreserved: *const ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesW(lplocaleenumproc: LOCALE_ENUMPROCW, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsA(lptimefmtenumproc: TIMEFMT_ENUMPROCA, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsEx(lptimefmtenumprocex: TIMEFMT_ENUMPROCEX, lplocalename: super::Foundation::PWSTR, dwflags: u32, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsW(lptimefmtenumproc: TIMEFMT_ENUMPROCW, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesA(lpuilanguageenumproc: UILANGUAGE_ENUMPROCA, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesW(lpuilanguageenumproc: UILANGUAGE_ENUMPROCW, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSString(locale: u32, dwfindnlsstringflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, pcchfound: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSStringEx(lplocalename: super::Foundation::PWSTR, dwfindnlsstringflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, pcchfound: *mut i32, lpversioninformation: *const NLSVERSIONINFO, lpreserved: *const ::core::ffi::c_void, sorthandle: super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindStringOrdinal(dwfindstringordinalflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, bignorecase: super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringA(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::Foundation::PSTR, cchdest: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringW(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32) -> i32;
    pub fn GetACP() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfo(codepage: u32, lpcpinfo: *mut CPINFO) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExA(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXA) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExW(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXW) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoEx(lplocalename: super::Foundation::PWSTR, calendar: u32, lpreserved: super::Foundation::PWSTR, caltype: u32, lpcaldata: super::Foundation::PWSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PWSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatA(locale: u32, dwflags: u32, lpvalue: super::Foundation::PSTR, lpformat: *const CURRENCYFMTA, lpcurrencystr: super::Foundation::PSTR, cchcurrency: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const CURRENCYFMTW, lpcurrencystr: super::Foundation::PWSTR, cchcurrency: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatW(locale: u32, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const CURRENCYFMTW, lpcurrencystr: super::Foundation::PWSTR, cchcurrency: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatA(locale: u32, dwflags: u32, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PSTR, lpdatestr: super::Foundation::PSTR, cchdate: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: ENUM_DATE_FORMATS_FLAGS, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lpdatestr: super::Foundation::PWSTR, cchdate: i32, lpcalendar: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatW(locale: u32, dwflags: u32, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lpdatestr: super::Foundation::PWSTR, cchdate: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDistanceOfClosestLanguageInList(pszlanguage: super::Foundation::PWSTR, pszlanguageslist: super::Foundation::PWSTR, wchlistdelimiter: u16, pclosestdistance: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormat(locale: u32, dwflags: u32, lpduration: *const super::Foundation::SYSTEMTIME, ullduration: u64, lpformat: super::Foundation::PWSTR, lpdurationstr: super::Foundation::PWSTR, cchduration: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpduration: *const super::Foundation::SYSTEMTIME, ullduration: u64, lpformat: super::Foundation::PWSTR, lpdurationstr: super::Foundation::PWSTR, cchduration: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIInfo(dwflags: u32, pcwszfilepath: super::Foundation::PWSTR, pfilemuiinfo: *mut FILEMUIINFO, pcbfilemuiinfo: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIPath(dwflags: u32, pcwszfilepath: super::Foundation::PWSTR, pwszlanguage: super::Foundation::PWSTR, pcchlanguage: *mut u32, pwszfilemuipath: super::Foundation::PWSTR, pcchfilemuipath: *mut u32, pululenumerator: *mut u64) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoA(location: i32, geotype: u32, lpgeodata: super::Foundation::PSTR, cchdata: i32, langid: u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoEx(location: super::Foundation::PWSTR, geotype: u32, geodata: super::Foundation::PWSTR, geodatacount: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoW(location: i32, geotype: u32, lpgeodata: super::Foundation::PWSTR, cchdata: i32, langid: u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoA(locale: u32, lctype: u32, lplcdata: super::Foundation::PSTR, cchdata: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoEx(lplocalename: super::Foundation::PWSTR, lctype: u32, lplcdata: super::Foundation::PWSTR, cchdata: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoW(locale: u32, lctype: u32, lplcdata: super::Foundation::PWSTR, cchdata: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersion(function: u32, locale: u32, lpversioninformation: *mut NLSVERSIONINFO) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersionEx(function: u32, lplocalename: super::Foundation::PWSTR, lpversioninformation: *mut NLSVERSIONINFOEX) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatA(locale: u32, dwflags: u32, lpvalue: super::Foundation::PSTR, lpformat: *const NUMBERFMTA, lpnumberstr: super::Foundation::PSTR, cchnumber: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const NUMBERFMTW, lpnumberstr: super::Foundation::PWSTR, cchnumber: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatW(locale: u32, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const NUMBERFMTW, lpnumberstr: super::Foundation::PWSTR, cchnumber: i32) -> i32;
    pub fn GetOEMCP() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringScripts(dwflags: u32, lpstring: super::Foundation::PWSTR, cchstring: i32, lpscripts: super::Foundation::PWSTR, cchscripts: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeA(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExA(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExW(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeW(dwinfotype: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    pub fn GetSystemDefaultLCID() -> u32;
    pub fn GetSystemDefaultLangID() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDefaultLocaleName(lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    pub fn GetSystemDefaultUILanguage() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharset(hdc: super::Graphics::Gdi::HDC) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharsetInfo(hdc: super::Graphics::Gdi::HDC, lpsig: *mut FONTSIGNATURE, dwflags: u32) -> i32;
    pub fn GetThreadLocale() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    pub fn GetThreadUILanguage() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatA(locale: u32, dwflags: u32, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PSTR, lptimestr: super::Foundation::PSTR, cchtime: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: TIME_FORMAT_FLAGS, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lptimestr: super::Foundation::PWSTR, cchtime: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatW(locale: u32, dwflags: u32, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lptimestr: super::Foundation::PWSTR, cchtime: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUILanguageInfo(dwflags: u32, pwmszlanguage: super::Foundation::PWSTR, pwszfallbacklanguages: super::Foundation::PWSTR, pcchfallbacklanguages: *mut u32, pattributes: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultGeoName(geoname: super::Foundation::PWSTR, geonamecount: i32) -> i32;
    pub fn GetUserDefaultLCID() -> u32;
    pub fn GetUserDefaultLangID() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultLocaleName(lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    pub fn GetUserDefaultUILanguage() -> u16;
    pub fn GetUserGeoID(geoclass: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToAscii(dwflags: u32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32, lpasciicharstr: super::Foundation::PWSTR, cchasciichar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToNameprepUnicode(dwflags: u32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32, lpnameprepcharstr: super::Foundation::PWSTR, cchnameprepchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToUnicode(dwflags: u32, lpasciicharstr: super::Foundation::PWSTR, cchasciichar: i32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByte(testchar: u8) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByteEx(codepage: u32, testchar: u8) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNLSDefinedString(function: u32, dwflags: u32, lpversioninformation: *const NLSVERSIONINFO, lpstring: super::Foundation::PWSTR, cchstr: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNormalizedString(normform: NORM_FORM, lpstring: super::Foundation::PWSTR, cwlength: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTextUnicode(lpv: *const ::core::ffi::c_void, isize: i32, lpiresult: *mut IS_TEXT_UNICODE_RESULT) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidCodePage(codepage: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLanguageGroup(languagegroup: u32, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocale(locale: u32, dwflags: IS_VALID_LOCALE_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocaleName(lplocalename: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidNLSVersion(function: u32, lplocalename: super::Foundation::PWSTR, lpversioninformation: *const NLSVERSIONINFOEX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellFormedTag(psztag: super::Foundation::PWSTR) -> u8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCIDToLocaleName(locale: u32, lpname: super::Foundation::PWSTR, cchname: i32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringA(locale: u32, dwmapflags: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::Foundation::PSTR, cchdest: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringEx(lplocalename: super::Foundation::PWSTR, dwmapflags: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32, lpversioninformation: *const NLSVERSIONINFO, lpreserved: *const ::core::ffi::c_void, sorthandle: super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringW(locale: u32, dwmapflags: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocaleNameToLCID(lpname: super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingDoAction(pbag: *mut MAPPING_PROPERTY_BAG, dwrangeindex: u32, pszactionid: super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreePropertyBag(pbag: *const MAPPING_PROPERTY_BAG) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreeServices(pserviceinfo: *const MAPPING_SERVICE_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingGetServices(poptions: *const MAPPING_ENUM_OPTIONS, prgservices: *mut *mut MAPPING_SERVICE_INFO, pdwservicescount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingRecognizeText(pserviceinfo: *const MAPPING_SERVICE_INFO, psztext: super::Foundation::PWSTR, dwlength: u32, dwindex: u32, poptions: *const MAPPING_OPTIONS, pbag: *mut MAPPING_PROPERTY_BAG) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultiByteToWideChar(codepage: u32, dwflags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr: super::Foundation::PSTR, cbmultibyte: i32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeString(normform: NORM_FORM, lpsrcstring: super::Foundation::PWSTR, cwsrclength: i32, lpdststring: super::Foundation::PWSTR, cwdstlength: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyUILanguageChange(dwflags: u32, pcwstrnewlanguage: super::Foundation::PWSTR, pcwstrpreviouslanguage: super::Foundation::PWSTR, dwreserved: u32, pdwstatusrtrn: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveLocaleName(lpnametoresolve: super::Foundation::PWSTR, lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    pub fn RestoreThreadPreferredUILanguages(snapshot: HSAVEDUILANGUAGES);
    pub fn ScriptApplyDigitSubstitution(psds: *const SCRIPT_DIGITSUBSTITUTE, psc: *mut SCRIPT_CONTROL, pss: *mut SCRIPT_STATE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pabc: *mut super::Graphics::Gdi::ABC, pijustify: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptBreak(pwcchars: super::Foundation::PWSTR, cchars: i32, psa: *const SCRIPT_ANALYSIS, psla: *mut SCRIPT_LOGATTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptCPtoX(icp: i32, ftrailing: super::Foundation::BOOL, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pix: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptCacheGetHeight(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, tmheight: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptFreeCache(psc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptGetCMap(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwcinchars: super::Foundation::PWSTR, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontAlternateGlyphs(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, wglyphid: u16, cmaxalternates: i32, palternateglyphs: *mut u16, pcalternates: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontFeatureTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, cmaxtags: i32, pfeaturetags: *mut u32, pctags: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontLanguageTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, cmaxtags: i32, plangsystags: *mut u32, pctags: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontProperties(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, sfp: *mut SCRIPT_FONTPROPERTIES) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontScriptTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, cmaxtags: i32, pscripttags: *mut u32, pctags: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetGlyphABCWidth(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, wglyph: u16, pabc: *mut super::Graphics::Gdi::ABC) -> ::windows_sys::core::HRESULT;
    pub fn ScriptGetLogicalWidths(psa: *const SCRIPT_ANALYSIS, cchars: i32, cglyphs: i32, piglyphwidth: *const i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, pidx: *const i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptGetProperties(ppsp: *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptIsComplex(pwcinchars: super::Foundation::PWSTR, cinchars: i32, dwflags: SCRIPT_IS_COMPLEX_FLAGS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemize(pwcinchars: super::Foundation::PWSTR, cinchars: i32, cmaxitems: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pitems: *mut SCRIPT_ITEM, pcitems: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemizeOpenType(pwcinchars: super::Foundation::PWSTR, cinchars: i32, cmaxitems: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pitems: *mut SCRIPT_ITEM, pscripttags: *mut u32, pcitems: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptJustify(psva: *const SCRIPT_VISATTR, piadvance: *const i32, cglyphs: i32, idx: i32, iminkashida: i32, pijustify: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptLayout(cruns: i32, pblevel: *const u8, pivisualtological: *mut i32, pilogicaltovisual: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPlace(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: *mut super::Graphics::Gdi::ABC) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptPlaceOpenType(
        hdc: super::Graphics::Gdi::HDC,
        psc: *mut *mut ::core::ffi::c_void,
        psa: *mut SCRIPT_ANALYSIS,
        tagscript: u32,
        taglangsys: u32,
        rcrangechars: *const i32,
        rprangeproperties: *const *const textrange_properties,
        cranges: i32,
        pwcchars: super::Foundation::PWSTR,
        pwlogclust: *const u16,
        pcharprops: *const script_charprop,
        cchars: i32,
        pwglyphs: *const u16,
        pglyphprops: *const script_glyphprop,
        cglyphs: i32,
        piadvance: *mut i32,
        pgoffset: *mut GOFFSET,
        pabc: *mut super::Graphics::Gdi::ABC,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPositionSingleGlyph(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> ::windows_sys::core::HRESULT;
    pub fn ScriptRecordDigitSubstitution(locale: u32, psds: *mut SCRIPT_DIGITSUBSTITUTE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShape(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwcchars: super::Foundation::PWSTR, cchars: i32, cmaxglyphs: i32, psa: *mut SCRIPT_ANALYSIS, pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShapeOpenType(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: *const i32, rprangeproperties: *const *const textrange_properties, cranges: i32, pwcchars: super::Foundation::PWSTR, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut script_charprop, pwoutglyphs: *mut u16, poutglyphprops: *mut script_glyphprop, pcglyphs: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptStringAnalyse(hdc: super::Graphics::Gdi::HDC, pstring: *const ::core::ffi::c_void, cstring: i32, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pidx: *const i32, ptabdef: *const SCRIPT_TABDEF, pbinclass: *const u8, pssa: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptStringCPtoX(ssa: *const ::core::ffi::c_void, icp: i32, ftrailing: super::Foundation::BOOL, px: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringFree(pssa: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringGetLogicalWidths(ssa: *const ::core::ffi::c_void, pidx: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringGetOrder(ssa: *const ::core::ffi::c_void, puorder: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptStringOut(ssa: *const ::core::ffi::c_void, ix: i32, iy: i32, uoptions: super::Graphics::Gdi::ETO_OPTIONS, prc: *const super::Foundation::RECT, iminsel: i32, imaxsel: i32, fdisabled: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringValidate(ssa: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringXtoCP(ssa: *const ::core::ffi::c_void, ix: i32, pich: *mut i32, pitrailing: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptString_pLogAttr(ssa: *const ::core::ffi::c_void) -> *mut SCRIPT_LOGATTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptString_pSize(ssa: *const ::core::ffi::c_void) -> *mut super::Foundation::SIZE;
    pub fn ScriptString_pcOutChars(ssa: *const ::core::ffi::c_void) -> *mut i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptSubstituteSingleGlyph(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptTextOut(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, x: i32, y: i32, fuoptions: u32, lprc: *const super::Foundation::RECT, psa: *const SCRIPT_ANALYSIS, pwcreserved: super::Foundation::PWSTR, ireserved: i32, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: *const i32, pgoffset: *const GOFFSET) -> ::windows_sys::core::HRESULT;
    pub fn ScriptXtoCP(ix: i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, picp: *mut i32, pitrailing: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoA(locale: u32, lctype: u32, lplcdata: super::Foundation::PSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoW(locale: u32, lctype: u32, lplcdata: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPreferredUILanguages(dwflags: u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pulnumlanguages: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadLocale(locale: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages(dwflags: u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pulnumlanguages: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages2(flags: u32, languages: super::Foundation::PWSTR, numlanguagesset: *mut u32, snapshot: *mut HSAVEDUILANGUAGES) -> super::Foundation::BOOL;
    pub fn SetThreadUILanguage(langid: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoID(geoid: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoName(geoname: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateCharsetInfo(lpsrc: *mut u32, lpcs: *mut CHARSETINFO, dwflags: TRANSLATE_CHARSET_INFO_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_ESCAPE(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SKIP(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_STOP(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_ESCAPE(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SKIP(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_STOP(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyScripts(dwflags: u32, lplocalescripts: super::Foundation::PWSTR, cchlocalescripts: i32, lptestscripts: super::Foundation::PWSTR, cchtestscripts: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WideCharToMultiByte(codepage: u32, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32, lpmultibytestr: super::Foundation::PSTR, cbmultibyte: i32, lpdefaultchar: super::Foundation::PSTR, lpuseddefaultchar: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR, imaxlength: i32) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR, imaxlength: i32) -> super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenA(lpstring: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenW(lpstring: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_UCharsToChars(us: *const u16, cs: super::Foundation::PSTR, length: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrcpy(dst: super::Foundation::PSTR, src: *const u16) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrncpy(dst: super::Foundation::PSTR, src: *const u16, n: i32) -> super::Foundation::PSTR;
    pub fn u_catclose(catd: *mut UResourceBundle);
    pub fn u_catgets(catd: *mut UResourceBundle, set_num: i32, msg_num: i32, s: *const u16, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_catopen(name: super::Foundation::PSTR, locale: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn u_charAge(c: i32, versionarray: *mut u8);
    pub fn u_charDigitValue(c: i32) -> i32;
    pub fn u_charDirection(c: i32) -> UCharDirection;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charFromName(namechoice: UCharNameChoice, name: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_charMirror(c: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charName(code: i32, namechoice: UCharNameChoice, buffer: super::Foundation::PSTR, bufferlength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_charType(c: i32) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charsToUChars(cs: super::Foundation::PSTR, us: *mut u16, length: i32);
    pub fn u_cleanup();
    pub fn u_countChar32(s: *const u16, length: i32) -> i32;
    pub fn u_digit(ch: i32, radix: i8) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_enumCharNames(start: i32, limit: i32, r#fn: *mut UEnumCharNamesFn, context: *mut ::core::ffi::c_void, namechoice: UCharNameChoice, perrorcode: *mut UErrorCode);
    pub fn u_enumCharTypes(enumrange: *mut UCharEnumTypeRange, context: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_errorName(code: UErrorCode) -> super::Foundation::PSTR;
    pub fn u_foldCase(c: i32, options: u32) -> i32;
    pub fn u_forDigit(digit: i32, radix: i8) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32;
    pub fn u_getBidiPairedBracket(c: i32) -> i32;
    pub fn u_getBinaryPropertySet(property: UProperty, perrorcode: *mut UErrorCode) -> *mut USet;
    pub fn u_getCombiningClass(c: i32) -> u8;
    pub fn u_getDataVersion(dataversionfillin: *mut u8, status: *mut UErrorCode);
    pub fn u_getFC_NFKC_Closure(c: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_getIntPropertyMap(property: UProperty, perrorcode: *mut UErrorCode) -> *mut UCPMap;
    pub fn u_getIntPropertyMaxValue(which: UProperty) -> i32;
    pub fn u_getIntPropertyMinValue(which: UProperty) -> i32;
    pub fn u_getIntPropertyValue(c: i32, which: UProperty) -> i32;
    pub fn u_getNumericValue(c: i32) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyEnum(alias: super::Foundation::PSTR) -> UProperty;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyName(property: UProperty, namechoice: UPropertyNameChoice) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueEnum(property: UProperty, alias: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueName(property: UProperty, value: i32, namechoice: UPropertyNameChoice) -> super::Foundation::PSTR;
    pub fn u_getUnicodeVersion(versionarray: *mut u8);
    pub fn u_getVersion(versionarray: *mut u8);
    pub fn u_hasBinaryProperty(c: i32, which: UProperty) -> i8;
    pub fn u_init(status: *mut UErrorCode);
    pub fn u_isIDIgnorable(c: i32) -> i8;
    pub fn u_isIDPart(c: i32) -> i8;
    pub fn u_isIDStart(c: i32) -> i8;
    pub fn u_isISOControl(c: i32) -> i8;
    pub fn u_isJavaIDPart(c: i32) -> i8;
    pub fn u_isJavaIDStart(c: i32) -> i8;
    pub fn u_isJavaSpaceChar(c: i32) -> i8;
    pub fn u_isMirrored(c: i32) -> i8;
    pub fn u_isUAlphabetic(c: i32) -> i8;
    pub fn u_isULowercase(c: i32) -> i8;
    pub fn u_isUUppercase(c: i32) -> i8;
    pub fn u_isUWhiteSpace(c: i32) -> i8;
    pub fn u_isWhitespace(c: i32) -> i8;
    pub fn u_isalnum(c: i32) -> i8;
    pub fn u_isalpha(c: i32) -> i8;
    pub fn u_isbase(c: i32) -> i8;
    pub fn u_isblank(c: i32) -> i8;
    pub fn u_iscntrl(c: i32) -> i8;
    pub fn u_isdefined(c: i32) -> i8;
    pub fn u_isdigit(c: i32) -> i8;
    pub fn u_isgraph(c: i32) -> i8;
    pub fn u_islower(c: i32) -> i8;
    pub fn u_isprint(c: i32) -> i8;
    pub fn u_ispunct(c: i32) -> i8;
    pub fn u_isspace(c: i32) -> i8;
    pub fn u_istitle(c: i32) -> i8;
    pub fn u_isupper(c: i32) -> i8;
    pub fn u_isxdigit(c: i32) -> i8;
    pub fn u_memcasecmp(s1: *const u16, s2: *const u16, length: i32, options: u32) -> i32;
    pub fn u_memchr(s: *const u16, c: u16, count: i32) -> *mut u16;
    pub fn u_memchr32(s: *const u16, c: i32, count: i32) -> *mut u16;
    pub fn u_memcmp(buf1: *const u16, buf2: *const u16, count: i32) -> i32;
    pub fn u_memcmpCodePointOrder(s1: *const u16, s2: *const u16, count: i32) -> i32;
    pub fn u_memcpy(dest: *mut u16, src: *const u16, count: i32) -> *mut u16;
    pub fn u_memmove(dest: *mut u16, src: *const u16, count: i32) -> *mut u16;
    pub fn u_memrchr(s: *const u16, c: u16, count: i32) -> *mut u16;
    pub fn u_memrchr32(s: *const u16, c: i32, count: i32) -> *mut u16;
    pub fn u_memset(dest: *mut u16, c: u16, count: i32) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn u_setMemoryFunctions(context: *const ::core::ffi::c_void, a: *mut UMemAllocFn, r: *mut UMemReallocFn, f: *mut UMemFreeFn, status: *mut UErrorCode);
    pub fn u_shapeArabic(source: *const u16, sourcelength: i32, dest: *mut u16, destsize: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_strCaseCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_strCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, codepointorder: i8) -> i32;
    pub fn u_strCompareIter(iter1: *mut UCharIterator, iter2: *mut UCharIterator, codepointorder: i8) -> i32;
    pub fn u_strFindFirst(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16;
    pub fn u_strFindLast(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16;
    pub fn u_strFoldCase(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromJavaModifiedUTF8WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    pub fn u_strFromUTF32(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    pub fn u_strFromUTF32WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8Lenient(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromWCS(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PWSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    pub fn u_strHasMoreChar32Than(s: *const u16, length: i32, number: i32) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToJavaModifiedUTF8(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToLower(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToTitle(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, titleiter: *mut UBreakIterator, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_strToUTF32(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i32;
    pub fn u_strToUTF32WithSub(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8WithSub(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUpper(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToWCS(dest: super::Foundation::PWSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PWSTR;
    pub fn u_strcasecmp(s1: *const u16, s2: *const u16, options: u32) -> i32;
    pub fn u_strcat(dst: *mut u16, src: *const u16) -> *mut u16;
    pub fn u_strchr(s: *const u16, c: u16) -> *mut u16;
    pub fn u_strchr32(s: *const u16, c: i32) -> *mut u16;
    pub fn u_strcmp(s1: *const u16, s2: *const u16) -> i32;
    pub fn u_strcmpCodePointOrder(s1: *const u16, s2: *const u16) -> i32;
    pub fn u_strcpy(dst: *mut u16, src: *const u16) -> *mut u16;
    pub fn u_strcspn(string: *const u16, matchset: *const u16) -> i32;
    pub fn u_strlen(s: *const u16) -> i32;
    pub fn u_strncasecmp(s1: *const u16, s2: *const u16, n: i32, options: u32) -> i32;
    pub fn u_strncat(dst: *mut u16, src: *const u16, n: i32) -> *mut u16;
    pub fn u_strncmp(ucs1: *const u16, ucs2: *const u16, n: i32) -> i32;
    pub fn u_strncmpCodePointOrder(s1: *const u16, s2: *const u16, n: i32) -> i32;
    pub fn u_strncpy(dst: *mut u16, src: *const u16, n: i32) -> *mut u16;
    pub fn u_strpbrk(string: *const u16, matchset: *const u16) -> *mut u16;
    pub fn u_strrchr(s: *const u16, c: u16) -> *mut u16;
    pub fn u_strrchr32(s: *const u16, c: i32) -> *mut u16;
    pub fn u_strrstr(s: *const u16, substring: *const u16) -> *mut u16;
    pub fn u_strspn(string: *const u16, matchset: *const u16) -> i32;
    pub fn u_strstr(s: *const u16, substring: *const u16) -> *mut u16;
    pub fn u_strtok_r(src: *mut u16, delim: *const u16, savestate: *mut *mut u16) -> *mut u16;
    pub fn u_tolower(c: i32) -> i32;
    pub fn u_totitle(c: i32) -> i32;
    pub fn u_toupper(c: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrcpy(dst: *mut u16, src: super::Foundation::PSTR) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrncpy(dst: *mut u16, src: super::Foundation::PSTR, n: i32) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_unescape(src: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32) -> i32;
    pub fn u_unescapeAt(charat: UNESCAPE_CHAR_AT, offset: *mut i32, length: i32, context: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionFromString(versionarray: *mut u8, versionstring: super::Foundation::PSTR);
    pub fn u_versionFromUString(versionarray: *mut u8, versionstring: *const u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionToString(versionarray: *const u8, versionstring: super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn ubidi_close(pbidi: *mut UBiDi);
    pub fn ubidi_countParagraphs(pbidi: *mut UBiDi) -> i32;
    pub fn ubidi_countRuns(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getBaseDirection(text: *const u16, length: i32) -> UBiDiDirection;
    pub fn ubidi_getClassCallback(pbidi: *mut UBiDi, r#fn: *mut UBiDiClassCallback, context: *const *const ::core::ffi::c_void);
    pub fn ubidi_getCustomizedClass(pbidi: *mut UBiDi, c: i32) -> UCharDirection;
    pub fn ubidi_getDirection(pbidi: *const UBiDi) -> UBiDiDirection;
    pub fn ubidi_getLength(pbidi: *const UBiDi) -> i32;
    pub fn ubidi_getLevelAt(pbidi: *const UBiDi, charindex: i32) -> u8;
    pub fn ubidi_getLevels(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> *mut u8;
    pub fn ubidi_getLogicalIndex(pbidi: *mut UBiDi, visualindex: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getLogicalMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode);
    pub fn ubidi_getLogicalRun(pbidi: *const UBiDi, logicalposition: i32, plogicallimit: *mut i32, plevel: *mut u8);
    pub fn ubidi_getParaLevel(pbidi: *const UBiDi) -> u8;
    pub fn ubidi_getParagraph(pbidi: *const UBiDi, charindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getParagraphByIndex(pbidi: *const UBiDi, paraindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode);
    pub fn ubidi_getProcessedLength(pbidi: *const UBiDi) -> i32;
    pub fn ubidi_getReorderingMode(pbidi: *mut UBiDi) -> UBiDiReorderingMode;
    pub fn ubidi_getReorderingOptions(pbidi: *mut UBiDi) -> u32;
    pub fn ubidi_getResultLength(pbidi: *const UBiDi) -> i32;
    pub fn ubidi_getText(pbidi: *const UBiDi) -> *mut u16;
    pub fn ubidi_getVisualIndex(pbidi: *mut UBiDi, logicalindex: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getVisualMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode);
    pub fn ubidi_getVisualRun(pbidi: *mut UBiDi, runindex: i32, plogicalstart: *mut i32, plength: *mut i32) -> UBiDiDirection;
    pub fn ubidi_invertMap(srcmap: *const i32, destmap: *mut i32, length: i32);
    pub fn ubidi_isInverse(pbidi: *mut UBiDi) -> i8;
    pub fn ubidi_isOrderParagraphsLTR(pbidi: *mut UBiDi) -> i8;
    pub fn ubidi_open() -> *mut UBiDi;
    pub fn ubidi_openSized(maxlength: i32, maxruncount: i32, perrorcode: *mut UErrorCode) -> *mut UBiDi;
    pub fn ubidi_orderParagraphsLTR(pbidi: *mut UBiDi, orderparagraphsltr: i8);
    pub fn ubidi_reorderLogical(levels: *const u8, length: i32, indexmap: *mut i32);
    pub fn ubidi_reorderVisual(levels: *const u8, length: i32, indexmap: *mut i32);
    pub fn ubidi_setClassCallback(pbidi: *mut UBiDi, newfn: UBiDiClassCallback, newcontext: *const ::core::ffi::c_void, oldfn: *mut UBiDiClassCallback, oldcontext: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode);
    pub fn ubidi_setContext(pbidi: *mut UBiDi, prologue: *const u16, prolength: i32, epilogue: *const u16, epilength: i32, perrorcode: *mut UErrorCode);
    pub fn ubidi_setInverse(pbidi: *mut UBiDi, isinverse: i8);
    pub fn ubidi_setLine(pparabidi: *const UBiDi, start: i32, limit: i32, plinebidi: *mut UBiDi, perrorcode: *mut UErrorCode);
    pub fn ubidi_setPara(pbidi: *mut UBiDi, text: *const u16, length: i32, paralevel: u8, embeddinglevels: *mut u8, perrorcode: *mut UErrorCode);
    pub fn ubidi_setReorderingMode(pbidi: *mut UBiDi, reorderingmode: UBiDiReorderingMode);
    pub fn ubidi_setReorderingOptions(pbidi: *mut UBiDi, reorderingoptions: u32);
    pub fn ubidi_writeReordered(pbidi: *mut UBiDi, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_writeReverse(src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubiditransform_close(pbiditransform: *mut UBiDiTransform);
    pub fn ubiditransform_open(perrorcode: *mut UErrorCode) -> *mut UBiDiTransform;
    pub fn ubiditransform_transform(pbiditransform: *mut UBiDiTransform, src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, inparalevel: u8, inorder: UBiDiOrder, outparalevel: u8, outorder: UBiDiOrder, domirroring: UBiDiMirroring, shapingoptions: u32, perrorcode: *mut UErrorCode) -> u32;
    pub fn ublock_getCode(c: i32) -> UBlockCode;
    pub fn ubrk_close(bi: *mut UBreakIterator);
    pub fn ubrk_countAvailable() -> i32;
    pub fn ubrk_current(bi: *const UBreakIterator) -> i32;
    pub fn ubrk_first(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_following(bi: *mut UBreakIterator, offset: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getAvailable(index: i32) -> super::Foundation::PSTR;
    pub fn ubrk_getBinaryRules(bi: *mut UBreakIterator, binaryrules: *mut u8, rulescapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getLocaleByType(bi: *const UBreakIterator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator, fillinvec: *mut i32, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: i32) -> i8;
    pub fn ubrk_last(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_next(bi: *mut UBreakIterator) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_open(r#type: UBreakIteratorType, locale: super::Foundation::PSTR, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_openBinaryRules(binaryrules: *const u8, ruleslength: i32, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_openRules(rules: *const u16, ruleslength: i32, text: *const u16, textlength: i32, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_preceding(bi: *mut UBreakIterator, offset: i32) -> i32;
    pub fn ubrk_previous(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode);
    pub fn ubrk_safeClone(bi: *const UBreakIterator, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_setText(bi: *mut UBreakIterator, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode);
    pub fn ucal_add(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode);
    pub fn ucal_clear(calendar: *mut *mut ::core::ffi::c_void);
    pub fn ucal_clearField(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields);
    pub fn ucal_clone(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn ucal_close(cal: *mut *mut ::core::ffi::c_void);
    pub fn ucal_countAvailable() -> i32;
    pub fn ucal_equivalentTo(cal1: *const *const ::core::ffi::c_void, cal2: *const *const ::core::ffi::c_void) -> i8;
    pub fn ucal_get(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields, status: *mut UErrorCode) -> i32;
    pub fn ucal_getAttribute(cal: *const *const ::core::ffi::c_void, attr: UCalendarAttribute) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn ucal_getCanonicalTimeZoneID(id: *const u16, len: i32, result: *mut u16, resultcapacity: i32, issystemid: *mut i8, status: *mut UErrorCode) -> i32;
    pub fn ucal_getDSTSavings(zoneid: *const u16, ec: *mut UErrorCode) -> i32;
    pub fn ucal_getDayOfWeekType(cal: *const *const ::core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> UCalendarWeekdayType;
    pub fn ucal_getDefaultTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn ucal_getFieldDifference(cal: *mut *mut ::core::ffi::c_void, target: f64, field: UCalendarDateFields, status: *mut UErrorCode) -> i32;
    pub fn ucal_getGregorianChange(cal: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> f64;
    pub fn ucal_getHostTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_getLimit(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields, r#type: UCalendarLimitType, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getLocaleByType(cal: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucal_getMillis(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    pub fn ucal_getNow() -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTZDataVersion(status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneDisplayName(cal: *const *const ::core::ffi::c_void, r#type: UCalendarDisplayNameType, locale: super::Foundation::PSTR, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ucal_getTimeZoneID(cal: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneIDForWindowsID(winid: *const u16, len: i32, region: super::Foundation::PSTR, id: *mut u16, idcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucal_getTimeZoneTransitionDate(cal: *const *const ::core::ffi::c_void, r#type: UTimeZoneTransitionType, transition: *mut f64, status: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getType(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucal_getWeekendTransition(cal: *const *const ::core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> i32;
    pub fn ucal_getWindowsTimeZoneID(id: *const u16, len: i32, winid: *mut u16, winidcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucal_inDaylightTime(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> i8;
    pub fn ucal_isSet(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields) -> i8;
    pub fn ucal_isWeekend(cal: *const *const ::core::ffi::c_void, date: f64, status: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_open(zoneid: *const u16, len: i32, locale: super::Foundation::PSTR, r#type: UCalendarType, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openCountryTimeZones(country: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openTimeZoneIDEnumeration(zonetype: USystemTimeZoneType, region: super::Foundation::PSTR, rawoffset: *const i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_roll(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode);
    pub fn ucal_set(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, value: i32);
    pub fn ucal_setAttribute(cal: *mut *mut ::core::ffi::c_void, attr: UCalendarAttribute, newvalue: i32);
    pub fn ucal_setDate(cal: *mut *mut ::core::ffi::c_void, year: i32, month: i32, date: i32, status: *mut UErrorCode);
    pub fn ucal_setDateTime(cal: *mut *mut ::core::ffi::c_void, year: i32, month: i32, date: i32, hour: i32, minute: i32, second: i32, status: *mut UErrorCode);
    pub fn ucal_setDefaultTimeZone(zoneid: *const u16, ec: *mut UErrorCode);
    pub fn ucal_setGregorianChange(cal: *mut *mut ::core::ffi::c_void, date: f64, perrorcode: *mut UErrorCode);
    pub fn ucal_setMillis(cal: *mut *mut ::core::ffi::c_void, datetime: f64, status: *mut UErrorCode);
    pub fn ucal_setTimeZone(cal: *mut *mut ::core::ffi::c_void, zoneid: *const u16, len: i32, status: *mut UErrorCode);
    pub fn ucasemap_close(csm: *mut UCaseMap);
    pub fn ucasemap_getBreakIterator(csm: *const UCaseMap) -> *mut UBreakIterator;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_getLocale(csm: *const UCaseMap) -> super::Foundation::PSTR;
    pub fn ucasemap_getOptions(csm: *const UCaseMap) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_open(locale: super::Foundation::PSTR, options: u32, perrorcode: *mut UErrorCode) -> *mut UCaseMap;
    pub fn ucasemap_setBreakIterator(csm: *mut UCaseMap, itertoadopt: *mut UBreakIterator, perrorcode: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_setLocale(csm: *mut UCaseMap, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode);
    pub fn ucasemap_setOptions(csm: *mut UCaseMap, options: u32, perrorcode: *mut UErrorCode);
    pub fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8FoldCase(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToLower(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToTitle(csm: *mut UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToUpper(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucfpos_close(ucfpos: *mut UConstrainedFieldPosition);
    pub fn ucfpos_constrainCategory(ucfpos: *mut UConstrainedFieldPosition, category: i32, ec: *mut UErrorCode);
    pub fn ucfpos_constrainField(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode);
    pub fn ucfpos_getCategory(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32;
    pub fn ucfpos_getField(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32;
    pub fn ucfpos_getIndexes(ucfpos: *const UConstrainedFieldPosition, pstart: *mut i32, plimit: *mut i32, ec: *mut UErrorCode);
    pub fn ucfpos_getInt64IterationContext(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i64;
    pub fn ucfpos_matchesField(ucfpos: *const UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) -> i8;
    pub fn ucfpos_open(ec: *mut UErrorCode) -> *mut UConstrainedFieldPosition;
    pub fn ucfpos_reset(ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode);
    pub fn ucfpos_setInt64IterationContext(ucfpos: *mut UConstrainedFieldPosition, context: i64, ec: *mut UErrorCode);
    pub fn ucfpos_setState(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, start: i32, limit: i32, ec: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteBytes(args: *mut UConverterFromUnicodeArgs, source: super::Foundation::PSTR, length: i32, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteSub(args: *mut UConverterFromUnicodeArgs, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteUChars(args: *mut UConverterFromUnicodeArgs, source: *const *const u16, sourcelimit: *const u16, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteSub(args: *mut UConverterToUnicodeArgs, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteUChars(args: *mut UConverterToUnicodeArgs, source: *const u16, length: i32, offsetindex: i32, err: *mut UErrorCode);
    pub fn ucnv_close(converter: *mut UConverter);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_compareNames(name1: super::Foundation::PSTR, name2: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convert(toconvertername: super::Foundation::PSTR, fromconvertername: super::Foundation::PSTR, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convertEx(targetcnv: *mut UConverter, sourcecnv: *mut UConverter, target: *mut *mut i8, targetlimit: super::Foundation::PSTR, source: *const *const i8, sourcelimit: super::Foundation::PSTR, pivotstart: *mut u16, pivotsource: *mut *mut u16, pivottarget: *mut *mut u16, pivotlimit: *const u16, reset: i8, flush: i8, perrorcode: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_countAliases(alias: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> u16;
    pub fn ucnv_countAvailable() -> i32;
    pub fn ucnv_countStandards() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_detectUnicodeSignature(source: super::Foundation::PSTR, sourcelength: i32, signaturelength: *mut i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucnv_fixFileSeparator(cnv: *const UConverter, source: *mut u16, sourcelen: i32);
    pub fn ucnv_flushCache() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromAlgorithmic(cnv: *mut UConverter, algorithmictype: UConverterType, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUChars(cnv: *mut UConverter, dest: super::Foundation::PSTR, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucnv_fromUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUnicode(converter: *mut UConverter, target: *mut *mut i8, targetlimit: super::Foundation::PSTR, source: *const *const u16, sourcelimit: *const u16, offsets: *mut i32, flush: i8, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAlias(alias: super::Foundation::PSTR, n: u16, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAliases(alias: super::Foundation::PSTR, aliases: *const *const i8, perrorcode: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAvailableName(n: i32) -> super::Foundation::PSTR;
    pub fn ucnv_getCCSID(converter: *const UConverter, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getCanonicalName(alias: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDefaultName() -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDisplayName(converter: *const UConverter, displaylocale: super::Foundation::PSTR, displayname: *mut u16, displaynamecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getFromUCallBack(converter: *const UConverter, action: *mut UConverterFromUCallback, context: *const *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getInvalidChars(converter: *const UConverter, errbytes: super::Foundation::PSTR, len: *mut i8, err: *mut UErrorCode);
    pub fn ucnv_getInvalidUChars(converter: *const UConverter, erruchars: *mut u16, len: *mut i8, err: *mut UErrorCode);
    pub fn ucnv_getMaxCharSize(converter: *const UConverter) -> i8;
    pub fn ucnv_getMinCharSize(converter: *const UConverter) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getName(converter: *const UConverter, err: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getNextUChar(converter: *mut UConverter, source: *const *const i8, sourcelimit: super::Foundation::PSTR, err: *mut UErrorCode) -> i32;
    pub fn ucnv_getPlatform(converter: *const UConverter, err: *mut UErrorCode) -> UConverterPlatform;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandard(n: u16, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandardName(name: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucnv_getStarters(converter: *const UConverter, starters: *mut i8, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getSubstChars(converter: *const UConverter, subchars: super::Foundation::PSTR, len: *mut i8, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getToUCallBack(converter: *const UConverter, action: *mut UConverterToUCallback, context: *const *const ::core::ffi::c_void);
    pub fn ucnv_getType(converter: *const UConverter) -> UConverterType;
    pub fn ucnv_getUnicodeSet(cnv: *const UConverter, setfillin: *mut USet, whichset: UConverterUnicodeSet, perrorcode: *mut UErrorCode);
    pub fn ucnv_isAmbiguous(cnv: *const UConverter) -> i8;
    pub fn ucnv_isFixedWidth(cnv: *mut UConverter, status: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_open(convertername: super::Foundation::PSTR, err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_openAllNames(perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucnv_openCCSID(codepage: i32, platform: UConverterPlatform, err: *mut UErrorCode) -> *mut UConverter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openPackage(packagename: super::Foundation::PSTR, convertername: super::Foundation::PSTR, err: *mut UErrorCode) -> *mut UConverter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openStandardNames(convname: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucnv_openU(name: *const u16, err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_reset(converter: *mut UConverter);
    pub fn ucnv_resetFromUnicode(converter: *mut UConverter);
    pub fn ucnv_resetToUnicode(converter: *mut UConverter);
    pub fn ucnv_safeClone(cnv: *const UConverter, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UConverter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setDefaultName(name: super::Foundation::PSTR);
    pub fn ucnv_setFallback(cnv: *mut UConverter, usesfallback: i8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setFromUCallBack(converter: *mut UConverter, newaction: UConverterFromUCallback, newcontext: *const ::core::ffi::c_void, oldaction: *mut UConverterFromUCallback, oldcontext: *const *const ::core::ffi::c_void, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setSubstChars(converter: *mut UConverter, subchars: super::Foundation::PSTR, len: i8, err: *mut UErrorCode);
    pub fn ucnv_setSubstString(cnv: *mut UConverter, s: *const u16, length: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setToUCallBack(converter: *mut UConverter, newaction: UConverterToUCallback, newcontext: *const ::core::ffi::c_void, oldaction: *mut UConverterToUCallback, oldcontext: *const *const ::core::ffi::c_void, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toAlgorithmic(algorithmictype: UConverterType, cnv: *mut UConverter, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUChars(cnv: *mut UConverter, dest: *mut u16, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucnv_toUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUnicode(converter: *mut UConverter, target: *mut *mut u16, targetlimit: *const u16, source: *const *const i8, sourcelimit: super::Foundation::PSTR, offsets: *mut i32, flush: i8, err: *mut UErrorCode);
    pub fn ucnv_usesFallback(cnv: *const UConverter) -> i8;
    pub fn ucnvsel_close(sel: *mut UConverterSelector);
    pub fn ucnvsel_open(converterlist: *const *const i8, converterlistsize: i32, excludedcodepoints: *const USet, whichset: UConverterUnicodeSet, status: *mut UErrorCode) -> *mut UConverterSelector;
    pub fn ucnvsel_openFromSerialized(buffer: *const ::core::ffi::c_void, length: i32, status: *mut UErrorCode) -> *mut UConverterSelector;
    pub fn ucnvsel_selectForString(sel: *const UConverterSelector, s: *const u16, length: i32, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnvsel_selectForUTF8(sel: *const UConverterSelector, s: super::Foundation::PSTR, length: i32, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucnvsel_serialize(sel: *const UConverterSelector, buffer: *mut ::core::ffi::c_void, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut u8, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_close(coll: *mut UCollator);
    pub fn ucol_closeElements(elems: *mut UCollationElements);
    pub fn ucol_countAvailable() -> i32;
    pub fn ucol_equal(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    pub fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute, status: *mut UErrorCode) -> UColAttributeValue;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn ucol_getBound(source: *const u8, sourcelength: i32, boundtype: UColBoundMode, nooflevels: u32, result: *mut u8, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_getContractionsAndExpansions(coll: *const UCollator, contractions: *mut USet, expansions: *mut USet, addprefixes: i8, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getDisplayName(objloc: super::Foundation::PSTR, disploc: super::Foundation::PSTR, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_getEquivalentReorderCodes(reordercode: i32, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getFunctionalEquivalent(result: super::Foundation::PSTR, resultcapacity: i32, keyword: super::Foundation::PSTR, locale: super::Foundation::PSTR, isavailable: *mut i8, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValues(keyword: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getLocaleByType(coll: *const UCollator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucol_getMaxExpansion(elems: *const UCollationElements, order: i32) -> i32;
    pub fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode;
    pub fn ucol_getOffset(elems: *const UCollationElements) -> i32;
    pub fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucol_getRules(coll: *const UCollator, length: *mut i32) -> *mut u16;
    pub fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption, buffer: *mut u16, bufferlen: i32) -> i32;
    pub fn ucol_getSortKey(coll: *const UCollator, source: *const u16, sourcelength: i32, result: *mut u8, resultlength: i32) -> i32;
    pub fn ucol_getStrength(coll: *const UCollator) -> UColAttributeValue;
    pub fn ucol_getTailoredSet(coll: *const UCollator, status: *mut UErrorCode) -> *mut USet;
    pub fn ucol_getUCAVersion(coll: *const UCollator, info: *mut u8);
    pub fn ucol_getVariableTop(coll: *const UCollator, status: *mut UErrorCode) -> u32;
    pub fn ucol_getVersion(coll: *const UCollator, info: *mut u8);
    pub fn ucol_greater(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    pub fn ucol_greaterOrEqual(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    pub fn ucol_keyHashCode(key: *const u8, length: i32) -> i32;
    pub fn ucol_mergeSortkeys(src1: *const u8, src1length: i32, src2: *const u8, src2length: i32, dest: *mut u8, destcapacity: i32) -> i32;
    pub fn ucol_next(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32;
    pub fn ucol_nextSortKeyPart(coll: *const UCollator, iter: *mut UCharIterator, state: *mut u32, dest: *mut u8, count: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_open(loc: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_openAvailableLocales(status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucol_openBinary(bin: *const u8, length: i32, base: *const UCollator, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_openElements(coll: *const UCollator, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UCollationElements;
    pub fn ucol_openRules(rules: *const u16, ruleslength: i32, normalizationmode: UColAttributeValue, strength: UColAttributeValue, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_previous(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32;
    pub fn ucol_primaryOrder(order: i32) -> i32;
    pub fn ucol_reset(elems: *mut UCollationElements);
    pub fn ucol_safeClone(coll: *const UCollator, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_secondaryOrder(order: i32) -> i32;
    pub fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute, value: UColAttributeValue, status: *mut UErrorCode);
    pub fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode, perrorcode: *mut UErrorCode);
    pub fn ucol_setOffset(elems: *mut UCollationElements, offset: i32, status: *mut UErrorCode);
    pub fn ucol_setReorderCodes(coll: *mut UCollator, reordercodes: *const i32, reordercodeslength: i32, perrorcode: *mut UErrorCode);
    pub fn ucol_setStrength(coll: *mut UCollator, strength: UColAttributeValue);
    pub fn ucol_setText(elems: *mut UCollationElements, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn ucol_strcoll(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> UCollationResult;
    pub fn ucol_strcollIter(coll: *const UCollator, siter: *mut UCharIterator, titer: *mut UCharIterator, status: *mut UErrorCode) -> UCollationResult;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_strcollUTF8(coll: *const UCollator, source: super::Foundation::PSTR, sourcelength: i32, target: super::Foundation::PSTR, targetlength: i32, status: *mut UErrorCode) -> UCollationResult;
    pub fn ucol_tertiaryOrder(order: i32) -> i32;
    pub fn ucpmap_get(map: *const UCPMap, c: i32) -> u32;
    pub fn ucpmap_getRange(map: *const UCPMap, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    pub fn ucptrie_close(trie: *mut UCPTrie);
    pub fn ucptrie_get(trie: *const UCPTrie, c: i32) -> u32;
    pub fn ucptrie_getRange(trie: *const UCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    pub fn ucptrie_getType(trie: *const UCPTrie) -> UCPTrieType;
    pub fn ucptrie_getValueWidth(trie: *const UCPTrie) -> UCPTrieValueWidth;
    pub fn ucptrie_internalSmallIndex(trie: *const UCPTrie, c: i32) -> i32;
    pub fn ucptrie_internalSmallU8Index(trie: *const UCPTrie, lt1: i32, t2: u8, t3: u8) -> i32;
    pub fn ucptrie_internalU8PrevIndex(trie: *const UCPTrie, c: i32, start: *const u8, src: *const u8) -> i32;
    pub fn ucptrie_openFromBinary(r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, data: *const ::core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut UCPTrie;
    pub fn ucptrie_toBinary(trie: *const UCPTrie, data: *mut ::core::ffi::c_void, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucsdet_close(ucsd: *mut UCharsetDetector);
    pub fn ucsdet_detect(ucsd: *mut UCharsetDetector, status: *mut UErrorCode) -> *mut UCharsetMatch;
    pub fn ucsdet_detectAll(ucsd: *mut UCharsetDetector, matchesfound: *mut i32, status: *mut UErrorCode) -> *mut *mut UCharsetMatch;
    pub fn ucsdet_enableInputFilter(ucsd: *mut UCharsetDetector, filter: i8) -> i8;
    pub fn ucsdet_getAllDetectableCharsets(ucsd: *const UCharsetDetector, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucsdet_getConfidence(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getLanguage(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getName(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucsdet_getUChars(ucsm: *const UCharsetMatch, buf: *mut u16, cap: i32, status: *mut UErrorCode) -> i32;
    pub fn ucsdet_isInputFilterEnabled(ucsd: *const UCharsetDetector) -> i8;
    pub fn ucsdet_open(status: *mut UErrorCode) -> *mut UCharsetDetector;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setDeclaredEncoding(ucsd: *mut UCharsetDetector, encoding: super::Foundation::PSTR, length: i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setText(ucsd: *mut UCharsetDetector, textin: super::Foundation::PSTR, len: i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_countCurrencies(locale: super::Foundation::PSTR, date: f64, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocale(locale: super::Foundation::PSTR, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocaleAndDate(locale: super::Foundation::PSTR, date: f64, index: i32, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn ucurr_getDefaultFractionDigits(currency: *const u16, ec: *mut UErrorCode) -> i32;
    pub fn ucurr_getDefaultFractionDigitsForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getName(currency: *const u16, locale: super::Foundation::PSTR, namestyle: UCurrNameStyle, ischoiceformat: *mut i8, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    pub fn ucurr_getNumericCode(currency: *const u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getPluralName(currency: *const u16, locale: super::Foundation::PSTR, ischoiceformat: *mut i8, pluralcount: super::Foundation::PSTR, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    pub fn ucurr_getRoundingIncrement(currency: *const u16, ec: *mut UErrorCode) -> f64;
    pub fn ucurr_getRoundingIncrementForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> f64;
    pub fn ucurr_isAvailable(isocode: *const u16, from: f64, to: f64, errorcode: *mut UErrorCode) -> i8;
    pub fn ucurr_openISOCurrencies(currtype: u32, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_register(isocode: *const u16, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    pub fn ucurr_unregister(key: *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i8;
    pub fn udat_adoptNumberFormat(fmt: *mut *mut ::core::ffi::c_void, numberformattoadopt: *mut *mut ::core::ffi::c_void);
    pub fn udat_adoptNumberFormatForFields(fmt: *mut *mut ::core::ffi::c_void, fields: *const u16, numberformattoset: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn udat_applyPattern(format: *mut *mut ::core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32);
    pub fn udat_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_close(format: *mut *mut ::core::ffi::c_void);
    pub fn udat_countAvailable() -> i32;
    pub fn udat_countSymbols(fmt: *const *const ::core::ffi::c_void, r#type: UDateFormatSymbolType) -> i32;
    pub fn udat_format(format: *const *const ::core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn udat_formatCalendar(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, result: *mut u16, capacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn udat_formatCalendarForFields(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, result: *mut u16, capacity: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    pub fn udat_formatForFields(format: *const *const ::core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    pub fn udat_get2DigitYearStart(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn udat_getBooleanAttribute(fmt: *const *const ::core::ffi::c_void, attr: UDateFormatBooleanAttribute, status: *mut UErrorCode) -> i8;
    pub fn udat_getCalendar(fmt: *const *const ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_getContext(fmt: *const *const ::core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getLocaleByType(fmt: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn udat_getNumberFormat(fmt: *const *const ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_getNumberFormatForField(fmt: *const *const ::core::ffi::c_void, field: u16) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_getSymbols(fmt: *const *const ::core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn udat_isLenient(fmt: *const *const ::core::ffi::c_void) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_open(timestyle: UDateFormatStyle, datestyle: UDateFormatStyle, locale: super::Foundation::PSTR, tzid: *const u16, tzidlength: i32, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_parse(format: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64;
    pub fn udat_parseCalendar(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode);
    pub fn udat_set2DigitYearStart(fmt: *mut *mut ::core::ffi::c_void, d: f64, status: *mut UErrorCode);
    pub fn udat_setBooleanAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UDateFormatBooleanAttribute, newvalue: i8, status: *mut UErrorCode);
    pub fn udat_setCalendar(fmt: *mut *mut ::core::ffi::c_void, calendartoset: *const *const ::core::ffi::c_void);
    pub fn udat_setContext(fmt: *mut *mut ::core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode);
    pub fn udat_setLenient(fmt: *mut *mut ::core::ffi::c_void, islenient: i8);
    pub fn udat_setNumberFormat(fmt: *mut *mut ::core::ffi::c_void, numberformattoset: *const *const ::core::ffi::c_void);
    pub fn udat_setSymbols(format: *mut *mut ::core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, value: *mut u16, valuelength: i32, status: *mut UErrorCode);
    pub fn udat_toCalendarDateField(field: UDateFormatField) -> UCalendarDateFields;
    pub fn udat_toPattern(fmt: *const *const ::core::ffi::c_void, localized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn udatpg_addPattern(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, r#override: i8, conflictingpattern: *mut u16, capacity: i32, plength: *mut i32, perrorcode: *mut UErrorCode) -> UDateTimePatternConflict;
    pub fn udatpg_clone(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udatpg_close(dtpg: *mut *mut ::core::ffi::c_void);
    pub fn udatpg_getAppendItemFormat(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getAppendItemName(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getBaseSkeleton(unuseddtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, length: i32, baseskeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getBestPattern(dtpg: *mut *mut ::core::ffi::c_void, skeleton: *const u16, length: i32, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getBestPatternWithOptions(dtpg: *mut *mut ::core::ffi::c_void, skeleton: *const u16, length: i32, options: UDateTimePatternMatchOptions, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getDateTimeFormat(dtpg: *const *const ::core::ffi::c_void, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getDecimal(dtpg: *const *const ::core::ffi::c_void, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getFieldDisplayName(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, width: UDateTimePGDisplayWidth, fieldname: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getPatternForSkeleton(dtpg: *const *const ::core::ffi::c_void, skeleton: *const u16, skeletonlength: i32, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getSkeleton(unuseddtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, length: i32, skeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udatpg_open(locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udatpg_openBaseSkeletons(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn udatpg_openEmpty(perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udatpg_openSkeletons(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn udatpg_replaceFieldTypes(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_replaceFieldTypesWithOptions(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, options: UDateTimePatternMatchOptions, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_setAppendItemFormat(dtpg: *mut *mut ::core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32);
    pub fn udatpg_setAppendItemName(dtpg: *mut *mut ::core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32);
    pub fn udatpg_setDateTimeFormat(dtpg: *const *const ::core::ffi::c_void, dtformat: *const u16, length: i32);
    pub fn udatpg_setDecimal(dtpg: *mut *mut ::core::ffi::c_void, decimal: *const u16, length: i32);
    pub fn udtitvfmt_close(formatter: *mut UDateIntervalFormat);
    pub fn udtitvfmt_closeResult(uresult: *mut UFormattedDateInterval);
    pub fn udtitvfmt_format(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut u16, resultcapacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udtitvfmt_open(locale: super::Foundation::PSTR, skeleton: *const u16, skeletonlength: i32, tzid: *const u16, tzidlength: i32, status: *mut UErrorCode) -> *mut UDateIntervalFormat;
    pub fn udtitvfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedDateInterval;
    pub fn udtitvfmt_resultAsValue(uresult: *const UFormattedDateInterval, ec: *mut UErrorCode) -> *mut UFormattedValue;
    pub fn uenum_close(en: *mut UEnumeration);
    pub fn uenum_count(en: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uenum_next(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn uenum_openCharStringsEnumeration(strings: *const *const i8, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uenum_openUCharStringsEnumeration(strings: *const *const u16, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uenum_reset(en: *mut UEnumeration, status: *mut UErrorCode);
    pub fn uenum_unext(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ufieldpositer_close(fpositer: *mut UFieldPositionIterator);
    pub fn ufieldpositer_next(fpositer: *mut UFieldPositionIterator, beginindex: *mut i32, endindex: *mut i32) -> i32;
    pub fn ufieldpositer_open(status: *mut UErrorCode) -> *mut UFieldPositionIterator;
    pub fn ufmt_close(fmt: *mut *mut ::core::ffi::c_void);
    pub fn ufmt_getArrayItemByIndex(fmt: *mut *mut ::core::ffi::c_void, n: i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn ufmt_getArrayLength(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> i32;
    pub fn ufmt_getDate(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ufmt_getDecNumChars(fmt: *mut *mut ::core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ufmt_getDouble(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    pub fn ufmt_getInt64(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i64;
    pub fn ufmt_getLong(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i32;
    pub fn ufmt_getObject(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    pub fn ufmt_getType(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> UFormattableType;
    pub fn ufmt_getUChars(fmt: *mut *mut ::core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ufmt_isNumeric(fmt: *const *const ::core::ffi::c_void) -> i8;
    pub fn ufmt_open(status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn ufmtval_getString(ufmtval: *const UFormattedValue, plength: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    pub fn ufmtval_nextPosition(ufmtval: *const UFormattedValue, ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ugender_getInstance(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UGenderInfo;
    pub fn ugender_getListGender(genderinfo: *const UGenderInfo, genders: *const UGender, size: i32, status: *mut UErrorCode) -> UGender;
    pub fn uidna_close(idna: *mut UIDNA);
    pub fn uidna_labelToASCII(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToASCII_UTF8(idna: *const UIDNA, label: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_labelToUnicode(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToUnicodeUTF8(idna: *const UIDNA, label: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_nameToASCII(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToASCII_UTF8(idna: *const UIDNA, name: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_nameToUnicode(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToUnicodeUTF8(idna: *const UIDNA, name: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_openUTS46(options: u32, perrorcode: *mut UErrorCode) -> *mut UIDNA;
    pub fn uiter_current32(iter: *mut UCharIterator) -> i32;
    pub fn uiter_getState(iter: *const UCharIterator) -> u32;
    pub fn uiter_next32(iter: *mut UCharIterator) -> i32;
    pub fn uiter_previous32(iter: *mut UCharIterator) -> i32;
    pub fn uiter_setState(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode);
    pub fn uiter_setString(iter: *mut UCharIterator, s: *const u16, length: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF16BE(iter: *mut UCharIterator, s: super::Foundation::PSTR, length: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF8(iter: *mut UCharIterator, s: super::Foundation::PSTR, length: i32);
    pub fn uldn_close(ldn: *mut ULocaleDisplayNames);
    pub fn uldn_getContext(ldn: *const ULocaleDisplayNames, r#type: UDisplayContextType, perrorcode: *mut UErrorCode) -> UDisplayContext;
    pub fn uldn_getDialectHandling(ldn: *const ULocaleDisplayNames) -> UDialectHandling;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_getLocale(ldn: *const ULocaleDisplayNames) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyDisplayName(ldn: *const ULocaleDisplayNames, key: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyValueDisplayName(ldn: *const ULocaleDisplayNames, key: super::Foundation::PSTR, value: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_languageDisplayName(ldn: *const ULocaleDisplayNames, lang: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_localeDisplayName(ldn: *const ULocaleDisplayNames, locale: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_open(locale: super::Foundation::PSTR, dialecthandling: UDialectHandling, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_openForContext(locale: super::Foundation::PSTR, contexts: *mut UDisplayContext, length: i32, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_regionDisplayName(ldn: *const ULocaleDisplayNames, region: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn uldn_scriptCodeDisplayName(ldn: *const ULocaleDisplayNames, scriptcode: UScriptCode, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_scriptDisplayName(ldn: *const ULocaleDisplayNames, script: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_variantDisplayName(ldn: *const ULocaleDisplayNames, variant: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ulistfmt_close(listfmt: *mut UListFormatter);
    pub fn ulistfmt_closeResult(uresult: *mut UFormattedList);
    pub fn ulistfmt_format(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ulistfmt_formatStringsToResult(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, uresult: *mut UFormattedList, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UListFormatter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_openForType(locale: super::Foundation::PSTR, r#type: UListFormatterType, width: UListFormatterWidth, status: *mut UErrorCode) -> *mut UListFormatter;
    pub fn ulistfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedList;
    pub fn ulistfmt_resultAsValue(uresult: *const UFormattedList, ec: *mut UErrorCode) -> *mut UFormattedValue;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguage(result: super::Foundation::PSTR, resultavailable: i32, outresult: *mut UAcceptResult, acceptlist: *const *const i8, acceptlistcount: i32, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguageFromHTTP(result: super::Foundation::PSTR, resultavailable: i32, outresult: *mut UAcceptResult, httpacceptlanguage: super::Foundation::PSTR, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_addLikelySubtags(localeid: super::Foundation::PSTR, maximizedlocaleid: super::Foundation::PSTR, maximizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_canonicalize(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    pub fn uloc_countAvailable() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_forLanguageTag(langtag: super::Foundation::PSTR, localeid: super::Foundation::PSTR, localeidcapacity: i32, parsedlength: *mut i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getAvailable(n: i32) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getBaseName(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCharacterOrientation(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> ULayoutType;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCountry(localeid: super::Foundation::PSTR, country: super::Foundation::PSTR, countrycapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDefault() -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayCountry(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, country: *mut u16, countrycapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeyword(keyword: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeywordValue(locale: super::Foundation::PSTR, keyword: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayLanguage(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, language: *mut u16, languagecapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayName(localeid: super::Foundation::PSTR, inlocaleid: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayScript(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, script: *mut u16, scriptcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayVariant(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, variant: *mut u16, variantcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Country(localeid: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Language(localeid: super::Foundation::PSTR) -> super::Foundation::PSTR;
    pub fn uloc_getISOCountries() -> *mut *mut i8;
    pub fn uloc_getISOLanguages() -> *mut *mut i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getKeywordValue(localeid: super::Foundation::PSTR, keywordname: super::Foundation::PSTR, buffer: super::Foundation::PSTR, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLCID(localeid: super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLanguage(localeid: super::Foundation::PSTR, language: super::Foundation::PSTR, languagecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLineOrientation(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> ULayoutType;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLocaleForLCID(hostid: u32, locale: super::Foundation::PSTR, localecapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getName(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getParent(localeid: super::Foundation::PSTR, parent: super::Foundation::PSTR, parentcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getScript(localeid: super::Foundation::PSTR, script: super::Foundation::PSTR, scriptcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getVariant(localeid: super::Foundation::PSTR, variant: super::Foundation::PSTR, variantcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_isRightToLeft(locale: super::Foundation::PSTR) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_minimizeSubtags(localeid: super::Foundation::PSTR, minimizedlocaleid: super::Foundation::PSTR, minimizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32;
    pub fn uloc_openAvailableByType(r#type: ULocAvailableType, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_openKeywords(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setDefault(localeid: super::Foundation::PSTR, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setKeywordValue(keywordname: super::Foundation::PSTR, keywordvalue: super::Foundation::PSTR, buffer: super::Foundation::PSTR, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLanguageTag(localeid: super::Foundation::PSTR, langtag: super::Foundation::PSTR, langtagcapacity: i32, strict: i8, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyKey(keyword: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyType(keyword: super::Foundation::PSTR, value: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleKey(keyword: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleType(keyword: super::Foundation::PSTR, value: super::Foundation::PSTR) -> super::Foundation::PSTR;
    pub fn ulocdata_close(uld: *mut ULocaleData);
    pub fn ulocdata_getCLDRVersion(versionarray: *mut u8, status: *mut UErrorCode);
    pub fn ulocdata_getDelimiter(uld: *mut ULocaleData, r#type: ULocaleDataDelimiterType, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ulocdata_getExemplarSet(uld: *mut ULocaleData, fillin: *mut USet, options: u32, extype: ULocaleDataExemplarSetType, status: *mut UErrorCode) -> *mut USet;
    pub fn ulocdata_getLocaleDisplayPattern(uld: *mut ULocaleData, pattern: *mut u16, patterncapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ulocdata_getLocaleSeparator(uld: *mut ULocaleData, separator: *mut u16, separatorcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getMeasurementSystem(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> UMeasurementSystem;
    pub fn ulocdata_getNoSubstitute(uld: *mut ULocaleData) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getPaperSize(localeid: super::Foundation::PSTR, height: *mut i32, width: *mut i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_open(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut ULocaleData;
    pub fn ulocdata_setNoSubstitute(uld: *mut ULocaleData, setting: i8);
    pub fn umsg_applyPattern(fmt: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn umsg_autoQuoteApostrophe(pattern: *const u16, patternlength: i32, dest: *mut u16, destcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn umsg_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    pub fn umsg_close(format: *mut *mut ::core::ffi::c_void);
    pub fn umsg_format(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_getLocale(fmt: *const *const ::core::ffi::c_void) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_open(pattern: *const u16, patternlength: i32, locale: super::Foundation::PSTR, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn umsg_parse(fmt: *const *const ::core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_setLocale(fmt: *mut *mut ::core::ffi::c_void, locale: super::Foundation::PSTR);
    pub fn umsg_toPattern(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn umsg_vformat(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32;
    pub fn umsg_vparse(fmt: *const *const ::core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, ap: *mut i8, status: *mut UErrorCode);
    pub fn umutablecptrie_buildImmutable(trie: *mut UMutableCPTrie, r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, perrorcode: *mut UErrorCode) -> *mut UCPTrie;
    pub fn umutablecptrie_clone(other: *const UMutableCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_close(trie: *mut UMutableCPTrie);
    pub fn umutablecptrie_fromUCPMap(map: *const UCPMap, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_fromUCPTrie(trie: *const UCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_get(trie: *const UMutableCPTrie, c: i32) -> u32;
    pub fn umutablecptrie_getRange(trie: *const UMutableCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    pub fn umutablecptrie_open(initialvalue: u32, errorvalue: u32, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_set(trie: *mut UMutableCPTrie, c: i32, value: u32, perrorcode: *mut UErrorCode);
    pub fn umutablecptrie_setRange(trie: *mut UMutableCPTrie, start: i32, end: i32, value: u32, perrorcode: *mut UErrorCode);
    pub fn unorm2_append(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_close(norm2: *mut UNormalizer2);
    pub fn unorm2_composePair(norm2: *const UNormalizer2, a: i32, b: i32) -> i32;
    pub fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: i32) -> u8;
    pub fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unorm2_getInstance(packagename: super::Foundation::PSTR, name: super::Foundation::PSTR, mode: UNormalization2Mode, perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFKCCasefoldInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFKCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFKDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: i32) -> i8;
    pub fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: i32) -> i8;
    pub fn unorm2_isInert(norm2: *const UNormalizer2, c: i32) -> i8;
    pub fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i8;
    pub fn unorm2_normalize(norm2: *const UNormalizer2, src: *const u16, length: i32, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_openFiltered(norm2: *const UNormalizer2, filterset: *const USet, perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> UNormalizationCheckResult;
    pub fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm_compare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unum_applyPattern(format: *mut *mut ::core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn unum_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn unum_close(fmt: *mut *mut ::core::ffi::c_void);
    pub fn unum_countAvailable() -> i32;
    pub fn unum_format(fmt: *const *const ::core::ffi::c_void, number: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_formatDecimal(fmt: *const *const ::core::ffi::c_void, number: super::Foundation::PSTR, length: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatDouble(fmt: *const *const ::core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatDoubleCurrency(fmt: *const *const ::core::ffi::c_void, number: f64, currency: *mut u16, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatDoubleForFields(format: *const *const ::core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    pub fn unum_formatInt64(fmt: *const *const ::core::ffi::c_void, number: i64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatUFormattable(fmt: *const *const ::core::ffi::c_void, number: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_getAttribute(fmt: *const *const ::core::ffi::c_void, attr: UNumberFormatAttribute) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn unum_getContext(fmt: *const *const ::core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext;
    pub fn unum_getDoubleAttribute(fmt: *const *const ::core::ffi::c_void, attr: UNumberFormatAttribute) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getLocaleByType(fmt: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn unum_getSymbol(fmt: *const *const ::core::ffi::c_void, symbol: UNumberFormatSymbol, buffer: *mut u16, size: i32, status: *mut UErrorCode) -> i32;
    pub fn unum_getTextAttribute(fmt: *const *const ::core::ffi::c_void, tag: UNumberFormatTextAttribute, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_open(style: UNumberFormatStyle, pattern: *const u16, patternlength: i32, locale: super::Foundation::PSTR, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn unum_parse(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_parseDecimal(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, outbuf: super::Foundation::PSTR, outbuflength: i32, status: *mut UErrorCode) -> i32;
    pub fn unum_parseDouble(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64;
    pub fn unum_parseDoubleCurrency(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, currency: *mut u16, status: *mut UErrorCode) -> f64;
    pub fn unum_parseInt64(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i64;
    pub fn unum_parseToUFormattable(fmt: *const *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn unum_setAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: i32);
    pub fn unum_setContext(fmt: *mut *mut ::core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode);
    pub fn unum_setDoubleAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: f64);
    pub fn unum_setSymbol(fmt: *mut *mut ::core::ffi::c_void, symbol: UNumberFormatSymbol, value: *const u16, length: i32, status: *mut UErrorCode);
    pub fn unum_setTextAttribute(fmt: *mut *mut ::core::ffi::c_void, tag: UNumberFormatTextAttribute, newvalue: *const u16, newvaluelength: i32, status: *mut UErrorCode);
    pub fn unum_toPattern(fmt: *const *const ::core::ffi::c_void, ispatternlocalized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn unumf_close(uformatter: *mut UNumberFormatter);
    pub fn unumf_closeResult(uresult: *mut UFormattedNumber);
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_formatDecimal(uformatter: *const UNumberFormatter, value: super::Foundation::PSTR, valuelen: i32, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    pub fn unumf_formatDouble(uformatter: *const UNumberFormatter, value: f64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    pub fn unumf_formatInt(uformatter: *const UNumberFormatter, value: i64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocale(skeleton: *const u16, skeletonlen: i32, locale: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UNumberFormatter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocaleWithError(skeleton: *const u16, skeletonlen: i32, locale: super::Foundation::PSTR, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberFormatter;
    pub fn unumf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumber;
    pub fn unumf_resultAsValue(uresult: *const UFormattedNumber, ec: *mut UErrorCode) -> *mut UFormattedValue;
    pub fn unumf_resultGetAllFieldPositions(uresult: *const UFormattedNumber, ufpositer: *mut UFieldPositionIterator, ec: *mut UErrorCode);
    pub fn unumf_resultNextFieldPosition(uresult: *const UFormattedNumber, ufpos: *mut UFieldPosition, ec: *mut UErrorCode) -> i8;
    pub fn unumf_resultToString(uresult: *const UFormattedNumber, buffer: *mut u16, buffercapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn unumsys_close(unumsys: *mut UNumberingSystem);
    pub fn unumsys_getDescription(unumsys: *const UNumberingSystem, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_getName(unumsys: *const UNumberingSystem) -> super::Foundation::PSTR;
    pub fn unumsys_getRadix(unumsys: *const UNumberingSystem) -> i32;
    pub fn unumsys_isAlgorithmic(unumsys: *const UNumberingSystem) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UNumberingSystem;
    pub fn unumsys_openAvailableNames(status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_openByName(name: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UNumberingSystem;
    pub fn uplrules_close(uplrules: *mut UPluralRules);
    pub fn uplrules_getKeywords(uplrules: *const UPluralRules, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UPluralRules;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_openForType(locale: super::Foundation::PSTR, r#type: UPluralType, status: *mut UErrorCode) -> *mut UPluralRules;
    pub fn uplrules_select(uplrules: *const UPluralRules, number: f64, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uplrules_selectFormatted(uplrules: *const UPluralRules, number: *const UFormattedNumber, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_appendReplacement(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_appendReplacementUText(regexp: *mut URegularExpression, replacementtext: *mut UText, dest: *mut UText, status: *mut UErrorCode);
    pub fn uregex_appendTail(regexp: *mut URegularExpression, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_appendTailUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_clone(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut URegularExpression;
    pub fn uregex_close(regexp: *mut URegularExpression);
    pub fn uregex_end(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_end64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64;
    pub fn uregex_find(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    pub fn uregex_find64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    pub fn uregex_findNext(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_flags(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_getFindProgressCallback(regexp: *const URegularExpression, callback: *mut URegexFindProgressCallback, context: *const *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_getMatchCallback(regexp: *const URegularExpression, callback: *mut URegexMatchCallback, context: *const *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_getStackLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_getText(regexp: *mut URegularExpression, textlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn uregex_getTimeLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_getUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_group(regexp: *mut URegularExpression, groupnum: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_groupCount(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_groupNumberFromCName(regexp: *mut URegularExpression, groupname: super::Foundation::PSTR, namelength: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_groupNumberFromName(regexp: *mut URegularExpression, groupname: *const u16, namelength: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_groupUText(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UText, grouplength: *mut i64, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_hasAnchoringBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_hasTransparentBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_hitEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_lookingAt(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    pub fn uregex_lookingAt64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    pub fn uregex_matches(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    pub fn uregex_matches64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    pub fn uregex_open(pattern: *const u16, patternlength: i32, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_openC(pattern: super::Foundation::PSTR, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    pub fn uregex_openUText(pattern: *mut UText, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    pub fn uregex_pattern(regexp: *const URegularExpression, patlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn uregex_patternUText(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_refreshUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode);
    pub fn uregex_regionEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_regionEnd64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64;
    pub fn uregex_regionStart(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_regionStart64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64;
    pub fn uregex_replaceAll(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_replaceAllUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_replaceFirst(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_replaceFirstUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_requireEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_reset(regexp: *mut URegularExpression, index: i32, status: *mut UErrorCode);
    pub fn uregex_reset64(regexp: *mut URegularExpression, index: i64, status: *mut UErrorCode);
    pub fn uregex_setFindProgressCallback(regexp: *mut URegularExpression, callback: URegexFindProgressCallback, context: *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_setMatchCallback(regexp: *mut URegularExpression, callback: URegexMatchCallback, context: *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_setRegion(regexp: *mut URegularExpression, regionstart: i32, regionlimit: i32, status: *mut UErrorCode);
    pub fn uregex_setRegion64(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, status: *mut UErrorCode);
    pub fn uregex_setRegionAndStart(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, startindex: i64, status: *mut UErrorCode);
    pub fn uregex_setStackLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode);
    pub fn uregex_setText(regexp: *mut URegularExpression, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn uregex_setTimeLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode);
    pub fn uregex_setUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode);
    pub fn uregex_split(regexp: *mut URegularExpression, destbuf: *mut u16, destcapacity: i32, requiredcapacity: *mut i32, destfields: *mut *mut u16, destfieldscapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_splitUText(regexp: *mut URegularExpression, destfields: *mut *mut UText, destfieldscapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_start(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_start64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64;
    pub fn uregex_useAnchoringBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode);
    pub fn uregex_useTransparentBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode);
    pub fn uregion_areEqual(uregion: *const URegion, otherregion: *const URegion) -> i8;
    pub fn uregion_contains(uregion: *const URegion, otherregion: *const URegion) -> i8;
    pub fn uregion_getAvailable(r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uregion_getContainedRegions(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uregion_getContainedRegionsOfType(uregion: *const URegion, r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uregion_getContainingRegion(uregion: *const URegion) -> *mut URegion;
    pub fn uregion_getContainingRegionOfType(uregion: *const URegion, r#type: URegionType) -> *mut URegion;
    pub fn uregion_getNumericCode(uregion: *const URegion) -> i32;
    pub fn uregion_getPreferredValues(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionCode(uregion: *const URegion) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionFromCode(regioncode: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut URegion;
    pub fn uregion_getRegionFromNumericCode(code: i32, status: *mut UErrorCode) -> *mut URegion;
    pub fn uregion_getType(uregion: *const URegion) -> URegionType;
    pub fn ureldatefmt_close(reldatefmt: *mut URelativeDateTimeFormatter);
    pub fn ureldatefmt_closeResult(ufrdt: *mut UFormattedRelativeDateTime);
    pub fn ureldatefmt_combineDateAndTime(reldatefmt: *const URelativeDateTimeFormatter, relativedatestring: *const u16, relativedatestringlen: i32, timestring: *const u16, timestringlen: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ureldatefmt_format(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ureldatefmt_formatNumeric(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ureldatefmt_formatNumericToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode);
    pub fn ureldatefmt_formatToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ureldatefmt_open(locale: super::Foundation::PSTR, nftoadopt: *mut *mut ::core::ffi::c_void, width: UDateRelativeDateTimeFormatterStyle, capitalizationcontext: UDisplayContext, status: *mut UErrorCode) -> *mut URelativeDateTimeFormatter;
    pub fn ureldatefmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedRelativeDateTime;
    pub fn ureldatefmt_resultAsValue(ufrdt: *const UFormattedRelativeDateTime, ec: *mut UErrorCode) -> *mut UFormattedValue;
    pub fn ures_close(resourcebundle: *mut UResourceBundle);
    pub fn ures_getBinary(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u8;
    pub fn ures_getByIndex(resourcebundle: *const UResourceBundle, indexr: i32, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getByKey(resourcebundle: *const UResourceBundle, key: super::Foundation::PSTR, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_getInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> i32;
    pub fn ures_getIntVector(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getKey(resourcebundle: *const UResourceBundle) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getLocaleByType(resourcebundle: *const UResourceBundle, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ures_getNextResource(resourcebundle: *mut UResourceBundle, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_getNextString(resourcebundle: *mut UResourceBundle, len: *mut i32, key: *const *const i8, status: *mut UErrorCode) -> *mut u16;
    pub fn ures_getSize(resourcebundle: *const UResourceBundle) -> i32;
    pub fn ures_getString(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ures_getStringByIndex(resourcebundle: *const UResourceBundle, indexs: i32, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getStringByKey(resb: *const UResourceBundle, key: super::Foundation::PSTR, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ures_getType(resourcebundle: *const UResourceBundle) -> UResType;
    pub fn ures_getUInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8String(resb: *const UResourceBundle, dest: super::Foundation::PSTR, length: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByIndex(resb: *const UResourceBundle, stringindex: i32, dest: super::Foundation::PSTR, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByKey(resb: *const UResourceBundle, key: super::Foundation::PSTR, dest: super::Foundation::PSTR, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ures_getVersion(resb: *const UResourceBundle, versioninfo: *mut u8);
    pub fn ures_hasNext(resourcebundle: *const UResourceBundle) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_open(packagename: super::Foundation::PSTR, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openAvailableLocales(packagename: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openDirect(packagename: super::Foundation::PSTR, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openU(packagename: *const u16, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_resetIterator(resourcebundle: *mut UResourceBundle);
    pub fn uscript_breaksBetweenLetters(script: UScriptCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getCode(nameorabbrorlocale: super::Foundation::PSTR, fillin: *mut UScriptCode, capacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getName(scriptcode: UScriptCode) -> super::Foundation::PSTR;
    pub fn uscript_getSampleString(script: UScriptCode, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn uscript_getScript(codepoint: i32, err: *mut UErrorCode) -> UScriptCode;
    pub fn uscript_getScriptExtensions(c: i32, scripts: *mut UScriptCode, capacity: i32, errorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getShortName(scriptcode: UScriptCode) -> super::Foundation::PSTR;
    pub fn uscript_getUsage(script: UScriptCode) -> UScriptUsage;
    pub fn uscript_hasScript(c: i32, sc: UScriptCode) -> i8;
    pub fn uscript_isCased(script: UScriptCode) -> i8;
    pub fn uscript_isRightToLeft(script: UScriptCode) -> i8;
    pub fn usearch_close(searchiter: *mut UStringSearch);
    pub fn usearch_first(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    pub fn usearch_following(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32;
    pub fn usearch_getAttribute(strsrch: *const UStringSearch, attribute: USearchAttribute) -> USearchAttributeValue;
    pub fn usearch_getBreakIterator(strsrch: *const UStringSearch) -> *mut UBreakIterator;
    pub fn usearch_getCollator(strsrch: *const UStringSearch) -> *mut UCollator;
    pub fn usearch_getMatchedLength(strsrch: *const UStringSearch) -> i32;
    pub fn usearch_getMatchedStart(strsrch: *const UStringSearch) -> i32;
    pub fn usearch_getMatchedText(strsrch: *const UStringSearch, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn usearch_getOffset(strsrch: *const UStringSearch) -> i32;
    pub fn usearch_getPattern(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16;
    pub fn usearch_getText(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16;
    pub fn usearch_last(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    pub fn usearch_next(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn usearch_open(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, locale: super::Foundation::PSTR, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch;
    pub fn usearch_openFromCollator(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, collator: *const UCollator, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch;
    pub fn usearch_preceding(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32;
    pub fn usearch_previous(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    pub fn usearch_reset(strsrch: *mut UStringSearch);
    pub fn usearch_setAttribute(strsrch: *mut UStringSearch, attribute: USearchAttribute, value: USearchAttributeValue, status: *mut UErrorCode);
    pub fn usearch_setBreakIterator(strsrch: *mut UStringSearch, breakiter: *mut UBreakIterator, status: *mut UErrorCode);
    pub fn usearch_setCollator(strsrch: *mut UStringSearch, collator: *const UCollator, status: *mut UErrorCode);
    pub fn usearch_setOffset(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode);
    pub fn usearch_setPattern(strsrch: *mut UStringSearch, pattern: *const u16, patternlength: i32, status: *mut UErrorCode);
    pub fn usearch_setText(strsrch: *mut UStringSearch, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn uset_add(set: *mut USet, c: i32);
    pub fn uset_addAll(set: *mut USet, additionalset: *const USet);
    pub fn uset_addAllCodePoints(set: *mut USet, str: *const u16, strlen: i32);
    pub fn uset_addRange(set: *mut USet, start: i32, end: i32);
    pub fn uset_addString(set: *mut USet, str: *const u16, strlen: i32);
    pub fn uset_applyIntPropertyValue(set: *mut USet, prop: UProperty, value: i32, ec: *mut UErrorCode);
    pub fn uset_applyPattern(set: *mut USet, pattern: *const u16, patternlength: i32, options: u32, status: *mut UErrorCode) -> i32;
    pub fn uset_applyPropertyAlias(set: *mut USet, prop: *const u16, proplength: i32, value: *const u16, valuelength: i32, ec: *mut UErrorCode);
    pub fn uset_charAt(set: *const USet, charindex: i32) -> i32;
    pub fn uset_clear(set: *mut USet);
    pub fn uset_clone(set: *const USet) -> *mut USet;
    pub fn uset_cloneAsThawed(set: *const USet) -> *mut USet;
    pub fn uset_close(set: *mut USet);
    pub fn uset_closeOver(set: *mut USet, attributes: i32);
    pub fn uset_compact(set: *mut USet);
    pub fn uset_complement(set: *mut USet);
    pub fn uset_complementAll(set: *mut USet, complement: *const USet);
    pub fn uset_contains(set: *const USet, c: i32) -> i8;
    pub fn uset_containsAll(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_containsAllCodePoints(set: *const USet, str: *const u16, strlen: i32) -> i8;
    pub fn uset_containsNone(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_containsRange(set: *const USet, start: i32, end: i32) -> i8;
    pub fn uset_containsSome(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_containsString(set: *const USet, str: *const u16, strlen: i32) -> i8;
    pub fn uset_equals(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_freeze(set: *mut USet);
    pub fn uset_getItem(set: *const USet, itemindex: i32, start: *mut i32, end: *mut i32, str: *mut u16, strcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn uset_getItemCount(set: *const USet) -> i32;
    pub fn uset_getSerializedRange(set: *const USerializedSet, rangeindex: i32, pstart: *mut i32, pend: *mut i32) -> i8;
    pub fn uset_getSerializedRangeCount(set: *const USerializedSet) -> i32;
    pub fn uset_getSerializedSet(fillset: *mut USerializedSet, src: *const u16, srclength: i32) -> i8;
    pub fn uset_indexOf(set: *const USet, c: i32) -> i32;
    pub fn uset_isEmpty(set: *const USet) -> i8;
    pub fn uset_isFrozen(set: *const USet) -> i8;
    pub fn uset_open(start: i32, end: i32) -> *mut USet;
    pub fn uset_openEmpty() -> *mut USet;
    pub fn uset_openPattern(pattern: *const u16, patternlength: i32, ec: *mut UErrorCode) -> *mut USet;
    pub fn uset_openPatternOptions(pattern: *const u16, patternlength: i32, options: u32, ec: *mut UErrorCode) -> *mut USet;
    pub fn uset_remove(set: *mut USet, c: i32);
    pub fn uset_removeAll(set: *mut USet, removeset: *const USet);
    pub fn uset_removeAllStrings(set: *mut USet);
    pub fn uset_removeRange(set: *mut USet, start: i32, end: i32);
    pub fn uset_removeString(set: *mut USet, str: *const u16, strlen: i32);
    pub fn uset_resemblesPattern(pattern: *const u16, patternlength: i32, pos: i32) -> i8;
    pub fn uset_retain(set: *mut USet, start: i32, end: i32);
    pub fn uset_retainAll(set: *mut USet, retain: *const USet);
    pub fn uset_serialize(set: *const USet, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn uset_serializedContains(set: *const USerializedSet, c: i32) -> i8;
    pub fn uset_set(set: *mut USet, start: i32, end: i32);
    pub fn uset_setSerializedToOne(fillset: *mut USerializedSet, c: i32);
    pub fn uset_size(set: *const USet) -> i32;
    pub fn uset_span(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32;
    pub fn uset_spanBack(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanBackUTF8(set: *const USet, s: super::Foundation::PSTR, length: i32, spancondition: USetSpanCondition) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanUTF8(set: *const USet, s: super::Foundation::PSTR, length: i32, spancondition: USetSpanCondition) -> i32;
    pub fn uset_toPattern(set: *const USet, result: *mut u16, resultcapacity: i32, escapeunprintable: i8, ec: *mut UErrorCode) -> i32;
    pub fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const u16, length1: i32, id2: *const u16, length2: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_areConfusableUTF8(sc: *const USpoofChecker, id1: super::Foundation::PSTR, length1: i32, id2: super::Foundation::PSTR, length2: i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_check(sc: *const USpoofChecker, id: *const u16, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_check2(sc: *const USpoofChecker, id: *const u16, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_check2UTF8(sc: *const USpoofChecker, id: super::Foundation::PSTR, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_checkUTF8(sc: *const USpoofChecker, id: super::Foundation::PSTR, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_close(sc: *mut USpoofChecker);
    pub fn uspoof_closeCheckResult(checkresult: *mut USpoofCheckResult);
    pub fn uspoof_getAllowedChars(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USet;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getAllowedLocales(sc: *mut USpoofChecker, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn uspoof_getCheckResultChecks(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> i32;
    pub fn uspoof_getCheckResultNumerics(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> *mut USet;
    pub fn uspoof_getCheckResultRestrictionLevel(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> URestrictionLevel;
    pub fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode) -> i32;
    pub fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *mut USet;
    pub fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *mut USet;
    pub fn uspoof_getRestrictionLevel(sc: *const USpoofChecker) -> URestrictionLevel;
    pub fn uspoof_getSkeleton(sc: *const USpoofChecker, r#type: u32, id: *const u16, length: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getSkeletonUTF8(sc: *const USpoofChecker, r#type: u32, id: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_openCheckResult(status: *mut UErrorCode) -> *mut USpoofCheckResult;
    pub fn uspoof_openFromSerialized(data: *const ::core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut USpoofChecker;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_openFromSource(confusables: super::Foundation::PSTR, confusableslen: i32, confusableswholescript: super::Foundation::PSTR, confusableswholescriptlen: i32, errtype: *mut i32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_serialize(sc: *mut USpoofChecker, data: *mut ::core::ffi::c_void, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_setAllowedLocales(sc: *mut USpoofChecker, localeslist: super::Foundation::PSTR, status: *mut UErrorCode);
    pub fn uspoof_setChecks(sc: *mut USpoofChecker, checks: i32, status: *mut UErrorCode);
    pub fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker, restrictionlevel: URestrictionLevel);
    pub fn usprep_close(profile: *mut UStringPrepProfile);
    #[cfg(feature = "Win32_Foundation")]
    pub fn usprep_open(path: super::Foundation::PSTR, filename: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UStringPrepProfile;
    pub fn usprep_openByType(r#type: UStringPrepProfileType, status: *mut UErrorCode) -> *mut UStringPrepProfile;
    pub fn usprep_prepare(prep: *const UStringPrepProfile, src: *const u16, srclength: i32, dest: *mut u16, destcapacity: i32, options: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32;
    pub fn utext_char32At(ut: *mut UText, nativeindex: i64) -> i32;
    pub fn utext_clone(dest: *mut UText, src: *const UText, deep: i8, readonly: i8, status: *mut UErrorCode) -> *mut UText;
    pub fn utext_close(ut: *mut UText) -> *mut UText;
    pub fn utext_copy(ut: *mut UText, nativestart: i64, nativelimit: i64, destindex: i64, r#move: i8, status: *mut UErrorCode);
    pub fn utext_current32(ut: *mut UText) -> i32;
    pub fn utext_equals(a: *const UText, b: *const UText) -> i8;
    pub fn utext_extract(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn utext_freeze(ut: *mut UText);
    pub fn utext_getNativeIndex(ut: *const UText) -> i64;
    pub fn utext_getPreviousNativeIndex(ut: *mut UText) -> i64;
    pub fn utext_hasMetaData(ut: *const UText) -> i8;
    pub fn utext_isLengthExpensive(ut: *const UText) -> i8;
    pub fn utext_isWritable(ut: *const UText) -> i8;
    pub fn utext_moveIndex32(ut: *mut UText, delta: i32) -> i8;
    pub fn utext_nativeLength(ut: *mut UText) -> i64;
    pub fn utext_next32(ut: *mut UText) -> i32;
    pub fn utext_next32From(ut: *mut UText, nativeindex: i64) -> i32;
    pub fn utext_openUChars(ut: *mut UText, s: *const u16, length: i64, status: *mut UErrorCode) -> *mut UText;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utext_openUTF8(ut: *mut UText, s: super::Foundation::PSTR, length: i64, status: *mut UErrorCode) -> *mut UText;
    pub fn utext_previous32(ut: *mut UText) -> i32;
    pub fn utext_previous32From(ut: *mut UText, nativeindex: i64) -> i32;
    pub fn utext_replace(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacementlength: i32, status: *mut UErrorCode) -> i32;
    pub fn utext_setNativeIndex(ut: *mut UText, nativeindex: i64);
    pub fn utext_setup(ut: *mut UText, extraspace: i32, status: *mut UErrorCode) -> *mut UText;
    pub fn utf8_appendCharSafeBody(s: *mut u8, i: i32, length: i32, c: i32, piserror: *mut i8) -> i32;
    pub fn utf8_back1SafeBody(s: *const u8, start: i32, i: i32) -> i32;
    pub fn utf8_nextCharSafeBody(s: *const u8, pi: *mut i32, length: i32, c: i32, strict: i8) -> i32;
    pub fn utf8_prevCharSafeBody(s: *const u8, start: i32, pi: *mut i32, c: i32, strict: i8) -> i32;
    pub fn utmscale_fromInt64(othertime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64;
    pub fn utmscale_getTimeScaleValue(timescale: UDateTimeScale, value: UTimeScaleValue, status: *mut UErrorCode) -> i64;
    pub fn utmscale_toInt64(universaltime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_format(outbuf: super::Foundation::PSTR, capacity: i32, indent: i32, fmt: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_functionName(fnnumber: i32) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_getFunctions(context: *const *const ::core::ffi::c_void, e: *mut UTraceEntry, x: *mut UTraceExit, d: *mut UTraceData);
    pub fn utrace_getLevel() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_setFunctions(context: *const ::core::ffi::c_void, e: UTraceEntry, x: UTraceExit, d: UTraceData);
    pub fn utrace_setLevel(tracelevel: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_vformat(outbuf: super::Foundation::PSTR, capacity: i32, indent: i32, fmt: super::Foundation::PSTR, args: *mut i8) -> i32;
    pub fn utrans_clone(trans: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn utrans_close(trans: *mut *mut ::core::ffi::c_void);
    pub fn utrans_countAvailableIDs() -> i32;
    pub fn utrans_getSourceSet(trans: *const *const ::core::ffi::c_void, ignorefilter: i8, fillin: *mut USet, status: *mut UErrorCode) -> *mut USet;
    pub fn utrans_getUnicodeID(trans: *const *const ::core::ffi::c_void, resultlength: *mut i32) -> *mut u16;
    pub fn utrans_openIDs(perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn utrans_openInverse(trans: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn utrans_openU(id: *const u16, idlength: i32, dir: UTransDirection, rules: *const u16, ruleslength: i32, parseerror: *mut UParseError, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn utrans_register(adoptedtrans: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn utrans_setFilter(trans: *mut *mut ::core::ffi::c_void, filterpattern: *const u16, filterpatternlen: i32, status: *mut UErrorCode);
    pub fn utrans_toRules(trans: *const *const ::core::ffi::c_void, escapeunprintable: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn utrans_trans(trans: *const *const ::core::ffi::c_void, rep: *mut *mut ::core::ffi::c_void, repfunc: *const UReplaceableCallbacks, start: i32, limit: *mut i32, status: *mut UErrorCode);
    pub fn utrans_transIncremental(trans: *const *const ::core::ffi::c_void, rep: *mut *mut ::core::ffi::c_void, repfunc: *const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode);
    pub fn utrans_transIncrementalUChars(trans: *const *const ::core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode);
    pub fn utrans_transUChars(trans: *const *const ::core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, start: i32, limit: *mut i32, status: *mut UErrorCode);
    pub fn utrans_unregisterID(id: *const u16, idlength: i32);
}
pub const ALL_SERVICES: u32 = 0u32;
pub const ALL_SERVICE_TYPES: u32 = 0u32;
pub const C1_ALPHA: u32 = 256u32;
pub const C1_BLANK: u32 = 64u32;
pub const C1_CNTRL: u32 = 32u32;
pub const C1_DEFINED: u32 = 512u32;
pub const C1_DIGIT: u32 = 4u32;
pub const C1_LOWER: u32 = 2u32;
pub const C1_PUNCT: u32 = 16u32;
pub const C1_SPACE: u32 = 8u32;
pub const C1_UPPER: u32 = 1u32;
pub const C1_XDIGIT: u32 = 128u32;
pub const C2_ARABICNUMBER: u32 = 6u32;
pub const C2_BLOCKSEPARATOR: u32 = 8u32;
pub const C2_COMMONSEPARATOR: u32 = 7u32;
pub const C2_EUROPENUMBER: u32 = 3u32;
pub const C2_EUROPESEPARATOR: u32 = 4u32;
pub const C2_EUROPETERMINATOR: u32 = 5u32;
pub const C2_LEFTTORIGHT: u32 = 1u32;
pub const C2_NOTAPPLICABLE: u32 = 0u32;
pub const C2_OTHERNEUTRAL: u32 = 11u32;
pub const C2_RIGHTTOLEFT: u32 = 2u32;
pub const C2_SEGMENTSEPARATOR: u32 = 9u32;
pub const C2_WHITESPACE: u32 = 10u32;
pub const C3_ALPHA: u32 = 32768u32;
pub const C3_DIACRITIC: u32 = 2u32;
pub const C3_FULLWIDTH: u32 = 128u32;
pub const C3_HALFWIDTH: u32 = 64u32;
pub const C3_HIGHSURROGATE: u32 = 2048u32;
pub const C3_HIRAGANA: u32 = 32u32;
pub const C3_IDEOGRAPH: u32 = 256u32;
pub const C3_KASHIDA: u32 = 512u32;
pub const C3_KATAKANA: u32 = 16u32;
pub const C3_LEXICAL: u32 = 1024u32;
pub const C3_LOWSURROGATE: u32 = 4096u32;
pub const C3_NONSPACING: u32 = 1u32;
pub const C3_NOTAPPLICABLE: u32 = 0u32;
pub const C3_SYMBOL: u32 = 8u32;
pub const C3_VOWELMARK: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type CALINFO_ENUMPROCA = unsafe extern "system" fn(param0: super::Foundation::PSTR) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CALINFO_ENUMPROCEXA = unsafe extern "system" fn(param0: super::Foundation::PSTR, param1: u32) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CALINFO_ENUMPROCEXEX = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: u32, param2: super::Foundation::PWSTR, param3: super::Foundation::LPARAM) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CALINFO_ENUMPROCEXW = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: u32) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CALINFO_ENUMPROCW = unsafe extern "system" fn(param0: super::Foundation::PWSTR) -> super::Foundation::BOOL;
pub const CAL_GREGORIAN: u32 = 1u32;
pub const CAL_GREGORIAN_ARABIC: u32 = 10u32;
pub const CAL_GREGORIAN_ME_FRENCH: u32 = 9u32;
pub const CAL_GREGORIAN_US: u32 = 2u32;
pub const CAL_GREGORIAN_XLIT_ENGLISH: u32 = 11u32;
pub const CAL_GREGORIAN_XLIT_FRENCH: u32 = 12u32;
pub const CAL_HEBREW: u32 = 8u32;
pub const CAL_HIJRI: u32 = 6u32;
pub const CAL_ICALINTVALUE: u32 = 1u32;
pub const CAL_ITWODIGITYEARMAX: u32 = 48u32;
pub const CAL_IYEAROFFSETRANGE: u32 = 3u32;
pub const CAL_JAPAN: u32 = 3u32;
pub const CAL_KOREA: u32 = 5u32;
pub const CAL_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const CAL_PERSIAN: u32 = 22u32;
pub const CAL_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const CAL_RETURN_NUMBER: u32 = 536870912u32;
pub const CAL_SABBREVDAYNAME1: u32 = 14u32;
pub const CAL_SABBREVDAYNAME2: u32 = 15u32;
pub const CAL_SABBREVDAYNAME3: u32 = 16u32;
pub const CAL_SABBREVDAYNAME4: u32 = 17u32;
pub const CAL_SABBREVDAYNAME5: u32 = 18u32;
pub const CAL_SABBREVDAYNAME6: u32 = 19u32;
pub const CAL_SABBREVDAYNAME7: u32 = 20u32;
pub const CAL_SABBREVERASTRING: u32 = 57u32;
pub const CAL_SABBREVMONTHNAME1: u32 = 34u32;
pub const CAL_SABBREVMONTHNAME10: u32 = 43u32;
pub const CAL_SABBREVMONTHNAME11: u32 = 44u32;
pub const CAL_SABBREVMONTHNAME12: u32 = 45u32;
pub const CAL_SABBREVMONTHNAME13: u32 = 46u32;
pub const CAL_SABBREVMONTHNAME2: u32 = 35u32;
pub const CAL_SABBREVMONTHNAME3: u32 = 36u32;
pub const CAL_SABBREVMONTHNAME4: u32 = 37u32;
pub const CAL_SABBREVMONTHNAME5: u32 = 38u32;
pub const CAL_SABBREVMONTHNAME6: u32 = 39u32;
pub const CAL_SABBREVMONTHNAME7: u32 = 40u32;
pub const CAL_SABBREVMONTHNAME8: u32 = 41u32;
pub const CAL_SABBREVMONTHNAME9: u32 = 42u32;
pub const CAL_SCALNAME: u32 = 2u32;
pub const CAL_SDAYNAME1: u32 = 7u32;
pub const CAL_SDAYNAME2: u32 = 8u32;
pub const CAL_SDAYNAME3: u32 = 9u32;
pub const CAL_SDAYNAME4: u32 = 10u32;
pub const CAL_SDAYNAME5: u32 = 11u32;
pub const CAL_SDAYNAME6: u32 = 12u32;
pub const CAL_SDAYNAME7: u32 = 13u32;
pub const CAL_SENGLISHABBREVERANAME: u32 = 60u32;
pub const CAL_SENGLISHERANAME: u32 = 59u32;
pub const CAL_SERASTRING: u32 = 4u32;
pub const CAL_SJAPANESEERAFIRSTYEAR: u32 = 61u32;
pub const CAL_SLONGDATE: u32 = 6u32;
pub const CAL_SMONTHDAY: u32 = 56u32;
pub const CAL_SMONTHNAME1: u32 = 21u32;
pub const CAL_SMONTHNAME10: u32 = 30u32;
pub const CAL_SMONTHNAME11: u32 = 31u32;
pub const CAL_SMONTHNAME12: u32 = 32u32;
pub const CAL_SMONTHNAME13: u32 = 33u32;
pub const CAL_SMONTHNAME2: u32 = 22u32;
pub const CAL_SMONTHNAME3: u32 = 23u32;
pub const CAL_SMONTHNAME4: u32 = 24u32;
pub const CAL_SMONTHNAME5: u32 = 25u32;
pub const CAL_SMONTHNAME6: u32 = 26u32;
pub const CAL_SMONTHNAME7: u32 = 27u32;
pub const CAL_SMONTHNAME8: u32 = 28u32;
pub const CAL_SMONTHNAME9: u32 = 29u32;
pub const CAL_SRELATIVELONGDATE: u32 = 58u32;
pub const CAL_SSHORTDATE: u32 = 5u32;
pub const CAL_SSHORTESTDAYNAME1: u32 = 49u32;
pub const CAL_SSHORTESTDAYNAME2: u32 = 50u32;
pub const CAL_SSHORTESTDAYNAME3: u32 = 51u32;
pub const CAL_SSHORTESTDAYNAME4: u32 = 52u32;
pub const CAL_SSHORTESTDAYNAME5: u32 = 53u32;
pub const CAL_SSHORTESTDAYNAME6: u32 = 54u32;
pub const CAL_SSHORTESTDAYNAME7: u32 = 55u32;
pub const CAL_SYEARMONTH: u32 = 47u32;
pub const CAL_TAIWAN: u32 = 4u32;
pub const CAL_THAI: u32 = 7u32;
pub const CAL_UMALQURA: u32 = 23u32;
pub const CAL_USE_CP_ACP: u32 = 1073741824u32;
pub const CANITER_SKIP_ZEROES: u32 = 1u32;
#[repr(C)]
pub struct CHARSETINFO(i32);
#[repr(C)]
pub struct CMLangConvertCharset(i32);
#[repr(C)]
pub struct CMLangString(i32);
#[repr(C)]
pub struct CMultiLanguage(i32);
#[cfg(feature = "Win32_Foundation")]
pub type CODEPAGE_ENUMPROCA = unsafe extern "system" fn(param0: super::Foundation::PSTR) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type CODEPAGE_ENUMPROCW = unsafe extern "system" fn(param0: super::Foundation::PWSTR) -> super::Foundation::BOOL;
#[repr(transparent)]
pub struct COMPARE_STRING_FLAGS(pub u32);
pub const LINGUISTIC_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(16u32);
pub const LINGUISTIC_IGNOREDIACRITIC: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(32u32);
pub const NORM_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(1u32);
pub const NORM_IGNOREKANATYPE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(65536u32);
pub const NORM_IGNORENONSPACE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(2u32);
pub const NORM_IGNORESYMBOLS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4u32);
pub const NORM_IGNOREWIDTH: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(131072u32);
pub const NORM_LINGUISTIC_CASING: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(134217728u32);
pub const SORT_DIGITSASNUMBERS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(8u32);
pub const SORT_STRINGSORT: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4096u32);
#[repr(transparent)]
pub struct CORRECTIVE_ACTION(pub i32);
pub const CORRECTIVE_ACTION_NONE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(0i32);
pub const CORRECTIVE_ACTION_GET_SUGGESTIONS: CORRECTIVE_ACTION = CORRECTIVE_ACTION(1i32);
pub const CORRECTIVE_ACTION_REPLACE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(2i32);
pub const CORRECTIVE_ACTION_DELETE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(3i32);
#[repr(C)]
pub struct CPINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CPINFOEXA(i32);
#[repr(C)]
pub struct CPINFOEXW(i32);
pub const CPIOD_FORCE_PROMPT: i32 = -2147483648i32;
pub const CPIOD_PEEK: i32 = 1073741824i32;
pub const CP_ACP: u32 = 0u32;
pub const CP_MACCP: u32 = 2u32;
pub const CP_OEMCP: u32 = 1u32;
pub const CP_SYMBOL: u32 = 42u32;
pub const CP_THREAD_ACP: u32 = 3u32;
pub const CP_UTF7: u32 = 65000u32;
pub const CP_UTF8: u32 = 65001u32;
pub const CSTR_EQUAL: u32 = 2u32;
pub const CSTR_GREATER_THAN: u32 = 3u32;
pub const CSTR_LESS_THAN: u32 = 1u32;
pub const CTRY_ALBANIA: u32 = 355u32;
pub const CTRY_ALGERIA: u32 = 213u32;
pub const CTRY_ARGENTINA: u32 = 54u32;
pub const CTRY_ARMENIA: u32 = 374u32;
pub const CTRY_AUSTRALIA: u32 = 61u32;
pub const CTRY_AUSTRIA: u32 = 43u32;
pub const CTRY_AZERBAIJAN: u32 = 994u32;
pub const CTRY_BAHRAIN: u32 = 973u32;
pub const CTRY_BELARUS: u32 = 375u32;
pub const CTRY_BELGIUM: u32 = 32u32;
pub const CTRY_BELIZE: u32 = 501u32;
pub const CTRY_BOLIVIA: u32 = 591u32;
pub const CTRY_BRAZIL: u32 = 55u32;
pub const CTRY_BRUNEI_DARUSSALAM: u32 = 673u32;
pub const CTRY_BULGARIA: u32 = 359u32;
pub const CTRY_CANADA: u32 = 2u32;
pub const CTRY_CARIBBEAN: u32 = 1u32;
pub const CTRY_CHILE: u32 = 56u32;
pub const CTRY_COLOMBIA: u32 = 57u32;
pub const CTRY_COSTA_RICA: u32 = 506u32;
pub const CTRY_CROATIA: u32 = 385u32;
pub const CTRY_CZECH: u32 = 420u32;
pub const CTRY_DEFAULT: u32 = 0u32;
pub const CTRY_DENMARK: u32 = 45u32;
pub const CTRY_DOMINICAN_REPUBLIC: u32 = 1u32;
pub const CTRY_ECUADOR: u32 = 593u32;
pub const CTRY_EGYPT: u32 = 20u32;
pub const CTRY_EL_SALVADOR: u32 = 503u32;
pub const CTRY_ESTONIA: u32 = 372u32;
pub const CTRY_FAEROE_ISLANDS: u32 = 298u32;
pub const CTRY_FINLAND: u32 = 358u32;
pub const CTRY_FRANCE: u32 = 33u32;
pub const CTRY_GEORGIA: u32 = 995u32;
pub const CTRY_GERMANY: u32 = 49u32;
pub const CTRY_GREECE: u32 = 30u32;
pub const CTRY_GUATEMALA: u32 = 502u32;
pub const CTRY_HONDURAS: u32 = 504u32;
pub const CTRY_HONG_KONG: u32 = 852u32;
pub const CTRY_HUNGARY: u32 = 36u32;
pub const CTRY_ICELAND: u32 = 354u32;
pub const CTRY_INDIA: u32 = 91u32;
pub const CTRY_INDONESIA: u32 = 62u32;
pub const CTRY_IRAN: u32 = 981u32;
pub const CTRY_IRAQ: u32 = 964u32;
pub const CTRY_IRELAND: u32 = 353u32;
pub const CTRY_ISRAEL: u32 = 972u32;
pub const CTRY_ITALY: u32 = 39u32;
pub const CTRY_JAMAICA: u32 = 1u32;
pub const CTRY_JAPAN: u32 = 81u32;
pub const CTRY_JORDAN: u32 = 962u32;
pub const CTRY_KAZAKSTAN: u32 = 7u32;
pub const CTRY_KENYA: u32 = 254u32;
pub const CTRY_KUWAIT: u32 = 965u32;
pub const CTRY_KYRGYZSTAN: u32 = 996u32;
pub const CTRY_LATVIA: u32 = 371u32;
pub const CTRY_LEBANON: u32 = 961u32;
pub const CTRY_LIBYA: u32 = 218u32;
pub const CTRY_LIECHTENSTEIN: u32 = 41u32;
pub const CTRY_LITHUANIA: u32 = 370u32;
pub const CTRY_LUXEMBOURG: u32 = 352u32;
pub const CTRY_MACAU: u32 = 853u32;
pub const CTRY_MACEDONIA: u32 = 389u32;
pub const CTRY_MALAYSIA: u32 = 60u32;
pub const CTRY_MALDIVES: u32 = 960u32;
pub const CTRY_MEXICO: u32 = 52u32;
pub const CTRY_MONACO: u32 = 33u32;
pub const CTRY_MONGOLIA: u32 = 976u32;
pub const CTRY_MOROCCO: u32 = 212u32;
pub const CTRY_NETHERLANDS: u32 = 31u32;
pub const CTRY_NEW_ZEALAND: u32 = 64u32;
pub const CTRY_NICARAGUA: u32 = 505u32;
pub const CTRY_NORWAY: u32 = 47u32;
pub const CTRY_OMAN: u32 = 968u32;
pub const CTRY_PAKISTAN: u32 = 92u32;
pub const CTRY_PANAMA: u32 = 507u32;
pub const CTRY_PARAGUAY: u32 = 595u32;
pub const CTRY_PERU: u32 = 51u32;
pub const CTRY_PHILIPPINES: u32 = 63u32;
pub const CTRY_POLAND: u32 = 48u32;
pub const CTRY_PORTUGAL: u32 = 351u32;
pub const CTRY_PRCHINA: u32 = 86u32;
pub const CTRY_PUERTO_RICO: u32 = 1u32;
pub const CTRY_QATAR: u32 = 974u32;
pub const CTRY_ROMANIA: u32 = 40u32;
pub const CTRY_RUSSIA: u32 = 7u32;
pub const CTRY_SAUDI_ARABIA: u32 = 966u32;
pub const CTRY_SERBIA: u32 = 381u32;
pub const CTRY_SINGAPORE: u32 = 65u32;
pub const CTRY_SLOVAK: u32 = 421u32;
pub const CTRY_SLOVENIA: u32 = 386u32;
pub const CTRY_SOUTH_AFRICA: u32 = 27u32;
pub const CTRY_SOUTH_KOREA: u32 = 82u32;
pub const CTRY_SPAIN: u32 = 34u32;
pub const CTRY_SWEDEN: u32 = 46u32;
pub const CTRY_SWITZERLAND: u32 = 41u32;
pub const CTRY_SYRIA: u32 = 963u32;
pub const CTRY_TAIWAN: u32 = 886u32;
pub const CTRY_TATARSTAN: u32 = 7u32;
pub const CTRY_THAILAND: u32 = 66u32;
pub const CTRY_TRINIDAD_Y_TOBAGO: u32 = 1u32;
pub const CTRY_TUNISIA: u32 = 216u32;
pub const CTRY_TURKEY: u32 = 90u32;
pub const CTRY_UAE: u32 = 971u32;
pub const CTRY_UKRAINE: u32 = 380u32;
pub const CTRY_UNITED_KINGDOM: u32 = 44u32;
pub const CTRY_UNITED_STATES: u32 = 1u32;
pub const CTRY_URUGUAY: u32 = 598u32;
pub const CTRY_UZBEKISTAN: u32 = 7u32;
pub const CTRY_VENEZUELA: u32 = 58u32;
pub const CTRY_VIET_NAM: u32 = 84u32;
pub const CTRY_YEMEN: u32 = 967u32;
pub const CTRY_ZIMBABWE: u32 = 263u32;
pub const CT_CTYPE1: u32 = 1u32;
pub const CT_CTYPE2: u32 = 2u32;
pub const CT_CTYPE3: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CURRENCYFMTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CURRENCYFMTW(i32);
#[cfg(feature = "Win32_Foundation")]
pub type DATEFMT_ENUMPROCA = unsafe extern "system" fn(param0: super::Foundation::PSTR) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DATEFMT_ENUMPROCEXA = unsafe extern "system" fn(param0: super::Foundation::PSTR, param1: u32) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DATEFMT_ENUMPROCEXEX = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: u32, param2: super::Foundation::LPARAM) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DATEFMT_ENUMPROCEXW = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: u32) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DATEFMT_ENUMPROCW = unsafe extern "system" fn(param0: super::Foundation::PWSTR) -> super::Foundation::BOOL;
#[repr(C)]
pub struct DetectEncodingInfo(i32);
pub const ELS_GUID_LANGUAGE_DETECTION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3481141425, data2: 37019, data3: 19861, data4: [168, 244, 97, 31, 124, 55, 119, 2] };
pub const ELS_GUID_SCRIPT_DETECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 761574457,
    data2: 27823,
    data3: 20331,
    data4: [182, 136, 229, 208, 244, 250, 167, 215],
};
pub const ELS_GUID_TRANSLITERATION_BENGALI_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4108310565,
    data2: 37284,
    data3: 18591,
    data4: [133, 94, 154, 217, 190, 229, 87, 39],
};
pub const ELS_GUID_TRANSLITERATION_CYRILLIC_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1037118104, data2: 23293, data3: 18691, data4: [161, 63, 225, 126, 108, 11, 254, 1] };
pub const ELS_GUID_TRANSLITERATION_DEVANAGARI_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3299138814, data2: 9825, data3: 19714, data4: [152, 53, 244, 129, 135, 16, 152, 3] };
pub const ELS_GUID_TRANSLITERATION_HANGUL_DECOMPOSITION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1268950817,
    data2: 58429,
    data3: 16823,
    data4: [179, 48, 83, 106, 225, 228, 136, 99],
};
pub const ELS_GUID_TRANSLITERATION_HANS_TO_HANT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1017957832,
    data2: 21904,
    data3: 17116,
    data4: [154, 123, 181, 166, 181, 179, 182, 59],
};
pub const ELS_GUID_TRANSLITERATION_HANT_TO_HANS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2745709371, data2: 62716, data3: 17142, data4: [160, 196, 4, 98, 254, 115, 23, 203] };
pub const ELS_GUID_TRANSLITERATION_MALAYALAM_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3636036529, data2: 63679, data3: 18987, data4: [188, 213, 91, 94, 162, 6, 19, 225] };
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct ENUMTEXTMETRICA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct ENUMTEXTMETRICW(i32);
pub const ENUM_ALL_CALENDARS: u32 = 4294967295u32;
#[repr(transparent)]
pub struct ENUM_DATE_FORMATS_FLAGS(pub u32);
pub const DATE_SHORTDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(1u32);
pub const DATE_LONGDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(2u32);
pub const DATE_YEARMONTH: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(8u32);
pub const DATE_MONTHDAY: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(128u32);
pub const DATE_AUTOLAYOUT: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(64u32);
pub const DATE_LTRREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(16u32);
pub const DATE_RTLREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(32u32);
pub const DATE_USE_ALT_CALENDAR: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(4u32);
#[repr(transparent)]
pub struct ENUM_SYSTEM_CODE_PAGES_FLAGS(pub u32);
pub const CP_INSTALLED: ENUM_SYSTEM_CODE_PAGES_FLAGS = ENUM_SYSTEM_CODE_PAGES_FLAGS(1u32);
pub const CP_SUPPORTED: ENUM_SYSTEM_CODE_PAGES_FLAGS = ENUM_SYSTEM_CODE_PAGES_FLAGS(2u32);
#[repr(transparent)]
pub struct ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(pub u32);
pub const LGRPID_INSTALLED: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS = ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(1u32);
pub const LGRPID_SUPPORTED: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS = ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(2u32);
#[repr(C)]
pub struct FILEMUIINFO(i32);
pub const FIND_ENDSWITH: u32 = 2097152u32;
pub const FIND_FROMEND: u32 = 8388608u32;
pub const FIND_FROMSTART: u32 = 4194304u32;
pub const FIND_STARTSWITH: u32 = 1048576u32;
#[repr(transparent)]
pub struct FOLD_STRING_MAP_FLAGS(pub u32);
pub const MAP_COMPOSITE: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(64u32);
pub const MAP_EXPAND_LIGATURES: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(8192u32);
pub const MAP_FOLDCZONE: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(16u32);
pub const MAP_FOLDDIGITS: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(128u32);
pub const MAP_PRECOMPOSED: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(32u32);
#[repr(C)]
pub struct FONTSIGNATURE(i32);
pub const GEOID_NOT_AVAILABLE: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
pub type GEO_ENUMNAMEPROC = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: super::Foundation::LPARAM) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type GEO_ENUMPROC = unsafe extern "system" fn(param0: i32) -> super::Foundation::BOOL;
#[repr(C)]
pub struct GOFFSET(i32);
pub const GSS_ALLOW_INHERITED_COMMON: u32 = 1u32;
pub const HIGHLEVEL_SERVICE_TYPES: u32 = 1u32;
pub const HIGH_SURROGATE_END: u32 = 56319u32;
pub const HIGH_SURROGATE_START: u32 = 55296u32;
#[repr(C)]
pub struct HIMC(i32);
#[repr(C)]
pub struct HIMCC(i32);
#[repr(C)]
pub struct HSAVEDUILANGUAGES(i32);
#[repr(transparent)]
pub struct IComprehensiveSpellCheckProvider(pub *mut ::core::ffi::c_void);
pub const IDN_ALLOW_UNASSIGNED: u32 = 1u32;
pub const IDN_EMAIL_ADDRESS: u32 = 4u32;
pub const IDN_RAW_PUNYCODE: u32 = 8u32;
pub const IDN_USE_STD3_ASCII_RULES: u32 = 2u32;
#[repr(transparent)]
pub struct IEnumCodePage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRfc1766(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumScript(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSpellingError(pub *mut ::core::ffi::c_void);
pub const IME_CMODE_ALPHANUMERIC: u32 = 0u32;
pub const IME_CMODE_CHARCODE: u32 = 32u32;
pub const IME_CMODE_CHINESE: u32 = 1u32;
pub const IME_CMODE_FULLSHAPE: u32 = 8u32;
pub const IME_CMODE_HANGUL: u32 = 1u32;
pub const IME_CMODE_HANJACONVERT: u32 = 64u32;
pub const IME_CMODE_JAPANESE: u32 = 1u32;
pub const IME_CMODE_KATAKANA: u32 = 2u32;
pub const IME_CMODE_LANGUAGE: u32 = 3u32;
pub const IME_CMODE_NATIVE: u32 = 1u32;
pub const IME_CMODE_NATIVESYMBOL: u32 = 128u32;
pub const IME_CMODE_ROMAN: u32 = 16u32;
#[repr(transparent)]
pub struct IMLangCodePages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangConvertCharset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangFontLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangFontLink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangLineBreakConsole(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringAStr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringBufA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringBufW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringWStr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiLanguage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiLanguage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOptionDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IS_TEXT_UNICODE_RESULT(pub u32);
pub const IS_TEXT_UNICODE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(1u32);
pub const IS_TEXT_UNICODE_REVERSE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(16u32);
pub const IS_TEXT_UNICODE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(2u32);
pub const IS_TEXT_UNICODE_REVERSE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(32u32);
pub const IS_TEXT_UNICODE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4u32);
pub const IS_TEXT_UNICODE_REVERSE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(64u32);
pub const IS_TEXT_UNICODE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(8u32);
pub const IS_TEXT_UNICODE_REVERSE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(128u32);
pub const IS_TEXT_UNICODE_ILLEGAL_CHARS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(256u32);
pub const IS_TEXT_UNICODE_ODD_LENGTH: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(512u32);
pub const IS_TEXT_UNICODE_NULL_BYTES: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4096u32);
pub const IS_TEXT_UNICODE_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(15u32);
pub const IS_TEXT_UNICODE_REVERSE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(240u32);
pub const IS_TEXT_UNICODE_NOT_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(3840u32);
pub const IS_TEXT_UNICODE_NOT_ASCII_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(61440u32);
#[repr(transparent)]
pub struct IS_VALID_LOCALE_FLAGS(pub u32);
pub const LCID_INSTALLED: IS_VALID_LOCALE_FLAGS = IS_VALID_LOCALE_FLAGS(1u32);
pub const LCID_SUPPORTED: IS_VALID_LOCALE_FLAGS = IS_VALID_LOCALE_FLAGS(2u32);
#[repr(transparent)]
pub struct ISpellCheckProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellCheckProviderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellChecker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellChecker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellCheckerChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellCheckerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellingError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDictionariesRegistrar(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type LANGGROUPLOCALE_ENUMPROCA = unsafe extern "system" fn(param0: u32, param1: u32, param2: super::Foundation::PSTR, param3: isize) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LANGGROUPLOCALE_ENUMPROCW = unsafe extern "system" fn(param0: u32, param1: u32, param2: super::Foundation::PWSTR, param3: isize) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LANGUAGEGROUP_ENUMPROCA = unsafe extern "system" fn(param0: u32, param1: super::Foundation::PSTR, param2: super::Foundation::PSTR, param3: u32, param4: isize) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LANGUAGEGROUP_ENUMPROCW = unsafe extern "system" fn(param0: u32, param1: super::Foundation::PWSTR, param2: super::Foundation::PWSTR, param3: u32, param4: isize) -> super::Foundation::BOOL;
pub const LCID_ALTERNATE_SORTS: u32 = 4u32;
pub const LCMAP_BYTEREV: u32 = 2048u32;
pub const LCMAP_FULLWIDTH: u32 = 8388608u32;
pub const LCMAP_HALFWIDTH: u32 = 4194304u32;
pub const LCMAP_HASH: u32 = 262144u32;
pub const LCMAP_HIRAGANA: u32 = 1048576u32;
pub const LCMAP_KATAKANA: u32 = 2097152u32;
pub const LCMAP_LINGUISTIC_CASING: u32 = 16777216u32;
pub const LCMAP_LOWERCASE: u32 = 256u32;
pub const LCMAP_SIMPLIFIED_CHINESE: u32 = 33554432u32;
pub const LCMAP_SORTHANDLE: u32 = 536870912u32;
pub const LCMAP_SORTKEY: u32 = 1024u32;
pub const LCMAP_TITLECASE: u32 = 768u32;
pub const LCMAP_TRADITIONAL_CHINESE: u32 = 67108864u32;
pub const LCMAP_UPPERCASE: u32 = 512u32;
pub const LGRPID_ARABIC: u32 = 13u32;
pub const LGRPID_ARMENIAN: u32 = 17u32;
pub const LGRPID_BALTIC: u32 = 3u32;
pub const LGRPID_CENTRAL_EUROPE: u32 = 2u32;
pub const LGRPID_CYRILLIC: u32 = 5u32;
pub const LGRPID_GEORGIAN: u32 = 16u32;
pub const LGRPID_GREEK: u32 = 4u32;
pub const LGRPID_HEBREW: u32 = 12u32;
pub const LGRPID_INDIC: u32 = 15u32;
pub const LGRPID_JAPANESE: u32 = 7u32;
pub const LGRPID_KOREAN: u32 = 8u32;
pub const LGRPID_SIMPLIFIED_CHINESE: u32 = 10u32;
pub const LGRPID_THAI: u32 = 11u32;
pub const LGRPID_TRADITIONAL_CHINESE: u32 = 9u32;
pub const LGRPID_TURKIC: u32 = 6u32;
pub const LGRPID_TURKISH: u32 = 6u32;
pub const LGRPID_VIETNAMESE: u32 = 14u32;
pub const LGRPID_WESTERN_EUROPE: u32 = 1u32;
#[repr(C)]
pub struct LOCALESIGNATURE(i32);
pub const LOCALE_ALL: u32 = 0u32;
pub const LOCALE_ALLOW_NEUTRAL_NAMES: u32 = 134217728u32;
pub const LOCALE_ALTERNATE_SORTS: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type LOCALE_ENUMPROCA = unsafe extern "system" fn(param0: super::Foundation::PSTR) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LOCALE_ENUMPROCEX = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: u32, param2: super::Foundation::LPARAM) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LOCALE_ENUMPROCW = unsafe extern "system" fn(param0: super::Foundation::PWSTR) -> super::Foundation::BOOL;
pub const LOCALE_FONTSIGNATURE: u32 = 88u32;
pub const LOCALE_ICALENDARTYPE: u32 = 4105u32;
pub const LOCALE_ICENTURY: u32 = 36u32;
pub const LOCALE_ICONSTRUCTEDLOCALE: u32 = 125u32;
pub const LOCALE_ICOUNTRY: u32 = 5u32;
pub const LOCALE_ICURRDIGITS: u32 = 25u32;
pub const LOCALE_ICURRENCY: u32 = 27u32;
pub const LOCALE_IDATE: u32 = 33u32;
pub const LOCALE_IDAYLZERO: u32 = 38u32;
pub const LOCALE_IDEFAULTANSICODEPAGE: u32 = 4100u32;
pub const LOCALE_IDEFAULTCODEPAGE: u32 = 11u32;
pub const LOCALE_IDEFAULTCOUNTRY: u32 = 10u32;
pub const LOCALE_IDEFAULTEBCDICCODEPAGE: u32 = 4114u32;
pub const LOCALE_IDEFAULTLANGUAGE: u32 = 9u32;
pub const LOCALE_IDEFAULTMACCODEPAGE: u32 = 4113u32;
pub const LOCALE_IDIALINGCODE: u32 = 5u32;
pub const LOCALE_IDIGITS: u32 = 17u32;
pub const LOCALE_IDIGITSUBSTITUTION: u32 = 4116u32;
pub const LOCALE_IFIRSTDAYOFWEEK: u32 = 4108u32;
pub const LOCALE_IFIRSTWEEKOFYEAR: u32 = 4109u32;
pub const LOCALE_IGEOID: u32 = 91u32;
pub const LOCALE_IINTLCURRDIGITS: u32 = 26u32;
pub const LOCALE_ILANGUAGE: u32 = 1u32;
pub const LOCALE_ILDATE: u32 = 34u32;
pub const LOCALE_ILZERO: u32 = 18u32;
pub const LOCALE_IMEASURE: u32 = 13u32;
pub const LOCALE_IMONLZERO: u32 = 39u32;
pub const LOCALE_INEGATIVEPERCENT: u32 = 116u32;
pub const LOCALE_INEGCURR: u32 = 28u32;
pub const LOCALE_INEGNUMBER: u32 = 4112u32;
pub const LOCALE_INEGSEPBYSPACE: u32 = 87u32;
pub const LOCALE_INEGSIGNPOSN: u32 = 83u32;
pub const LOCALE_INEGSYMPRECEDES: u32 = 86u32;
pub const LOCALE_INEUTRAL: u32 = 113u32;
pub const LOCALE_IOPTIONALCALENDAR: u32 = 4107u32;
pub const LOCALE_IPAPERSIZE: u32 = 4106u32;
pub const LOCALE_IPOSITIVEPERCENT: u32 = 117u32;
pub const LOCALE_IPOSSEPBYSPACE: u32 = 85u32;
pub const LOCALE_IPOSSIGNPOSN: u32 = 82u32;
pub const LOCALE_IPOSSYMPRECEDES: u32 = 84u32;
pub const LOCALE_IREADINGLAYOUT: u32 = 112u32;
pub const LOCALE_ITIME: u32 = 35u32;
pub const LOCALE_ITIMEMARKPOSN: u32 = 4101u32;
pub const LOCALE_ITLZERO: u32 = 37u32;
pub const LOCALE_IUSEUTF8LEGACYACP: u32 = 1638u32;
pub const LOCALE_IUSEUTF8LEGACYOEMCP: u32 = 2457u32;
pub const LOCALE_NEUTRALDATA: u32 = 16u32;
pub const LOCALE_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const LOCALE_REPLACEMENT: u32 = 8u32;
pub const LOCALE_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const LOCALE_RETURN_NUMBER: u32 = 536870912u32;
pub const LOCALE_S1159: u32 = 40u32;
pub const LOCALE_S2359: u32 = 41u32;
pub const LOCALE_SABBREVCTRYNAME: u32 = 7u32;
pub const LOCALE_SABBREVDAYNAME1: u32 = 49u32;
pub const LOCALE_SABBREVDAYNAME2: u32 = 50u32;
pub const LOCALE_SABBREVDAYNAME3: u32 = 51u32;
pub const LOCALE_SABBREVDAYNAME4: u32 = 52u32;
pub const LOCALE_SABBREVDAYNAME5: u32 = 53u32;
pub const LOCALE_SABBREVDAYNAME6: u32 = 54u32;
pub const LOCALE_SABBREVDAYNAME7: u32 = 55u32;
pub const LOCALE_SABBREVLANGNAME: u32 = 3u32;
pub const LOCALE_SABBREVMONTHNAME1: u32 = 68u32;
pub const LOCALE_SABBREVMONTHNAME10: u32 = 77u32;
pub const LOCALE_SABBREVMONTHNAME11: u32 = 78u32;
pub const LOCALE_SABBREVMONTHNAME12: u32 = 79u32;
pub const LOCALE_SABBREVMONTHNAME13: u32 = 4111u32;
pub const LOCALE_SABBREVMONTHNAME2: u32 = 69u32;
pub const LOCALE_SABBREVMONTHNAME3: u32 = 70u32;
pub const LOCALE_SABBREVMONTHNAME4: u32 = 71u32;
pub const LOCALE_SABBREVMONTHNAME5: u32 = 72u32;
pub const LOCALE_SABBREVMONTHNAME6: u32 = 73u32;
pub const LOCALE_SABBREVMONTHNAME7: u32 = 74u32;
pub const LOCALE_SABBREVMONTHNAME8: u32 = 75u32;
pub const LOCALE_SABBREVMONTHNAME9: u32 = 76u32;
pub const LOCALE_SAM: u32 = 40u32;
pub const LOCALE_SCONSOLEFALLBACKNAME: u32 = 110u32;
pub const LOCALE_SCOUNTRY: u32 = 6u32;
pub const LOCALE_SCURRENCY: u32 = 20u32;
pub const LOCALE_SDATE: u32 = 29u32;
pub const LOCALE_SDAYNAME1: u32 = 42u32;
pub const LOCALE_SDAYNAME2: u32 = 43u32;
pub const LOCALE_SDAYNAME3: u32 = 44u32;
pub const LOCALE_SDAYNAME4: u32 = 45u32;
pub const LOCALE_SDAYNAME5: u32 = 46u32;
pub const LOCALE_SDAYNAME6: u32 = 47u32;
pub const LOCALE_SDAYNAME7: u32 = 48u32;
pub const LOCALE_SDECIMAL: u32 = 14u32;
pub const LOCALE_SDURATION: u32 = 93u32;
pub const LOCALE_SENGCOUNTRY: u32 = 4098u32;
pub const LOCALE_SENGCURRNAME: u32 = 4103u32;
pub const LOCALE_SENGLANGUAGE: u32 = 4097u32;
pub const LOCALE_SENGLISHCOUNTRYNAME: u32 = 4098u32;
pub const LOCALE_SENGLISHDISPLAYNAME: u32 = 114u32;
pub const LOCALE_SENGLISHLANGUAGENAME: u32 = 4097u32;
pub const LOCALE_SGROUPING: u32 = 16u32;
pub const LOCALE_SINTLSYMBOL: u32 = 21u32;
pub const LOCALE_SISO3166CTRYNAME: u32 = 90u32;
pub const LOCALE_SISO3166CTRYNAME2: u32 = 104u32;
pub const LOCALE_SISO639LANGNAME: u32 = 89u32;
pub const LOCALE_SISO639LANGNAME2: u32 = 103u32;
pub const LOCALE_SKEYBOARDSTOINSTALL: u32 = 94u32;
pub const LOCALE_SLANGDISPLAYNAME: u32 = 111u32;
pub const LOCALE_SLANGUAGE: u32 = 2u32;
pub const LOCALE_SLIST: u32 = 12u32;
pub const LOCALE_SLOCALIZEDCOUNTRYNAME: u32 = 6u32;
pub const LOCALE_SLOCALIZEDDISPLAYNAME: u32 = 2u32;
pub const LOCALE_SLOCALIZEDLANGUAGENAME: u32 = 111u32;
pub const LOCALE_SLONGDATE: u32 = 32u32;
pub const LOCALE_SMONDECIMALSEP: u32 = 22u32;
pub const LOCALE_SMONGROUPING: u32 = 24u32;
pub const LOCALE_SMONTHDAY: u32 = 120u32;
pub const LOCALE_SMONTHNAME1: u32 = 56u32;
pub const LOCALE_SMONTHNAME10: u32 = 65u32;
pub const LOCALE_SMONTHNAME11: u32 = 66u32;
pub const LOCALE_SMONTHNAME12: u32 = 67u32;
pub const LOCALE_SMONTHNAME13: u32 = 4110u32;
pub const LOCALE_SMONTHNAME2: u32 = 57u32;
pub const LOCALE_SMONTHNAME3: u32 = 58u32;
pub const LOCALE_SMONTHNAME4: u32 = 59u32;
pub const LOCALE_SMONTHNAME5: u32 = 60u32;
pub const LOCALE_SMONTHNAME6: u32 = 61u32;
pub const LOCALE_SMONTHNAME7: u32 = 62u32;
pub const LOCALE_SMONTHNAME8: u32 = 63u32;
pub const LOCALE_SMONTHNAME9: u32 = 64u32;
pub const LOCALE_SMONTHOUSANDSEP: u32 = 23u32;
pub const LOCALE_SNAME: u32 = 92u32;
pub const LOCALE_SNAN: u32 = 105u32;
pub const LOCALE_SNATIVECOUNTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECURRNAME: u32 = 4104u32;
pub const LOCALE_SNATIVEDIGITS: u32 = 19u32;
pub const LOCALE_SNATIVEDISPLAYNAME: u32 = 115u32;
pub const LOCALE_SNATIVELANGNAME: u32 = 4u32;
pub const LOCALE_SNATIVELANGUAGENAME: u32 = 4u32;
pub const LOCALE_SNEGATIVESIGN: u32 = 81u32;
pub const LOCALE_SNEGINFINITY: u32 = 107u32;
pub const LOCALE_SOPENTYPELANGUAGETAG: u32 = 122u32;
pub const LOCALE_SPARENT: u32 = 109u32;
pub const LOCALE_SPECIFICDATA: u32 = 32u32;
pub const LOCALE_SPERCENT: u32 = 118u32;
pub const LOCALE_SPERMILLE: u32 = 119u32;
pub const LOCALE_SPM: u32 = 41u32;
pub const LOCALE_SPOSINFINITY: u32 = 106u32;
pub const LOCALE_SPOSITIVESIGN: u32 = 80u32;
pub const LOCALE_SRELATIVELONGDATE: u32 = 124u32;
pub const LOCALE_SSCRIPTS: u32 = 108u32;
pub const LOCALE_SSHORTDATE: u32 = 31u32;
pub const LOCALE_SSHORTESTAM: u32 = 126u32;
pub const LOCALE_SSHORTESTDAYNAME1: u32 = 96u32;
pub const LOCALE_SSHORTESTDAYNAME2: u32 = 97u32;
pub const LOCALE_SSHORTESTDAYNAME3: u32 = 98u32;
pub const LOCALE_SSHORTESTDAYNAME4: u32 = 99u32;
pub const LOCALE_SSHORTESTDAYNAME5: u32 = 100u32;
pub const LOCALE_SSHORTESTDAYNAME6: u32 = 101u32;
pub const LOCALE_SSHORTESTDAYNAME7: u32 = 102u32;
pub const LOCALE_SSHORTESTPM: u32 = 127u32;
pub const LOCALE_SSHORTTIME: u32 = 121u32;
pub const LOCALE_SSORTLOCALE: u32 = 123u32;
pub const LOCALE_SSORTNAME: u32 = 4115u32;
pub const LOCALE_STHOUSAND: u32 = 15u32;
pub const LOCALE_STIME: u32 = 30u32;
pub const LOCALE_STIMEFORMAT: u32 = 4099u32;
pub const LOCALE_SUPPLEMENTAL: u32 = 2u32;
pub const LOCALE_SYEARMONTH: u32 = 4102u32;
pub const LOCALE_USE_CP_ACP: u32 = 1073741824u32;
pub const LOCALE_WINDOWS: u32 = 1u32;
pub const LOWLEVEL_SERVICE_TYPES: u32 = 2u32;
pub const LOW_SURROGATE_END: u32 = 57343u32;
pub const LOW_SURROGATE_START: u32 = 56320u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MAPPING_DATA_RANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MAPPING_ENUM_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MAPPING_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MAPPING_PROPERTY_BAG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MAPPING_SERVICE_INFO(i32);
pub const MAX_DEFAULTCHAR: u32 = 2u32;
pub const MAX_LEADBYTES: u32 = 12u32;
pub const MAX_LOCALE_NAME: u32 = 32u32;
pub const MAX_MIMECP_NAME: u32 = 64u32;
pub const MAX_MIMECSET_NAME: u32 = 50u32;
pub const MAX_MIMEFACE_NAME: u32 = 32u32;
pub const MAX_RFC1766_NAME: u32 = 6u32;
pub const MAX_SCRIPT_NAME: u32 = 48u32;
#[repr(transparent)]
pub struct MIMECONTF(pub i32);
pub const MIMECONTF_MAILNEWS: MIMECONTF = MIMECONTF(1i32);
pub const MIMECONTF_BROWSER: MIMECONTF = MIMECONTF(2i32);
pub const MIMECONTF_MINIMAL: MIMECONTF = MIMECONTF(4i32);
pub const MIMECONTF_IMPORT: MIMECONTF = MIMECONTF(8i32);
pub const MIMECONTF_SAVABLE_MAILNEWS: MIMECONTF = MIMECONTF(256i32);
pub const MIMECONTF_SAVABLE_BROWSER: MIMECONTF = MIMECONTF(512i32);
pub const MIMECONTF_EXPORT: MIMECONTF = MIMECONTF(1024i32);
pub const MIMECONTF_PRIVCONVERTER: MIMECONTF = MIMECONTF(65536i32);
pub const MIMECONTF_VALID: MIMECONTF = MIMECONTF(131072i32);
pub const MIMECONTF_VALID_NLS: MIMECONTF = MIMECONTF(262144i32);
pub const MIMECONTF_MIME_IE4: MIMECONTF = MIMECONTF(268435456i32);
pub const MIMECONTF_MIME_LATEST: MIMECONTF = MIMECONTF(536870912i32);
pub const MIMECONTF_MIME_REGISTRY: MIMECONTF = MIMECONTF(1073741824i32);
#[repr(C)]
pub struct MIMECPINFO(i32);
#[repr(C)]
pub struct MIMECSETINFO(i32);
pub const MIN_SPELLING_NTDDI: u32 = 100794368u32;
#[repr(transparent)]
pub struct MLDETECTCP(pub i32);
pub const MLDETECTCP_NONE: MLDETECTCP = MLDETECTCP(0i32);
pub const MLDETECTCP_7BIT: MLDETECTCP = MLDETECTCP(1i32);
pub const MLDETECTCP_8BIT: MLDETECTCP = MLDETECTCP(2i32);
pub const MLDETECTCP_DBCS: MLDETECTCP = MLDETECTCP(4i32);
pub const MLDETECTCP_HTML: MLDETECTCP = MLDETECTCP(8i32);
pub const MLDETECTCP_NUMBER: MLDETECTCP = MLDETECTCP(16i32);
#[repr(transparent)]
pub struct MLSTR_FLAGS(pub i32);
pub const MLSTR_READ: MLSTR_FLAGS = MLSTR_FLAGS(1i32);
pub const MLSTR_WRITE: MLSTR_FLAGS = MLSTR_FLAGS(2i32);
pub const MUI_COMPLEX_SCRIPT_FILTER: u32 = 512u32;
pub const MUI_CONSOLE_FILTER: u32 = 256u32;
pub const MUI_FILEINFO_VERSION: u32 = 1u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN: u32 = 2u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI: u32 = 4u32;
pub const MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL: u32 = 1u32;
pub const MUI_FORMAT_INF_COMPAT: u32 = 2u32;
pub const MUI_FORMAT_REG_COMPAT: u32 = 1u32;
pub const MUI_FULL_LANGUAGE: u32 = 1u32;
pub const MUI_IMMUTABLE_LOOKUP: u32 = 16u32;
pub const MUI_LANGUAGE_EXACT: u32 = 16u32;
pub const MUI_LANGUAGE_ID: u32 = 4u32;
pub const MUI_LANGUAGE_INSTALLED: u32 = 32u32;
pub const MUI_LANGUAGE_LICENSED: u32 = 64u32;
pub const MUI_LANGUAGE_NAME: u32 = 8u32;
pub const MUI_LANG_NEUTRAL_PE_FILE: u32 = 256u32;
pub const MUI_LIP_LANGUAGE: u32 = 4u32;
pub const MUI_MACHINE_LANGUAGE_SETTINGS: u32 = 1024u32;
pub const MUI_MERGE_SYSTEM_FALLBACK: u32 = 16u32;
pub const MUI_MERGE_USER_FALLBACK: u32 = 32u32;
pub const MUI_NON_LANG_NEUTRAL_FILE: u32 = 512u32;
pub const MUI_PARTIAL_LANGUAGE: u32 = 2u32;
pub const MUI_QUERY_CHECKSUM: u32 = 2u32;
pub const MUI_QUERY_LANGUAGE_NAME: u32 = 4u32;
pub const MUI_QUERY_RESOURCE_TYPES: u32 = 8u32;
pub const MUI_QUERY_TYPE: u32 = 1u32;
pub const MUI_RESET_FILTERS: u32 = 1u32;
pub const MUI_SKIP_STRING_CACHE: u32 = 8u32;
pub const MUI_THREAD_LANGUAGES: u32 = 64u32;
pub const MUI_USER_PREFERRED_UI_LANGUAGES: u32 = 16u32;
pub const MUI_USE_INSTALLED_LANGUAGES: u32 = 32u32;
pub const MUI_USE_SEARCH_ALL_LANGUAGES: u32 = 64u32;
pub const MUI_VERIFY_FILE_EXISTS: u32 = 4u32;
#[repr(transparent)]
pub struct MULTI_BYTE_TO_WIDE_CHAR_FLAGS(pub u32);
pub const MB_COMPOSITE: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(2u32);
pub const MB_ERR_INVALID_CHARS: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(8u32);
pub const MB_PRECOMPOSED: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(1u32);
pub const MB_USEGLYPHCHARS: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(4u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct NEWTEXTMETRICEXA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct NEWTEXTMETRICEXW(i32);
#[repr(C)]
pub struct NLSVERSIONINFO(i32);
#[repr(C)]
pub struct NLSVERSIONINFOEX(i32);
pub const NLS_CP_CPINFO: u32 = 268435456u32;
pub const NLS_CP_MBTOWC: u32 = 1073741824u32;
pub const NLS_CP_WCTOMB: u32 = 2147483648u32;
#[repr(transparent)]
pub struct NORM_FORM(pub i32);
pub const NormalizationOther: NORM_FORM = NORM_FORM(0i32);
pub const NormalizationC: NORM_FORM = NORM_FORM(1i32);
pub const NormalizationD: NORM_FORM = NORM_FORM(2i32);
pub const NormalizationKC: NORM_FORM = NORM_FORM(5i32);
pub const NormalizationKD: NORM_FORM = NORM_FORM(6i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NUMBERFMTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NUMBERFMTW(i32);
pub const NUMSYS_NAME_CAPACITY: u32 = 8u32;
pub const OFFLINE_SERVICES: u32 = 2u32;
pub const ONLINE_SERVICES: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_MAPPINGCALLBACKPROC = unsafe extern "system" fn(pbag: *mut MAPPING_PROPERTY_BAG, data: *mut ::core::ffi::c_void, dwdatasize: u32, result: ::windows_sys::core::HRESULT);
#[repr(C)]
pub struct RFC1766INFO(i32);
#[repr(transparent)]
pub struct SCRIPTCONTF(pub i32);
pub const sidDefault: SCRIPTCONTF = SCRIPTCONTF(0i32);
pub const sidMerge: SCRIPTCONTF = SCRIPTCONTF(1i32);
pub const sidAsciiSym: SCRIPTCONTF = SCRIPTCONTF(2i32);
pub const sidAsciiLatin: SCRIPTCONTF = SCRIPTCONTF(3i32);
pub const sidLatin: SCRIPTCONTF = SCRIPTCONTF(4i32);
pub const sidGreek: SCRIPTCONTF = SCRIPTCONTF(5i32);
pub const sidCyrillic: SCRIPTCONTF = SCRIPTCONTF(6i32);
pub const sidArmenian: SCRIPTCONTF = SCRIPTCONTF(7i32);
pub const sidHebrew: SCRIPTCONTF = SCRIPTCONTF(8i32);
pub const sidArabic: SCRIPTCONTF = SCRIPTCONTF(9i32);
pub const sidDevanagari: SCRIPTCONTF = SCRIPTCONTF(10i32);
pub const sidBengali: SCRIPTCONTF = SCRIPTCONTF(11i32);
pub const sidGurmukhi: SCRIPTCONTF = SCRIPTCONTF(12i32);
pub const sidGujarati: SCRIPTCONTF = SCRIPTCONTF(13i32);
pub const sidOriya: SCRIPTCONTF = SCRIPTCONTF(14i32);
pub const sidTamil: SCRIPTCONTF = SCRIPTCONTF(15i32);
pub const sidTelugu: SCRIPTCONTF = SCRIPTCONTF(16i32);
pub const sidKannada: SCRIPTCONTF = SCRIPTCONTF(17i32);
pub const sidMalayalam: SCRIPTCONTF = SCRIPTCONTF(18i32);
pub const sidThai: SCRIPTCONTF = SCRIPTCONTF(19i32);
pub const sidLao: SCRIPTCONTF = SCRIPTCONTF(20i32);
pub const sidTibetan: SCRIPTCONTF = SCRIPTCONTF(21i32);
pub const sidGeorgian: SCRIPTCONTF = SCRIPTCONTF(22i32);
pub const sidHangul: SCRIPTCONTF = SCRIPTCONTF(23i32);
pub const sidKana: SCRIPTCONTF = SCRIPTCONTF(24i32);
pub const sidBopomofo: SCRIPTCONTF = SCRIPTCONTF(25i32);
pub const sidHan: SCRIPTCONTF = SCRIPTCONTF(26i32);
pub const sidEthiopic: SCRIPTCONTF = SCRIPTCONTF(27i32);
pub const sidCanSyllabic: SCRIPTCONTF = SCRIPTCONTF(28i32);
pub const sidCherokee: SCRIPTCONTF = SCRIPTCONTF(29i32);
pub const sidYi: SCRIPTCONTF = SCRIPTCONTF(30i32);
pub const sidBraille: SCRIPTCONTF = SCRIPTCONTF(31i32);
pub const sidRunic: SCRIPTCONTF = SCRIPTCONTF(32i32);
pub const sidOgham: SCRIPTCONTF = SCRIPTCONTF(33i32);
pub const sidSinhala: SCRIPTCONTF = SCRIPTCONTF(34i32);
pub const sidSyriac: SCRIPTCONTF = SCRIPTCONTF(35i32);
pub const sidBurmese: SCRIPTCONTF = SCRIPTCONTF(36i32);
pub const sidKhmer: SCRIPTCONTF = SCRIPTCONTF(37i32);
pub const sidThaana: SCRIPTCONTF = SCRIPTCONTF(38i32);
pub const sidMongolian: SCRIPTCONTF = SCRIPTCONTF(39i32);
pub const sidUserDefined: SCRIPTCONTF = SCRIPTCONTF(40i32);
pub const sidLim: SCRIPTCONTF = SCRIPTCONTF(41i32);
pub const sidFEFirst: SCRIPTCONTF = SCRIPTCONTF(23i32);
pub const sidFELast: SCRIPTCONTF = SCRIPTCONTF(26i32);
#[repr(transparent)]
pub struct SCRIPTFONTCONTF(pub i32);
pub const SCRIPTCONTF_FIXED_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(1i32);
pub const SCRIPTCONTF_PROPORTIONAL_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(2i32);
pub const SCRIPTCONTF_SCRIPT_USER: SCRIPTFONTCONTF = SCRIPTFONTCONTF(65536i32);
pub const SCRIPTCONTF_SCRIPT_HIDE: SCRIPTFONTCONTF = SCRIPTFONTCONTF(131072i32);
pub const SCRIPTCONTF_SCRIPT_SYSTEM: SCRIPTFONTCONTF = SCRIPTFONTCONTF(262144i32);
#[repr(C)]
pub struct SCRIPTINFO(i32);
#[repr(C)]
pub struct SCRIPT_ANALYSIS(i32);
#[repr(C)]
pub struct SCRIPT_CONTROL(i32);
#[repr(C)]
pub struct SCRIPT_DIGITSUBSTITUTE(i32);
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0u32;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2u32;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1u32;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3u32;
#[repr(C)]
pub struct SCRIPT_FONTPROPERTIES(i32);
#[repr(transparent)]
pub struct SCRIPT_IS_COMPLEX_FLAGS(pub u32);
pub const SIC_ASCIIDIGIT: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(2u32);
pub const SIC_COMPLEX: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(1u32);
pub const SIC_NEUTRAL: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(4u32);
#[repr(C)]
pub struct SCRIPT_ITEM(i32);
#[repr(transparent)]
pub struct SCRIPT_JUSTIFY(pub i32);
pub const SCRIPT_JUSTIFY_NONE: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(0i32);
pub const SCRIPT_JUSTIFY_ARABIC_BLANK: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(1i32);
pub const SCRIPT_JUSTIFY_CHARACTER: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(2i32);
pub const SCRIPT_JUSTIFY_RESERVED1: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(3i32);
pub const SCRIPT_JUSTIFY_BLANK: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(4i32);
pub const SCRIPT_JUSTIFY_RESERVED2: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(5i32);
pub const SCRIPT_JUSTIFY_RESERVED3: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(6i32);
pub const SCRIPT_JUSTIFY_ARABIC_NORMAL: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(7i32);
pub const SCRIPT_JUSTIFY_ARABIC_KASHIDA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(8i32);
pub const SCRIPT_JUSTIFY_ARABIC_ALEF: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(9i32);
pub const SCRIPT_JUSTIFY_ARABIC_HA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(10i32);
pub const SCRIPT_JUSTIFY_ARABIC_RA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(11i32);
pub const SCRIPT_JUSTIFY_ARABIC_BA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(12i32);
pub const SCRIPT_JUSTIFY_ARABIC_BARA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(13i32);
pub const SCRIPT_JUSTIFY_ARABIC_SEEN: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(14i32);
pub const SCRIPT_JUSTIFY_ARABIC_SEEN_M: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(15i32);
#[repr(C)]
pub struct SCRIPT_LOGATTR(i32);
#[repr(C)]
pub struct SCRIPT_PROPERTIES(i32);
#[repr(C)]
pub struct SCRIPT_STATE(i32);
#[repr(C)]
pub struct SCRIPT_TABDEF(i32);
pub const SCRIPT_TAG_UNKNOWN: u32 = 0u32;
pub const SCRIPT_UNDEFINED: u32 = 0u32;
#[repr(C)]
pub struct SCRIPT_VISATTR(i32);
pub const SGCM_RTL: u32 = 1u32;
pub const SORTING_PARADIGM_ICU: u32 = 16777216u32;
pub const SORTING_PARADIGM_NLS: u32 = 0u32;
pub const SSA_BREAK: u32 = 64u32;
pub const SSA_CLIP: u32 = 4u32;
pub const SSA_DONTGLYPH: u32 = 1073741824u32;
pub const SSA_DZWG: u32 = 16u32;
pub const SSA_FALLBACK: u32 = 32u32;
pub const SSA_FIT: u32 = 8u32;
pub const SSA_FULLMEASURE: u32 = 67108864u32;
pub const SSA_GCP: u32 = 512u32;
pub const SSA_GLYPHS: u32 = 128u32;
pub const SSA_HIDEHOTKEY: u32 = 8192u32;
pub const SSA_HOTKEY: u32 = 1024u32;
pub const SSA_HOTKEYONLY: u32 = 9216u32;
pub const SSA_LAYOUTRTL: u32 = 536870912u32;
pub const SSA_LINK: u32 = 4096u32;
pub const SSA_LPKANSIFALLBACK: u32 = 134217728u32;
pub const SSA_METAFILE: u32 = 2048u32;
pub const SSA_NOKASHIDA: u32 = 2147483648u32;
pub const SSA_PASSWORD: u32 = 1u32;
pub const SSA_PIDX: u32 = 268435456u32;
pub const SSA_RTL: u32 = 256u32;
pub const SSA_TAB: u32 = 2u32;
#[repr(transparent)]
pub struct SYSGEOCLASS(pub i32);
pub const GEOCLASS_NATION: SYSGEOCLASS = SYSGEOCLASS(16i32);
pub const GEOCLASS_REGION: SYSGEOCLASS = SYSGEOCLASS(14i32);
pub const GEOCLASS_ALL: SYSGEOCLASS = SYSGEOCLASS(0i32);
#[repr(transparent)]
pub struct SYSGEOTYPE(pub i32);
pub const GEO_NATION: SYSGEOTYPE = SYSGEOTYPE(1i32);
pub const GEO_LATITUDE: SYSGEOTYPE = SYSGEOTYPE(2i32);
pub const GEO_LONGITUDE: SYSGEOTYPE = SYSGEOTYPE(3i32);
pub const GEO_ISO2: SYSGEOTYPE = SYSGEOTYPE(4i32);
pub const GEO_ISO3: SYSGEOTYPE = SYSGEOTYPE(5i32);
pub const GEO_RFC1766: SYSGEOTYPE = SYSGEOTYPE(6i32);
pub const GEO_LCID: SYSGEOTYPE = SYSGEOTYPE(7i32);
pub const GEO_FRIENDLYNAME: SYSGEOTYPE = SYSGEOTYPE(8i32);
pub const GEO_OFFICIALNAME: SYSGEOTYPE = SYSGEOTYPE(9i32);
pub const GEO_TIMEZONES: SYSGEOTYPE = SYSGEOTYPE(10i32);
pub const GEO_OFFICIALLANGUAGES: SYSGEOTYPE = SYSGEOTYPE(11i32);
pub const GEO_ISO_UN_NUMBER: SYSGEOTYPE = SYSGEOTYPE(12i32);
pub const GEO_PARENT: SYSGEOTYPE = SYSGEOTYPE(13i32);
pub const GEO_DIALINGCODE: SYSGEOTYPE = SYSGEOTYPE(14i32);
pub const GEO_CURRENCYCODE: SYSGEOTYPE = SYSGEOTYPE(15i32);
pub const GEO_CURRENCYSYMBOL: SYSGEOTYPE = SYSGEOTYPE(16i32);
pub const GEO_NAME: SYSGEOTYPE = SYSGEOTYPE(17i32);
pub const GEO_ID: SYSGEOTYPE = SYSGEOTYPE(18i32);
#[repr(transparent)]
pub struct SYSNLS_FUNCTION(pub i32);
pub const COMPARE_STRING: SYSNLS_FUNCTION = SYSNLS_FUNCTION(1i32);
#[repr(C)]
pub struct SpellCheckerFactory(i32);
#[cfg(feature = "Win32_Foundation")]
pub type TIMEFMT_ENUMPROCA = unsafe extern "system" fn(param0: super::Foundation::PSTR) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TIMEFMT_ENUMPROCEX = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: super::Foundation::LPARAM) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TIMEFMT_ENUMPROCW = unsafe extern "system" fn(param0: super::Foundation::PWSTR) -> super::Foundation::BOOL;
#[repr(transparent)]
pub struct TIME_FORMAT_FLAGS(pub u32);
pub const TIME_NOMINUTESORSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(1u32);
pub const TIME_NOSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(2u32);
pub const TIME_NOTIMEMARKER: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(4u32);
pub const TIME_FORCE24HOURFORMAT: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(8u32);
#[repr(transparent)]
pub struct TRANSLATE_CHARSET_INFO_FLAGS(pub u32);
pub const TCI_SRCCHARSET: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(1u32);
pub const TCI_SRCCODEPAGE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(2u32);
pub const TCI_SRCFONTSIG: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(3u32);
pub const TCI_SRCLOCALE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(4096u32);
pub const U16_MAX_LENGTH: u32 = 2u32;
pub const U8_MAX_LENGTH: u32 = 4u32;
#[repr(transparent)]
pub struct UAcceptResult(pub i32);
pub const ULOC_ACCEPT_FAILED: UAcceptResult = UAcceptResult(0i32);
pub const ULOC_ACCEPT_VALID: UAcceptResult = UAcceptResult(1i32);
pub const ULOC_ACCEPT_FALLBACK: UAcceptResult = UAcceptResult(2i32);
#[repr(transparent)]
pub struct UAlphabeticIndexLabelType(pub i32);
pub const U_ALPHAINDEX_NORMAL: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(0i32);
pub const U_ALPHAINDEX_UNDERFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(1i32);
pub const U_ALPHAINDEX_INFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(2i32);
pub const U_ALPHAINDEX_OVERFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(3i32);
pub const UBIDI_DEFAULT_LTR: u32 = 254u32;
pub const UBIDI_DEFAULT_RTL: u32 = 255u32;
pub const UBIDI_DO_MIRRORING: u32 = 2u32;
pub const UBIDI_INSERT_LRM_FOR_NUMERIC: u32 = 4u32;
pub const UBIDI_KEEP_BASE_COMBINING: u32 = 1u32;
pub const UBIDI_LEVEL_OVERRIDE: u32 = 128u32;
pub const UBIDI_MAP_NOWHERE: i32 = -1i32;
pub const UBIDI_MAX_EXPLICIT_LEVEL: u32 = 125u32;
pub const UBIDI_OUTPUT_REVERSE: u32 = 16u32;
pub const UBIDI_REMOVE_BIDI_CONTROLS: u32 = 8u32;
#[repr(C)]
pub struct UBiDi(i32);
pub type UBiDiClassCallback = unsafe extern "system" fn(context: *const ::core::ffi::c_void, c: i32) -> UCharDirection;
#[repr(transparent)]
pub struct UBiDiDirection(pub i32);
pub const UBIDI_LTR: UBiDiDirection = UBiDiDirection(0i32);
pub const UBIDI_RTL: UBiDiDirection = UBiDiDirection(1i32);
pub const UBIDI_MIXED: UBiDiDirection = UBiDiDirection(2i32);
pub const UBIDI_NEUTRAL: UBiDiDirection = UBiDiDirection(3i32);
#[repr(transparent)]
pub struct UBiDiMirroring(pub i32);
pub const UBIDI_MIRRORING_OFF: UBiDiMirroring = UBiDiMirroring(0i32);
pub const UBIDI_MIRRORING_ON: UBiDiMirroring = UBiDiMirroring(1i32);
#[repr(transparent)]
pub struct UBiDiOrder(pub i32);
pub const UBIDI_LOGICAL: UBiDiOrder = UBiDiOrder(0i32);
pub const UBIDI_VISUAL: UBiDiOrder = UBiDiOrder(1i32);
#[repr(transparent)]
pub struct UBiDiReorderingMode(pub i32);
pub const UBIDI_REORDER_DEFAULT: UBiDiReorderingMode = UBiDiReorderingMode(0i32);
pub const UBIDI_REORDER_NUMBERS_SPECIAL: UBiDiReorderingMode = UBiDiReorderingMode(1i32);
pub const UBIDI_REORDER_GROUP_NUMBERS_WITH_R: UBiDiReorderingMode = UBiDiReorderingMode(2i32);
pub const UBIDI_REORDER_RUNS_ONLY: UBiDiReorderingMode = UBiDiReorderingMode(3i32);
pub const UBIDI_REORDER_INVERSE_NUMBERS_AS_L: UBiDiReorderingMode = UBiDiReorderingMode(4i32);
pub const UBIDI_REORDER_INVERSE_LIKE_DIRECT: UBiDiReorderingMode = UBiDiReorderingMode(5i32);
pub const UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL: UBiDiReorderingMode = UBiDiReorderingMode(6i32);
#[repr(transparent)]
pub struct UBiDiReorderingOption(pub i32);
pub const UBIDI_OPTION_DEFAULT: UBiDiReorderingOption = UBiDiReorderingOption(0i32);
pub const UBIDI_OPTION_INSERT_MARKS: UBiDiReorderingOption = UBiDiReorderingOption(1i32);
pub const UBIDI_OPTION_REMOVE_CONTROLS: UBiDiReorderingOption = UBiDiReorderingOption(2i32);
pub const UBIDI_OPTION_STREAMING: UBiDiReorderingOption = UBiDiReorderingOption(4i32);
#[repr(C)]
pub struct UBiDiTransform(i32);
#[repr(transparent)]
pub struct UBidiPairedBracketType(pub i32);
pub const U_BPT_NONE: UBidiPairedBracketType = UBidiPairedBracketType(0i32);
pub const U_BPT_OPEN: UBidiPairedBracketType = UBidiPairedBracketType(1i32);
pub const U_BPT_CLOSE: UBidiPairedBracketType = UBidiPairedBracketType(2i32);
#[repr(transparent)]
pub struct UBlockCode(pub i32);
pub const UBLOCK_NO_BLOCK: UBlockCode = UBlockCode(0i32);
pub const UBLOCK_BASIC_LATIN: UBlockCode = UBlockCode(1i32);
pub const UBLOCK_LATIN_1_SUPPLEMENT: UBlockCode = UBlockCode(2i32);
pub const UBLOCK_LATIN_EXTENDED_A: UBlockCode = UBlockCode(3i32);
pub const UBLOCK_LATIN_EXTENDED_B: UBlockCode = UBlockCode(4i32);
pub const UBLOCK_IPA_EXTENSIONS: UBlockCode = UBlockCode(5i32);
pub const UBLOCK_SPACING_MODIFIER_LETTERS: UBlockCode = UBlockCode(6i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS: UBlockCode = UBlockCode(7i32);
pub const UBLOCK_GREEK: UBlockCode = UBlockCode(8i32);
pub const UBLOCK_CYRILLIC: UBlockCode = UBlockCode(9i32);
pub const UBLOCK_ARMENIAN: UBlockCode = UBlockCode(10i32);
pub const UBLOCK_HEBREW: UBlockCode = UBlockCode(11i32);
pub const UBLOCK_ARABIC: UBlockCode = UBlockCode(12i32);
pub const UBLOCK_SYRIAC: UBlockCode = UBlockCode(13i32);
pub const UBLOCK_THAANA: UBlockCode = UBlockCode(14i32);
pub const UBLOCK_DEVANAGARI: UBlockCode = UBlockCode(15i32);
pub const UBLOCK_BENGALI: UBlockCode = UBlockCode(16i32);
pub const UBLOCK_GURMUKHI: UBlockCode = UBlockCode(17i32);
pub const UBLOCK_GUJARATI: UBlockCode = UBlockCode(18i32);
pub const UBLOCK_ORIYA: UBlockCode = UBlockCode(19i32);
pub const UBLOCK_TAMIL: UBlockCode = UBlockCode(20i32);
pub const UBLOCK_TELUGU: UBlockCode = UBlockCode(21i32);
pub const UBLOCK_KANNADA: UBlockCode = UBlockCode(22i32);
pub const UBLOCK_MALAYALAM: UBlockCode = UBlockCode(23i32);
pub const UBLOCK_SINHALA: UBlockCode = UBlockCode(24i32);
pub const UBLOCK_THAI: UBlockCode = UBlockCode(25i32);
pub const UBLOCK_LAO: UBlockCode = UBlockCode(26i32);
pub const UBLOCK_TIBETAN: UBlockCode = UBlockCode(27i32);
pub const UBLOCK_MYANMAR: UBlockCode = UBlockCode(28i32);
pub const UBLOCK_GEORGIAN: UBlockCode = UBlockCode(29i32);
pub const UBLOCK_HANGUL_JAMO: UBlockCode = UBlockCode(30i32);
pub const UBLOCK_ETHIOPIC: UBlockCode = UBlockCode(31i32);
pub const UBLOCK_CHEROKEE: UBlockCode = UBlockCode(32i32);
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UBlockCode = UBlockCode(33i32);
pub const UBLOCK_OGHAM: UBlockCode = UBlockCode(34i32);
pub const UBLOCK_RUNIC: UBlockCode = UBlockCode(35i32);
pub const UBLOCK_KHMER: UBlockCode = UBlockCode(36i32);
pub const UBLOCK_MONGOLIAN: UBlockCode = UBlockCode(37i32);
pub const UBLOCK_LATIN_EXTENDED_ADDITIONAL: UBlockCode = UBlockCode(38i32);
pub const UBLOCK_GREEK_EXTENDED: UBlockCode = UBlockCode(39i32);
pub const UBLOCK_GENERAL_PUNCTUATION: UBlockCode = UBlockCode(40i32);
pub const UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS: UBlockCode = UBlockCode(41i32);
pub const UBLOCK_CURRENCY_SYMBOLS: UBlockCode = UBlockCode(42i32);
pub const UBLOCK_COMBINING_MARKS_FOR_SYMBOLS: UBlockCode = UBlockCode(43i32);
pub const UBLOCK_LETTERLIKE_SYMBOLS: UBlockCode = UBlockCode(44i32);
pub const UBLOCK_NUMBER_FORMS: UBlockCode = UBlockCode(45i32);
pub const UBLOCK_ARROWS: UBlockCode = UBlockCode(46i32);
pub const UBLOCK_MATHEMATICAL_OPERATORS: UBlockCode = UBlockCode(47i32);
pub const UBLOCK_MISCELLANEOUS_TECHNICAL: UBlockCode = UBlockCode(48i32);
pub const UBLOCK_CONTROL_PICTURES: UBlockCode = UBlockCode(49i32);
pub const UBLOCK_OPTICAL_CHARACTER_RECOGNITION: UBlockCode = UBlockCode(50i32);
pub const UBLOCK_ENCLOSED_ALPHANUMERICS: UBlockCode = UBlockCode(51i32);
pub const UBLOCK_BOX_DRAWING: UBlockCode = UBlockCode(52i32);
pub const UBLOCK_BLOCK_ELEMENTS: UBlockCode = UBlockCode(53i32);
pub const UBLOCK_GEOMETRIC_SHAPES: UBlockCode = UBlockCode(54i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS: UBlockCode = UBlockCode(55i32);
pub const UBLOCK_DINGBATS: UBlockCode = UBlockCode(56i32);
pub const UBLOCK_BRAILLE_PATTERNS: UBlockCode = UBlockCode(57i32);
pub const UBLOCK_CJK_RADICALS_SUPPLEMENT: UBlockCode = UBlockCode(58i32);
pub const UBLOCK_KANGXI_RADICALS: UBlockCode = UBlockCode(59i32);
pub const UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UBlockCode = UBlockCode(60i32);
pub const UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION: UBlockCode = UBlockCode(61i32);
pub const UBLOCK_HIRAGANA: UBlockCode = UBlockCode(62i32);
pub const UBLOCK_KATAKANA: UBlockCode = UBlockCode(63i32);
pub const UBLOCK_BOPOMOFO: UBlockCode = UBlockCode(64i32);
pub const UBLOCK_HANGUL_COMPATIBILITY_JAMO: UBlockCode = UBlockCode(65i32);
pub const UBLOCK_KANBUN: UBlockCode = UBlockCode(66i32);
pub const UBLOCK_BOPOMOFO_EXTENDED: UBlockCode = UBlockCode(67i32);
pub const UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS: UBlockCode = UBlockCode(68i32);
pub const UBLOCK_CJK_COMPATIBILITY: UBlockCode = UBlockCode(69i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UBlockCode = UBlockCode(70i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS: UBlockCode = UBlockCode(71i32);
pub const UBLOCK_YI_SYLLABLES: UBlockCode = UBlockCode(72i32);
pub const UBLOCK_YI_RADICALS: UBlockCode = UBlockCode(73i32);
pub const UBLOCK_HANGUL_SYLLABLES: UBlockCode = UBlockCode(74i32);
pub const UBLOCK_HIGH_SURROGATES: UBlockCode = UBlockCode(75i32);
pub const UBLOCK_HIGH_PRIVATE_USE_SURROGATES: UBlockCode = UBlockCode(76i32);
pub const UBLOCK_LOW_SURROGATES: UBlockCode = UBlockCode(77i32);
pub const UBLOCK_PRIVATE_USE_AREA: UBlockCode = UBlockCode(78i32);
pub const UBLOCK_PRIVATE_USE: UBlockCode = UBlockCode(78i32);
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS: UBlockCode = UBlockCode(79i32);
pub const UBLOCK_ALPHABETIC_PRESENTATION_FORMS: UBlockCode = UBlockCode(80i32);
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_A: UBlockCode = UBlockCode(81i32);
pub const UBLOCK_COMBINING_HALF_MARKS: UBlockCode = UBlockCode(82i32);
pub const UBLOCK_CJK_COMPATIBILITY_FORMS: UBlockCode = UBlockCode(83i32);
pub const UBLOCK_SMALL_FORM_VARIANTS: UBlockCode = UBlockCode(84i32);
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_B: UBlockCode = UBlockCode(85i32);
pub const UBLOCK_SPECIALS: UBlockCode = UBlockCode(86i32);
pub const UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS: UBlockCode = UBlockCode(87i32);
pub const UBLOCK_OLD_ITALIC: UBlockCode = UBlockCode(88i32);
pub const UBLOCK_GOTHIC: UBlockCode = UBlockCode(89i32);
pub const UBLOCK_DESERET: UBlockCode = UBlockCode(90i32);
pub const UBLOCK_BYZANTINE_MUSICAL_SYMBOLS: UBlockCode = UBlockCode(91i32);
pub const UBLOCK_MUSICAL_SYMBOLS: UBlockCode = UBlockCode(92i32);
pub const UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UBlockCode = UBlockCode(93i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UBlockCode = UBlockCode(94i32);
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UBlockCode = UBlockCode(95i32);
pub const UBLOCK_TAGS: UBlockCode = UBlockCode(96i32);
pub const UBLOCK_CYRILLIC_SUPPLEMENT: UBlockCode = UBlockCode(97i32);
pub const UBLOCK_CYRILLIC_SUPPLEMENTARY: UBlockCode = UBlockCode(97i32);
pub const UBLOCK_TAGALOG: UBlockCode = UBlockCode(98i32);
pub const UBLOCK_HANUNOO: UBlockCode = UBlockCode(99i32);
pub const UBLOCK_BUHID: UBlockCode = UBlockCode(100i32);
pub const UBLOCK_TAGBANWA: UBlockCode = UBlockCode(101i32);
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UBlockCode = UBlockCode(102i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_A: UBlockCode = UBlockCode(103i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_B: UBlockCode = UBlockCode(104i32);
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UBlockCode = UBlockCode(105i32);
pub const UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UBlockCode = UBlockCode(106i32);
pub const UBLOCK_KATAKANA_PHONETIC_EXTENSIONS: UBlockCode = UBlockCode(107i32);
pub const UBLOCK_VARIATION_SELECTORS: UBlockCode = UBlockCode(108i32);
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A: UBlockCode = UBlockCode(109i32);
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B: UBlockCode = UBlockCode(110i32);
pub const UBLOCK_LIMBU: UBlockCode = UBlockCode(111i32);
pub const UBLOCK_TAI_LE: UBlockCode = UBlockCode(112i32);
pub const UBLOCK_KHMER_SYMBOLS: UBlockCode = UBlockCode(113i32);
pub const UBLOCK_PHONETIC_EXTENSIONS: UBlockCode = UBlockCode(114i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS: UBlockCode = UBlockCode(115i32);
pub const UBLOCK_YIJING_HEXAGRAM_SYMBOLS: UBlockCode = UBlockCode(116i32);
pub const UBLOCK_LINEAR_B_SYLLABARY: UBlockCode = UBlockCode(117i32);
pub const UBLOCK_LINEAR_B_IDEOGRAMS: UBlockCode = UBlockCode(118i32);
pub const UBLOCK_AEGEAN_NUMBERS: UBlockCode = UBlockCode(119i32);
pub const UBLOCK_UGARITIC: UBlockCode = UBlockCode(120i32);
pub const UBLOCK_SHAVIAN: UBlockCode = UBlockCode(121i32);
pub const UBLOCK_OSMANYA: UBlockCode = UBlockCode(122i32);
pub const UBLOCK_CYPRIOT_SYLLABARY: UBlockCode = UBlockCode(123i32);
pub const UBLOCK_TAI_XUAN_JING_SYMBOLS: UBlockCode = UBlockCode(124i32);
pub const UBLOCK_VARIATION_SELECTORS_SUPPLEMENT: UBlockCode = UBlockCode(125i32);
pub const UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION: UBlockCode = UBlockCode(126i32);
pub const UBLOCK_ANCIENT_GREEK_NUMBERS: UBlockCode = UBlockCode(127i32);
pub const UBLOCK_ARABIC_SUPPLEMENT: UBlockCode = UBlockCode(128i32);
pub const UBLOCK_BUGINESE: UBlockCode = UBlockCode(129i32);
pub const UBLOCK_CJK_STROKES: UBlockCode = UBlockCode(130i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UBlockCode = UBlockCode(131i32);
pub const UBLOCK_COPTIC: UBlockCode = UBlockCode(132i32);
pub const UBLOCK_ETHIOPIC_EXTENDED: UBlockCode = UBlockCode(133i32);
pub const UBLOCK_ETHIOPIC_SUPPLEMENT: UBlockCode = UBlockCode(134i32);
pub const UBLOCK_GEORGIAN_SUPPLEMENT: UBlockCode = UBlockCode(135i32);
pub const UBLOCK_GLAGOLITIC: UBlockCode = UBlockCode(136i32);
pub const UBLOCK_KHAROSHTHI: UBlockCode = UBlockCode(137i32);
pub const UBLOCK_MODIFIER_TONE_LETTERS: UBlockCode = UBlockCode(138i32);
pub const UBLOCK_NEW_TAI_LUE: UBlockCode = UBlockCode(139i32);
pub const UBLOCK_OLD_PERSIAN: UBlockCode = UBlockCode(140i32);
pub const UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT: UBlockCode = UBlockCode(141i32);
pub const UBLOCK_SUPPLEMENTAL_PUNCTUATION: UBlockCode = UBlockCode(142i32);
pub const UBLOCK_SYLOTI_NAGRI: UBlockCode = UBlockCode(143i32);
pub const UBLOCK_TIFINAGH: UBlockCode = UBlockCode(144i32);
pub const UBLOCK_VERTICAL_FORMS: UBlockCode = UBlockCode(145i32);
pub const UBLOCK_NKO: UBlockCode = UBlockCode(146i32);
pub const UBLOCK_BALINESE: UBlockCode = UBlockCode(147i32);
pub const UBLOCK_LATIN_EXTENDED_C: UBlockCode = UBlockCode(148i32);
pub const UBLOCK_LATIN_EXTENDED_D: UBlockCode = UBlockCode(149i32);
pub const UBLOCK_PHAGS_PA: UBlockCode = UBlockCode(150i32);
pub const UBLOCK_PHOENICIAN: UBlockCode = UBlockCode(151i32);
pub const UBLOCK_CUNEIFORM: UBlockCode = UBlockCode(152i32);
pub const UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION: UBlockCode = UBlockCode(153i32);
pub const UBLOCK_COUNTING_ROD_NUMERALS: UBlockCode = UBlockCode(154i32);
pub const UBLOCK_SUNDANESE: UBlockCode = UBlockCode(155i32);
pub const UBLOCK_LEPCHA: UBlockCode = UBlockCode(156i32);
pub const UBLOCK_OL_CHIKI: UBlockCode = UBlockCode(157i32);
pub const UBLOCK_CYRILLIC_EXTENDED_A: UBlockCode = UBlockCode(158i32);
pub const UBLOCK_VAI: UBlockCode = UBlockCode(159i32);
pub const UBLOCK_CYRILLIC_EXTENDED_B: UBlockCode = UBlockCode(160i32);
pub const UBLOCK_SAURASHTRA: UBlockCode = UBlockCode(161i32);
pub const UBLOCK_KAYAH_LI: UBlockCode = UBlockCode(162i32);
pub const UBLOCK_REJANG: UBlockCode = UBlockCode(163i32);
pub const UBLOCK_CHAM: UBlockCode = UBlockCode(164i32);
pub const UBLOCK_ANCIENT_SYMBOLS: UBlockCode = UBlockCode(165i32);
pub const UBLOCK_PHAISTOS_DISC: UBlockCode = UBlockCode(166i32);
pub const UBLOCK_LYCIAN: UBlockCode = UBlockCode(167i32);
pub const UBLOCK_CARIAN: UBlockCode = UBlockCode(168i32);
pub const UBLOCK_LYDIAN: UBlockCode = UBlockCode(169i32);
pub const UBLOCK_MAHJONG_TILES: UBlockCode = UBlockCode(170i32);
pub const UBLOCK_DOMINO_TILES: UBlockCode = UBlockCode(171i32);
pub const UBLOCK_SAMARITAN: UBlockCode = UBlockCode(172i32);
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UBlockCode = UBlockCode(173i32);
pub const UBLOCK_TAI_THAM: UBlockCode = UBlockCode(174i32);
pub const UBLOCK_VEDIC_EXTENSIONS: UBlockCode = UBlockCode(175i32);
pub const UBLOCK_LISU: UBlockCode = UBlockCode(176i32);
pub const UBLOCK_BAMUM: UBlockCode = UBlockCode(177i32);
pub const UBLOCK_COMMON_INDIC_NUMBER_FORMS: UBlockCode = UBlockCode(178i32);
pub const UBLOCK_DEVANAGARI_EXTENDED: UBlockCode = UBlockCode(179i32);
pub const UBLOCK_HANGUL_JAMO_EXTENDED_A: UBlockCode = UBlockCode(180i32);
pub const UBLOCK_JAVANESE: UBlockCode = UBlockCode(181i32);
pub const UBLOCK_MYANMAR_EXTENDED_A: UBlockCode = UBlockCode(182i32);
pub const UBLOCK_TAI_VIET: UBlockCode = UBlockCode(183i32);
pub const UBLOCK_MEETEI_MAYEK: UBlockCode = UBlockCode(184i32);
pub const UBLOCK_HANGUL_JAMO_EXTENDED_B: UBlockCode = UBlockCode(185i32);
pub const UBLOCK_IMPERIAL_ARAMAIC: UBlockCode = UBlockCode(186i32);
pub const UBLOCK_OLD_SOUTH_ARABIAN: UBlockCode = UBlockCode(187i32);
pub const UBLOCK_AVESTAN: UBlockCode = UBlockCode(188i32);
pub const UBLOCK_INSCRIPTIONAL_PARTHIAN: UBlockCode = UBlockCode(189i32);
pub const UBLOCK_INSCRIPTIONAL_PAHLAVI: UBlockCode = UBlockCode(190i32);
pub const UBLOCK_OLD_TURKIC: UBlockCode = UBlockCode(191i32);
pub const UBLOCK_RUMI_NUMERAL_SYMBOLS: UBlockCode = UBlockCode(192i32);
pub const UBLOCK_KAITHI: UBlockCode = UBlockCode(193i32);
pub const UBLOCK_EGYPTIAN_HIEROGLYPHS: UBlockCode = UBlockCode(194i32);
pub const UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UBlockCode = UBlockCode(195i32);
pub const UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UBlockCode = UBlockCode(196i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UBlockCode = UBlockCode(197i32);
pub const UBLOCK_MANDAIC: UBlockCode = UBlockCode(198i32);
pub const UBLOCK_BATAK: UBlockCode = UBlockCode(199i32);
pub const UBLOCK_ETHIOPIC_EXTENDED_A: UBlockCode = UBlockCode(200i32);
pub const UBLOCK_BRAHMI: UBlockCode = UBlockCode(201i32);
pub const UBLOCK_BAMUM_SUPPLEMENT: UBlockCode = UBlockCode(202i32);
pub const UBLOCK_KANA_SUPPLEMENT: UBlockCode = UBlockCode(203i32);
pub const UBLOCK_PLAYING_CARDS: UBlockCode = UBlockCode(204i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = UBlockCode(205i32);
pub const UBLOCK_EMOTICONS: UBlockCode = UBlockCode(206i32);
pub const UBLOCK_TRANSPORT_AND_MAP_SYMBOLS: UBlockCode = UBlockCode(207i32);
pub const UBLOCK_ALCHEMICAL_SYMBOLS: UBlockCode = UBlockCode(208i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UBlockCode = UBlockCode(209i32);
pub const UBLOCK_ARABIC_EXTENDED_A: UBlockCode = UBlockCode(210i32);
pub const UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UBlockCode = UBlockCode(211i32);
pub const UBLOCK_CHAKMA: UBlockCode = UBlockCode(212i32);
pub const UBLOCK_MEETEI_MAYEK_EXTENSIONS: UBlockCode = UBlockCode(213i32);
pub const UBLOCK_MEROITIC_CURSIVE: UBlockCode = UBlockCode(214i32);
pub const UBLOCK_MEROITIC_HIEROGLYPHS: UBlockCode = UBlockCode(215i32);
pub const UBLOCK_MIAO: UBlockCode = UBlockCode(216i32);
pub const UBLOCK_SHARADA: UBlockCode = UBlockCode(217i32);
pub const UBLOCK_SORA_SOMPENG: UBlockCode = UBlockCode(218i32);
pub const UBLOCK_SUNDANESE_SUPPLEMENT: UBlockCode = UBlockCode(219i32);
pub const UBLOCK_TAKRI: UBlockCode = UBlockCode(220i32);
pub const UBLOCK_BASSA_VAH: UBlockCode = UBlockCode(221i32);
pub const UBLOCK_CAUCASIAN_ALBANIAN: UBlockCode = UBlockCode(222i32);
pub const UBLOCK_COPTIC_EPACT_NUMBERS: UBlockCode = UBlockCode(223i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED: UBlockCode = UBlockCode(224i32);
pub const UBLOCK_DUPLOYAN: UBlockCode = UBlockCode(225i32);
pub const UBLOCK_ELBASAN: UBlockCode = UBlockCode(226i32);
pub const UBLOCK_GEOMETRIC_SHAPES_EXTENDED: UBlockCode = UBlockCode(227i32);
pub const UBLOCK_GRANTHA: UBlockCode = UBlockCode(228i32);
pub const UBLOCK_KHOJKI: UBlockCode = UBlockCode(229i32);
pub const UBLOCK_KHUDAWADI: UBlockCode = UBlockCode(230i32);
pub const UBLOCK_LATIN_EXTENDED_E: UBlockCode = UBlockCode(231i32);
pub const UBLOCK_LINEAR_A: UBlockCode = UBlockCode(232i32);
pub const UBLOCK_MAHAJANI: UBlockCode = UBlockCode(233i32);
pub const UBLOCK_MANICHAEAN: UBlockCode = UBlockCode(234i32);
pub const UBLOCK_MENDE_KIKAKUI: UBlockCode = UBlockCode(235i32);
pub const UBLOCK_MODI: UBlockCode = UBlockCode(236i32);
pub const UBLOCK_MRO: UBlockCode = UBlockCode(237i32);
pub const UBLOCK_MYANMAR_EXTENDED_B: UBlockCode = UBlockCode(238i32);
pub const UBLOCK_NABATAEAN: UBlockCode = UBlockCode(239i32);
pub const UBLOCK_OLD_NORTH_ARABIAN: UBlockCode = UBlockCode(240i32);
pub const UBLOCK_OLD_PERMIC: UBlockCode = UBlockCode(241i32);
pub const UBLOCK_ORNAMENTAL_DINGBATS: UBlockCode = UBlockCode(242i32);
pub const UBLOCK_PAHAWH_HMONG: UBlockCode = UBlockCode(243i32);
pub const UBLOCK_PALMYRENE: UBlockCode = UBlockCode(244i32);
pub const UBLOCK_PAU_CIN_HAU: UBlockCode = UBlockCode(245i32);
pub const UBLOCK_PSALTER_PAHLAVI: UBlockCode = UBlockCode(246i32);
pub const UBLOCK_SHORTHAND_FORMAT_CONTROLS: UBlockCode = UBlockCode(247i32);
pub const UBLOCK_SIDDHAM: UBlockCode = UBlockCode(248i32);
pub const UBLOCK_SINHALA_ARCHAIC_NUMBERS: UBlockCode = UBlockCode(249i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_C: UBlockCode = UBlockCode(250i32);
pub const UBLOCK_TIRHUTA: UBlockCode = UBlockCode(251i32);
pub const UBLOCK_WARANG_CITI: UBlockCode = UBlockCode(252i32);
pub const UBLOCK_AHOM: UBlockCode = UBlockCode(253i32);
pub const UBLOCK_ANATOLIAN_HIEROGLYPHS: UBlockCode = UBlockCode(254i32);
pub const UBLOCK_CHEROKEE_SUPPLEMENT: UBlockCode = UBlockCode(255i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UBlockCode = UBlockCode(256i32);
pub const UBLOCK_EARLY_DYNASTIC_CUNEIFORM: UBlockCode = UBlockCode(257i32);
pub const UBLOCK_HATRAN: UBlockCode = UBlockCode(258i32);
pub const UBLOCK_MULTANI: UBlockCode = UBlockCode(259i32);
pub const UBLOCK_OLD_HUNGARIAN: UBlockCode = UBlockCode(260i32);
pub const UBLOCK_SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = UBlockCode(261i32);
pub const UBLOCK_SUTTON_SIGNWRITING: UBlockCode = UBlockCode(262i32);
pub const UBLOCK_ADLAM: UBlockCode = UBlockCode(263i32);
pub const UBLOCK_BHAIKSUKI: UBlockCode = UBlockCode(264i32);
pub const UBLOCK_CYRILLIC_EXTENDED_C: UBlockCode = UBlockCode(265i32);
pub const UBLOCK_GLAGOLITIC_SUPPLEMENT: UBlockCode = UBlockCode(266i32);
pub const UBLOCK_IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UBlockCode = UBlockCode(267i32);
pub const UBLOCK_MARCHEN: UBlockCode = UBlockCode(268i32);
pub const UBLOCK_MONGOLIAN_SUPPLEMENT: UBlockCode = UBlockCode(269i32);
pub const UBLOCK_NEWA: UBlockCode = UBlockCode(270i32);
pub const UBLOCK_OSAGE: UBlockCode = UBlockCode(271i32);
pub const UBLOCK_TANGUT: UBlockCode = UBlockCode(272i32);
pub const UBLOCK_TANGUT_COMPONENTS: UBlockCode = UBlockCode(273i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UBlockCode = UBlockCode(274i32);
pub const UBLOCK_KANA_EXTENDED_A: UBlockCode = UBlockCode(275i32);
pub const UBLOCK_MASARAM_GONDI: UBlockCode = UBlockCode(276i32);
pub const UBLOCK_NUSHU: UBlockCode = UBlockCode(277i32);
pub const UBLOCK_SOYOMBO: UBlockCode = UBlockCode(278i32);
pub const UBLOCK_SYRIAC_SUPPLEMENT: UBlockCode = UBlockCode(279i32);
pub const UBLOCK_ZANABAZAR_SQUARE: UBlockCode = UBlockCode(280i32);
pub const UBLOCK_CHESS_SYMBOLS: UBlockCode = UBlockCode(281i32);
pub const UBLOCK_DOGRA: UBlockCode = UBlockCode(282i32);
pub const UBLOCK_GEORGIAN_EXTENDED: UBlockCode = UBlockCode(283i32);
pub const UBLOCK_GUNJALA_GONDI: UBlockCode = UBlockCode(284i32);
pub const UBLOCK_HANIFI_ROHINGYA: UBlockCode = UBlockCode(285i32);
pub const UBLOCK_INDIC_SIYAQ_NUMBERS: UBlockCode = UBlockCode(286i32);
pub const UBLOCK_MAKASAR: UBlockCode = UBlockCode(287i32);
pub const UBLOCK_MAYAN_NUMERALS: UBlockCode = UBlockCode(288i32);
pub const UBLOCK_MEDEFAIDRIN: UBlockCode = UBlockCode(289i32);
pub const UBLOCK_OLD_SOGDIAN: UBlockCode = UBlockCode(290i32);
pub const UBLOCK_SOGDIAN: UBlockCode = UBlockCode(291i32);
pub const UBLOCK_EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UBlockCode = UBlockCode(292i32);
pub const UBLOCK_ELYMAIC: UBlockCode = UBlockCode(293i32);
pub const UBLOCK_NANDINAGARI: UBlockCode = UBlockCode(294i32);
pub const UBLOCK_NYIAKENG_PUACHUE_HMONG: UBlockCode = UBlockCode(295i32);
pub const UBLOCK_OTTOMAN_SIYAQ_NUMBERS: UBlockCode = UBlockCode(296i32);
pub const UBLOCK_SMALL_KANA_EXTENSION: UBlockCode = UBlockCode(297i32);
pub const UBLOCK_SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UBlockCode = UBlockCode(298i32);
pub const UBLOCK_TAMIL_SUPPLEMENT: UBlockCode = UBlockCode(299i32);
pub const UBLOCK_WANCHO: UBlockCode = UBlockCode(300i32);
pub const UBLOCK_CHORASMIAN: UBlockCode = UBlockCode(301i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UBlockCode = UBlockCode(302i32);
pub const UBLOCK_DIVES_AKURU: UBlockCode = UBlockCode(303i32);
pub const UBLOCK_KHITAN_SMALL_SCRIPT: UBlockCode = UBlockCode(304i32);
pub const UBLOCK_LISU_SUPPLEMENT: UBlockCode = UBlockCode(305i32);
pub const UBLOCK_SYMBOLS_FOR_LEGACY_COMPUTING: UBlockCode = UBlockCode(306i32);
pub const UBLOCK_TANGUT_SUPPLEMENT: UBlockCode = UBlockCode(307i32);
pub const UBLOCK_YEZIDI: UBlockCode = UBlockCode(308i32);
pub const UBLOCK_INVALID_CODE: UBlockCode = UBlockCode(-1i32);
#[repr(C)]
pub struct UBreakIterator(i32);
#[repr(transparent)]
pub struct UBreakIteratorType(pub i32);
pub const UBRK_CHARACTER: UBreakIteratorType = UBreakIteratorType(0i32);
pub const UBRK_WORD: UBreakIteratorType = UBreakIteratorType(1i32);
pub const UBRK_LINE: UBreakIteratorType = UBreakIteratorType(2i32);
pub const UBRK_SENTENCE: UBreakIteratorType = UBreakIteratorType(3i32);
pub const UCHAR_MAX_VALUE: u32 = 1114111u32;
pub const UCHAR_MIN_VALUE: u32 = 0u32;
pub const UCLN_NO_AUTO_CLEANUP: u32 = 1u32;
pub const UCNV_MAX_CONVERTER_NAME_LENGTH: u32 = 60u32;
pub const UCNV_SI: u32 = 15u32;
pub const UCNV_SO: u32 = 14u32;
pub const UCONFIG_ENABLE_PLUGINS: u32 = 0u32;
#[repr(C)]
pub struct UCPMap(i32);
#[repr(transparent)]
pub struct UCPMapRangeOption(pub i32);
pub const UCPMAP_RANGE_NORMAL: UCPMapRangeOption = UCPMapRangeOption(0i32);
pub const UCPMAP_RANGE_FIXED_LEAD_SURROGATES: UCPMapRangeOption = UCPMapRangeOption(1i32);
pub const UCPMAP_RANGE_FIXED_ALL_SURROGATES: UCPMapRangeOption = UCPMapRangeOption(2i32);
pub type UCPMapValueFilter = unsafe extern "system" fn(context: *const ::core::ffi::c_void, value: u32) -> u32;
pub const UCPTRIE_ERROR_VALUE_NEG_DATA_OFFSET: i32 = 1i32;
pub const UCPTRIE_FAST_DATA_BLOCK_LENGTH: i32 = 64i32;
pub const UCPTRIE_FAST_DATA_MASK: i32 = 63i32;
pub const UCPTRIE_FAST_SHIFT: i32 = 6i32;
pub const UCPTRIE_HIGH_VALUE_NEG_DATA_OFFSET: i32 = 2i32;
pub const UCPTRIE_SMALL_MAX: i32 = 4095i32;
#[repr(C)]
pub struct UCPTrie(i32);
#[repr(C)]
pub struct UCPTrieData(i32);
#[repr(transparent)]
pub struct UCPTrieType(pub i32);
pub const UCPTRIE_TYPE_ANY: UCPTrieType = UCPTrieType(-1i32);
pub const UCPTRIE_TYPE_FAST: UCPTrieType = UCPTrieType(0i32);
pub const UCPTRIE_TYPE_SMALL: UCPTrieType = UCPTrieType(1i32);
#[repr(transparent)]
pub struct UCPTrieValueWidth(pub i32);
pub const UCPTRIE_VALUE_BITS_ANY: UCPTrieValueWidth = UCPTrieValueWidth(-1i32);
pub const UCPTRIE_VALUE_BITS_16: UCPTrieValueWidth = UCPTrieValueWidth(0i32);
pub const UCPTRIE_VALUE_BITS_32: UCPTrieValueWidth = UCPTrieValueWidth(1i32);
pub const UCPTRIE_VALUE_BITS_8: UCPTrieValueWidth = UCPTrieValueWidth(2i32);
#[repr(transparent)]
pub struct UCalendarAMPMs(pub i32);
pub const UCAL_AM: UCalendarAMPMs = UCalendarAMPMs(0i32);
pub const UCAL_PM: UCalendarAMPMs = UCalendarAMPMs(1i32);
#[repr(transparent)]
pub struct UCalendarAttribute(pub i32);
pub const UCAL_LENIENT: UCalendarAttribute = UCalendarAttribute(0i32);
pub const UCAL_FIRST_DAY_OF_WEEK: UCalendarAttribute = UCalendarAttribute(1i32);
pub const UCAL_MINIMAL_DAYS_IN_FIRST_WEEK: UCalendarAttribute = UCalendarAttribute(2i32);
pub const UCAL_REPEATED_WALL_TIME: UCalendarAttribute = UCalendarAttribute(3i32);
pub const UCAL_SKIPPED_WALL_TIME: UCalendarAttribute = UCalendarAttribute(4i32);
#[repr(transparent)]
pub struct UCalendarDateFields(pub i32);
pub const UCAL_ERA: UCalendarDateFields = UCalendarDateFields(0i32);
pub const UCAL_YEAR: UCalendarDateFields = UCalendarDateFields(1i32);
pub const UCAL_MONTH: UCalendarDateFields = UCalendarDateFields(2i32);
pub const UCAL_WEEK_OF_YEAR: UCalendarDateFields = UCalendarDateFields(3i32);
pub const UCAL_WEEK_OF_MONTH: UCalendarDateFields = UCalendarDateFields(4i32);
pub const UCAL_DATE: UCalendarDateFields = UCalendarDateFields(5i32);
pub const UCAL_DAY_OF_YEAR: UCalendarDateFields = UCalendarDateFields(6i32);
pub const UCAL_DAY_OF_WEEK: UCalendarDateFields = UCalendarDateFields(7i32);
pub const UCAL_DAY_OF_WEEK_IN_MONTH: UCalendarDateFields = UCalendarDateFields(8i32);
pub const UCAL_AM_PM: UCalendarDateFields = UCalendarDateFields(9i32);
pub const UCAL_HOUR: UCalendarDateFields = UCalendarDateFields(10i32);
pub const UCAL_HOUR_OF_DAY: UCalendarDateFields = UCalendarDateFields(11i32);
pub const UCAL_MINUTE: UCalendarDateFields = UCalendarDateFields(12i32);
pub const UCAL_SECOND: UCalendarDateFields = UCalendarDateFields(13i32);
pub const UCAL_MILLISECOND: UCalendarDateFields = UCalendarDateFields(14i32);
pub const UCAL_ZONE_OFFSET: UCalendarDateFields = UCalendarDateFields(15i32);
pub const UCAL_DST_OFFSET: UCalendarDateFields = UCalendarDateFields(16i32);
pub const UCAL_YEAR_WOY: UCalendarDateFields = UCalendarDateFields(17i32);
pub const UCAL_DOW_LOCAL: UCalendarDateFields = UCalendarDateFields(18i32);
pub const UCAL_EXTENDED_YEAR: UCalendarDateFields = UCalendarDateFields(19i32);
pub const UCAL_JULIAN_DAY: UCalendarDateFields = UCalendarDateFields(20i32);
pub const UCAL_MILLISECONDS_IN_DAY: UCalendarDateFields = UCalendarDateFields(21i32);
pub const UCAL_IS_LEAP_MONTH: UCalendarDateFields = UCalendarDateFields(22i32);
pub const UCAL_FIELD_COUNT: UCalendarDateFields = UCalendarDateFields(23i32);
pub const UCAL_DAY_OF_MONTH: UCalendarDateFields = UCalendarDateFields(5i32);
#[repr(transparent)]
pub struct UCalendarDaysOfWeek(pub i32);
pub const UCAL_SUNDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(1i32);
pub const UCAL_MONDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(2i32);
pub const UCAL_TUESDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(3i32);
pub const UCAL_WEDNESDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(4i32);
pub const UCAL_THURSDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(5i32);
pub const UCAL_FRIDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(6i32);
pub const UCAL_SATURDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(7i32);
#[repr(transparent)]
pub struct UCalendarDisplayNameType(pub i32);
pub const UCAL_STANDARD: UCalendarDisplayNameType = UCalendarDisplayNameType(0i32);
pub const UCAL_SHORT_STANDARD: UCalendarDisplayNameType = UCalendarDisplayNameType(1i32);
pub const UCAL_DST: UCalendarDisplayNameType = UCalendarDisplayNameType(2i32);
pub const UCAL_SHORT_DST: UCalendarDisplayNameType = UCalendarDisplayNameType(3i32);
#[repr(transparent)]
pub struct UCalendarLimitType(pub i32);
pub const UCAL_MINIMUM: UCalendarLimitType = UCalendarLimitType(0i32);
pub const UCAL_MAXIMUM: UCalendarLimitType = UCalendarLimitType(1i32);
pub const UCAL_GREATEST_MINIMUM: UCalendarLimitType = UCalendarLimitType(2i32);
pub const UCAL_LEAST_MAXIMUM: UCalendarLimitType = UCalendarLimitType(3i32);
pub const UCAL_ACTUAL_MINIMUM: UCalendarLimitType = UCalendarLimitType(4i32);
pub const UCAL_ACTUAL_MAXIMUM: UCalendarLimitType = UCalendarLimitType(5i32);
#[repr(transparent)]
pub struct UCalendarMonths(pub i32);
pub const UCAL_JANUARY: UCalendarMonths = UCalendarMonths(0i32);
pub const UCAL_FEBRUARY: UCalendarMonths = UCalendarMonths(1i32);
pub const UCAL_MARCH: UCalendarMonths = UCalendarMonths(2i32);
pub const UCAL_APRIL: UCalendarMonths = UCalendarMonths(3i32);
pub const UCAL_MAY: UCalendarMonths = UCalendarMonths(4i32);
pub const UCAL_JUNE: UCalendarMonths = UCalendarMonths(5i32);
pub const UCAL_JULY: UCalendarMonths = UCalendarMonths(6i32);
pub const UCAL_AUGUST: UCalendarMonths = UCalendarMonths(7i32);
pub const UCAL_SEPTEMBER: UCalendarMonths = UCalendarMonths(8i32);
pub const UCAL_OCTOBER: UCalendarMonths = UCalendarMonths(9i32);
pub const UCAL_NOVEMBER: UCalendarMonths = UCalendarMonths(10i32);
pub const UCAL_DECEMBER: UCalendarMonths = UCalendarMonths(11i32);
pub const UCAL_UNDECIMBER: UCalendarMonths = UCalendarMonths(12i32);
#[repr(transparent)]
pub struct UCalendarType(pub i32);
pub const UCAL_TRADITIONAL: UCalendarType = UCalendarType(0i32);
pub const UCAL_DEFAULT: UCalendarType = UCalendarType(0i32);
pub const UCAL_GREGORIAN: UCalendarType = UCalendarType(1i32);
#[repr(transparent)]
pub struct UCalendarWallTimeOption(pub i32);
pub const UCAL_WALLTIME_LAST: UCalendarWallTimeOption = UCalendarWallTimeOption(0i32);
pub const UCAL_WALLTIME_FIRST: UCalendarWallTimeOption = UCalendarWallTimeOption(1i32);
pub const UCAL_WALLTIME_NEXT_VALID: UCalendarWallTimeOption = UCalendarWallTimeOption(2i32);
#[repr(transparent)]
pub struct UCalendarWeekdayType(pub i32);
pub const UCAL_WEEKDAY: UCalendarWeekdayType = UCalendarWeekdayType(0i32);
pub const UCAL_WEEKEND: UCalendarWeekdayType = UCalendarWeekdayType(1i32);
pub const UCAL_WEEKEND_ONSET: UCalendarWeekdayType = UCalendarWeekdayType(2i32);
pub const UCAL_WEEKEND_CEASE: UCalendarWeekdayType = UCalendarWeekdayType(3i32);
#[repr(C)]
pub struct UCaseMap(i32);
#[repr(transparent)]
pub struct UCharCategory(pub i32);
pub const U_UNASSIGNED: UCharCategory = UCharCategory(0i32);
pub const U_GENERAL_OTHER_TYPES: UCharCategory = UCharCategory(0i32);
pub const U_UPPERCASE_LETTER: UCharCategory = UCharCategory(1i32);
pub const U_LOWERCASE_LETTER: UCharCategory = UCharCategory(2i32);
pub const U_TITLECASE_LETTER: UCharCategory = UCharCategory(3i32);
pub const U_MODIFIER_LETTER: UCharCategory = UCharCategory(4i32);
pub const U_OTHER_LETTER: UCharCategory = UCharCategory(5i32);
pub const U_NON_SPACING_MARK: UCharCategory = UCharCategory(6i32);
pub const U_ENCLOSING_MARK: UCharCategory = UCharCategory(7i32);
pub const U_COMBINING_SPACING_MARK: UCharCategory = UCharCategory(8i32);
pub const U_DECIMAL_DIGIT_NUMBER: UCharCategory = UCharCategory(9i32);
pub const U_LETTER_NUMBER: UCharCategory = UCharCategory(10i32);
pub const U_OTHER_NUMBER: UCharCategory = UCharCategory(11i32);
pub const U_SPACE_SEPARATOR: UCharCategory = UCharCategory(12i32);
pub const U_LINE_SEPARATOR: UCharCategory = UCharCategory(13i32);
pub const U_PARAGRAPH_SEPARATOR: UCharCategory = UCharCategory(14i32);
pub const U_CONTROL_CHAR: UCharCategory = UCharCategory(15i32);
pub const U_FORMAT_CHAR: UCharCategory = UCharCategory(16i32);
pub const U_PRIVATE_USE_CHAR: UCharCategory = UCharCategory(17i32);
pub const U_SURROGATE: UCharCategory = UCharCategory(18i32);
pub const U_DASH_PUNCTUATION: UCharCategory = UCharCategory(19i32);
pub const U_START_PUNCTUATION: UCharCategory = UCharCategory(20i32);
pub const U_END_PUNCTUATION: UCharCategory = UCharCategory(21i32);
pub const U_CONNECTOR_PUNCTUATION: UCharCategory = UCharCategory(22i32);
pub const U_OTHER_PUNCTUATION: UCharCategory = UCharCategory(23i32);
pub const U_MATH_SYMBOL: UCharCategory = UCharCategory(24i32);
pub const U_CURRENCY_SYMBOL: UCharCategory = UCharCategory(25i32);
pub const U_MODIFIER_SYMBOL: UCharCategory = UCharCategory(26i32);
pub const U_OTHER_SYMBOL: UCharCategory = UCharCategory(27i32);
pub const U_INITIAL_PUNCTUATION: UCharCategory = UCharCategory(28i32);
pub const U_FINAL_PUNCTUATION: UCharCategory = UCharCategory(29i32);
pub const U_CHAR_CATEGORY_COUNT: UCharCategory = UCharCategory(30i32);
#[repr(transparent)]
pub struct UCharDirection(pub i32);
pub const U_LEFT_TO_RIGHT: UCharDirection = UCharDirection(0i32);
pub const U_RIGHT_TO_LEFT: UCharDirection = UCharDirection(1i32);
pub const U_EUROPEAN_NUMBER: UCharDirection = UCharDirection(2i32);
pub const U_EUROPEAN_NUMBER_SEPARATOR: UCharDirection = UCharDirection(3i32);
pub const U_EUROPEAN_NUMBER_TERMINATOR: UCharDirection = UCharDirection(4i32);
pub const U_ARABIC_NUMBER: UCharDirection = UCharDirection(5i32);
pub const U_COMMON_NUMBER_SEPARATOR: UCharDirection = UCharDirection(6i32);
pub const U_BLOCK_SEPARATOR: UCharDirection = UCharDirection(7i32);
pub const U_SEGMENT_SEPARATOR: UCharDirection = UCharDirection(8i32);
pub const U_WHITE_SPACE_NEUTRAL: UCharDirection = UCharDirection(9i32);
pub const U_OTHER_NEUTRAL: UCharDirection = UCharDirection(10i32);
pub const U_LEFT_TO_RIGHT_EMBEDDING: UCharDirection = UCharDirection(11i32);
pub const U_LEFT_TO_RIGHT_OVERRIDE: UCharDirection = UCharDirection(12i32);
pub const U_RIGHT_TO_LEFT_ARABIC: UCharDirection = UCharDirection(13i32);
pub const U_RIGHT_TO_LEFT_EMBEDDING: UCharDirection = UCharDirection(14i32);
pub const U_RIGHT_TO_LEFT_OVERRIDE: UCharDirection = UCharDirection(15i32);
pub const U_POP_DIRECTIONAL_FORMAT: UCharDirection = UCharDirection(16i32);
pub const U_DIR_NON_SPACING_MARK: UCharDirection = UCharDirection(17i32);
pub const U_BOUNDARY_NEUTRAL: UCharDirection = UCharDirection(18i32);
pub const U_FIRST_STRONG_ISOLATE: UCharDirection = UCharDirection(19i32);
pub const U_LEFT_TO_RIGHT_ISOLATE: UCharDirection = UCharDirection(20i32);
pub const U_RIGHT_TO_LEFT_ISOLATE: UCharDirection = UCharDirection(21i32);
pub const U_POP_DIRECTIONAL_ISOLATE: UCharDirection = UCharDirection(22i32);
pub type UCharEnumTypeRange = unsafe extern "system" fn(context: *const ::core::ffi::c_void, start: i32, limit: i32, r#type: UCharCategory) -> i8;
#[repr(C)]
pub struct UCharIterator(i32);
pub type UCharIteratorCurrent = unsafe extern "system" fn(iter: *mut UCharIterator) -> i32;
pub type UCharIteratorGetIndex = unsafe extern "system" fn(iter: *mut UCharIterator, origin: UCharIteratorOrigin) -> i32;
pub type UCharIteratorGetState = unsafe extern "system" fn(iter: *const UCharIterator) -> u32;
pub type UCharIteratorHasNext = unsafe extern "system" fn(iter: *mut UCharIterator) -> i8;
pub type UCharIteratorHasPrevious = unsafe extern "system" fn(iter: *mut UCharIterator) -> i8;
pub type UCharIteratorMove = unsafe extern "system" fn(iter: *mut UCharIterator, delta: i32, origin: UCharIteratorOrigin) -> i32;
pub type UCharIteratorNext = unsafe extern "system" fn(iter: *mut UCharIterator) -> i32;
#[repr(transparent)]
pub struct UCharIteratorOrigin(pub i32);
pub const UITER_START: UCharIteratorOrigin = UCharIteratorOrigin(0i32);
pub const UITER_CURRENT: UCharIteratorOrigin = UCharIteratorOrigin(1i32);
pub const UITER_LIMIT: UCharIteratorOrigin = UCharIteratorOrigin(2i32);
pub const UITER_ZERO: UCharIteratorOrigin = UCharIteratorOrigin(3i32);
pub const UITER_LENGTH: UCharIteratorOrigin = UCharIteratorOrigin(4i32);
pub type UCharIteratorPrevious = unsafe extern "system" fn(iter: *mut UCharIterator) -> i32;
pub type UCharIteratorReserved = unsafe extern "system" fn(iter: *mut UCharIterator, something: i32) -> i32;
pub type UCharIteratorSetState = unsafe extern "system" fn(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode);
#[repr(transparent)]
pub struct UCharNameChoice(pub i32);
pub const U_UNICODE_CHAR_NAME: UCharNameChoice = UCharNameChoice(0i32);
pub const U_EXTENDED_CHAR_NAME: UCharNameChoice = UCharNameChoice(2i32);
pub const U_CHAR_NAME_ALIAS: UCharNameChoice = UCharNameChoice(3i32);
#[repr(C)]
pub struct UCharsetDetector(i32);
#[repr(C)]
pub struct UCharsetMatch(i32);
#[repr(transparent)]
pub struct UColAttribute(pub i32);
pub const UCOL_FRENCH_COLLATION: UColAttribute = UColAttribute(0i32);
pub const UCOL_ALTERNATE_HANDLING: UColAttribute = UColAttribute(1i32);
pub const UCOL_CASE_FIRST: UColAttribute = UColAttribute(2i32);
pub const UCOL_CASE_LEVEL: UColAttribute = UColAttribute(3i32);
pub const UCOL_NORMALIZATION_MODE: UColAttribute = UColAttribute(4i32);
pub const UCOL_DECOMPOSITION_MODE: UColAttribute = UColAttribute(4i32);
pub const UCOL_STRENGTH: UColAttribute = UColAttribute(5i32);
pub const UCOL_NUMERIC_COLLATION: UColAttribute = UColAttribute(7i32);
pub const UCOL_ATTRIBUTE_COUNT: UColAttribute = UColAttribute(8i32);
#[repr(transparent)]
pub struct UColAttributeValue(pub i32);
pub const UCOL_DEFAULT: UColAttributeValue = UColAttributeValue(-1i32);
pub const UCOL_PRIMARY: UColAttributeValue = UColAttributeValue(0i32);
pub const UCOL_SECONDARY: UColAttributeValue = UColAttributeValue(1i32);
pub const UCOL_TERTIARY: UColAttributeValue = UColAttributeValue(2i32);
pub const UCOL_DEFAULT_STRENGTH: UColAttributeValue = UColAttributeValue(2i32);
pub const UCOL_CE_STRENGTH_LIMIT: UColAttributeValue = UColAttributeValue(3i32);
pub const UCOL_QUATERNARY: UColAttributeValue = UColAttributeValue(3i32);
pub const UCOL_IDENTICAL: UColAttributeValue = UColAttributeValue(15i32);
pub const UCOL_STRENGTH_LIMIT: UColAttributeValue = UColAttributeValue(16i32);
pub const UCOL_OFF: UColAttributeValue = UColAttributeValue(16i32);
pub const UCOL_ON: UColAttributeValue = UColAttributeValue(17i32);
pub const UCOL_SHIFTED: UColAttributeValue = UColAttributeValue(20i32);
pub const UCOL_NON_IGNORABLE: UColAttributeValue = UColAttributeValue(21i32);
pub const UCOL_LOWER_FIRST: UColAttributeValue = UColAttributeValue(24i32);
pub const UCOL_UPPER_FIRST: UColAttributeValue = UColAttributeValue(25i32);
#[repr(transparent)]
pub struct UColBoundMode(pub i32);
pub const UCOL_BOUND_LOWER: UColBoundMode = UColBoundMode(0i32);
pub const UCOL_BOUND_UPPER: UColBoundMode = UColBoundMode(1i32);
pub const UCOL_BOUND_UPPER_LONG: UColBoundMode = UColBoundMode(2i32);
#[repr(transparent)]
pub struct UColReorderCode(pub i32);
pub const UCOL_REORDER_CODE_DEFAULT: UColReorderCode = UColReorderCode(-1i32);
pub const UCOL_REORDER_CODE_NONE: UColReorderCode = UColReorderCode(103i32);
pub const UCOL_REORDER_CODE_OTHERS: UColReorderCode = UColReorderCode(103i32);
pub const UCOL_REORDER_CODE_SPACE: UColReorderCode = UColReorderCode(4096i32);
pub const UCOL_REORDER_CODE_FIRST: UColReorderCode = UColReorderCode(4096i32);
pub const UCOL_REORDER_CODE_PUNCTUATION: UColReorderCode = UColReorderCode(4097i32);
pub const UCOL_REORDER_CODE_SYMBOL: UColReorderCode = UColReorderCode(4098i32);
pub const UCOL_REORDER_CODE_CURRENCY: UColReorderCode = UColReorderCode(4099i32);
pub const UCOL_REORDER_CODE_DIGIT: UColReorderCode = UColReorderCode(4100i32);
#[repr(transparent)]
pub struct UColRuleOption(pub i32);
pub const UCOL_TAILORING_ONLY: UColRuleOption = UColRuleOption(0i32);
pub const UCOL_FULL_RULES: UColRuleOption = UColRuleOption(1i32);
#[repr(C)]
pub struct UCollationElements(i32);
#[repr(transparent)]
pub struct UCollationResult(pub i32);
pub const UCOL_EQUAL: UCollationResult = UCollationResult(0i32);
pub const UCOL_GREATER: UCollationResult = UCollationResult(1i32);
pub const UCOL_LESS: UCollationResult = UCollationResult(-1i32);
#[repr(C)]
pub struct UCollator(i32);
#[repr(C)]
pub struct UConstrainedFieldPosition(i32);
#[repr(C)]
pub struct UConverter(i32);
#[repr(transparent)]
pub struct UConverterCallbackReason(pub i32);
pub const UCNV_UNASSIGNED: UConverterCallbackReason = UConverterCallbackReason(0i32);
pub const UCNV_ILLEGAL: UConverterCallbackReason = UConverterCallbackReason(1i32);
pub const UCNV_IRREGULAR: UConverterCallbackReason = UConverterCallbackReason(2i32);
pub const UCNV_RESET: UConverterCallbackReason = UConverterCallbackReason(3i32);
pub const UCNV_CLOSE: UConverterCallbackReason = UConverterCallbackReason(4i32);
pub const UCNV_CLONE: UConverterCallbackReason = UConverterCallbackReason(5i32);
#[cfg(feature = "Win32_Foundation")]
pub type UConverterFromUCallback = unsafe extern "system" fn(context: *const ::core::ffi::c_void, args: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UConverterFromUnicodeArgs(i32);
#[repr(transparent)]
pub struct UConverterPlatform(pub i32);
pub const UCNV_UNKNOWN: UConverterPlatform = UConverterPlatform(-1i32);
pub const UCNV_IBM: UConverterPlatform = UConverterPlatform(0i32);
#[repr(C)]
pub struct UConverterSelector(i32);
#[cfg(feature = "Win32_Foundation")]
pub type UConverterToUCallback = unsafe extern "system" fn(context: *const ::core::ffi::c_void, args: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UConverterToUnicodeArgs(i32);
#[repr(transparent)]
pub struct UConverterType(pub i32);
pub const UCNV_UNSUPPORTED_CONVERTER: UConverterType = UConverterType(-1i32);
pub const UCNV_SBCS: UConverterType = UConverterType(0i32);
pub const UCNV_DBCS: UConverterType = UConverterType(1i32);
pub const UCNV_MBCS: UConverterType = UConverterType(2i32);
pub const UCNV_LATIN_1: UConverterType = UConverterType(3i32);
pub const UCNV_UTF8: UConverterType = UConverterType(4i32);
pub const UCNV_UTF16_BigEndian: UConverterType = UConverterType(5i32);
pub const UCNV_UTF16_LittleEndian: UConverterType = UConverterType(6i32);
pub const UCNV_UTF32_BigEndian: UConverterType = UConverterType(7i32);
pub const UCNV_UTF32_LittleEndian: UConverterType = UConverterType(8i32);
pub const UCNV_EBCDIC_STATEFUL: UConverterType = UConverterType(9i32);
pub const UCNV_ISO_2022: UConverterType = UConverterType(10i32);
pub const UCNV_LMBCS_1: UConverterType = UConverterType(11i32);
pub const UCNV_LMBCS_2: UConverterType = UConverterType(12i32);
pub const UCNV_LMBCS_3: UConverterType = UConverterType(13i32);
pub const UCNV_LMBCS_4: UConverterType = UConverterType(14i32);
pub const UCNV_LMBCS_5: UConverterType = UConverterType(15i32);
pub const UCNV_LMBCS_6: UConverterType = UConverterType(16i32);
pub const UCNV_LMBCS_8: UConverterType = UConverterType(17i32);
pub const UCNV_LMBCS_11: UConverterType = UConverterType(18i32);
pub const UCNV_LMBCS_16: UConverterType = UConverterType(19i32);
pub const UCNV_LMBCS_17: UConverterType = UConverterType(20i32);
pub const UCNV_LMBCS_18: UConverterType = UConverterType(21i32);
pub const UCNV_LMBCS_19: UConverterType = UConverterType(22i32);
pub const UCNV_LMBCS_LAST: UConverterType = UConverterType(22i32);
pub const UCNV_HZ: UConverterType = UConverterType(23i32);
pub const UCNV_SCSU: UConverterType = UConverterType(24i32);
pub const UCNV_ISCII: UConverterType = UConverterType(25i32);
pub const UCNV_US_ASCII: UConverterType = UConverterType(26i32);
pub const UCNV_UTF7: UConverterType = UConverterType(27i32);
pub const UCNV_BOCU1: UConverterType = UConverterType(28i32);
pub const UCNV_UTF16: UConverterType = UConverterType(29i32);
pub const UCNV_UTF32: UConverterType = UConverterType(30i32);
pub const UCNV_CESU8: UConverterType = UConverterType(31i32);
pub const UCNV_IMAP_MAILBOX: UConverterType = UConverterType(32i32);
pub const UCNV_COMPOUND_TEXT: UConverterType = UConverterType(33i32);
pub const UCNV_NUMBER_OF_SUPPORTED_CONVERTER_TYPES: UConverterType = UConverterType(34i32);
#[repr(transparent)]
pub struct UConverterUnicodeSet(pub i32);
pub const UCNV_ROUNDTRIP_SET: UConverterUnicodeSet = UConverterUnicodeSet(0i32);
pub const UCNV_ROUNDTRIP_AND_FALLBACK_SET: UConverterUnicodeSet = UConverterUnicodeSet(1i32);
#[repr(transparent)]
pub struct UCurrCurrencyType(pub i32);
pub const UCURR_ALL: UCurrCurrencyType = UCurrCurrencyType(2147483647i32);
pub const UCURR_COMMON: UCurrCurrencyType = UCurrCurrencyType(1i32);
pub const UCURR_UNCOMMON: UCurrCurrencyType = UCurrCurrencyType(2i32);
pub const UCURR_DEPRECATED: UCurrCurrencyType = UCurrCurrencyType(4i32);
pub const UCURR_NON_DEPRECATED: UCurrCurrencyType = UCurrCurrencyType(8i32);
#[repr(transparent)]
pub struct UCurrNameStyle(pub i32);
pub const UCURR_SYMBOL_NAME: UCurrNameStyle = UCurrNameStyle(0i32);
pub const UCURR_LONG_NAME: UCurrNameStyle = UCurrNameStyle(1i32);
pub const UCURR_NARROW_SYMBOL_NAME: UCurrNameStyle = UCurrNameStyle(2i32);
#[repr(transparent)]
pub struct UCurrencySpacing(pub i32);
pub const UNUM_CURRENCY_MATCH: UCurrencySpacing = UCurrencySpacing(0i32);
pub const UNUM_CURRENCY_SURROUNDING_MATCH: UCurrencySpacing = UCurrencySpacing(1i32);
pub const UNUM_CURRENCY_INSERT: UCurrencySpacing = UCurrencySpacing(2i32);
pub const UNUM_CURRENCY_SPACING_COUNT: UCurrencySpacing = UCurrencySpacing(3i32);
#[repr(transparent)]
pub struct UCurrencyUsage(pub i32);
pub const UCURR_USAGE_STANDARD: UCurrencyUsage = UCurrencyUsage(0i32);
pub const UCURR_USAGE_CASH: UCurrencyUsage = UCurrencyUsage(1i32);
#[repr(transparent)]
pub struct UDateAbsoluteUnit(pub i32);
pub const UDAT_ABSOLUTE_SUNDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(0i32);
pub const UDAT_ABSOLUTE_MONDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(1i32);
pub const UDAT_ABSOLUTE_TUESDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(2i32);
pub const UDAT_ABSOLUTE_WEDNESDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(3i32);
pub const UDAT_ABSOLUTE_THURSDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(4i32);
pub const UDAT_ABSOLUTE_FRIDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(5i32);
pub const UDAT_ABSOLUTE_SATURDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(6i32);
pub const UDAT_ABSOLUTE_DAY: UDateAbsoluteUnit = UDateAbsoluteUnit(7i32);
pub const UDAT_ABSOLUTE_WEEK: UDateAbsoluteUnit = UDateAbsoluteUnit(8i32);
pub const UDAT_ABSOLUTE_MONTH: UDateAbsoluteUnit = UDateAbsoluteUnit(9i32);
pub const UDAT_ABSOLUTE_YEAR: UDateAbsoluteUnit = UDateAbsoluteUnit(10i32);
pub const UDAT_ABSOLUTE_NOW: UDateAbsoluteUnit = UDateAbsoluteUnit(11i32);
pub const UDAT_ABSOLUTE_UNIT_COUNT: UDateAbsoluteUnit = UDateAbsoluteUnit(12i32);
#[repr(transparent)]
pub struct UDateDirection(pub i32);
pub const UDAT_DIRECTION_LAST_2: UDateDirection = UDateDirection(0i32);
pub const UDAT_DIRECTION_LAST: UDateDirection = UDateDirection(1i32);
pub const UDAT_DIRECTION_THIS: UDateDirection = UDateDirection(2i32);
pub const UDAT_DIRECTION_NEXT: UDateDirection = UDateDirection(3i32);
pub const UDAT_DIRECTION_NEXT_2: UDateDirection = UDateDirection(4i32);
pub const UDAT_DIRECTION_PLAIN: UDateDirection = UDateDirection(5i32);
pub const UDAT_DIRECTION_COUNT: UDateDirection = UDateDirection(6i32);
#[repr(transparent)]
pub struct UDateFormatBooleanAttribute(pub i32);
pub const UDAT_PARSE_ALLOW_WHITESPACE: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(0i32);
pub const UDAT_PARSE_ALLOW_NUMERIC: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(1i32);
pub const UDAT_PARSE_PARTIAL_LITERAL_MATCH: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(2i32);
pub const UDAT_PARSE_MULTIPLE_PATTERNS_FOR_MATCH: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(3i32);
pub const UDAT_BOOLEAN_ATTRIBUTE_COUNT: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(4i32);
#[repr(transparent)]
pub struct UDateFormatField(pub i32);
pub const UDAT_ERA_FIELD: UDateFormatField = UDateFormatField(0i32);
pub const UDAT_YEAR_FIELD: UDateFormatField = UDateFormatField(1i32);
pub const UDAT_MONTH_FIELD: UDateFormatField = UDateFormatField(2i32);
pub const UDAT_DATE_FIELD: UDateFormatField = UDateFormatField(3i32);
pub const UDAT_HOUR_OF_DAY1_FIELD: UDateFormatField = UDateFormatField(4i32);
pub const UDAT_HOUR_OF_DAY0_FIELD: UDateFormatField = UDateFormatField(5i32);
pub const UDAT_MINUTE_FIELD: UDateFormatField = UDateFormatField(6i32);
pub const UDAT_SECOND_FIELD: UDateFormatField = UDateFormatField(7i32);
pub const UDAT_FRACTIONAL_SECOND_FIELD: UDateFormatField = UDateFormatField(8i32);
pub const UDAT_DAY_OF_WEEK_FIELD: UDateFormatField = UDateFormatField(9i32);
pub const UDAT_DAY_OF_YEAR_FIELD: UDateFormatField = UDateFormatField(10i32);
pub const UDAT_DAY_OF_WEEK_IN_MONTH_FIELD: UDateFormatField = UDateFormatField(11i32);
pub const UDAT_WEEK_OF_YEAR_FIELD: UDateFormatField = UDateFormatField(12i32);
pub const UDAT_WEEK_OF_MONTH_FIELD: UDateFormatField = UDateFormatField(13i32);
pub const UDAT_AM_PM_FIELD: UDateFormatField = UDateFormatField(14i32);
pub const UDAT_HOUR1_FIELD: UDateFormatField = UDateFormatField(15i32);
pub const UDAT_HOUR0_FIELD: UDateFormatField = UDateFormatField(16i32);
pub const UDAT_TIMEZONE_FIELD: UDateFormatField = UDateFormatField(17i32);
pub const UDAT_YEAR_WOY_FIELD: UDateFormatField = UDateFormatField(18i32);
pub const UDAT_DOW_LOCAL_FIELD: UDateFormatField = UDateFormatField(19i32);
pub const UDAT_EXTENDED_YEAR_FIELD: UDateFormatField = UDateFormatField(20i32);
pub const UDAT_JULIAN_DAY_FIELD: UDateFormatField = UDateFormatField(21i32);
pub const UDAT_MILLISECONDS_IN_DAY_FIELD: UDateFormatField = UDateFormatField(22i32);
pub const UDAT_TIMEZONE_RFC_FIELD: UDateFormatField = UDateFormatField(23i32);
pub const UDAT_TIMEZONE_GENERIC_FIELD: UDateFormatField = UDateFormatField(24i32);
pub const UDAT_STANDALONE_DAY_FIELD: UDateFormatField = UDateFormatField(25i32);
pub const UDAT_STANDALONE_MONTH_FIELD: UDateFormatField = UDateFormatField(26i32);
pub const UDAT_QUARTER_FIELD: UDateFormatField = UDateFormatField(27i32);
pub const UDAT_STANDALONE_QUARTER_FIELD: UDateFormatField = UDateFormatField(28i32);
pub const UDAT_TIMEZONE_SPECIAL_FIELD: UDateFormatField = UDateFormatField(29i32);
pub const UDAT_YEAR_NAME_FIELD: UDateFormatField = UDateFormatField(30i32);
pub const UDAT_TIMEZONE_LOCALIZED_GMT_OFFSET_FIELD: UDateFormatField = UDateFormatField(31i32);
pub const UDAT_TIMEZONE_ISO_FIELD: UDateFormatField = UDateFormatField(32i32);
pub const UDAT_TIMEZONE_ISO_LOCAL_FIELD: UDateFormatField = UDateFormatField(33i32);
pub const UDAT_AM_PM_MIDNIGHT_NOON_FIELD: UDateFormatField = UDateFormatField(35i32);
pub const UDAT_FLEXIBLE_DAY_PERIOD_FIELD: UDateFormatField = UDateFormatField(36i32);
#[repr(transparent)]
pub struct UDateFormatStyle(pub i32);
pub const UDAT_FULL: UDateFormatStyle = UDateFormatStyle(0i32);
pub const UDAT_LONG: UDateFormatStyle = UDateFormatStyle(1i32);
pub const UDAT_MEDIUM: UDateFormatStyle = UDateFormatStyle(2i32);
pub const UDAT_SHORT: UDateFormatStyle = UDateFormatStyle(3i32);
pub const UDAT_DEFAULT: UDateFormatStyle = UDateFormatStyle(2i32);
pub const UDAT_RELATIVE: UDateFormatStyle = UDateFormatStyle(128i32);
pub const UDAT_FULL_RELATIVE: UDateFormatStyle = UDateFormatStyle(128i32);
pub const UDAT_LONG_RELATIVE: UDateFormatStyle = UDateFormatStyle(129i32);
pub const UDAT_MEDIUM_RELATIVE: UDateFormatStyle = UDateFormatStyle(130i32);
pub const UDAT_SHORT_RELATIVE: UDateFormatStyle = UDateFormatStyle(131i32);
pub const UDAT_NONE: UDateFormatStyle = UDateFormatStyle(-1i32);
pub const UDAT_PATTERN: UDateFormatStyle = UDateFormatStyle(-2i32);
#[repr(transparent)]
pub struct UDateFormatSymbolType(pub i32);
pub const UDAT_ERAS: UDateFormatSymbolType = UDateFormatSymbolType(0i32);
pub const UDAT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(1i32);
pub const UDAT_SHORT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(2i32);
pub const UDAT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(3i32);
pub const UDAT_SHORT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(4i32);
pub const UDAT_AM_PMS: UDateFormatSymbolType = UDateFormatSymbolType(5i32);
pub const UDAT_LOCALIZED_CHARS: UDateFormatSymbolType = UDateFormatSymbolType(6i32);
pub const UDAT_ERA_NAMES: UDateFormatSymbolType = UDateFormatSymbolType(7i32);
pub const UDAT_NARROW_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(8i32);
pub const UDAT_NARROW_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(9i32);
pub const UDAT_STANDALONE_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(10i32);
pub const UDAT_STANDALONE_SHORT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(11i32);
pub const UDAT_STANDALONE_NARROW_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(12i32);
pub const UDAT_STANDALONE_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(13i32);
pub const UDAT_STANDALONE_SHORT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(14i32);
pub const UDAT_STANDALONE_NARROW_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(15i32);
pub const UDAT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(16i32);
pub const UDAT_SHORT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(17i32);
pub const UDAT_STANDALONE_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(18i32);
pub const UDAT_STANDALONE_SHORT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(19i32);
pub const UDAT_SHORTER_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(20i32);
pub const UDAT_STANDALONE_SHORTER_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(21i32);
pub const UDAT_CYCLIC_YEARS_WIDE: UDateFormatSymbolType = UDateFormatSymbolType(22i32);
pub const UDAT_CYCLIC_YEARS_ABBREVIATED: UDateFormatSymbolType = UDateFormatSymbolType(23i32);
pub const UDAT_CYCLIC_YEARS_NARROW: UDateFormatSymbolType = UDateFormatSymbolType(24i32);
pub const UDAT_ZODIAC_NAMES_WIDE: UDateFormatSymbolType = UDateFormatSymbolType(25i32);
pub const UDAT_ZODIAC_NAMES_ABBREVIATED: UDateFormatSymbolType = UDateFormatSymbolType(26i32);
pub const UDAT_ZODIAC_NAMES_NARROW: UDateFormatSymbolType = UDateFormatSymbolType(27i32);
#[repr(C)]
pub struct UDateFormatSymbols(i32);
#[repr(C)]
pub struct UDateIntervalFormat(i32);
#[repr(transparent)]
pub struct UDateRelativeDateTimeFormatterStyle(pub i32);
pub const UDAT_STYLE_LONG: UDateRelativeDateTimeFormatterStyle = UDateRelativeDateTimeFormatterStyle(0i32);
pub const UDAT_STYLE_SHORT: UDateRelativeDateTimeFormatterStyle = UDateRelativeDateTimeFormatterStyle(1i32);
pub const UDAT_STYLE_NARROW: UDateRelativeDateTimeFormatterStyle = UDateRelativeDateTimeFormatterStyle(2i32);
#[repr(transparent)]
pub struct UDateRelativeUnit(pub i32);
pub const UDAT_RELATIVE_SECONDS: UDateRelativeUnit = UDateRelativeUnit(0i32);
pub const UDAT_RELATIVE_MINUTES: UDateRelativeUnit = UDateRelativeUnit(1i32);
pub const UDAT_RELATIVE_HOURS: UDateRelativeUnit = UDateRelativeUnit(2i32);
pub const UDAT_RELATIVE_DAYS: UDateRelativeUnit = UDateRelativeUnit(3i32);
pub const UDAT_RELATIVE_WEEKS: UDateRelativeUnit = UDateRelativeUnit(4i32);
pub const UDAT_RELATIVE_MONTHS: UDateRelativeUnit = UDateRelativeUnit(5i32);
pub const UDAT_RELATIVE_YEARS: UDateRelativeUnit = UDateRelativeUnit(6i32);
pub const UDAT_RELATIVE_UNIT_COUNT: UDateRelativeUnit = UDateRelativeUnit(7i32);
#[repr(transparent)]
pub struct UDateTimePGDisplayWidth(pub i32);
pub const UDATPG_WIDE: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(0i32);
pub const UDATPG_ABBREVIATED: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(1i32);
pub const UDATPG_NARROW: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(2i32);
#[repr(transparent)]
pub struct UDateTimePatternConflict(pub i32);
pub const UDATPG_NO_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(0i32);
pub const UDATPG_BASE_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(1i32);
pub const UDATPG_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(2i32);
#[repr(transparent)]
pub struct UDateTimePatternField(pub i32);
pub const UDATPG_ERA_FIELD: UDateTimePatternField = UDateTimePatternField(0i32);
pub const UDATPG_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(1i32);
pub const UDATPG_QUARTER_FIELD: UDateTimePatternField = UDateTimePatternField(2i32);
pub const UDATPG_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(3i32);
pub const UDATPG_WEEK_OF_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(4i32);
pub const UDATPG_WEEK_OF_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(5i32);
pub const UDATPG_WEEKDAY_FIELD: UDateTimePatternField = UDateTimePatternField(6i32);
pub const UDATPG_DAY_OF_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(7i32);
pub const UDATPG_DAY_OF_WEEK_IN_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(8i32);
pub const UDATPG_DAY_FIELD: UDateTimePatternField = UDateTimePatternField(9i32);
pub const UDATPG_DAYPERIOD_FIELD: UDateTimePatternField = UDateTimePatternField(10i32);
pub const UDATPG_HOUR_FIELD: UDateTimePatternField = UDateTimePatternField(11i32);
pub const UDATPG_MINUTE_FIELD: UDateTimePatternField = UDateTimePatternField(12i32);
pub const UDATPG_SECOND_FIELD: UDateTimePatternField = UDateTimePatternField(13i32);
pub const UDATPG_FRACTIONAL_SECOND_FIELD: UDateTimePatternField = UDateTimePatternField(14i32);
pub const UDATPG_ZONE_FIELD: UDateTimePatternField = UDateTimePatternField(15i32);
pub const UDATPG_FIELD_COUNT: UDateTimePatternField = UDateTimePatternField(16i32);
#[repr(transparent)]
pub struct UDateTimePatternMatchOptions(pub i32);
pub const UDATPG_MATCH_NO_OPTIONS: UDateTimePatternMatchOptions = UDateTimePatternMatchOptions(0i32);
pub const UDATPG_MATCH_HOUR_FIELD_LENGTH: UDateTimePatternMatchOptions = UDateTimePatternMatchOptions(2048i32);
pub const UDATPG_MATCH_ALL_FIELDS_LENGTH: UDateTimePatternMatchOptions = UDateTimePatternMatchOptions(65535i32);
#[repr(transparent)]
pub struct UDateTimeScale(pub i32);
pub const UDTS_JAVA_TIME: UDateTimeScale = UDateTimeScale(0i32);
pub const UDTS_UNIX_TIME: UDateTimeScale = UDateTimeScale(1i32);
pub const UDTS_ICU4C_TIME: UDateTimeScale = UDateTimeScale(2i32);
pub const UDTS_WINDOWS_FILE_TIME: UDateTimeScale = UDateTimeScale(3i32);
pub const UDTS_DOTNET_DATE_TIME: UDateTimeScale = UDateTimeScale(4i32);
pub const UDTS_MAC_OLD_TIME: UDateTimeScale = UDateTimeScale(5i32);
pub const UDTS_MAC_TIME: UDateTimeScale = UDateTimeScale(6i32);
pub const UDTS_EXCEL_TIME: UDateTimeScale = UDateTimeScale(7i32);
pub const UDTS_DB2_TIME: UDateTimeScale = UDateTimeScale(8i32);
pub const UDTS_UNIX_MICROSECONDS_TIME: UDateTimeScale = UDateTimeScale(9i32);
#[repr(transparent)]
pub struct UDecompositionType(pub i32);
pub const U_DT_NONE: UDecompositionType = UDecompositionType(0i32);
pub const U_DT_CANONICAL: UDecompositionType = UDecompositionType(1i32);
pub const U_DT_COMPAT: UDecompositionType = UDecompositionType(2i32);
pub const U_DT_CIRCLE: UDecompositionType = UDecompositionType(3i32);
pub const U_DT_FINAL: UDecompositionType = UDecompositionType(4i32);
pub const U_DT_FONT: UDecompositionType = UDecompositionType(5i32);
pub const U_DT_FRACTION: UDecompositionType = UDecompositionType(6i32);
pub const U_DT_INITIAL: UDecompositionType = UDecompositionType(7i32);
pub const U_DT_ISOLATED: UDecompositionType = UDecompositionType(8i32);
pub const U_DT_MEDIAL: UDecompositionType = UDecompositionType(9i32);
pub const U_DT_NARROW: UDecompositionType = UDecompositionType(10i32);
pub const U_DT_NOBREAK: UDecompositionType = UDecompositionType(11i32);
pub const U_DT_SMALL: UDecompositionType = UDecompositionType(12i32);
pub const U_DT_SQUARE: UDecompositionType = UDecompositionType(13i32);
pub const U_DT_SUB: UDecompositionType = UDecompositionType(14i32);
pub const U_DT_SUPER: UDecompositionType = UDecompositionType(15i32);
pub const U_DT_VERTICAL: UDecompositionType = UDecompositionType(16i32);
pub const U_DT_WIDE: UDecompositionType = UDecompositionType(17i32);
#[repr(transparent)]
pub struct UDialectHandling(pub i32);
pub const ULDN_STANDARD_NAMES: UDialectHandling = UDialectHandling(0i32);
pub const ULDN_DIALECT_NAMES: UDialectHandling = UDialectHandling(1i32);
#[repr(transparent)]
pub struct UDisplayContext(pub i32);
pub const UDISPCTX_STANDARD_NAMES: UDisplayContext = UDisplayContext(0i32);
pub const UDISPCTX_DIALECT_NAMES: UDisplayContext = UDisplayContext(1i32);
pub const UDISPCTX_CAPITALIZATION_NONE: UDisplayContext = UDisplayContext(256i32);
pub const UDISPCTX_CAPITALIZATION_FOR_MIDDLE_OF_SENTENCE: UDisplayContext = UDisplayContext(257i32);
pub const UDISPCTX_CAPITALIZATION_FOR_BEGINNING_OF_SENTENCE: UDisplayContext = UDisplayContext(258i32);
pub const UDISPCTX_CAPITALIZATION_FOR_UI_LIST_OR_MENU: UDisplayContext = UDisplayContext(259i32);
pub const UDISPCTX_CAPITALIZATION_FOR_STANDALONE: UDisplayContext = UDisplayContext(260i32);
pub const UDISPCTX_LENGTH_FULL: UDisplayContext = UDisplayContext(512i32);
pub const UDISPCTX_LENGTH_SHORT: UDisplayContext = UDisplayContext(513i32);
pub const UDISPCTX_SUBSTITUTE: UDisplayContext = UDisplayContext(768i32);
pub const UDISPCTX_NO_SUBSTITUTE: UDisplayContext = UDisplayContext(769i32);
#[repr(transparent)]
pub struct UDisplayContextType(pub i32);
pub const UDISPCTX_TYPE_DIALECT_HANDLING: UDisplayContextType = UDisplayContextType(0i32);
pub const UDISPCTX_TYPE_CAPITALIZATION: UDisplayContextType = UDisplayContextType(1i32);
pub const UDISPCTX_TYPE_DISPLAY_LENGTH: UDisplayContextType = UDisplayContextType(2i32);
pub const UDISPCTX_TYPE_SUBSTITUTE_HANDLING: UDisplayContextType = UDisplayContextType(3i32);
#[repr(transparent)]
pub struct UEastAsianWidth(pub i32);
pub const U_EA_NEUTRAL: UEastAsianWidth = UEastAsianWidth(0i32);
pub const U_EA_AMBIGUOUS: UEastAsianWidth = UEastAsianWidth(1i32);
pub const U_EA_HALFWIDTH: UEastAsianWidth = UEastAsianWidth(2i32);
pub const U_EA_FULLWIDTH: UEastAsianWidth = UEastAsianWidth(3i32);
pub const U_EA_NARROW: UEastAsianWidth = UEastAsianWidth(4i32);
pub const U_EA_WIDE: UEastAsianWidth = UEastAsianWidth(5i32);
#[cfg(feature = "Win32_Foundation")]
pub type UEnumCharNamesFn = unsafe extern "system" fn(context: *mut ::core::ffi::c_void, code: i32, namechoice: UCharNameChoice, name: super::Foundation::PSTR, length: i32) -> i8;
#[repr(C)]
pub struct UEnumeration(i32);
#[repr(transparent)]
pub struct UErrorCode(pub i32);
pub const U_USING_FALLBACK_WARNING: UErrorCode = UErrorCode(-128i32);
pub const U_ERROR_WARNING_START: UErrorCode = UErrorCode(-128i32);
pub const U_USING_DEFAULT_WARNING: UErrorCode = UErrorCode(-127i32);
pub const U_SAFECLONE_ALLOCATED_WARNING: UErrorCode = UErrorCode(-126i32);
pub const U_STATE_OLD_WARNING: UErrorCode = UErrorCode(-125i32);
pub const U_STRING_NOT_TERMINATED_WARNING: UErrorCode = UErrorCode(-124i32);
pub const U_SORT_KEY_TOO_SHORT_WARNING: UErrorCode = UErrorCode(-123i32);
pub const U_AMBIGUOUS_ALIAS_WARNING: UErrorCode = UErrorCode(-122i32);
pub const U_DIFFERENT_UCA_VERSION: UErrorCode = UErrorCode(-121i32);
pub const U_PLUGIN_CHANGED_LEVEL_WARNING: UErrorCode = UErrorCode(-120i32);
pub const U_ZERO_ERROR: UErrorCode = UErrorCode(0i32);
pub const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = UErrorCode(1i32);
pub const U_MISSING_RESOURCE_ERROR: UErrorCode = UErrorCode(2i32);
pub const U_INVALID_FORMAT_ERROR: UErrorCode = UErrorCode(3i32);
pub const U_FILE_ACCESS_ERROR: UErrorCode = UErrorCode(4i32);
pub const U_INTERNAL_PROGRAM_ERROR: UErrorCode = UErrorCode(5i32);
pub const U_MESSAGE_PARSE_ERROR: UErrorCode = UErrorCode(6i32);
pub const U_MEMORY_ALLOCATION_ERROR: UErrorCode = UErrorCode(7i32);
pub const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = UErrorCode(8i32);
pub const U_PARSE_ERROR: UErrorCode = UErrorCode(9i32);
pub const U_INVALID_CHAR_FOUND: UErrorCode = UErrorCode(10i32);
pub const U_TRUNCATED_CHAR_FOUND: UErrorCode = UErrorCode(11i32);
pub const U_ILLEGAL_CHAR_FOUND: UErrorCode = UErrorCode(12i32);
pub const U_INVALID_TABLE_FORMAT: UErrorCode = UErrorCode(13i32);
pub const U_INVALID_TABLE_FILE: UErrorCode = UErrorCode(14i32);
pub const U_BUFFER_OVERFLOW_ERROR: UErrorCode = UErrorCode(15i32);
pub const U_UNSUPPORTED_ERROR: UErrorCode = UErrorCode(16i32);
pub const U_RESOURCE_TYPE_MISMATCH: UErrorCode = UErrorCode(17i32);
pub const U_ILLEGAL_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(18i32);
pub const U_UNSUPPORTED_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(19i32);
pub const U_NO_SPACE_AVAILABLE: UErrorCode = UErrorCode(20i32);
pub const U_CE_NOT_FOUND_ERROR: UErrorCode = UErrorCode(21i32);
pub const U_PRIMARY_TOO_LONG_ERROR: UErrorCode = UErrorCode(22i32);
pub const U_STATE_TOO_OLD_ERROR: UErrorCode = UErrorCode(23i32);
pub const U_TOO_MANY_ALIASES_ERROR: UErrorCode = UErrorCode(24i32);
pub const U_ENUM_OUT_OF_SYNC_ERROR: UErrorCode = UErrorCode(25i32);
pub const U_INVARIANT_CONVERSION_ERROR: UErrorCode = UErrorCode(26i32);
pub const U_INVALID_STATE_ERROR: UErrorCode = UErrorCode(27i32);
pub const U_COLLATOR_VERSION_MISMATCH: UErrorCode = UErrorCode(28i32);
pub const U_USELESS_COLLATOR_ERROR: UErrorCode = UErrorCode(29i32);
pub const U_NO_WRITE_PERMISSION: UErrorCode = UErrorCode(30i32);
pub const U_BAD_VARIABLE_DEFINITION: UErrorCode = UErrorCode(65536i32);
pub const U_PARSE_ERROR_START: UErrorCode = UErrorCode(65536i32);
pub const U_MALFORMED_RULE: UErrorCode = UErrorCode(65537i32);
pub const U_MALFORMED_SET: UErrorCode = UErrorCode(65538i32);
pub const U_MALFORMED_SYMBOL_REFERENCE: UErrorCode = UErrorCode(65539i32);
pub const U_MALFORMED_UNICODE_ESCAPE: UErrorCode = UErrorCode(65540i32);
pub const U_MALFORMED_VARIABLE_DEFINITION: UErrorCode = UErrorCode(65541i32);
pub const U_MALFORMED_VARIABLE_REFERENCE: UErrorCode = UErrorCode(65542i32);
pub const U_MISMATCHED_SEGMENT_DELIMITERS: UErrorCode = UErrorCode(65543i32);
pub const U_MISPLACED_ANCHOR_START: UErrorCode = UErrorCode(65544i32);
pub const U_MISPLACED_CURSOR_OFFSET: UErrorCode = UErrorCode(65545i32);
pub const U_MISPLACED_QUANTIFIER: UErrorCode = UErrorCode(65546i32);
pub const U_MISSING_OPERATOR: UErrorCode = UErrorCode(65547i32);
pub const U_MISSING_SEGMENT_CLOSE: UErrorCode = UErrorCode(65548i32);
pub const U_MULTIPLE_ANTE_CONTEXTS: UErrorCode = UErrorCode(65549i32);
pub const U_MULTIPLE_CURSORS: UErrorCode = UErrorCode(65550i32);
pub const U_MULTIPLE_POST_CONTEXTS: UErrorCode = UErrorCode(65551i32);
pub const U_TRAILING_BACKSLASH: UErrorCode = UErrorCode(65552i32);
pub const U_UNDEFINED_SEGMENT_REFERENCE: UErrorCode = UErrorCode(65553i32);
pub const U_UNDEFINED_VARIABLE: UErrorCode = UErrorCode(65554i32);
pub const U_UNQUOTED_SPECIAL: UErrorCode = UErrorCode(65555i32);
pub const U_UNTERMINATED_QUOTE: UErrorCode = UErrorCode(65556i32);
pub const U_RULE_MASK_ERROR: UErrorCode = UErrorCode(65557i32);
pub const U_MISPLACED_COMPOUND_FILTER: UErrorCode = UErrorCode(65558i32);
pub const U_MULTIPLE_COMPOUND_FILTERS: UErrorCode = UErrorCode(65559i32);
pub const U_INVALID_RBT_SYNTAX: UErrorCode = UErrorCode(65560i32);
pub const U_INVALID_PROPERTY_PATTERN: UErrorCode = UErrorCode(65561i32);
pub const U_MALFORMED_PRAGMA: UErrorCode = UErrorCode(65562i32);
pub const U_UNCLOSED_SEGMENT: UErrorCode = UErrorCode(65563i32);
pub const U_ILLEGAL_CHAR_IN_SEGMENT: UErrorCode = UErrorCode(65564i32);
pub const U_VARIABLE_RANGE_EXHAUSTED: UErrorCode = UErrorCode(65565i32);
pub const U_VARIABLE_RANGE_OVERLAP: UErrorCode = UErrorCode(65566i32);
pub const U_ILLEGAL_CHARACTER: UErrorCode = UErrorCode(65567i32);
pub const U_INTERNAL_TRANSLITERATOR_ERROR: UErrorCode = UErrorCode(65568i32);
pub const U_INVALID_ID: UErrorCode = UErrorCode(65569i32);
pub const U_INVALID_FUNCTION: UErrorCode = UErrorCode(65570i32);
pub const U_UNEXPECTED_TOKEN: UErrorCode = UErrorCode(65792i32);
pub const U_FMT_PARSE_ERROR_START: UErrorCode = UErrorCode(65792i32);
pub const U_MULTIPLE_DECIMAL_SEPARATORS: UErrorCode = UErrorCode(65793i32);
pub const U_MULTIPLE_DECIMAL_SEPERATORS: UErrorCode = UErrorCode(65793i32);
pub const U_MULTIPLE_EXPONENTIAL_SYMBOLS: UErrorCode = UErrorCode(65794i32);
pub const U_MALFORMED_EXPONENTIAL_PATTERN: UErrorCode = UErrorCode(65795i32);
pub const U_MULTIPLE_PERCENT_SYMBOLS: UErrorCode = UErrorCode(65796i32);
pub const U_MULTIPLE_PERMILL_SYMBOLS: UErrorCode = UErrorCode(65797i32);
pub const U_MULTIPLE_PAD_SPECIFIERS: UErrorCode = UErrorCode(65798i32);
pub const U_PATTERN_SYNTAX_ERROR: UErrorCode = UErrorCode(65799i32);
pub const U_ILLEGAL_PAD_POSITION: UErrorCode = UErrorCode(65800i32);
pub const U_UNMATCHED_BRACES: UErrorCode = UErrorCode(65801i32);
pub const U_UNSUPPORTED_PROPERTY: UErrorCode = UErrorCode(65802i32);
pub const U_UNSUPPORTED_ATTRIBUTE: UErrorCode = UErrorCode(65803i32);
pub const U_ARGUMENT_TYPE_MISMATCH: UErrorCode = UErrorCode(65804i32);
pub const U_DUPLICATE_KEYWORD: UErrorCode = UErrorCode(65805i32);
pub const U_UNDEFINED_KEYWORD: UErrorCode = UErrorCode(65806i32);
pub const U_DEFAULT_KEYWORD_MISSING: UErrorCode = UErrorCode(65807i32);
pub const U_DECIMAL_NUMBER_SYNTAX_ERROR: UErrorCode = UErrorCode(65808i32);
pub const U_FORMAT_INEXACT_ERROR: UErrorCode = UErrorCode(65809i32);
pub const U_NUMBER_ARG_OUTOFBOUNDS_ERROR: UErrorCode = UErrorCode(65810i32);
pub const U_NUMBER_SKELETON_SYNTAX_ERROR: UErrorCode = UErrorCode(65811i32);
pub const U_BRK_INTERNAL_ERROR: UErrorCode = UErrorCode(66048i32);
pub const U_BRK_ERROR_START: UErrorCode = UErrorCode(66048i32);
pub const U_BRK_HEX_DIGITS_EXPECTED: UErrorCode = UErrorCode(66049i32);
pub const U_BRK_SEMICOLON_EXPECTED: UErrorCode = UErrorCode(66050i32);
pub const U_BRK_RULE_SYNTAX: UErrorCode = UErrorCode(66051i32);
pub const U_BRK_UNCLOSED_SET: UErrorCode = UErrorCode(66052i32);
pub const U_BRK_ASSIGN_ERROR: UErrorCode = UErrorCode(66053i32);
pub const U_BRK_VARIABLE_REDFINITION: UErrorCode = UErrorCode(66054i32);
pub const U_BRK_MISMATCHED_PAREN: UErrorCode = UErrorCode(66055i32);
pub const U_BRK_NEW_LINE_IN_QUOTED_STRING: UErrorCode = UErrorCode(66056i32);
pub const U_BRK_UNDEFINED_VARIABLE: UErrorCode = UErrorCode(66057i32);
pub const U_BRK_INIT_ERROR: UErrorCode = UErrorCode(66058i32);
pub const U_BRK_RULE_EMPTY_SET: UErrorCode = UErrorCode(66059i32);
pub const U_BRK_UNRECOGNIZED_OPTION: UErrorCode = UErrorCode(66060i32);
pub const U_BRK_MALFORMED_RULE_TAG: UErrorCode = UErrorCode(66061i32);
pub const U_REGEX_INTERNAL_ERROR: UErrorCode = UErrorCode(66304i32);
pub const U_REGEX_ERROR_START: UErrorCode = UErrorCode(66304i32);
pub const U_REGEX_RULE_SYNTAX: UErrorCode = UErrorCode(66305i32);
pub const U_REGEX_INVALID_STATE: UErrorCode = UErrorCode(66306i32);
pub const U_REGEX_BAD_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(66307i32);
pub const U_REGEX_PROPERTY_SYNTAX: UErrorCode = UErrorCode(66308i32);
pub const U_REGEX_UNIMPLEMENTED: UErrorCode = UErrorCode(66309i32);
pub const U_REGEX_MISMATCHED_PAREN: UErrorCode = UErrorCode(66310i32);
pub const U_REGEX_NUMBER_TOO_BIG: UErrorCode = UErrorCode(66311i32);
pub const U_REGEX_BAD_INTERVAL: UErrorCode = UErrorCode(66312i32);
pub const U_REGEX_MAX_LT_MIN: UErrorCode = UErrorCode(66313i32);
pub const U_REGEX_INVALID_BACK_REF: UErrorCode = UErrorCode(66314i32);
pub const U_REGEX_INVALID_FLAG: UErrorCode = UErrorCode(66315i32);
pub const U_REGEX_LOOK_BEHIND_LIMIT: UErrorCode = UErrorCode(66316i32);
pub const U_REGEX_SET_CONTAINS_STRING: UErrorCode = UErrorCode(66317i32);
pub const U_REGEX_MISSING_CLOSE_BRACKET: UErrorCode = UErrorCode(66319i32);
pub const U_REGEX_INVALID_RANGE: UErrorCode = UErrorCode(66320i32);
pub const U_REGEX_STACK_OVERFLOW: UErrorCode = UErrorCode(66321i32);
pub const U_REGEX_TIME_OUT: UErrorCode = UErrorCode(66322i32);
pub const U_REGEX_STOPPED_BY_CALLER: UErrorCode = UErrorCode(66323i32);
pub const U_REGEX_PATTERN_TOO_BIG: UErrorCode = UErrorCode(66324i32);
pub const U_REGEX_INVALID_CAPTURE_GROUP_NAME: UErrorCode = UErrorCode(66325i32);
pub const U_IDNA_PROHIBITED_ERROR: UErrorCode = UErrorCode(66560i32);
pub const U_IDNA_ERROR_START: UErrorCode = UErrorCode(66560i32);
pub const U_IDNA_UNASSIGNED_ERROR: UErrorCode = UErrorCode(66561i32);
pub const U_IDNA_CHECK_BIDI_ERROR: UErrorCode = UErrorCode(66562i32);
pub const U_IDNA_STD3_ASCII_RULES_ERROR: UErrorCode = UErrorCode(66563i32);
pub const U_IDNA_ACE_PREFIX_ERROR: UErrorCode = UErrorCode(66564i32);
pub const U_IDNA_VERIFICATION_ERROR: UErrorCode = UErrorCode(66565i32);
pub const U_IDNA_LABEL_TOO_LONG_ERROR: UErrorCode = UErrorCode(66566i32);
pub const U_IDNA_ZERO_LENGTH_LABEL_ERROR: UErrorCode = UErrorCode(66567i32);
pub const U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR: UErrorCode = UErrorCode(66568i32);
pub const U_STRINGPREP_PROHIBITED_ERROR: UErrorCode = UErrorCode(66560i32);
pub const U_STRINGPREP_UNASSIGNED_ERROR: UErrorCode = UErrorCode(66561i32);
pub const U_STRINGPREP_CHECK_BIDI_ERROR: UErrorCode = UErrorCode(66562i32);
pub const U_PLUGIN_ERROR_START: UErrorCode = UErrorCode(66816i32);
pub const U_PLUGIN_TOO_HIGH: UErrorCode = UErrorCode(66816i32);
pub const U_PLUGIN_DIDNT_SET_LEVEL: UErrorCode = UErrorCode(66817i32);
#[repr(transparent)]
pub struct UFieldCategory(pub i32);
pub const UFIELD_CATEGORY_UNDEFINED: UFieldCategory = UFieldCategory(0i32);
pub const UFIELD_CATEGORY_DATE: UFieldCategory = UFieldCategory(1i32);
pub const UFIELD_CATEGORY_NUMBER: UFieldCategory = UFieldCategory(2i32);
pub const UFIELD_CATEGORY_LIST: UFieldCategory = UFieldCategory(3i32);
pub const UFIELD_CATEGORY_RELATIVE_DATETIME: UFieldCategory = UFieldCategory(4i32);
pub const UFIELD_CATEGORY_DATE_INTERVAL: UFieldCategory = UFieldCategory(5i32);
pub const UFIELD_CATEGORY_LIST_SPAN: UFieldCategory = UFieldCategory(4099i32);
pub const UFIELD_CATEGORY_DATE_INTERVAL_SPAN: UFieldCategory = UFieldCategory(4101i32);
#[repr(C)]
pub struct UFieldPosition(i32);
#[repr(C)]
pub struct UFieldPositionIterator(i32);
#[repr(transparent)]
pub struct UFormattableType(pub i32);
pub const UFMT_DATE: UFormattableType = UFormattableType(0i32);
pub const UFMT_DOUBLE: UFormattableType = UFormattableType(1i32);
pub const UFMT_LONG: UFormattableType = UFormattableType(2i32);
pub const UFMT_STRING: UFormattableType = UFormattableType(3i32);
pub const UFMT_ARRAY: UFormattableType = UFormattableType(4i32);
pub const UFMT_INT64: UFormattableType = UFormattableType(5i32);
pub const UFMT_OBJECT: UFormattableType = UFormattableType(6i32);
#[repr(C)]
pub struct UFormattedDateInterval(i32);
#[repr(C)]
pub struct UFormattedList(i32);
#[repr(C)]
pub struct UFormattedNumber(i32);
#[repr(C)]
pub struct UFormattedNumberRange(i32);
#[repr(C)]
pub struct UFormattedRelativeDateTime(i32);
#[repr(C)]
pub struct UFormattedValue(i32);
#[repr(transparent)]
pub struct UGender(pub i32);
pub const UGENDER_MALE: UGender = UGender(0i32);
pub const UGENDER_FEMALE: UGender = UGender(1i32);
pub const UGENDER_OTHER: UGender = UGender(2i32);
#[repr(C)]
pub struct UGenderInfo(i32);
#[repr(transparent)]
pub struct UGraphemeClusterBreak(pub i32);
pub const U_GCB_OTHER: UGraphemeClusterBreak = UGraphemeClusterBreak(0i32);
pub const U_GCB_CONTROL: UGraphemeClusterBreak = UGraphemeClusterBreak(1i32);
pub const U_GCB_CR: UGraphemeClusterBreak = UGraphemeClusterBreak(2i32);
pub const U_GCB_EXTEND: UGraphemeClusterBreak = UGraphemeClusterBreak(3i32);
pub const U_GCB_L: UGraphemeClusterBreak = UGraphemeClusterBreak(4i32);
pub const U_GCB_LF: UGraphemeClusterBreak = UGraphemeClusterBreak(5i32);
pub const U_GCB_LV: UGraphemeClusterBreak = UGraphemeClusterBreak(6i32);
pub const U_GCB_LVT: UGraphemeClusterBreak = UGraphemeClusterBreak(7i32);
pub const U_GCB_T: UGraphemeClusterBreak = UGraphemeClusterBreak(8i32);
pub const U_GCB_V: UGraphemeClusterBreak = UGraphemeClusterBreak(9i32);
pub const U_GCB_SPACING_MARK: UGraphemeClusterBreak = UGraphemeClusterBreak(10i32);
pub const U_GCB_PREPEND: UGraphemeClusterBreak = UGraphemeClusterBreak(11i32);
pub const U_GCB_REGIONAL_INDICATOR: UGraphemeClusterBreak = UGraphemeClusterBreak(12i32);
pub const U_GCB_E_BASE: UGraphemeClusterBreak = UGraphemeClusterBreak(13i32);
pub const U_GCB_E_BASE_GAZ: UGraphemeClusterBreak = UGraphemeClusterBreak(14i32);
pub const U_GCB_E_MODIFIER: UGraphemeClusterBreak = UGraphemeClusterBreak(15i32);
pub const U_GCB_GLUE_AFTER_ZWJ: UGraphemeClusterBreak = UGraphemeClusterBreak(16i32);
pub const U_GCB_ZWJ: UGraphemeClusterBreak = UGraphemeClusterBreak(17i32);
#[repr(transparent)]
pub struct UHangulSyllableType(pub i32);
pub const U_HST_NOT_APPLICABLE: UHangulSyllableType = UHangulSyllableType(0i32);
pub const U_HST_LEADING_JAMO: UHangulSyllableType = UHangulSyllableType(1i32);
pub const U_HST_VOWEL_JAMO: UHangulSyllableType = UHangulSyllableType(2i32);
pub const U_HST_TRAILING_JAMO: UHangulSyllableType = UHangulSyllableType(3i32);
pub const U_HST_LV_SYLLABLE: UHangulSyllableType = UHangulSyllableType(4i32);
pub const U_HST_LVT_SYLLABLE: UHangulSyllableType = UHangulSyllableType(5i32);
#[repr(C)]
pub struct UHashtable(i32);
#[repr(C)]
pub struct UIDNA(i32);
#[repr(C)]
pub struct UIDNAInfo(i32);
pub const UIDNA_CHECK_BIDI: i32 = 4i32;
pub const UIDNA_CHECK_CONTEXTJ: i32 = 8i32;
pub const UIDNA_CHECK_CONTEXTO: i32 = 64i32;
pub const UIDNA_DEFAULT: i32 = 0i32;
pub const UIDNA_ERROR_BIDI: i32 = 2048i32;
pub const UIDNA_ERROR_CONTEXTJ: i32 = 4096i32;
pub const UIDNA_ERROR_CONTEXTO_DIGITS: i32 = 16384i32;
pub const UIDNA_ERROR_CONTEXTO_PUNCTUATION: i32 = 8192i32;
pub const UIDNA_ERROR_DISALLOWED: i32 = 128i32;
pub const UIDNA_ERROR_DOMAIN_NAME_TOO_LONG: i32 = 4i32;
pub const UIDNA_ERROR_EMPTY_LABEL: i32 = 1i32;
pub const UIDNA_ERROR_HYPHEN_3_4: i32 = 32i32;
pub const UIDNA_ERROR_INVALID_ACE_LABEL: i32 = 1024i32;
pub const UIDNA_ERROR_LABEL_HAS_DOT: i32 = 512i32;
pub const UIDNA_ERROR_LABEL_TOO_LONG: i32 = 2i32;
pub const UIDNA_ERROR_LEADING_COMBINING_MARK: i32 = 64i32;
pub const UIDNA_ERROR_LEADING_HYPHEN: i32 = 8i32;
pub const UIDNA_ERROR_PUNYCODE: i32 = 256i32;
pub const UIDNA_ERROR_TRAILING_HYPHEN: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_ASCII: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_UNICODE: i32 = 32i32;
pub const UIDNA_USE_STD3_RULES: i32 = 2i32;
#[cfg(feature = "Win32_Foundation")]
pub type UILANGUAGE_ENUMPROCA = unsafe extern "system" fn(param0: super::Foundation::PSTR, param1: isize) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type UILANGUAGE_ENUMPROCW = unsafe extern "system" fn(param0: super::Foundation::PWSTR, param1: isize) -> super::Foundation::BOOL;
pub const UITER_UNKNOWN_INDEX: i32 = -2i32;
#[repr(transparent)]
pub struct UIndicPositionalCategory(pub i32);
pub const U_INPC_NA: UIndicPositionalCategory = UIndicPositionalCategory(0i32);
pub const U_INPC_BOTTOM: UIndicPositionalCategory = UIndicPositionalCategory(1i32);
pub const U_INPC_BOTTOM_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(2i32);
pub const U_INPC_BOTTOM_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(3i32);
pub const U_INPC_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(4i32);
pub const U_INPC_LEFT_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(5i32);
pub const U_INPC_OVERSTRUCK: UIndicPositionalCategory = UIndicPositionalCategory(6i32);
pub const U_INPC_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(7i32);
pub const U_INPC_TOP: UIndicPositionalCategory = UIndicPositionalCategory(8i32);
pub const U_INPC_TOP_AND_BOTTOM: UIndicPositionalCategory = UIndicPositionalCategory(9i32);
pub const U_INPC_TOP_AND_BOTTOM_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(10i32);
pub const U_INPC_TOP_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(11i32);
pub const U_INPC_TOP_AND_LEFT_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(12i32);
pub const U_INPC_TOP_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(13i32);
pub const U_INPC_VISUAL_ORDER_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(14i32);
pub const U_INPC_TOP_AND_BOTTOM_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(15i32);
#[repr(transparent)]
pub struct UIndicSyllabicCategory(pub i32);
pub const U_INSC_OTHER: UIndicSyllabicCategory = UIndicSyllabicCategory(0i32);
pub const U_INSC_AVAGRAHA: UIndicSyllabicCategory = UIndicSyllabicCategory(1i32);
pub const U_INSC_BINDU: UIndicSyllabicCategory = UIndicSyllabicCategory(2i32);
pub const U_INSC_BRAHMI_JOINING_NUMBER: UIndicSyllabicCategory = UIndicSyllabicCategory(3i32);
pub const U_INSC_CANTILLATION_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(4i32);
pub const U_INSC_CONSONANT: UIndicSyllabicCategory = UIndicSyllabicCategory(5i32);
pub const U_INSC_CONSONANT_DEAD: UIndicSyllabicCategory = UIndicSyllabicCategory(6i32);
pub const U_INSC_CONSONANT_FINAL: UIndicSyllabicCategory = UIndicSyllabicCategory(7i32);
pub const U_INSC_CONSONANT_HEAD_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(8i32);
pub const U_INSC_CONSONANT_INITIAL_POSTFIXED: UIndicSyllabicCategory = UIndicSyllabicCategory(9i32);
pub const U_INSC_CONSONANT_KILLER: UIndicSyllabicCategory = UIndicSyllabicCategory(10i32);
pub const U_INSC_CONSONANT_MEDIAL: UIndicSyllabicCategory = UIndicSyllabicCategory(11i32);
pub const U_INSC_CONSONANT_PLACEHOLDER: UIndicSyllabicCategory = UIndicSyllabicCategory(12i32);
pub const U_INSC_CONSONANT_PRECEDING_REPHA: UIndicSyllabicCategory = UIndicSyllabicCategory(13i32);
pub const U_INSC_CONSONANT_PREFIXED: UIndicSyllabicCategory = UIndicSyllabicCategory(14i32);
pub const U_INSC_CONSONANT_SUBJOINED: UIndicSyllabicCategory = UIndicSyllabicCategory(15i32);
pub const U_INSC_CONSONANT_SUCCEEDING_REPHA: UIndicSyllabicCategory = UIndicSyllabicCategory(16i32);
pub const U_INSC_CONSONANT_WITH_STACKER: UIndicSyllabicCategory = UIndicSyllabicCategory(17i32);
pub const U_INSC_GEMINATION_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(18i32);
pub const U_INSC_INVISIBLE_STACKER: UIndicSyllabicCategory = UIndicSyllabicCategory(19i32);
pub const U_INSC_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(20i32);
pub const U_INSC_MODIFYING_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(21i32);
pub const U_INSC_NON_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(22i32);
pub const U_INSC_NUKTA: UIndicSyllabicCategory = UIndicSyllabicCategory(23i32);
pub const U_INSC_NUMBER: UIndicSyllabicCategory = UIndicSyllabicCategory(24i32);
pub const U_INSC_NUMBER_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(25i32);
pub const U_INSC_PURE_KILLER: UIndicSyllabicCategory = UIndicSyllabicCategory(26i32);
pub const U_INSC_REGISTER_SHIFTER: UIndicSyllabicCategory = UIndicSyllabicCategory(27i32);
pub const U_INSC_SYLLABLE_MODIFIER: UIndicSyllabicCategory = UIndicSyllabicCategory(28i32);
pub const U_INSC_TONE_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(29i32);
pub const U_INSC_TONE_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(30i32);
pub const U_INSC_VIRAMA: UIndicSyllabicCategory = UIndicSyllabicCategory(31i32);
pub const U_INSC_VISARGA: UIndicSyllabicCategory = UIndicSyllabicCategory(32i32);
pub const U_INSC_VOWEL: UIndicSyllabicCategory = UIndicSyllabicCategory(33i32);
pub const U_INSC_VOWEL_DEPENDENT: UIndicSyllabicCategory = UIndicSyllabicCategory(34i32);
pub const U_INSC_VOWEL_INDEPENDENT: UIndicSyllabicCategory = UIndicSyllabicCategory(35i32);
#[repr(transparent)]
pub struct UJoiningGroup(pub i32);
pub const U_JG_NO_JOINING_GROUP: UJoiningGroup = UJoiningGroup(0i32);
pub const U_JG_AIN: UJoiningGroup = UJoiningGroup(1i32);
pub const U_JG_ALAPH: UJoiningGroup = UJoiningGroup(2i32);
pub const U_JG_ALEF: UJoiningGroup = UJoiningGroup(3i32);
pub const U_JG_BEH: UJoiningGroup = UJoiningGroup(4i32);
pub const U_JG_BETH: UJoiningGroup = UJoiningGroup(5i32);
pub const U_JG_DAL: UJoiningGroup = UJoiningGroup(6i32);
pub const U_JG_DALATH_RISH: UJoiningGroup = UJoiningGroup(7i32);
pub const U_JG_E: UJoiningGroup = UJoiningGroup(8i32);
pub const U_JG_FEH: UJoiningGroup = UJoiningGroup(9i32);
pub const U_JG_FINAL_SEMKATH: UJoiningGroup = UJoiningGroup(10i32);
pub const U_JG_GAF: UJoiningGroup = UJoiningGroup(11i32);
pub const U_JG_GAMAL: UJoiningGroup = UJoiningGroup(12i32);
pub const U_JG_HAH: UJoiningGroup = UJoiningGroup(13i32);
pub const U_JG_TEH_MARBUTA_GOAL: UJoiningGroup = UJoiningGroup(14i32);
pub const U_JG_HAMZA_ON_HEH_GOAL: UJoiningGroup = UJoiningGroup(14i32);
pub const U_JG_HE: UJoiningGroup = UJoiningGroup(15i32);
pub const U_JG_HEH: UJoiningGroup = UJoiningGroup(16i32);
pub const U_JG_HEH_GOAL: UJoiningGroup = UJoiningGroup(17i32);
pub const U_JG_HETH: UJoiningGroup = UJoiningGroup(18i32);
pub const U_JG_KAF: UJoiningGroup = UJoiningGroup(19i32);
pub const U_JG_KAPH: UJoiningGroup = UJoiningGroup(20i32);
pub const U_JG_KNOTTED_HEH: UJoiningGroup = UJoiningGroup(21i32);
pub const U_JG_LAM: UJoiningGroup = UJoiningGroup(22i32);
pub const U_JG_LAMADH: UJoiningGroup = UJoiningGroup(23i32);
pub const U_JG_MEEM: UJoiningGroup = UJoiningGroup(24i32);
pub const U_JG_MIM: UJoiningGroup = UJoiningGroup(25i32);
pub const U_JG_NOON: UJoiningGroup = UJoiningGroup(26i32);
pub const U_JG_NUN: UJoiningGroup = UJoiningGroup(27i32);
pub const U_JG_PE: UJoiningGroup = UJoiningGroup(28i32);
pub const U_JG_QAF: UJoiningGroup = UJoiningGroup(29i32);
pub const U_JG_QAPH: UJoiningGroup = UJoiningGroup(30i32);
pub const U_JG_REH: UJoiningGroup = UJoiningGroup(31i32);
pub const U_JG_REVERSED_PE: UJoiningGroup = UJoiningGroup(32i32);
pub const U_JG_SAD: UJoiningGroup = UJoiningGroup(33i32);
pub const U_JG_SADHE: UJoiningGroup = UJoiningGroup(34i32);
pub const U_JG_SEEN: UJoiningGroup = UJoiningGroup(35i32);
pub const U_JG_SEMKATH: UJoiningGroup = UJoiningGroup(36i32);
pub const U_JG_SHIN: UJoiningGroup = UJoiningGroup(37i32);
pub const U_JG_SWASH_KAF: UJoiningGroup = UJoiningGroup(38i32);
pub const U_JG_SYRIAC_WAW: UJoiningGroup = UJoiningGroup(39i32);
pub const U_JG_TAH: UJoiningGroup = UJoiningGroup(40i32);
pub const U_JG_TAW: UJoiningGroup = UJoiningGroup(41i32);
pub const U_JG_TEH_MARBUTA: UJoiningGroup = UJoiningGroup(42i32);
pub const U_JG_TETH: UJoiningGroup = UJoiningGroup(43i32);
pub const U_JG_WAW: UJoiningGroup = UJoiningGroup(44i32);
pub const U_JG_YEH: UJoiningGroup = UJoiningGroup(45i32);
pub const U_JG_YEH_BARREE: UJoiningGroup = UJoiningGroup(46i32);
pub const U_JG_YEH_WITH_TAIL: UJoiningGroup = UJoiningGroup(47i32);
pub const U_JG_YUDH: UJoiningGroup = UJoiningGroup(48i32);
pub const U_JG_YUDH_HE: UJoiningGroup = UJoiningGroup(49i32);
pub const U_JG_ZAIN: UJoiningGroup = UJoiningGroup(50i32);
pub const U_JG_FE: UJoiningGroup = UJoiningGroup(51i32);
pub const U_JG_KHAPH: UJoiningGroup = UJoiningGroup(52i32);
pub const U_JG_ZHAIN: UJoiningGroup = UJoiningGroup(53i32);
pub const U_JG_BURUSHASKI_YEH_BARREE: UJoiningGroup = UJoiningGroup(54i32);
pub const U_JG_FARSI_YEH: UJoiningGroup = UJoiningGroup(55i32);
pub const U_JG_NYA: UJoiningGroup = UJoiningGroup(56i32);
pub const U_JG_ROHINGYA_YEH: UJoiningGroup = UJoiningGroup(57i32);
pub const U_JG_MANICHAEAN_ALEPH: UJoiningGroup = UJoiningGroup(58i32);
pub const U_JG_MANICHAEAN_AYIN: UJoiningGroup = UJoiningGroup(59i32);
pub const U_JG_MANICHAEAN_BETH: UJoiningGroup = UJoiningGroup(60i32);
pub const U_JG_MANICHAEAN_DALETH: UJoiningGroup = UJoiningGroup(61i32);
pub const U_JG_MANICHAEAN_DHAMEDH: UJoiningGroup = UJoiningGroup(62i32);
pub const U_JG_MANICHAEAN_FIVE: UJoiningGroup = UJoiningGroup(63i32);
pub const U_JG_MANICHAEAN_GIMEL: UJoiningGroup = UJoiningGroup(64i32);
pub const U_JG_MANICHAEAN_HETH: UJoiningGroup = UJoiningGroup(65i32);
pub const U_JG_MANICHAEAN_HUNDRED: UJoiningGroup = UJoiningGroup(66i32);
pub const U_JG_MANICHAEAN_KAPH: UJoiningGroup = UJoiningGroup(67i32);
pub const U_JG_MANICHAEAN_LAMEDH: UJoiningGroup = UJoiningGroup(68i32);
pub const U_JG_MANICHAEAN_MEM: UJoiningGroup = UJoiningGroup(69i32);
pub const U_JG_MANICHAEAN_NUN: UJoiningGroup = UJoiningGroup(70i32);
pub const U_JG_MANICHAEAN_ONE: UJoiningGroup = UJoiningGroup(71i32);
pub const U_JG_MANICHAEAN_PE: UJoiningGroup = UJoiningGroup(72i32);
pub const U_JG_MANICHAEAN_QOPH: UJoiningGroup = UJoiningGroup(73i32);
pub const U_JG_MANICHAEAN_RESH: UJoiningGroup = UJoiningGroup(74i32);
pub const U_JG_MANICHAEAN_SADHE: UJoiningGroup = UJoiningGroup(75i32);
pub const U_JG_MANICHAEAN_SAMEKH: UJoiningGroup = UJoiningGroup(76i32);
pub const U_JG_MANICHAEAN_TAW: UJoiningGroup = UJoiningGroup(77i32);
pub const U_JG_MANICHAEAN_TEN: UJoiningGroup = UJoiningGroup(78i32);
pub const U_JG_MANICHAEAN_TETH: UJoiningGroup = UJoiningGroup(79i32);
pub const U_JG_MANICHAEAN_THAMEDH: UJoiningGroup = UJoiningGroup(80i32);
pub const U_JG_MANICHAEAN_TWENTY: UJoiningGroup = UJoiningGroup(81i32);
pub const U_JG_MANICHAEAN_WAW: UJoiningGroup = UJoiningGroup(82i32);
pub const U_JG_MANICHAEAN_YODH: UJoiningGroup = UJoiningGroup(83i32);
pub const U_JG_MANICHAEAN_ZAYIN: UJoiningGroup = UJoiningGroup(84i32);
pub const U_JG_STRAIGHT_WAW: UJoiningGroup = UJoiningGroup(85i32);
pub const U_JG_AFRICAN_FEH: UJoiningGroup = UJoiningGroup(86i32);
pub const U_JG_AFRICAN_NOON: UJoiningGroup = UJoiningGroup(87i32);
pub const U_JG_AFRICAN_QAF: UJoiningGroup = UJoiningGroup(88i32);
pub const U_JG_MALAYALAM_BHA: UJoiningGroup = UJoiningGroup(89i32);
pub const U_JG_MALAYALAM_JA: UJoiningGroup = UJoiningGroup(90i32);
pub const U_JG_MALAYALAM_LLA: UJoiningGroup = UJoiningGroup(91i32);
pub const U_JG_MALAYALAM_LLLA: UJoiningGroup = UJoiningGroup(92i32);
pub const U_JG_MALAYALAM_NGA: UJoiningGroup = UJoiningGroup(93i32);
pub const U_JG_MALAYALAM_NNA: UJoiningGroup = UJoiningGroup(94i32);
pub const U_JG_MALAYALAM_NNNA: UJoiningGroup = UJoiningGroup(95i32);
pub const U_JG_MALAYALAM_NYA: UJoiningGroup = UJoiningGroup(96i32);
pub const U_JG_MALAYALAM_RA: UJoiningGroup = UJoiningGroup(97i32);
pub const U_JG_MALAYALAM_SSA: UJoiningGroup = UJoiningGroup(98i32);
pub const U_JG_MALAYALAM_TTA: UJoiningGroup = UJoiningGroup(99i32);
pub const U_JG_HANIFI_ROHINGYA_KINNA_YA: UJoiningGroup = UJoiningGroup(100i32);
pub const U_JG_HANIFI_ROHINGYA_PA: UJoiningGroup = UJoiningGroup(101i32);
#[repr(transparent)]
pub struct UJoiningType(pub i32);
pub const U_JT_NON_JOINING: UJoiningType = UJoiningType(0i32);
pub const U_JT_JOIN_CAUSING: UJoiningType = UJoiningType(1i32);
pub const U_JT_DUAL_JOINING: UJoiningType = UJoiningType(2i32);
pub const U_JT_LEFT_JOINING: UJoiningType = UJoiningType(3i32);
pub const U_JT_RIGHT_JOINING: UJoiningType = UJoiningType(4i32);
pub const U_JT_TRANSPARENT: UJoiningType = UJoiningType(5i32);
pub const ULOC_COUNTRY_CAPACITY: u32 = 4u32;
pub const ULOC_FULLNAME_CAPACITY: u32 = 157u32;
pub const ULOC_KEYWORDS_CAPACITY: u32 = 96u32;
pub const ULOC_KEYWORD_AND_VALUES_CAPACITY: u32 = 100u32;
pub const ULOC_KEYWORD_ASSIGN_UNICODE: u32 = 61u32;
pub const ULOC_KEYWORD_ITEM_SEPARATOR_UNICODE: u32 = 59u32;
pub const ULOC_KEYWORD_SEPARATOR_UNICODE: u32 = 64u32;
pub const ULOC_LANG_CAPACITY: u32 = 12u32;
pub const ULOC_SCRIPT_CAPACITY: u32 = 6u32;
#[repr(transparent)]
pub struct ULayoutType(pub i32);
pub const ULOC_LAYOUT_LTR: ULayoutType = ULayoutType(0i32);
pub const ULOC_LAYOUT_RTL: ULayoutType = ULayoutType(1i32);
pub const ULOC_LAYOUT_TTB: ULayoutType = ULayoutType(2i32);
pub const ULOC_LAYOUT_BTT: ULayoutType = ULayoutType(3i32);
pub const ULOC_LAYOUT_UNKNOWN: ULayoutType = ULayoutType(4i32);
#[repr(transparent)]
pub struct ULineBreak(pub i32);
pub const U_LB_UNKNOWN: ULineBreak = ULineBreak(0i32);
pub const U_LB_AMBIGUOUS: ULineBreak = ULineBreak(1i32);
pub const U_LB_ALPHABETIC: ULineBreak = ULineBreak(2i32);
pub const U_LB_BREAK_BOTH: ULineBreak = ULineBreak(3i32);
pub const U_LB_BREAK_AFTER: ULineBreak = ULineBreak(4i32);
pub const U_LB_BREAK_BEFORE: ULineBreak = ULineBreak(5i32);
pub const U_LB_MANDATORY_BREAK: ULineBreak = ULineBreak(6i32);
pub const U_LB_CONTINGENT_BREAK: ULineBreak = ULineBreak(7i32);
pub const U_LB_CLOSE_PUNCTUATION: ULineBreak = ULineBreak(8i32);
pub const U_LB_COMBINING_MARK: ULineBreak = ULineBreak(9i32);
pub const U_LB_CARRIAGE_RETURN: ULineBreak = ULineBreak(10i32);
pub const U_LB_EXCLAMATION: ULineBreak = ULineBreak(11i32);
pub const U_LB_GLUE: ULineBreak = ULineBreak(12i32);
pub const U_LB_HYPHEN: ULineBreak = ULineBreak(13i32);
pub const U_LB_IDEOGRAPHIC: ULineBreak = ULineBreak(14i32);
pub const U_LB_INSEPARABLE: ULineBreak = ULineBreak(15i32);
pub const U_LB_INSEPERABLE: ULineBreak = ULineBreak(15i32);
pub const U_LB_INFIX_NUMERIC: ULineBreak = ULineBreak(16i32);
pub const U_LB_LINE_FEED: ULineBreak = ULineBreak(17i32);
pub const U_LB_NONSTARTER: ULineBreak = ULineBreak(18i32);
pub const U_LB_NUMERIC: ULineBreak = ULineBreak(19i32);
pub const U_LB_OPEN_PUNCTUATION: ULineBreak = ULineBreak(20i32);
pub const U_LB_POSTFIX_NUMERIC: ULineBreak = ULineBreak(21i32);
pub const U_LB_PREFIX_NUMERIC: ULineBreak = ULineBreak(22i32);
pub const U_LB_QUOTATION: ULineBreak = ULineBreak(23i32);
pub const U_LB_COMPLEX_CONTEXT: ULineBreak = ULineBreak(24i32);
pub const U_LB_SURROGATE: ULineBreak = ULineBreak(25i32);
pub const U_LB_SPACE: ULineBreak = ULineBreak(26i32);
pub const U_LB_BREAK_SYMBOLS: ULineBreak = ULineBreak(27i32);
pub const U_LB_ZWSPACE: ULineBreak = ULineBreak(28i32);
pub const U_LB_NEXT_LINE: ULineBreak = ULineBreak(29i32);
pub const U_LB_WORD_JOINER: ULineBreak = ULineBreak(30i32);
pub const U_LB_H2: ULineBreak = ULineBreak(31i32);
pub const U_LB_H3: ULineBreak = ULineBreak(32i32);
pub const U_LB_JL: ULineBreak = ULineBreak(33i32);
pub const U_LB_JT: ULineBreak = ULineBreak(34i32);
pub const U_LB_JV: ULineBreak = ULineBreak(35i32);
pub const U_LB_CLOSE_PARENTHESIS: ULineBreak = ULineBreak(36i32);
pub const U_LB_CONDITIONAL_JAPANESE_STARTER: ULineBreak = ULineBreak(37i32);
pub const U_LB_HEBREW_LETTER: ULineBreak = ULineBreak(38i32);
pub const U_LB_REGIONAL_INDICATOR: ULineBreak = ULineBreak(39i32);
pub const U_LB_E_BASE: ULineBreak = ULineBreak(40i32);
pub const U_LB_E_MODIFIER: ULineBreak = ULineBreak(41i32);
pub const U_LB_ZWJ: ULineBreak = ULineBreak(42i32);
#[repr(transparent)]
pub struct ULineBreakTag(pub i32);
pub const UBRK_LINE_SOFT: ULineBreakTag = ULineBreakTag(0i32);
pub const UBRK_LINE_SOFT_LIMIT: ULineBreakTag = ULineBreakTag(100i32);
pub const UBRK_LINE_HARD: ULineBreakTag = ULineBreakTag(100i32);
pub const UBRK_LINE_HARD_LIMIT: ULineBreakTag = ULineBreakTag(200i32);
#[repr(C)]
pub struct UListFormatter(i32);
#[repr(transparent)]
pub struct UListFormatterField(pub i32);
pub const ULISTFMT_LITERAL_FIELD: UListFormatterField = UListFormatterField(0i32);
pub const ULISTFMT_ELEMENT_FIELD: UListFormatterField = UListFormatterField(1i32);
#[repr(transparent)]
pub struct UListFormatterType(pub i32);
pub const ULISTFMT_TYPE_AND: UListFormatterType = UListFormatterType(0i32);
pub const ULISTFMT_TYPE_OR: UListFormatterType = UListFormatterType(1i32);
pub const ULISTFMT_TYPE_UNITS: UListFormatterType = UListFormatterType(2i32);
#[repr(transparent)]
pub struct UListFormatterWidth(pub i32);
pub const ULISTFMT_WIDTH_WIDE: UListFormatterWidth = UListFormatterWidth(0i32);
pub const ULISTFMT_WIDTH_SHORT: UListFormatterWidth = UListFormatterWidth(1i32);
pub const ULISTFMT_WIDTH_NARROW: UListFormatterWidth = UListFormatterWidth(2i32);
#[repr(transparent)]
pub struct ULocAvailableType(pub i32);
pub const ULOC_AVAILABLE_DEFAULT: ULocAvailableType = ULocAvailableType(0i32);
pub const ULOC_AVAILABLE_ONLY_LEGACY_ALIASES: ULocAvailableType = ULocAvailableType(1i32);
pub const ULOC_AVAILABLE_WITH_LEGACY_ALIASES: ULocAvailableType = ULocAvailableType(2i32);
#[repr(transparent)]
pub struct ULocDataLocaleType(pub i32);
pub const ULOC_ACTUAL_LOCALE: ULocDataLocaleType = ULocDataLocaleType(0i32);
pub const ULOC_VALID_LOCALE: ULocDataLocaleType = ULocDataLocaleType(1i32);
#[repr(C)]
pub struct ULocaleData(i32);
#[repr(transparent)]
pub struct ULocaleDataDelimiterType(pub i32);
pub const ULOCDATA_QUOTATION_START: ULocaleDataDelimiterType = ULocaleDataDelimiterType(0i32);
pub const ULOCDATA_QUOTATION_END: ULocaleDataDelimiterType = ULocaleDataDelimiterType(1i32);
pub const ULOCDATA_ALT_QUOTATION_START: ULocaleDataDelimiterType = ULocaleDataDelimiterType(2i32);
pub const ULOCDATA_ALT_QUOTATION_END: ULocaleDataDelimiterType = ULocaleDataDelimiterType(3i32);
#[repr(transparent)]
pub struct ULocaleDataExemplarSetType(pub i32);
pub const ULOCDATA_ES_STANDARD: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(0i32);
pub const ULOCDATA_ES_AUXILIARY: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(1i32);
pub const ULOCDATA_ES_INDEX: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(2i32);
pub const ULOCDATA_ES_PUNCTUATION: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(3i32);
#[repr(C)]
pub struct ULocaleDisplayNames(i32);
pub const UMSGPAT_ARG_NAME_NOT_NUMBER: i32 = -1i32;
pub const UMSGPAT_ARG_NAME_NOT_VALID: i32 = -2i32;
#[repr(transparent)]
pub struct UMeasureFormatWidth(pub i32);
pub const UMEASFMT_WIDTH_WIDE: UMeasureFormatWidth = UMeasureFormatWidth(0i32);
pub const UMEASFMT_WIDTH_SHORT: UMeasureFormatWidth = UMeasureFormatWidth(1i32);
pub const UMEASFMT_WIDTH_NARROW: UMeasureFormatWidth = UMeasureFormatWidth(2i32);
pub const UMEASFMT_WIDTH_NUMERIC: UMeasureFormatWidth = UMeasureFormatWidth(3i32);
pub const UMEASFMT_WIDTH_COUNT: UMeasureFormatWidth = UMeasureFormatWidth(4i32);
#[repr(transparent)]
pub struct UMeasurementSystem(pub i32);
pub const UMS_SI: UMeasurementSystem = UMeasurementSystem(0i32);
pub const UMS_US: UMeasurementSystem = UMeasurementSystem(1i32);
pub const UMS_UK: UMeasurementSystem = UMeasurementSystem(2i32);
pub type UMemAllocFn = unsafe extern "system" fn(context: *const ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void;
pub type UMemFreeFn = unsafe extern "system" fn(context: *const ::core::ffi::c_void, mem: *mut ::core::ffi::c_void);
pub type UMemReallocFn = unsafe extern "system" fn(context: *const ::core::ffi::c_void, mem: *mut ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct UMessagePatternApostropheMode(pub i32);
pub const UMSGPAT_APOS_DOUBLE_OPTIONAL: UMessagePatternApostropheMode = UMessagePatternApostropheMode(0i32);
pub const UMSGPAT_APOS_DOUBLE_REQUIRED: UMessagePatternApostropheMode = UMessagePatternApostropheMode(1i32);
#[repr(transparent)]
pub struct UMessagePatternArgType(pub i32);
pub const UMSGPAT_ARG_TYPE_NONE: UMessagePatternArgType = UMessagePatternArgType(0i32);
pub const UMSGPAT_ARG_TYPE_SIMPLE: UMessagePatternArgType = UMessagePatternArgType(1i32);
pub const UMSGPAT_ARG_TYPE_CHOICE: UMessagePatternArgType = UMessagePatternArgType(2i32);
pub const UMSGPAT_ARG_TYPE_PLURAL: UMessagePatternArgType = UMessagePatternArgType(3i32);
pub const UMSGPAT_ARG_TYPE_SELECT: UMessagePatternArgType = UMessagePatternArgType(4i32);
pub const UMSGPAT_ARG_TYPE_SELECTORDINAL: UMessagePatternArgType = UMessagePatternArgType(5i32);
#[repr(transparent)]
pub struct UMessagePatternPartType(pub i32);
pub const UMSGPAT_PART_TYPE_MSG_START: UMessagePatternPartType = UMessagePatternPartType(0i32);
pub const UMSGPAT_PART_TYPE_MSG_LIMIT: UMessagePatternPartType = UMessagePatternPartType(1i32);
pub const UMSGPAT_PART_TYPE_SKIP_SYNTAX: UMessagePatternPartType = UMessagePatternPartType(2i32);
pub const UMSGPAT_PART_TYPE_INSERT_CHAR: UMessagePatternPartType = UMessagePatternPartType(3i32);
pub const UMSGPAT_PART_TYPE_REPLACE_NUMBER: UMessagePatternPartType = UMessagePatternPartType(4i32);
pub const UMSGPAT_PART_TYPE_ARG_START: UMessagePatternPartType = UMessagePatternPartType(5i32);
pub const UMSGPAT_PART_TYPE_ARG_LIMIT: UMessagePatternPartType = UMessagePatternPartType(6i32);
pub const UMSGPAT_PART_TYPE_ARG_NUMBER: UMessagePatternPartType = UMessagePatternPartType(7i32);
pub const UMSGPAT_PART_TYPE_ARG_NAME: UMessagePatternPartType = UMessagePatternPartType(8i32);
pub const UMSGPAT_PART_TYPE_ARG_TYPE: UMessagePatternPartType = UMessagePatternPartType(9i32);
pub const UMSGPAT_PART_TYPE_ARG_STYLE: UMessagePatternPartType = UMessagePatternPartType(10i32);
pub const UMSGPAT_PART_TYPE_ARG_SELECTOR: UMessagePatternPartType = UMessagePatternPartType(11i32);
pub const UMSGPAT_PART_TYPE_ARG_INT: UMessagePatternPartType = UMessagePatternPartType(12i32);
pub const UMSGPAT_PART_TYPE_ARG_DOUBLE: UMessagePatternPartType = UMessagePatternPartType(13i32);
#[repr(C)]
pub struct UMutableCPTrie(i32);
pub type UNESCAPE_CHAR_AT = unsafe extern "system" fn(offset: i32, context: *mut ::core::ffi::c_void) -> u16;
#[repr(C)]
pub struct UNICODERANGE(i32);
pub const UNISCRIBE_OPENTYPE: u32 = 256u32;
pub const UNORM_INPUT_IS_FCD: u32 = 131072u32;
#[repr(transparent)]
pub struct UNormalization2Mode(pub i32);
pub const UNORM2_COMPOSE: UNormalization2Mode = UNormalization2Mode(0i32);
pub const UNORM2_DECOMPOSE: UNormalization2Mode = UNormalization2Mode(1i32);
pub const UNORM2_FCD: UNormalization2Mode = UNormalization2Mode(2i32);
pub const UNORM2_COMPOSE_CONTIGUOUS: UNormalization2Mode = UNormalization2Mode(3i32);
#[repr(transparent)]
pub struct UNormalizationCheckResult(pub i32);
pub const UNORM_NO: UNormalizationCheckResult = UNormalizationCheckResult(0i32);
pub const UNORM_YES: UNormalizationCheckResult = UNormalizationCheckResult(1i32);
pub const UNORM_MAYBE: UNormalizationCheckResult = UNormalizationCheckResult(2i32);
#[repr(transparent)]
pub struct UNormalizationMode(pub i32);
pub const UNORM_NONE: UNormalizationMode = UNormalizationMode(1i32);
pub const UNORM_NFD: UNormalizationMode = UNormalizationMode(2i32);
pub const UNORM_NFKD: UNormalizationMode = UNormalizationMode(3i32);
pub const UNORM_NFC: UNormalizationMode = UNormalizationMode(4i32);
pub const UNORM_DEFAULT: UNormalizationMode = UNormalizationMode(4i32);
pub const UNORM_NFKC: UNormalizationMode = UNormalizationMode(5i32);
pub const UNORM_FCD: UNormalizationMode = UNormalizationMode(6i32);
pub const UNORM_MODE_COUNT: UNormalizationMode = UNormalizationMode(7i32);
#[repr(C)]
pub struct UNormalizer2(i32);
#[repr(transparent)]
pub struct UNumberCompactStyle(pub i32);
pub const UNUM_SHORT: UNumberCompactStyle = UNumberCompactStyle(0i32);
pub const UNUM_LONG: UNumberCompactStyle = UNumberCompactStyle(1i32);
#[repr(transparent)]
pub struct UNumberDecimalSeparatorDisplay(pub i32);
pub const UNUM_DECIMAL_SEPARATOR_AUTO: UNumberDecimalSeparatorDisplay = UNumberDecimalSeparatorDisplay(0i32);
pub const UNUM_DECIMAL_SEPARATOR_ALWAYS: UNumberDecimalSeparatorDisplay = UNumberDecimalSeparatorDisplay(1i32);
pub const UNUM_DECIMAL_SEPARATOR_COUNT: UNumberDecimalSeparatorDisplay = UNumberDecimalSeparatorDisplay(2i32);
#[repr(transparent)]
pub struct UNumberFormatAttribute(pub i32);
pub const UNUM_PARSE_INT_ONLY: UNumberFormatAttribute = UNumberFormatAttribute(0i32);
pub const UNUM_GROUPING_USED: UNumberFormatAttribute = UNumberFormatAttribute(1i32);
pub const UNUM_DECIMAL_ALWAYS_SHOWN: UNumberFormatAttribute = UNumberFormatAttribute(2i32);
pub const UNUM_MAX_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(3i32);
pub const UNUM_MIN_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(4i32);
pub const UNUM_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(5i32);
pub const UNUM_MAX_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(6i32);
pub const UNUM_MIN_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(7i32);
pub const UNUM_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(8i32);
pub const UNUM_MULTIPLIER: UNumberFormatAttribute = UNumberFormatAttribute(9i32);
pub const UNUM_GROUPING_SIZE: UNumberFormatAttribute = UNumberFormatAttribute(10i32);
pub const UNUM_ROUNDING_MODE: UNumberFormatAttribute = UNumberFormatAttribute(11i32);
pub const UNUM_ROUNDING_INCREMENT: UNumberFormatAttribute = UNumberFormatAttribute(12i32);
pub const UNUM_FORMAT_WIDTH: UNumberFormatAttribute = UNumberFormatAttribute(13i32);
pub const UNUM_PADDING_POSITION: UNumberFormatAttribute = UNumberFormatAttribute(14i32);
pub const UNUM_SECONDARY_GROUPING_SIZE: UNumberFormatAttribute = UNumberFormatAttribute(15i32);
pub const UNUM_SIGNIFICANT_DIGITS_USED: UNumberFormatAttribute = UNumberFormatAttribute(16i32);
pub const UNUM_MIN_SIGNIFICANT_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(17i32);
pub const UNUM_MAX_SIGNIFICANT_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(18i32);
pub const UNUM_LENIENT_PARSE: UNumberFormatAttribute = UNumberFormatAttribute(19i32);
pub const UNUM_PARSE_ALL_INPUT: UNumberFormatAttribute = UNumberFormatAttribute(20i32);
pub const UNUM_SCALE: UNumberFormatAttribute = UNumberFormatAttribute(21i32);
pub const UNUM_MINIMUM_GROUPING_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(22i32);
pub const UNUM_CURRENCY_USAGE: UNumberFormatAttribute = UNumberFormatAttribute(23i32);
pub const UNUM_FORMAT_FAIL_IF_MORE_THAN_MAX_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(4096i32);
pub const UNUM_PARSE_NO_EXPONENT: UNumberFormatAttribute = UNumberFormatAttribute(4097i32);
pub const UNUM_PARSE_DECIMAL_MARK_REQUIRED: UNumberFormatAttribute = UNumberFormatAttribute(4098i32);
pub const UNUM_PARSE_CASE_SENSITIVE: UNumberFormatAttribute = UNumberFormatAttribute(4099i32);
pub const UNUM_SIGN_ALWAYS_SHOWN: UNumberFormatAttribute = UNumberFormatAttribute(4100i32);
#[repr(transparent)]
pub struct UNumberFormatAttributeValue(pub i32);
pub const UNUM_FORMAT_ATTRIBUTE_VALUE_HIDDEN: UNumberFormatAttributeValue = UNumberFormatAttributeValue(0i32);
#[repr(transparent)]
pub struct UNumberFormatFields(pub i32);
pub const UNUM_INTEGER_FIELD: UNumberFormatFields = UNumberFormatFields(0i32);
pub const UNUM_FRACTION_FIELD: UNumberFormatFields = UNumberFormatFields(1i32);
pub const UNUM_DECIMAL_SEPARATOR_FIELD: UNumberFormatFields = UNumberFormatFields(2i32);
pub const UNUM_EXPONENT_SYMBOL_FIELD: UNumberFormatFields = UNumberFormatFields(3i32);
pub const UNUM_EXPONENT_SIGN_FIELD: UNumberFormatFields = UNumberFormatFields(4i32);
pub const UNUM_EXPONENT_FIELD: UNumberFormatFields = UNumberFormatFields(5i32);
pub const UNUM_GROUPING_SEPARATOR_FIELD: UNumberFormatFields = UNumberFormatFields(6i32);
pub const UNUM_CURRENCY_FIELD: UNumberFormatFields = UNumberFormatFields(7i32);
pub const UNUM_PERCENT_FIELD: UNumberFormatFields = UNumberFormatFields(8i32);
pub const UNUM_PERMILL_FIELD: UNumberFormatFields = UNumberFormatFields(9i32);
pub const UNUM_SIGN_FIELD: UNumberFormatFields = UNumberFormatFields(10i32);
pub const UNUM_MEASURE_UNIT_FIELD: UNumberFormatFields = UNumberFormatFields(11i32);
pub const UNUM_COMPACT_FIELD: UNumberFormatFields = UNumberFormatFields(12i32);
#[repr(transparent)]
pub struct UNumberFormatPadPosition(pub i32);
pub const UNUM_PAD_BEFORE_PREFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(0i32);
pub const UNUM_PAD_AFTER_PREFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(1i32);
pub const UNUM_PAD_BEFORE_SUFFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(2i32);
pub const UNUM_PAD_AFTER_SUFFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(3i32);
#[repr(transparent)]
pub struct UNumberFormatRoundingMode(pub i32);
pub const UNUM_ROUND_CEILING: UNumberFormatRoundingMode = UNumberFormatRoundingMode(0i32);
pub const UNUM_ROUND_FLOOR: UNumberFormatRoundingMode = UNumberFormatRoundingMode(1i32);
pub const UNUM_ROUND_DOWN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(2i32);
pub const UNUM_ROUND_UP: UNumberFormatRoundingMode = UNumberFormatRoundingMode(3i32);
pub const UNUM_ROUND_HALFEVEN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(4i32);
pub const UNUM_ROUND_HALFDOWN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(5i32);
pub const UNUM_ROUND_HALFUP: UNumberFormatRoundingMode = UNumberFormatRoundingMode(6i32);
pub const UNUM_ROUND_UNNECESSARY: UNumberFormatRoundingMode = UNumberFormatRoundingMode(7i32);
#[repr(transparent)]
pub struct UNumberFormatStyle(pub i32);
pub const UNUM_PATTERN_DECIMAL: UNumberFormatStyle = UNumberFormatStyle(0i32);
pub const UNUM_DECIMAL: UNumberFormatStyle = UNumberFormatStyle(1i32);
pub const UNUM_CURRENCY: UNumberFormatStyle = UNumberFormatStyle(2i32);
pub const UNUM_PERCENT: UNumberFormatStyle = UNumberFormatStyle(3i32);
pub const UNUM_SCIENTIFIC: UNumberFormatStyle = UNumberFormatStyle(4i32);
pub const UNUM_SPELLOUT: UNumberFormatStyle = UNumberFormatStyle(5i32);
pub const UNUM_ORDINAL: UNumberFormatStyle = UNumberFormatStyle(6i32);
pub const UNUM_DURATION: UNumberFormatStyle = UNumberFormatStyle(7i32);
pub const UNUM_NUMBERING_SYSTEM: UNumberFormatStyle = UNumberFormatStyle(8i32);
pub const UNUM_PATTERN_RULEBASED: UNumberFormatStyle = UNumberFormatStyle(9i32);
pub const UNUM_CURRENCY_ISO: UNumberFormatStyle = UNumberFormatStyle(10i32);
pub const UNUM_CURRENCY_PLURAL: UNumberFormatStyle = UNumberFormatStyle(11i32);
pub const UNUM_CURRENCY_ACCOUNTING: UNumberFormatStyle = UNumberFormatStyle(12i32);
pub const UNUM_CASH_CURRENCY: UNumberFormatStyle = UNumberFormatStyle(13i32);
pub const UNUM_DECIMAL_COMPACT_SHORT: UNumberFormatStyle = UNumberFormatStyle(14i32);
pub const UNUM_DECIMAL_COMPACT_LONG: UNumberFormatStyle = UNumberFormatStyle(15i32);
pub const UNUM_CURRENCY_STANDARD: UNumberFormatStyle = UNumberFormatStyle(16i32);
pub const UNUM_DEFAULT: UNumberFormatStyle = UNumberFormatStyle(1i32);
pub const UNUM_IGNORE: UNumberFormatStyle = UNumberFormatStyle(0i32);
#[repr(transparent)]
pub struct UNumberFormatSymbol(pub i32);
pub const UNUM_DECIMAL_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(0i32);
pub const UNUM_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(1i32);
pub const UNUM_PATTERN_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(2i32);
pub const UNUM_PERCENT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(3i32);
pub const UNUM_ZERO_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(4i32);
pub const UNUM_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(5i32);
pub const UNUM_MINUS_SIGN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(6i32);
pub const UNUM_PLUS_SIGN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(7i32);
pub const UNUM_CURRENCY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(8i32);
pub const UNUM_INTL_CURRENCY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(9i32);
pub const UNUM_MONETARY_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(10i32);
pub const UNUM_EXPONENTIAL_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(11i32);
pub const UNUM_PERMILL_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(12i32);
pub const UNUM_PAD_ESCAPE_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(13i32);
pub const UNUM_INFINITY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(14i32);
pub const UNUM_NAN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(15i32);
pub const UNUM_SIGNIFICANT_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(16i32);
pub const UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(17i32);
pub const UNUM_ONE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(18i32);
pub const UNUM_TWO_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(19i32);
pub const UNUM_THREE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(20i32);
pub const UNUM_FOUR_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(21i32);
pub const UNUM_FIVE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(22i32);
pub const UNUM_SIX_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(23i32);
pub const UNUM_SEVEN_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(24i32);
pub const UNUM_EIGHT_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(25i32);
pub const UNUM_NINE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(26i32);
pub const UNUM_EXPONENT_MULTIPLICATION_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(27i32);
#[repr(transparent)]
pub struct UNumberFormatTextAttribute(pub i32);
pub const UNUM_POSITIVE_PREFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(0i32);
pub const UNUM_POSITIVE_SUFFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(1i32);
pub const UNUM_NEGATIVE_PREFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(2i32);
pub const UNUM_NEGATIVE_SUFFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(3i32);
pub const UNUM_PADDING_CHARACTER: UNumberFormatTextAttribute = UNumberFormatTextAttribute(4i32);
pub const UNUM_CURRENCY_CODE: UNumberFormatTextAttribute = UNumberFormatTextAttribute(5i32);
pub const UNUM_DEFAULT_RULESET: UNumberFormatTextAttribute = UNumberFormatTextAttribute(6i32);
pub const UNUM_PUBLIC_RULESETS: UNumberFormatTextAttribute = UNumberFormatTextAttribute(7i32);
#[repr(C)]
pub struct UNumberFormatter(i32);
#[repr(transparent)]
pub struct UNumberGroupingStrategy(pub i32);
pub const UNUM_GROUPING_OFF: UNumberGroupingStrategy = UNumberGroupingStrategy(0i32);
pub const UNUM_GROUPING_MIN2: UNumberGroupingStrategy = UNumberGroupingStrategy(1i32);
pub const UNUM_GROUPING_AUTO: UNumberGroupingStrategy = UNumberGroupingStrategy(2i32);
pub const UNUM_GROUPING_ON_ALIGNED: UNumberGroupingStrategy = UNumberGroupingStrategy(3i32);
pub const UNUM_GROUPING_THOUSANDS: UNumberGroupingStrategy = UNumberGroupingStrategy(4i32);
#[repr(transparent)]
pub struct UNumberRangeCollapse(pub i32);
pub const UNUM_RANGE_COLLAPSE_AUTO: UNumberRangeCollapse = UNumberRangeCollapse(0i32);
pub const UNUM_RANGE_COLLAPSE_NONE: UNumberRangeCollapse = UNumberRangeCollapse(1i32);
pub const UNUM_RANGE_COLLAPSE_UNIT: UNumberRangeCollapse = UNumberRangeCollapse(2i32);
pub const UNUM_RANGE_COLLAPSE_ALL: UNumberRangeCollapse = UNumberRangeCollapse(3i32);
#[repr(transparent)]
pub struct UNumberRangeIdentityFallback(pub i32);
pub const UNUM_IDENTITY_FALLBACK_SINGLE_VALUE: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(0i32);
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY_OR_SINGLE_VALUE: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(1i32);
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(2i32);
pub const UNUM_IDENTITY_FALLBACK_RANGE: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(3i32);
#[repr(transparent)]
pub struct UNumberRangeIdentityResult(pub i32);
pub const UNUM_IDENTITY_RESULT_EQUAL_BEFORE_ROUNDING: UNumberRangeIdentityResult = UNumberRangeIdentityResult(0i32);
pub const UNUM_IDENTITY_RESULT_EQUAL_AFTER_ROUNDING: UNumberRangeIdentityResult = UNumberRangeIdentityResult(1i32);
pub const UNUM_IDENTITY_RESULT_NOT_EQUAL: UNumberRangeIdentityResult = UNumberRangeIdentityResult(2i32);
#[repr(transparent)]
pub struct UNumberSignDisplay(pub i32);
pub const UNUM_SIGN_AUTO: UNumberSignDisplay = UNumberSignDisplay(0i32);
pub const UNUM_SIGN_ALWAYS: UNumberSignDisplay = UNumberSignDisplay(1i32);
pub const UNUM_SIGN_NEVER: UNumberSignDisplay = UNumberSignDisplay(2i32);
pub const UNUM_SIGN_ACCOUNTING: UNumberSignDisplay = UNumberSignDisplay(3i32);
pub const UNUM_SIGN_ACCOUNTING_ALWAYS: UNumberSignDisplay = UNumberSignDisplay(4i32);
pub const UNUM_SIGN_EXCEPT_ZERO: UNumberSignDisplay = UNumberSignDisplay(5i32);
pub const UNUM_SIGN_ACCOUNTING_EXCEPT_ZERO: UNumberSignDisplay = UNumberSignDisplay(6i32);
pub const UNUM_SIGN_COUNT: UNumberSignDisplay = UNumberSignDisplay(7i32);
#[repr(transparent)]
pub struct UNumberUnitWidth(pub i32);
pub const UNUM_UNIT_WIDTH_NARROW: UNumberUnitWidth = UNumberUnitWidth(0i32);
pub const UNUM_UNIT_WIDTH_SHORT: UNumberUnitWidth = UNumberUnitWidth(1i32);
pub const UNUM_UNIT_WIDTH_FULL_NAME: UNumberUnitWidth = UNumberUnitWidth(2i32);
pub const UNUM_UNIT_WIDTH_ISO_CODE: UNumberUnitWidth = UNumberUnitWidth(3i32);
pub const UNUM_UNIT_WIDTH_HIDDEN: UNumberUnitWidth = UNumberUnitWidth(4i32);
pub const UNUM_UNIT_WIDTH_COUNT: UNumberUnitWidth = UNumberUnitWidth(5i32);
#[repr(C)]
pub struct UNumberingSystem(i32);
#[repr(transparent)]
pub struct UNumericType(pub i32);
pub const U_NT_NONE: UNumericType = UNumericType(0i32);
pub const U_NT_DECIMAL: UNumericType = UNumericType(1i32);
pub const U_NT_DIGIT: UNumericType = UNumericType(2i32);
pub const U_NT_NUMERIC: UNumericType = UNumericType(3i32);
#[repr(C)]
pub struct UParseError(i32);
#[repr(C)]
pub struct UPluralRules(i32);
#[repr(transparent)]
pub struct UPluralType(pub i32);
pub const UPLURAL_TYPE_CARDINAL: UPluralType = UPluralType(0i32);
pub const UPLURAL_TYPE_ORDINAL: UPluralType = UPluralType(1i32);
#[repr(transparent)]
pub struct UProperty(pub i32);
pub const UCHAR_ALPHABETIC: UProperty = UProperty(0i32);
pub const UCHAR_BINARY_START: UProperty = UProperty(0i32);
pub const UCHAR_ASCII_HEX_DIGIT: UProperty = UProperty(1i32);
pub const UCHAR_BIDI_CONTROL: UProperty = UProperty(2i32);
pub const UCHAR_BIDI_MIRRORED: UProperty = UProperty(3i32);
pub const UCHAR_DASH: UProperty = UProperty(4i32);
pub const UCHAR_DEFAULT_IGNORABLE_CODE_POINT: UProperty = UProperty(5i32);
pub const UCHAR_DEPRECATED: UProperty = UProperty(6i32);
pub const UCHAR_DIACRITIC: UProperty = UProperty(7i32);
pub const UCHAR_EXTENDER: UProperty = UProperty(8i32);
pub const UCHAR_FULL_COMPOSITION_EXCLUSION: UProperty = UProperty(9i32);
pub const UCHAR_GRAPHEME_BASE: UProperty = UProperty(10i32);
pub const UCHAR_GRAPHEME_EXTEND: UProperty = UProperty(11i32);
pub const UCHAR_GRAPHEME_LINK: UProperty = UProperty(12i32);
pub const UCHAR_HEX_DIGIT: UProperty = UProperty(13i32);
pub const UCHAR_HYPHEN: UProperty = UProperty(14i32);
pub const UCHAR_ID_CONTINUE: UProperty = UProperty(15i32);
pub const UCHAR_ID_START: UProperty = UProperty(16i32);
pub const UCHAR_IDEOGRAPHIC: UProperty = UProperty(17i32);
pub const UCHAR_IDS_BINARY_OPERATOR: UProperty = UProperty(18i32);
pub const UCHAR_IDS_TRINARY_OPERATOR: UProperty = UProperty(19i32);
pub const UCHAR_JOIN_CONTROL: UProperty = UProperty(20i32);
pub const UCHAR_LOGICAL_ORDER_EXCEPTION: UProperty = UProperty(21i32);
pub const UCHAR_LOWERCASE: UProperty = UProperty(22i32);
pub const UCHAR_MATH: UProperty = UProperty(23i32);
pub const UCHAR_NONCHARACTER_CODE_POINT: UProperty = UProperty(24i32);
pub const UCHAR_QUOTATION_MARK: UProperty = UProperty(25i32);
pub const UCHAR_RADICAL: UProperty = UProperty(26i32);
pub const UCHAR_SOFT_DOTTED: UProperty = UProperty(27i32);
pub const UCHAR_TERMINAL_PUNCTUATION: UProperty = UProperty(28i32);
pub const UCHAR_UNIFIED_IDEOGRAPH: UProperty = UProperty(29i32);
pub const UCHAR_UPPERCASE: UProperty = UProperty(30i32);
pub const UCHAR_WHITE_SPACE: UProperty = UProperty(31i32);
pub const UCHAR_XID_CONTINUE: UProperty = UProperty(32i32);
pub const UCHAR_XID_START: UProperty = UProperty(33i32);
pub const UCHAR_CASE_SENSITIVE: UProperty = UProperty(34i32);
pub const UCHAR_S_TERM: UProperty = UProperty(35i32);
pub const UCHAR_VARIATION_SELECTOR: UProperty = UProperty(36i32);
pub const UCHAR_NFD_INERT: UProperty = UProperty(37i32);
pub const UCHAR_NFKD_INERT: UProperty = UProperty(38i32);
pub const UCHAR_NFC_INERT: UProperty = UProperty(39i32);
pub const UCHAR_NFKC_INERT: UProperty = UProperty(40i32);
pub const UCHAR_SEGMENT_STARTER: UProperty = UProperty(41i32);
pub const UCHAR_PATTERN_SYNTAX: UProperty = UProperty(42i32);
pub const UCHAR_PATTERN_WHITE_SPACE: UProperty = UProperty(43i32);
pub const UCHAR_POSIX_ALNUM: UProperty = UProperty(44i32);
pub const UCHAR_POSIX_BLANK: UProperty = UProperty(45i32);
pub const UCHAR_POSIX_GRAPH: UProperty = UProperty(46i32);
pub const UCHAR_POSIX_PRINT: UProperty = UProperty(47i32);
pub const UCHAR_POSIX_XDIGIT: UProperty = UProperty(48i32);
pub const UCHAR_CASED: UProperty = UProperty(49i32);
pub const UCHAR_CASE_IGNORABLE: UProperty = UProperty(50i32);
pub const UCHAR_CHANGES_WHEN_LOWERCASED: UProperty = UProperty(51i32);
pub const UCHAR_CHANGES_WHEN_UPPERCASED: UProperty = UProperty(52i32);
pub const UCHAR_CHANGES_WHEN_TITLECASED: UProperty = UProperty(53i32);
pub const UCHAR_CHANGES_WHEN_CASEFOLDED: UProperty = UProperty(54i32);
pub const UCHAR_CHANGES_WHEN_CASEMAPPED: UProperty = UProperty(55i32);
pub const UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED: UProperty = UProperty(56i32);
pub const UCHAR_EMOJI: UProperty = UProperty(57i32);
pub const UCHAR_EMOJI_PRESENTATION: UProperty = UProperty(58i32);
pub const UCHAR_EMOJI_MODIFIER: UProperty = UProperty(59i32);
pub const UCHAR_EMOJI_MODIFIER_BASE: UProperty = UProperty(60i32);
pub const UCHAR_EMOJI_COMPONENT: UProperty = UProperty(61i32);
pub const UCHAR_REGIONAL_INDICATOR: UProperty = UProperty(62i32);
pub const UCHAR_PREPENDED_CONCATENATION_MARK: UProperty = UProperty(63i32);
pub const UCHAR_EXTENDED_PICTOGRAPHIC: UProperty = UProperty(64i32);
pub const UCHAR_BIDI_CLASS: UProperty = UProperty(4096i32);
pub const UCHAR_INT_START: UProperty = UProperty(4096i32);
pub const UCHAR_BLOCK: UProperty = UProperty(4097i32);
pub const UCHAR_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4098i32);
pub const UCHAR_DECOMPOSITION_TYPE: UProperty = UProperty(4099i32);
pub const UCHAR_EAST_ASIAN_WIDTH: UProperty = UProperty(4100i32);
pub const UCHAR_GENERAL_CATEGORY: UProperty = UProperty(4101i32);
pub const UCHAR_JOINING_GROUP: UProperty = UProperty(4102i32);
pub const UCHAR_JOINING_TYPE: UProperty = UProperty(4103i32);
pub const UCHAR_LINE_BREAK: UProperty = UProperty(4104i32);
pub const UCHAR_NUMERIC_TYPE: UProperty = UProperty(4105i32);
pub const UCHAR_SCRIPT: UProperty = UProperty(4106i32);
pub const UCHAR_HANGUL_SYLLABLE_TYPE: UProperty = UProperty(4107i32);
pub const UCHAR_NFD_QUICK_CHECK: UProperty = UProperty(4108i32);
pub const UCHAR_NFKD_QUICK_CHECK: UProperty = UProperty(4109i32);
pub const UCHAR_NFC_QUICK_CHECK: UProperty = UProperty(4110i32);
pub const UCHAR_NFKC_QUICK_CHECK: UProperty = UProperty(4111i32);
pub const UCHAR_LEAD_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4112i32);
pub const UCHAR_TRAIL_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4113i32);
pub const UCHAR_GRAPHEME_CLUSTER_BREAK: UProperty = UProperty(4114i32);
pub const UCHAR_SENTENCE_BREAK: UProperty = UProperty(4115i32);
pub const UCHAR_WORD_BREAK: UProperty = UProperty(4116i32);
pub const UCHAR_BIDI_PAIRED_BRACKET_TYPE: UProperty = UProperty(4117i32);
pub const UCHAR_INDIC_POSITIONAL_CATEGORY: UProperty = UProperty(4118i32);
pub const UCHAR_INDIC_SYLLABIC_CATEGORY: UProperty = UProperty(4119i32);
pub const UCHAR_VERTICAL_ORIENTATION: UProperty = UProperty(4120i32);
pub const UCHAR_GENERAL_CATEGORY_MASK: UProperty = UProperty(8192i32);
pub const UCHAR_MASK_START: UProperty = UProperty(8192i32);
pub const UCHAR_NUMERIC_VALUE: UProperty = UProperty(12288i32);
pub const UCHAR_DOUBLE_START: UProperty = UProperty(12288i32);
pub const UCHAR_AGE: UProperty = UProperty(16384i32);
pub const UCHAR_STRING_START: UProperty = UProperty(16384i32);
pub const UCHAR_BIDI_MIRRORING_GLYPH: UProperty = UProperty(16385i32);
pub const UCHAR_CASE_FOLDING: UProperty = UProperty(16386i32);
pub const UCHAR_LOWERCASE_MAPPING: UProperty = UProperty(16388i32);
pub const UCHAR_NAME: UProperty = UProperty(16389i32);
pub const UCHAR_SIMPLE_CASE_FOLDING: UProperty = UProperty(16390i32);
pub const UCHAR_SIMPLE_LOWERCASE_MAPPING: UProperty = UProperty(16391i32);
pub const UCHAR_SIMPLE_TITLECASE_MAPPING: UProperty = UProperty(16392i32);
pub const UCHAR_SIMPLE_UPPERCASE_MAPPING: UProperty = UProperty(16393i32);
pub const UCHAR_TITLECASE_MAPPING: UProperty = UProperty(16394i32);
pub const UCHAR_UPPERCASE_MAPPING: UProperty = UProperty(16396i32);
pub const UCHAR_BIDI_PAIRED_BRACKET: UProperty = UProperty(16397i32);
pub const UCHAR_SCRIPT_EXTENSIONS: UProperty = UProperty(28672i32);
pub const UCHAR_OTHER_PROPERTY_START: UProperty = UProperty(28672i32);
pub const UCHAR_INVALID_CODE: UProperty = UProperty(-1i32);
#[repr(transparent)]
pub struct UPropertyNameChoice(pub i32);
pub const U_SHORT_PROPERTY_NAME: UPropertyNameChoice = UPropertyNameChoice(0i32);
pub const U_LONG_PROPERTY_NAME: UPropertyNameChoice = UPropertyNameChoice(1i32);
pub type URegexFindProgressCallback = unsafe extern "system" fn(context: *const ::core::ffi::c_void, matchindex: i64) -> i8;
pub type URegexMatchCallback = unsafe extern "system" fn(context: *const ::core::ffi::c_void, steps: i32) -> i8;
#[repr(transparent)]
pub struct URegexpFlag(pub i32);
pub const UREGEX_CASE_INSENSITIVE: URegexpFlag = URegexpFlag(2i32);
pub const UREGEX_COMMENTS: URegexpFlag = URegexpFlag(4i32);
pub const UREGEX_DOTALL: URegexpFlag = URegexpFlag(32i32);
pub const UREGEX_LITERAL: URegexpFlag = URegexpFlag(16i32);
pub const UREGEX_MULTILINE: URegexpFlag = URegexpFlag(8i32);
pub const UREGEX_UNIX_LINES: URegexpFlag = URegexpFlag(1i32);
pub const UREGEX_UWORD: URegexpFlag = URegexpFlag(256i32);
pub const UREGEX_ERROR_ON_UNKNOWN_ESCAPES: URegexpFlag = URegexpFlag(512i32);
#[repr(C)]
pub struct URegion(i32);
#[repr(transparent)]
pub struct URegionType(pub i32);
pub const URGN_UNKNOWN: URegionType = URegionType(0i32);
pub const URGN_TERRITORY: URegionType = URegionType(1i32);
pub const URGN_WORLD: URegionType = URegionType(2i32);
pub const URGN_CONTINENT: URegionType = URegionType(3i32);
pub const URGN_SUBCONTINENT: URegionType = URegionType(4i32);
pub const URGN_GROUPING: URegionType = URegionType(5i32);
pub const URGN_DEPRECATED: URegionType = URegionType(6i32);
#[repr(C)]
pub struct URegularExpression(i32);
#[repr(C)]
pub struct URelativeDateTimeFormatter(i32);
#[repr(transparent)]
pub struct URelativeDateTimeFormatterField(pub i32);
pub const UDAT_REL_LITERAL_FIELD: URelativeDateTimeFormatterField = URelativeDateTimeFormatterField(0i32);
pub const UDAT_REL_NUMERIC_FIELD: URelativeDateTimeFormatterField = URelativeDateTimeFormatterField(1i32);
#[repr(transparent)]
pub struct URelativeDateTimeUnit(pub i32);
pub const UDAT_REL_UNIT_YEAR: URelativeDateTimeUnit = URelativeDateTimeUnit(0i32);
pub const UDAT_REL_UNIT_QUARTER: URelativeDateTimeUnit = URelativeDateTimeUnit(1i32);
pub const UDAT_REL_UNIT_MONTH: URelativeDateTimeUnit = URelativeDateTimeUnit(2i32);
pub const UDAT_REL_UNIT_WEEK: URelativeDateTimeUnit = URelativeDateTimeUnit(3i32);
pub const UDAT_REL_UNIT_DAY: URelativeDateTimeUnit = URelativeDateTimeUnit(4i32);
pub const UDAT_REL_UNIT_HOUR: URelativeDateTimeUnit = URelativeDateTimeUnit(5i32);
pub const UDAT_REL_UNIT_MINUTE: URelativeDateTimeUnit = URelativeDateTimeUnit(6i32);
pub const UDAT_REL_UNIT_SECOND: URelativeDateTimeUnit = URelativeDateTimeUnit(7i32);
pub const UDAT_REL_UNIT_SUNDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(8i32);
pub const UDAT_REL_UNIT_MONDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(9i32);
pub const UDAT_REL_UNIT_TUESDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(10i32);
pub const UDAT_REL_UNIT_WEDNESDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(11i32);
pub const UDAT_REL_UNIT_THURSDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(12i32);
pub const UDAT_REL_UNIT_FRIDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(13i32);
pub const UDAT_REL_UNIT_SATURDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(14i32);
#[repr(C)]
pub struct UReplaceableCallbacks(i32);
#[repr(transparent)]
pub struct UResType(pub i32);
pub const URES_NONE: UResType = UResType(-1i32);
pub const URES_STRING: UResType = UResType(0i32);
pub const URES_BINARY: UResType = UResType(1i32);
pub const URES_TABLE: UResType = UResType(2i32);
pub const URES_ALIAS: UResType = UResType(3i32);
pub const URES_INT: UResType = UResType(7i32);
pub const URES_ARRAY: UResType = UResType(8i32);
pub const URES_INT_VECTOR: UResType = UResType(14i32);
#[repr(C)]
pub struct UResourceBundle(i32);
#[repr(transparent)]
pub struct URestrictionLevel(pub i32);
pub const USPOOF_ASCII: URestrictionLevel = URestrictionLevel(268435456i32);
pub const USPOOF_SINGLE_SCRIPT_RESTRICTIVE: URestrictionLevel = URestrictionLevel(536870912i32);
pub const USPOOF_HIGHLY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(805306368i32);
pub const USPOOF_MODERATELY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(1073741824i32);
pub const USPOOF_MINIMALLY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(1342177280i32);
pub const USPOOF_UNRESTRICTIVE: URestrictionLevel = URestrictionLevel(1610612736i32);
pub const USPOOF_RESTRICTION_LEVEL_MASK: URestrictionLevel = URestrictionLevel(2130706432i32);
pub const USEARCH_DONE: i32 = -1i32;
pub const USET_ADD_CASE_MAPPINGS: i32 = 4i32;
pub const USET_CASE_INSENSITIVE: i32 = 2i32;
pub const USET_IGNORE_SPACE: i32 = 1i32;
pub const USET_SERIALIZED_STATIC_ARRAY_CAPACITY: i32 = 8i32;
pub const USPREP_ALLOW_UNASSIGNED: u32 = 1u32;
pub const USPREP_DEFAULT: u32 = 0u32;
pub const USP_E_SCRIPT_NOT_IN_FONT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
#[repr(transparent)]
pub struct UScriptCode(pub i32);
pub const USCRIPT_INVALID_CODE: UScriptCode = UScriptCode(-1i32);
pub const USCRIPT_COMMON: UScriptCode = UScriptCode(0i32);
pub const USCRIPT_INHERITED: UScriptCode = UScriptCode(1i32);
pub const USCRIPT_ARABIC: UScriptCode = UScriptCode(2i32);
pub const USCRIPT_ARMENIAN: UScriptCode = UScriptCode(3i32);
pub const USCRIPT_BENGALI: UScriptCode = UScriptCode(4i32);
pub const USCRIPT_BOPOMOFO: UScriptCode = UScriptCode(5i32);
pub const USCRIPT_CHEROKEE: UScriptCode = UScriptCode(6i32);
pub const USCRIPT_COPTIC: UScriptCode = UScriptCode(7i32);
pub const USCRIPT_CYRILLIC: UScriptCode = UScriptCode(8i32);
pub const USCRIPT_DESERET: UScriptCode = UScriptCode(9i32);
pub const USCRIPT_DEVANAGARI: UScriptCode = UScriptCode(10i32);
pub const USCRIPT_ETHIOPIC: UScriptCode = UScriptCode(11i32);
pub const USCRIPT_GEORGIAN: UScriptCode = UScriptCode(12i32);
pub const USCRIPT_GOTHIC: UScriptCode = UScriptCode(13i32);
pub const USCRIPT_GREEK: UScriptCode = UScriptCode(14i32);
pub const USCRIPT_GUJARATI: UScriptCode = UScriptCode(15i32);
pub const USCRIPT_GURMUKHI: UScriptCode = UScriptCode(16i32);
pub const USCRIPT_HAN: UScriptCode = UScriptCode(17i32);
pub const USCRIPT_HANGUL: UScriptCode = UScriptCode(18i32);
pub const USCRIPT_HEBREW: UScriptCode = UScriptCode(19i32);
pub const USCRIPT_HIRAGANA: UScriptCode = UScriptCode(20i32);
pub const USCRIPT_KANNADA: UScriptCode = UScriptCode(21i32);
pub const USCRIPT_KATAKANA: UScriptCode = UScriptCode(22i32);
pub const USCRIPT_KHMER: UScriptCode = UScriptCode(23i32);
pub const USCRIPT_LAO: UScriptCode = UScriptCode(24i32);
pub const USCRIPT_LATIN: UScriptCode = UScriptCode(25i32);
pub const USCRIPT_MALAYALAM: UScriptCode = UScriptCode(26i32);
pub const USCRIPT_MONGOLIAN: UScriptCode = UScriptCode(27i32);
pub const USCRIPT_MYANMAR: UScriptCode = UScriptCode(28i32);
pub const USCRIPT_OGHAM: UScriptCode = UScriptCode(29i32);
pub const USCRIPT_OLD_ITALIC: UScriptCode = UScriptCode(30i32);
pub const USCRIPT_ORIYA: UScriptCode = UScriptCode(31i32);
pub const USCRIPT_RUNIC: UScriptCode = UScriptCode(32i32);
pub const USCRIPT_SINHALA: UScriptCode = UScriptCode(33i32);
pub const USCRIPT_SYRIAC: UScriptCode = UScriptCode(34i32);
pub const USCRIPT_TAMIL: UScriptCode = UScriptCode(35i32);
pub const USCRIPT_TELUGU: UScriptCode = UScriptCode(36i32);
pub const USCRIPT_THAANA: UScriptCode = UScriptCode(37i32);
pub const USCRIPT_THAI: UScriptCode = UScriptCode(38i32);
pub const USCRIPT_TIBETAN: UScriptCode = UScriptCode(39i32);
pub const USCRIPT_CANADIAN_ABORIGINAL: UScriptCode = UScriptCode(40i32);
pub const USCRIPT_UCAS: UScriptCode = UScriptCode(40i32);
pub const USCRIPT_YI: UScriptCode = UScriptCode(41i32);
pub const USCRIPT_TAGALOG: UScriptCode = UScriptCode(42i32);
pub const USCRIPT_HANUNOO: UScriptCode = UScriptCode(43i32);
pub const USCRIPT_BUHID: UScriptCode = UScriptCode(44i32);
pub const USCRIPT_TAGBANWA: UScriptCode = UScriptCode(45i32);
pub const USCRIPT_BRAILLE: UScriptCode = UScriptCode(46i32);
pub const USCRIPT_CYPRIOT: UScriptCode = UScriptCode(47i32);
pub const USCRIPT_LIMBU: UScriptCode = UScriptCode(48i32);
pub const USCRIPT_LINEAR_B: UScriptCode = UScriptCode(49i32);
pub const USCRIPT_OSMANYA: UScriptCode = UScriptCode(50i32);
pub const USCRIPT_SHAVIAN: UScriptCode = UScriptCode(51i32);
pub const USCRIPT_TAI_LE: UScriptCode = UScriptCode(52i32);
pub const USCRIPT_UGARITIC: UScriptCode = UScriptCode(53i32);
pub const USCRIPT_KATAKANA_OR_HIRAGANA: UScriptCode = UScriptCode(54i32);
pub const USCRIPT_BUGINESE: UScriptCode = UScriptCode(55i32);
pub const USCRIPT_GLAGOLITIC: UScriptCode = UScriptCode(56i32);
pub const USCRIPT_KHAROSHTHI: UScriptCode = UScriptCode(57i32);
pub const USCRIPT_SYLOTI_NAGRI: UScriptCode = UScriptCode(58i32);
pub const USCRIPT_NEW_TAI_LUE: UScriptCode = UScriptCode(59i32);
pub const USCRIPT_TIFINAGH: UScriptCode = UScriptCode(60i32);
pub const USCRIPT_OLD_PERSIAN: UScriptCode = UScriptCode(61i32);
pub const USCRIPT_BALINESE: UScriptCode = UScriptCode(62i32);
pub const USCRIPT_BATAK: UScriptCode = UScriptCode(63i32);
pub const USCRIPT_BLISSYMBOLS: UScriptCode = UScriptCode(64i32);
pub const USCRIPT_BRAHMI: UScriptCode = UScriptCode(65i32);
pub const USCRIPT_CHAM: UScriptCode = UScriptCode(66i32);
pub const USCRIPT_CIRTH: UScriptCode = UScriptCode(67i32);
pub const USCRIPT_OLD_CHURCH_SLAVONIC_CYRILLIC: UScriptCode = UScriptCode(68i32);
pub const USCRIPT_DEMOTIC_EGYPTIAN: UScriptCode = UScriptCode(69i32);
pub const USCRIPT_HIERATIC_EGYPTIAN: UScriptCode = UScriptCode(70i32);
pub const USCRIPT_EGYPTIAN_HIEROGLYPHS: UScriptCode = UScriptCode(71i32);
pub const USCRIPT_KHUTSURI: UScriptCode = UScriptCode(72i32);
pub const USCRIPT_SIMPLIFIED_HAN: UScriptCode = UScriptCode(73i32);
pub const USCRIPT_TRADITIONAL_HAN: UScriptCode = UScriptCode(74i32);
pub const USCRIPT_PAHAWH_HMONG: UScriptCode = UScriptCode(75i32);
pub const USCRIPT_OLD_HUNGARIAN: UScriptCode = UScriptCode(76i32);
pub const USCRIPT_HARAPPAN_INDUS: UScriptCode = UScriptCode(77i32);
pub const USCRIPT_JAVANESE: UScriptCode = UScriptCode(78i32);
pub const USCRIPT_KAYAH_LI: UScriptCode = UScriptCode(79i32);
pub const USCRIPT_LATIN_FRAKTUR: UScriptCode = UScriptCode(80i32);
pub const USCRIPT_LATIN_GAELIC: UScriptCode = UScriptCode(81i32);
pub const USCRIPT_LEPCHA: UScriptCode = UScriptCode(82i32);
pub const USCRIPT_LINEAR_A: UScriptCode = UScriptCode(83i32);
pub const USCRIPT_MANDAIC: UScriptCode = UScriptCode(84i32);
pub const USCRIPT_MANDAEAN: UScriptCode = UScriptCode(84i32);
pub const USCRIPT_MAYAN_HIEROGLYPHS: UScriptCode = UScriptCode(85i32);
pub const USCRIPT_MEROITIC_HIEROGLYPHS: UScriptCode = UScriptCode(86i32);
pub const USCRIPT_MEROITIC: UScriptCode = UScriptCode(86i32);
pub const USCRIPT_NKO: UScriptCode = UScriptCode(87i32);
pub const USCRIPT_ORKHON: UScriptCode = UScriptCode(88i32);
pub const USCRIPT_OLD_PERMIC: UScriptCode = UScriptCode(89i32);
pub const USCRIPT_PHAGS_PA: UScriptCode = UScriptCode(90i32);
pub const USCRIPT_PHOENICIAN: UScriptCode = UScriptCode(91i32);
pub const USCRIPT_MIAO: UScriptCode = UScriptCode(92i32);
pub const USCRIPT_PHONETIC_POLLARD: UScriptCode = UScriptCode(92i32);
pub const USCRIPT_RONGORONGO: UScriptCode = UScriptCode(93i32);
pub const USCRIPT_SARATI: UScriptCode = UScriptCode(94i32);
pub const USCRIPT_ESTRANGELO_SYRIAC: UScriptCode = UScriptCode(95i32);
pub const USCRIPT_WESTERN_SYRIAC: UScriptCode = UScriptCode(96i32);
pub const USCRIPT_EASTERN_SYRIAC: UScriptCode = UScriptCode(97i32);
pub const USCRIPT_TENGWAR: UScriptCode = UScriptCode(98i32);
pub const USCRIPT_VAI: UScriptCode = UScriptCode(99i32);
pub const USCRIPT_VISIBLE_SPEECH: UScriptCode = UScriptCode(100i32);
pub const USCRIPT_CUNEIFORM: UScriptCode = UScriptCode(101i32);
pub const USCRIPT_UNWRITTEN_LANGUAGES: UScriptCode = UScriptCode(102i32);
pub const USCRIPT_UNKNOWN: UScriptCode = UScriptCode(103i32);
pub const USCRIPT_CARIAN: UScriptCode = UScriptCode(104i32);
pub const USCRIPT_JAPANESE: UScriptCode = UScriptCode(105i32);
pub const USCRIPT_LANNA: UScriptCode = UScriptCode(106i32);
pub const USCRIPT_LYCIAN: UScriptCode = UScriptCode(107i32);
pub const USCRIPT_LYDIAN: UScriptCode = UScriptCode(108i32);
pub const USCRIPT_OL_CHIKI: UScriptCode = UScriptCode(109i32);
pub const USCRIPT_REJANG: UScriptCode = UScriptCode(110i32);
pub const USCRIPT_SAURASHTRA: UScriptCode = UScriptCode(111i32);
pub const USCRIPT_SIGN_WRITING: UScriptCode = UScriptCode(112i32);
pub const USCRIPT_SUNDANESE: UScriptCode = UScriptCode(113i32);
pub const USCRIPT_MOON: UScriptCode = UScriptCode(114i32);
pub const USCRIPT_MEITEI_MAYEK: UScriptCode = UScriptCode(115i32);
pub const USCRIPT_IMPERIAL_ARAMAIC: UScriptCode = UScriptCode(116i32);
pub const USCRIPT_AVESTAN: UScriptCode = UScriptCode(117i32);
pub const USCRIPT_CHAKMA: UScriptCode = UScriptCode(118i32);
pub const USCRIPT_KOREAN: UScriptCode = UScriptCode(119i32);
pub const USCRIPT_KAITHI: UScriptCode = UScriptCode(120i32);
pub const USCRIPT_MANICHAEAN: UScriptCode = UScriptCode(121i32);
pub const USCRIPT_INSCRIPTIONAL_PAHLAVI: UScriptCode = UScriptCode(122i32);
pub const USCRIPT_PSALTER_PAHLAVI: UScriptCode = UScriptCode(123i32);
pub const USCRIPT_BOOK_PAHLAVI: UScriptCode = UScriptCode(124i32);
pub const USCRIPT_INSCRIPTIONAL_PARTHIAN: UScriptCode = UScriptCode(125i32);
pub const USCRIPT_SAMARITAN: UScriptCode = UScriptCode(126i32);
pub const USCRIPT_TAI_VIET: UScriptCode = UScriptCode(127i32);
pub const USCRIPT_MATHEMATICAL_NOTATION: UScriptCode = UScriptCode(128i32);
pub const USCRIPT_SYMBOLS: UScriptCode = UScriptCode(129i32);
pub const USCRIPT_BAMUM: UScriptCode = UScriptCode(130i32);
pub const USCRIPT_LISU: UScriptCode = UScriptCode(131i32);
pub const USCRIPT_NAKHI_GEBA: UScriptCode = UScriptCode(132i32);
pub const USCRIPT_OLD_SOUTH_ARABIAN: UScriptCode = UScriptCode(133i32);
pub const USCRIPT_BASSA_VAH: UScriptCode = UScriptCode(134i32);
pub const USCRIPT_DUPLOYAN: UScriptCode = UScriptCode(135i32);
pub const USCRIPT_ELBASAN: UScriptCode = UScriptCode(136i32);
pub const USCRIPT_GRANTHA: UScriptCode = UScriptCode(137i32);
pub const USCRIPT_KPELLE: UScriptCode = UScriptCode(138i32);
pub const USCRIPT_LOMA: UScriptCode = UScriptCode(139i32);
pub const USCRIPT_MENDE: UScriptCode = UScriptCode(140i32);
pub const USCRIPT_MEROITIC_CURSIVE: UScriptCode = UScriptCode(141i32);
pub const USCRIPT_OLD_NORTH_ARABIAN: UScriptCode = UScriptCode(142i32);
pub const USCRIPT_NABATAEAN: UScriptCode = UScriptCode(143i32);
pub const USCRIPT_PALMYRENE: UScriptCode = UScriptCode(144i32);
pub const USCRIPT_KHUDAWADI: UScriptCode = UScriptCode(145i32);
pub const USCRIPT_SINDHI: UScriptCode = UScriptCode(145i32);
pub const USCRIPT_WARANG_CITI: UScriptCode = UScriptCode(146i32);
pub const USCRIPT_AFAKA: UScriptCode = UScriptCode(147i32);
pub const USCRIPT_JURCHEN: UScriptCode = UScriptCode(148i32);
pub const USCRIPT_MRO: UScriptCode = UScriptCode(149i32);
pub const USCRIPT_NUSHU: UScriptCode = UScriptCode(150i32);
pub const USCRIPT_SHARADA: UScriptCode = UScriptCode(151i32);
pub const USCRIPT_SORA_SOMPENG: UScriptCode = UScriptCode(152i32);
pub const USCRIPT_TAKRI: UScriptCode = UScriptCode(153i32);
pub const USCRIPT_TANGUT: UScriptCode = UScriptCode(154i32);
pub const USCRIPT_WOLEAI: UScriptCode = UScriptCode(155i32);
pub const USCRIPT_ANATOLIAN_HIEROGLYPHS: UScriptCode = UScriptCode(156i32);
pub const USCRIPT_KHOJKI: UScriptCode = UScriptCode(157i32);
pub const USCRIPT_TIRHUTA: UScriptCode = UScriptCode(158i32);
pub const USCRIPT_CAUCASIAN_ALBANIAN: UScriptCode = UScriptCode(159i32);
pub const USCRIPT_MAHAJANI: UScriptCode = UScriptCode(160i32);
pub const USCRIPT_AHOM: UScriptCode = UScriptCode(161i32);
pub const USCRIPT_HATRAN: UScriptCode = UScriptCode(162i32);
pub const USCRIPT_MODI: UScriptCode = UScriptCode(163i32);
pub const USCRIPT_MULTANI: UScriptCode = UScriptCode(164i32);
pub const USCRIPT_PAU_CIN_HAU: UScriptCode = UScriptCode(165i32);
pub const USCRIPT_SIDDHAM: UScriptCode = UScriptCode(166i32);
pub const USCRIPT_ADLAM: UScriptCode = UScriptCode(167i32);
pub const USCRIPT_BHAIKSUKI: UScriptCode = UScriptCode(168i32);
pub const USCRIPT_MARCHEN: UScriptCode = UScriptCode(169i32);
pub const USCRIPT_NEWA: UScriptCode = UScriptCode(170i32);
pub const USCRIPT_OSAGE: UScriptCode = UScriptCode(171i32);
pub const USCRIPT_HAN_WITH_BOPOMOFO: UScriptCode = UScriptCode(172i32);
pub const USCRIPT_JAMO: UScriptCode = UScriptCode(173i32);
pub const USCRIPT_SYMBOLS_EMOJI: UScriptCode = UScriptCode(174i32);
pub const USCRIPT_MASARAM_GONDI: UScriptCode = UScriptCode(175i32);
pub const USCRIPT_SOYOMBO: UScriptCode = UScriptCode(176i32);
pub const USCRIPT_ZANABAZAR_SQUARE: UScriptCode = UScriptCode(177i32);
pub const USCRIPT_DOGRA: UScriptCode = UScriptCode(178i32);
pub const USCRIPT_GUNJALA_GONDI: UScriptCode = UScriptCode(179i32);
pub const USCRIPT_MAKASAR: UScriptCode = UScriptCode(180i32);
pub const USCRIPT_MEDEFAIDRIN: UScriptCode = UScriptCode(181i32);
pub const USCRIPT_HANIFI_ROHINGYA: UScriptCode = UScriptCode(182i32);
pub const USCRIPT_SOGDIAN: UScriptCode = UScriptCode(183i32);
pub const USCRIPT_OLD_SOGDIAN: UScriptCode = UScriptCode(184i32);
pub const USCRIPT_ELYMAIC: UScriptCode = UScriptCode(185i32);
pub const USCRIPT_NYIAKENG_PUACHUE_HMONG: UScriptCode = UScriptCode(186i32);
pub const USCRIPT_NANDINAGARI: UScriptCode = UScriptCode(187i32);
pub const USCRIPT_WANCHO: UScriptCode = UScriptCode(188i32);
pub const USCRIPT_CHORASMIAN: UScriptCode = UScriptCode(189i32);
pub const USCRIPT_DIVES_AKURU: UScriptCode = UScriptCode(190i32);
pub const USCRIPT_KHITAN_SMALL_SCRIPT: UScriptCode = UScriptCode(191i32);
pub const USCRIPT_YEZIDI: UScriptCode = UScriptCode(192i32);
#[repr(transparent)]
pub struct UScriptUsage(pub i32);
pub const USCRIPT_USAGE_NOT_ENCODED: UScriptUsage = UScriptUsage(0i32);
pub const USCRIPT_USAGE_UNKNOWN: UScriptUsage = UScriptUsage(1i32);
pub const USCRIPT_USAGE_EXCLUDED: UScriptUsage = UScriptUsage(2i32);
pub const USCRIPT_USAGE_LIMITED_USE: UScriptUsage = UScriptUsage(3i32);
pub const USCRIPT_USAGE_ASPIRATIONAL: UScriptUsage = UScriptUsage(4i32);
pub const USCRIPT_USAGE_RECOMMENDED: UScriptUsage = UScriptUsage(5i32);
#[repr(C)]
pub struct USearch(i32);
#[repr(transparent)]
pub struct USearchAttribute(pub i32);
pub const USEARCH_OVERLAP: USearchAttribute = USearchAttribute(0i32);
pub const USEARCH_ELEMENT_COMPARISON: USearchAttribute = USearchAttribute(2i32);
#[repr(transparent)]
pub struct USearchAttributeValue(pub i32);
pub const USEARCH_DEFAULT: USearchAttributeValue = USearchAttributeValue(-1i32);
pub const USEARCH_OFF: USearchAttributeValue = USearchAttributeValue(0i32);
pub const USEARCH_ON: USearchAttributeValue = USearchAttributeValue(1i32);
pub const USEARCH_STANDARD_ELEMENT_COMPARISON: USearchAttributeValue = USearchAttributeValue(2i32);
pub const USEARCH_PATTERN_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = USearchAttributeValue(3i32);
pub const USEARCH_ANY_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = USearchAttributeValue(4i32);
#[repr(transparent)]
pub struct USentenceBreak(pub i32);
pub const U_SB_OTHER: USentenceBreak = USentenceBreak(0i32);
pub const U_SB_ATERM: USentenceBreak = USentenceBreak(1i32);
pub const U_SB_CLOSE: USentenceBreak = USentenceBreak(2i32);
pub const U_SB_FORMAT: USentenceBreak = USentenceBreak(3i32);
pub const U_SB_LOWER: USentenceBreak = USentenceBreak(4i32);
pub const U_SB_NUMERIC: USentenceBreak = USentenceBreak(5i32);
pub const U_SB_OLETTER: USentenceBreak = USentenceBreak(6i32);
pub const U_SB_SEP: USentenceBreak = USentenceBreak(7i32);
pub const U_SB_SP: USentenceBreak = USentenceBreak(8i32);
pub const U_SB_STERM: USentenceBreak = USentenceBreak(9i32);
pub const U_SB_UPPER: USentenceBreak = USentenceBreak(10i32);
pub const U_SB_CR: USentenceBreak = USentenceBreak(11i32);
pub const U_SB_EXTEND: USentenceBreak = USentenceBreak(12i32);
pub const U_SB_LF: USentenceBreak = USentenceBreak(13i32);
pub const U_SB_SCONTINUE: USentenceBreak = USentenceBreak(14i32);
#[repr(transparent)]
pub struct USentenceBreakTag(pub i32);
pub const UBRK_SENTENCE_TERM: USentenceBreakTag = USentenceBreakTag(0i32);
pub const UBRK_SENTENCE_TERM_LIMIT: USentenceBreakTag = USentenceBreakTag(100i32);
pub const UBRK_SENTENCE_SEP: USentenceBreakTag = USentenceBreakTag(100i32);
pub const UBRK_SENTENCE_SEP_LIMIT: USentenceBreakTag = USentenceBreakTag(200i32);
#[repr(C)]
pub struct USerializedSet(i32);
#[repr(C)]
pub struct USet(i32);
#[repr(transparent)]
pub struct USetSpanCondition(pub i32);
pub const USET_SPAN_NOT_CONTAINED: USetSpanCondition = USetSpanCondition(0i32);
pub const USET_SPAN_CONTAINED: USetSpanCondition = USetSpanCondition(1i32);
pub const USET_SPAN_SIMPLE: USetSpanCondition = USetSpanCondition(2i32);
#[repr(C)]
pub struct USpoofCheckResult(i32);
#[repr(C)]
pub struct USpoofChecker(i32);
#[repr(transparent)]
pub struct USpoofChecks(pub i32);
pub const USPOOF_SINGLE_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(1i32);
pub const USPOOF_MIXED_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(2i32);
pub const USPOOF_WHOLE_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(4i32);
pub const USPOOF_CONFUSABLE: USpoofChecks = USpoofChecks(7i32);
pub const USPOOF_RESTRICTION_LEVEL: USpoofChecks = USpoofChecks(16i32);
pub const USPOOF_INVISIBLE: USpoofChecks = USpoofChecks(32i32);
pub const USPOOF_CHAR_LIMIT: USpoofChecks = USpoofChecks(64i32);
pub const USPOOF_MIXED_NUMBERS: USpoofChecks = USpoofChecks(128i32);
pub const USPOOF_HIDDEN_OVERLAY: USpoofChecks = USpoofChecks(256i32);
pub const USPOOF_ALL_CHECKS: USpoofChecks = USpoofChecks(65535i32);
pub const USPOOF_AUX_INFO: USpoofChecks = USpoofChecks(1073741824i32);
pub type UStringCaseMapper = unsafe extern "system" fn(csm: *const UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
#[repr(C)]
pub struct UStringPrepProfile(i32);
#[repr(transparent)]
pub struct UStringPrepProfileType(pub i32);
pub const USPREP_RFC3491_NAMEPREP: UStringPrepProfileType = UStringPrepProfileType(0i32);
pub const USPREP_RFC3530_NFS4_CS_PREP: UStringPrepProfileType = UStringPrepProfileType(1i32);
pub const USPREP_RFC3530_NFS4_CS_PREP_CI: UStringPrepProfileType = UStringPrepProfileType(2i32);
pub const USPREP_RFC3530_NFS4_CIS_PREP: UStringPrepProfileType = UStringPrepProfileType(3i32);
pub const USPREP_RFC3530_NFS4_MIXED_PREP_PREFIX: UStringPrepProfileType = UStringPrepProfileType(4i32);
pub const USPREP_RFC3530_NFS4_MIXED_PREP_SUFFIX: UStringPrepProfileType = UStringPrepProfileType(5i32);
pub const USPREP_RFC3722_ISCSI: UStringPrepProfileType = UStringPrepProfileType(6i32);
pub const USPREP_RFC3920_NODEPREP: UStringPrepProfileType = UStringPrepProfileType(7i32);
pub const USPREP_RFC3920_RESOURCEPREP: UStringPrepProfileType = UStringPrepProfileType(8i32);
pub const USPREP_RFC4011_MIB: UStringPrepProfileType = UStringPrepProfileType(9i32);
pub const USPREP_RFC4013_SASLPREP: UStringPrepProfileType = UStringPrepProfileType(10i32);
pub const USPREP_RFC4505_TRACE: UStringPrepProfileType = UStringPrepProfileType(11i32);
pub const USPREP_RFC4518_LDAP: UStringPrepProfileType = UStringPrepProfileType(12i32);
pub const USPREP_RFC4518_LDAP_CI: UStringPrepProfileType = UStringPrepProfileType(13i32);
#[repr(C)]
pub struct UStringSearch(i32);
#[repr(transparent)]
pub struct UStringTrieBuildOption(pub i32);
pub const USTRINGTRIE_BUILD_FAST: UStringTrieBuildOption = UStringTrieBuildOption(0i32);
pub const USTRINGTRIE_BUILD_SMALL: UStringTrieBuildOption = UStringTrieBuildOption(1i32);
#[repr(transparent)]
pub struct UStringTrieResult(pub i32);
pub const USTRINGTRIE_NO_MATCH: UStringTrieResult = UStringTrieResult(0i32);
pub const USTRINGTRIE_NO_VALUE: UStringTrieResult = UStringTrieResult(1i32);
pub const USTRINGTRIE_FINAL_VALUE: UStringTrieResult = UStringTrieResult(2i32);
pub const USTRINGTRIE_INTERMEDIATE_VALUE: UStringTrieResult = UStringTrieResult(3i32);
#[repr(transparent)]
pub struct USystemTimeZoneType(pub i32);
pub const UCAL_ZONE_TYPE_ANY: USystemTimeZoneType = USystemTimeZoneType(0i32);
pub const UCAL_ZONE_TYPE_CANONICAL: USystemTimeZoneType = USystemTimeZoneType(1i32);
pub const UCAL_ZONE_TYPE_CANONICAL_LOCATION: USystemTimeZoneType = USystemTimeZoneType(2i32);
pub const UTEXT_MAGIC: i32 = 878368812i32;
pub const UTEXT_PROVIDER_HAS_META_DATA: i32 = 4i32;
pub const UTEXT_PROVIDER_LENGTH_IS_EXPENSIVE: i32 = 1i32;
pub const UTEXT_PROVIDER_OWNS_TEXT: i32 = 5i32;
pub const UTEXT_PROVIDER_STABLE_CHUNKS: i32 = 2i32;
pub const UTEXT_PROVIDER_WRITABLE: i32 = 3i32;
pub const UTF16_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF32_MAX_CHAR_LENGTH: u32 = 1u32;
pub const UTF8_ERROR_VALUE_1: u32 = 21u32;
pub const UTF8_ERROR_VALUE_2: u32 = 159u32;
pub const UTF8_MAX_CHAR_LENGTH: u32 = 4u32;
pub const UTF_ERROR_VALUE: u32 = 65535u32;
pub const UTF_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF_SIZE: u32 = 16u32;
#[repr(C)]
pub struct UText(i32);
pub type UTextAccess = unsafe extern "system" fn(ut: *mut UText, nativeindex: i64, forward: i8) -> i8;
pub type UTextClone = unsafe extern "system" fn(dest: *mut UText, src: *const UText, deep: i8, status: *mut UErrorCode) -> *mut UText;
pub type UTextClose = unsafe extern "system" fn(ut: *mut UText);
pub type UTextCopy = unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, nativedest: i64, r#move: i8, status: *mut UErrorCode);
pub type UTextExtract = unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
#[repr(C)]
pub struct UTextFuncs(i32);
pub type UTextMapNativeIndexToUTF16 = unsafe extern "system" fn(ut: *const UText, nativeindex: i64) -> i32;
pub type UTextMapOffsetToNative = unsafe extern "system" fn(ut: *const UText) -> i64;
pub type UTextNativeLength = unsafe extern "system" fn(ut: *mut UText) -> i64;
pub type UTextReplace = unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacmentlength: i32, status: *mut UErrorCode) -> i32;
#[repr(transparent)]
pub struct UTimeScaleValue(pub i32);
pub const UTSV_UNITS_VALUE: UTimeScaleValue = UTimeScaleValue(0i32);
pub const UTSV_EPOCH_OFFSET_VALUE: UTimeScaleValue = UTimeScaleValue(1i32);
pub const UTSV_FROM_MIN_VALUE: UTimeScaleValue = UTimeScaleValue(2i32);
pub const UTSV_FROM_MAX_VALUE: UTimeScaleValue = UTimeScaleValue(3i32);
pub const UTSV_TO_MIN_VALUE: UTimeScaleValue = UTimeScaleValue(4i32);
pub const UTSV_TO_MAX_VALUE: UTimeScaleValue = UTimeScaleValue(5i32);
#[repr(transparent)]
pub struct UTimeZoneFormatGMTOffsetPatternType(pub i32);
pub const UTZFMT_PAT_POSITIVE_HM: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(0i32);
pub const UTZFMT_PAT_POSITIVE_HMS: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(1i32);
pub const UTZFMT_PAT_NEGATIVE_HM: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(2i32);
pub const UTZFMT_PAT_NEGATIVE_HMS: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(3i32);
pub const UTZFMT_PAT_POSITIVE_H: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(4i32);
pub const UTZFMT_PAT_NEGATIVE_H: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(5i32);
pub const UTZFMT_PAT_COUNT: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(6i32);
#[repr(transparent)]
pub struct UTimeZoneFormatParseOption(pub i32);
pub const UTZFMT_PARSE_OPTION_NONE: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(0i32);
pub const UTZFMT_PARSE_OPTION_ALL_STYLES: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(1i32);
pub const UTZFMT_PARSE_OPTION_TZ_DATABASE_ABBREVIATIONS: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(2i32);
#[repr(transparent)]
pub struct UTimeZoneFormatStyle(pub i32);
pub const UTZFMT_STYLE_GENERIC_LOCATION: UTimeZoneFormatStyle = UTimeZoneFormatStyle(0i32);
pub const UTZFMT_STYLE_GENERIC_LONG: UTimeZoneFormatStyle = UTimeZoneFormatStyle(1i32);
pub const UTZFMT_STYLE_GENERIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(2i32);
pub const UTZFMT_STYLE_SPECIFIC_LONG: UTimeZoneFormatStyle = UTimeZoneFormatStyle(3i32);
pub const UTZFMT_STYLE_SPECIFIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(4i32);
pub const UTZFMT_STYLE_LOCALIZED_GMT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(5i32);
pub const UTZFMT_STYLE_LOCALIZED_GMT_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(6i32);
pub const UTZFMT_STYLE_ISO_BASIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(7i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(8i32);
pub const UTZFMT_STYLE_ISO_BASIC_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(9i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(10i32);
pub const UTZFMT_STYLE_ISO_BASIC_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(11i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(12i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(13i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_LOCAL_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(14i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(15i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_LOCAL_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(16i32);
pub const UTZFMT_STYLE_ZONE_ID: UTimeZoneFormatStyle = UTimeZoneFormatStyle(17i32);
pub const UTZFMT_STYLE_ZONE_ID_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(18i32);
pub const UTZFMT_STYLE_EXEMPLAR_LOCATION: UTimeZoneFormatStyle = UTimeZoneFormatStyle(19i32);
#[repr(transparent)]
pub struct UTimeZoneFormatTimeType(pub i32);
pub const UTZFMT_TIME_TYPE_UNKNOWN: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(0i32);
pub const UTZFMT_TIME_TYPE_STANDARD: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(1i32);
pub const UTZFMT_TIME_TYPE_DAYLIGHT: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(2i32);
#[repr(transparent)]
pub struct UTimeZoneNameType(pub i32);
pub const UTZNM_UNKNOWN: UTimeZoneNameType = UTimeZoneNameType(0i32);
pub const UTZNM_LONG_GENERIC: UTimeZoneNameType = UTimeZoneNameType(1i32);
pub const UTZNM_LONG_STANDARD: UTimeZoneNameType = UTimeZoneNameType(2i32);
pub const UTZNM_LONG_DAYLIGHT: UTimeZoneNameType = UTimeZoneNameType(4i32);
pub const UTZNM_SHORT_GENERIC: UTimeZoneNameType = UTimeZoneNameType(8i32);
pub const UTZNM_SHORT_STANDARD: UTimeZoneNameType = UTimeZoneNameType(16i32);
pub const UTZNM_SHORT_DAYLIGHT: UTimeZoneNameType = UTimeZoneNameType(32i32);
pub const UTZNM_EXEMPLAR_LOCATION: UTimeZoneNameType = UTimeZoneNameType(64i32);
#[repr(transparent)]
pub struct UTimeZoneTransitionType(pub i32);
pub const UCAL_TZ_TRANSITION_NEXT: UTimeZoneTransitionType = UTimeZoneTransitionType(0i32);
pub const UCAL_TZ_TRANSITION_NEXT_INCLUSIVE: UTimeZoneTransitionType = UTimeZoneTransitionType(1i32);
pub const UCAL_TZ_TRANSITION_PREVIOUS: UTimeZoneTransitionType = UTimeZoneTransitionType(2i32);
pub const UCAL_TZ_TRANSITION_PREVIOUS_INCLUSIVE: UTimeZoneTransitionType = UTimeZoneTransitionType(3i32);
#[cfg(feature = "Win32_Foundation")]
pub type UTraceData = unsafe extern "system" fn(context: *const ::core::ffi::c_void, fnnumber: i32, level: i32, fmt: super::Foundation::PSTR, args: *mut i8);
pub type UTraceEntry = unsafe extern "system" fn(context: *const ::core::ffi::c_void, fnnumber: i32);
#[cfg(feature = "Win32_Foundation")]
pub type UTraceExit = unsafe extern "system" fn(context: *const ::core::ffi::c_void, fnnumber: i32, fmt: super::Foundation::PSTR, args: *mut i8);
#[repr(transparent)]
pub struct UTraceFunctionNumber(pub i32);
pub const UTRACE_FUNCTION_START: UTraceFunctionNumber = UTraceFunctionNumber(0i32);
pub const UTRACE_U_INIT: UTraceFunctionNumber = UTraceFunctionNumber(0i32);
pub const UTRACE_U_CLEANUP: UTraceFunctionNumber = UTraceFunctionNumber(1i32);
pub const UTRACE_CONVERSION_START: UTraceFunctionNumber = UTraceFunctionNumber(4096i32);
pub const UTRACE_UCNV_OPEN: UTraceFunctionNumber = UTraceFunctionNumber(4096i32);
pub const UTRACE_UCNV_OPEN_PACKAGE: UTraceFunctionNumber = UTraceFunctionNumber(4097i32);
pub const UTRACE_UCNV_OPEN_ALGORITHMIC: UTraceFunctionNumber = UTraceFunctionNumber(4098i32);
pub const UTRACE_UCNV_CLONE: UTraceFunctionNumber = UTraceFunctionNumber(4099i32);
pub const UTRACE_UCNV_CLOSE: UTraceFunctionNumber = UTraceFunctionNumber(4100i32);
pub const UTRACE_UCNV_FLUSH_CACHE: UTraceFunctionNumber = UTraceFunctionNumber(4101i32);
pub const UTRACE_UCNV_LOAD: UTraceFunctionNumber = UTraceFunctionNumber(4102i32);
pub const UTRACE_UCNV_UNLOAD: UTraceFunctionNumber = UTraceFunctionNumber(4103i32);
pub const UTRACE_COLLATION_START: UTraceFunctionNumber = UTraceFunctionNumber(8192i32);
pub const UTRACE_UCOL_OPEN: UTraceFunctionNumber = UTraceFunctionNumber(8192i32);
pub const UTRACE_UCOL_CLOSE: UTraceFunctionNumber = UTraceFunctionNumber(8193i32);
pub const UTRACE_UCOL_STRCOLL: UTraceFunctionNumber = UTraceFunctionNumber(8194i32);
pub const UTRACE_UCOL_GET_SORTKEY: UTraceFunctionNumber = UTraceFunctionNumber(8195i32);
pub const UTRACE_UCOL_GETLOCALE: UTraceFunctionNumber = UTraceFunctionNumber(8196i32);
pub const UTRACE_UCOL_NEXTSORTKEYPART: UTraceFunctionNumber = UTraceFunctionNumber(8197i32);
pub const UTRACE_UCOL_STRCOLLITER: UTraceFunctionNumber = UTraceFunctionNumber(8198i32);
pub const UTRACE_UCOL_OPEN_FROM_SHORT_STRING: UTraceFunctionNumber = UTraceFunctionNumber(8199i32);
pub const UTRACE_UCOL_STRCOLLUTF8: UTraceFunctionNumber = UTraceFunctionNumber(8200i32);
pub const UTRACE_UDATA_START: UTraceFunctionNumber = UTraceFunctionNumber(12288i32);
pub const UTRACE_UDATA_RESOURCE: UTraceFunctionNumber = UTraceFunctionNumber(12288i32);
pub const UTRACE_UDATA_BUNDLE: UTraceFunctionNumber = UTraceFunctionNumber(12289i32);
pub const UTRACE_UDATA_DATA_FILE: UTraceFunctionNumber = UTraceFunctionNumber(12290i32);
pub const UTRACE_UDATA_RES_FILE: UTraceFunctionNumber = UTraceFunctionNumber(12291i32);
#[repr(transparent)]
pub struct UTraceLevel(pub i32);
pub const UTRACE_OFF: UTraceLevel = UTraceLevel(-1i32);
pub const UTRACE_ERROR: UTraceLevel = UTraceLevel(0i32);
pub const UTRACE_WARNING: UTraceLevel = UTraceLevel(3i32);
pub const UTRACE_OPEN_CLOSE: UTraceLevel = UTraceLevel(5i32);
pub const UTRACE_INFO: UTraceLevel = UTraceLevel(7i32);
pub const UTRACE_VERBOSE: UTraceLevel = UTraceLevel(9i32);
#[repr(transparent)]
pub struct UTransDirection(pub i32);
pub const UTRANS_FORWARD: UTransDirection = UTransDirection(0i32);
pub const UTRANS_REVERSE: UTransDirection = UTransDirection(1i32);
#[repr(C)]
pub struct UTransPosition(i32);
#[repr(transparent)]
pub struct UVerticalOrientation(pub i32);
pub const U_VO_ROTATED: UVerticalOrientation = UVerticalOrientation(0i32);
pub const U_VO_TRANSFORMED_ROTATED: UVerticalOrientation = UVerticalOrientation(1i32);
pub const U_VO_TRANSFORMED_UPRIGHT: UVerticalOrientation = UVerticalOrientation(2i32);
pub const U_VO_UPRIGHT: UVerticalOrientation = UVerticalOrientation(3i32);
#[repr(transparent)]
pub struct UWordBreak(pub i32);
pub const UBRK_WORD_NONE: UWordBreak = UWordBreak(0i32);
pub const UBRK_WORD_NONE_LIMIT: UWordBreak = UWordBreak(100i32);
pub const UBRK_WORD_NUMBER: UWordBreak = UWordBreak(100i32);
pub const UBRK_WORD_NUMBER_LIMIT: UWordBreak = UWordBreak(200i32);
pub const UBRK_WORD_LETTER: UWordBreak = UWordBreak(200i32);
pub const UBRK_WORD_LETTER_LIMIT: UWordBreak = UWordBreak(300i32);
pub const UBRK_WORD_KANA: UWordBreak = UWordBreak(300i32);
pub const UBRK_WORD_KANA_LIMIT: UWordBreak = UWordBreak(400i32);
pub const UBRK_WORD_IDEO: UWordBreak = UWordBreak(400i32);
pub const UBRK_WORD_IDEO_LIMIT: UWordBreak = UWordBreak(500i32);
#[repr(transparent)]
pub struct UWordBreakValues(pub i32);
pub const U_WB_OTHER: UWordBreakValues = UWordBreakValues(0i32);
pub const U_WB_ALETTER: UWordBreakValues = UWordBreakValues(1i32);
pub const U_WB_FORMAT: UWordBreakValues = UWordBreakValues(2i32);
pub const U_WB_KATAKANA: UWordBreakValues = UWordBreakValues(3i32);
pub const U_WB_MIDLETTER: UWordBreakValues = UWordBreakValues(4i32);
pub const U_WB_MIDNUM: UWordBreakValues = UWordBreakValues(5i32);
pub const U_WB_NUMERIC: UWordBreakValues = UWordBreakValues(6i32);
pub const U_WB_EXTENDNUMLET: UWordBreakValues = UWordBreakValues(7i32);
pub const U_WB_CR: UWordBreakValues = UWordBreakValues(8i32);
pub const U_WB_EXTEND: UWordBreakValues = UWordBreakValues(9i32);
pub const U_WB_LF: UWordBreakValues = UWordBreakValues(10i32);
pub const U_WB_MIDNUMLET: UWordBreakValues = UWordBreakValues(11i32);
pub const U_WB_NEWLINE: UWordBreakValues = UWordBreakValues(12i32);
pub const U_WB_REGIONAL_INDICATOR: UWordBreakValues = UWordBreakValues(13i32);
pub const U_WB_HEBREW_LETTER: UWordBreakValues = UWordBreakValues(14i32);
pub const U_WB_SINGLE_QUOTE: UWordBreakValues = UWordBreakValues(15i32);
pub const U_WB_DOUBLE_QUOTE: UWordBreakValues = UWordBreakValues(16i32);
pub const U_WB_E_BASE: UWordBreakValues = UWordBreakValues(17i32);
pub const U_WB_E_BASE_GAZ: UWordBreakValues = UWordBreakValues(18i32);
pub const U_WB_E_MODIFIER: UWordBreakValues = UWordBreakValues(19i32);
pub const U_WB_GLUE_AFTER_ZWJ: UWordBreakValues = UWordBreakValues(20i32);
pub const U_WB_ZWJ: UWordBreakValues = UWordBreakValues(21i32);
pub const U_WB_WSEGSPACE: UWordBreakValues = UWordBreakValues(22i32);
pub const U_ASCII_FAMILY: u32 = 0u32;
pub const U_CHECK_DYLOAD: u32 = 1u32;
pub const U_COMBINED_IMPLEMENTATION: u32 = 1u32;
pub const U_COMPARE_CODE_POINT_ORDER: u32 = 32768u32;
pub const U_COMPARE_IGNORE_CASE: u32 = 65536u32;
pub const U_COPYRIGHT_STRING_LENGTH: u32 = 128u32;
pub const U_DEFAULT_SHOW_DRAFT: u32 = 0u32;
pub const U_DEFINE_FALSE_AND_TRUE: u32 = 1u32;
pub const U_DISABLE_RENAMING: u32 = 1u32;
pub const U_EBCDIC_FAMILY: u32 = 1u32;
pub const U_EDITS_NO_RESET: u32 = 8192u32;
pub const U_ENABLE_DYLOAD: u32 = 1u32;
pub const U_ENABLE_TRACING: u32 = 0u32;
pub const U_FOLD_CASE_DEFAULT: u32 = 0u32;
pub const U_FOLD_CASE_EXCLUDE_SPECIAL_I: u32 = 1u32;
pub const U_HAVE_RBNF: u32 = 0u32;
pub const U_HAVE_STD_STRING: u32 = 0u32;
pub const U_HIDE_DEPRECATED_API: u32 = 1u32;
pub const U_HIDE_DRAFT_API: u32 = 1u32;
pub const U_HIDE_INTERNAL_API: u32 = 1u32;
pub const U_HIDE_OBSOLETE_API: u32 = 1u32;
pub const U_IOSTREAM_SOURCE: u32 = 199711u32;
pub const U_MAX_VERSION_LENGTH: u32 = 4u32;
pub const U_MAX_VERSION_STRING_LENGTH: u32 = 20u32;
pub const U_MILLIS_PER_DAY: u32 = 86400000u32;
pub const U_MILLIS_PER_HOUR: u32 = 3600000u32;
pub const U_MILLIS_PER_MINUTE: u32 = 60000u32;
pub const U_MILLIS_PER_SECOND: u32 = 1000u32;
pub const U_NO_DEFAULT_INCLUDE_UTF_HEADERS: u32 = 1u32;
pub const U_OMIT_UNCHANGED_TEXT: u32 = 16384u32;
pub const U_OVERRIDE_CXX_ALLOCATION: u32 = 1u32;
pub const U_PARSE_CONTEXT_LEN: i32 = 16i32;
pub const U_PF_AIX: u32 = 3100u32;
pub const U_PF_ANDROID: u32 = 4050u32;
pub const U_PF_BROWSER_NATIVE_CLIENT: u32 = 4020u32;
pub const U_PF_BSD: u32 = 3000u32;
pub const U_PF_CYGWIN: u32 = 1900u32;
pub const U_PF_DARWIN: u32 = 3500u32;
pub const U_PF_EMSCRIPTEN: u32 = 5010u32;
pub const U_PF_FUCHSIA: u32 = 4100u32;
pub const U_PF_HPUX: u32 = 2100u32;
pub const U_PF_IPHONE: u32 = 3550u32;
pub const U_PF_IRIX: u32 = 3200u32;
pub const U_PF_LINUX: u32 = 4000u32;
pub const U_PF_MINGW: u32 = 1800u32;
pub const U_PF_OS390: u32 = 9000u32;
pub const U_PF_OS400: u32 = 9400u32;
pub const U_PF_QNX: u32 = 3700u32;
pub const U_PF_SOLARIS: u32 = 2600u32;
pub const U_PF_UNKNOWN: u32 = 0u32;
pub const U_PF_WINDOWS: u32 = 1000u32;
pub const U_SENTINEL: i32 = -1i32;
pub const U_SHAPE_AGGREGATE_TASHKEEL: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_MASK: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_AL: u32 = 128u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_LR: u32 = 96u32;
pub const U_SHAPE_DIGITS_AN2EN: u32 = 64u32;
pub const U_SHAPE_DIGITS_EN2AN: u32 = 32u32;
pub const U_SHAPE_DIGITS_MASK: u32 = 224u32;
pub const U_SHAPE_DIGITS_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_RESERVED: u32 = 160u32;
pub const U_SHAPE_DIGIT_TYPE_AN: u32 = 0u32;
pub const U_SHAPE_DIGIT_TYPE_AN_EXTENDED: u32 = 256u32;
pub const U_SHAPE_DIGIT_TYPE_MASK: u32 = 768u32;
pub const U_SHAPE_DIGIT_TYPE_RESERVED: u32 = 512u32;
pub const U_SHAPE_LAMALEF_AUTO: u32 = 65536u32;
pub const U_SHAPE_LAMALEF_BEGIN: u32 = 3u32;
pub const U_SHAPE_LAMALEF_END: u32 = 2u32;
pub const U_SHAPE_LAMALEF_MASK: u32 = 65539u32;
pub const U_SHAPE_LAMALEF_NEAR: u32 = 1u32;
pub const U_SHAPE_LAMALEF_RESIZE: u32 = 0u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING: u32 = 3u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_END: u32 = 2u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_NEAR: u32 = 1u32;
pub const U_SHAPE_LENGTH_GROW_SHRINK: u32 = 0u32;
pub const U_SHAPE_LENGTH_MASK: u32 = 65539u32;
pub const U_SHAPE_LETTERS_MASK: u32 = 24u32;
pub const U_SHAPE_LETTERS_NOOP: u32 = 0u32;
pub const U_SHAPE_LETTERS_SHAPE: u32 = 8u32;
pub const U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED: u32 = 24u32;
pub const U_SHAPE_LETTERS_UNSHAPE: u32 = 16u32;
pub const U_SHAPE_PRESERVE_PRESENTATION: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_MASK: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_NOOP: u32 = 0u32;
pub const U_SHAPE_SEEN_MASK: u32 = 7340032u32;
pub const U_SHAPE_SEEN_TWOCELL_NEAR: u32 = 2097152u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END: u32 = 67108864u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK: u32 = 67108864u32;
pub const U_SHAPE_TAIL_NEW_UNICODE: u32 = 134217728u32;
pub const U_SHAPE_TAIL_TYPE_MASK: u32 = 134217728u32;
pub const U_SHAPE_TASHKEEL_BEGIN: u32 = 262144u32;
pub const U_SHAPE_TASHKEEL_END: u32 = 393216u32;
pub const U_SHAPE_TASHKEEL_MASK: u32 = 917504u32;
pub const U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL: u32 = 786432u32;
pub const U_SHAPE_TASHKEEL_RESIZE: u32 = 524288u32;
pub const U_SHAPE_TEXT_DIRECTION_LOGICAL: u32 = 0u32;
pub const U_SHAPE_TEXT_DIRECTION_MASK: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_LTR: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_RTL: u32 = 0u32;
pub const U_SHAPE_YEHHAMZA_MASK: u32 = 58720256u32;
pub const U_SHAPE_YEHHAMZA_TWOCELL_NEAR: u32 = 16777216u32;
pub const U_SHOW_CPLUSPLUS_API: u32 = 0u32;
pub const U_SIZEOF_UCHAR: u32 = 2u32;
pub const U_TITLECASE_ADJUST_TO_CASED: u32 = 1024u32;
pub const U_TITLECASE_NO_BREAK_ADJUSTMENT: u32 = 512u32;
pub const U_TITLECASE_NO_LOWERCASE: u32 = 256u32;
pub const U_TITLECASE_SENTENCES: u32 = 64u32;
pub const U_TITLECASE_WHOLE_STRING: u32 = 32u32;
pub const VS_ALLOW_LATIN: u32 = 1u32;
pub const WC_COMPOSITECHECK: u32 = 512u32;
pub const WC_DEFAULTCHAR: u32 = 64u32;
pub const WC_DISCARDNS: u32 = 16u32;
pub const WC_ERR_INVALID_CHARS: u32 = 128u32;
pub const WC_NO_BEST_FIT_CHARS: u32 = 1024u32;
pub const WC_SEPCHARS: u32 = 32u32;
#[repr(transparent)]
pub struct WORDLIST_TYPE(pub i32);
pub const WORDLIST_TYPE_IGNORE: WORDLIST_TYPE = WORDLIST_TYPE(0i32);
pub const WORDLIST_TYPE_ADD: WORDLIST_TYPE = WORDLIST_TYPE(1i32);
pub const WORDLIST_TYPE_EXCLUDE: WORDLIST_TYPE = WORDLIST_TYPE(2i32);
pub const WORDLIST_TYPE_AUTOCORRECT: WORDLIST_TYPE = WORDLIST_TYPE(3i32);
#[repr(C)]
pub struct opentype_feature_record(i32);
#[repr(C)]
pub struct script_charprop(i32);
#[repr(C)]
pub struct script_glyphprop(i32);
#[repr(transparent)]
pub struct tagMLCONVCHARF(pub i32);
pub const MLCONVCHARF_AUTODETECT: tagMLCONVCHARF = tagMLCONVCHARF(1i32);
pub const MLCONVCHARF_ENTITIZE: tagMLCONVCHARF = tagMLCONVCHARF(2i32);
pub const MLCONVCHARF_NCR_ENTITIZE: tagMLCONVCHARF = tagMLCONVCHARF(2i32);
pub const MLCONVCHARF_NAME_ENTITIZE: tagMLCONVCHARF = tagMLCONVCHARF(4i32);
pub const MLCONVCHARF_USEDEFCHAR: tagMLCONVCHARF = tagMLCONVCHARF(8i32);
pub const MLCONVCHARF_NOBESTFITCHARS: tagMLCONVCHARF = tagMLCONVCHARF(16i32);
pub const MLCONVCHARF_DETECTJPN: tagMLCONVCHARF = tagMLCONVCHARF(32i32);
#[repr(transparent)]
pub struct tagMLCPF(pub i32);
pub const MLDETECTF_MAILNEWS: tagMLCPF = tagMLCPF(1i32);
pub const MLDETECTF_BROWSER: tagMLCPF = tagMLCPF(2i32);
pub const MLDETECTF_VALID: tagMLCPF = tagMLCPF(4i32);
pub const MLDETECTF_VALID_NLS: tagMLCPF = tagMLCPF(8i32);
pub const MLDETECTF_PRESERVE_ORDER: tagMLCPF = tagMLCPF(16i32);
pub const MLDETECTF_PREFERRED_ONLY: tagMLCPF = tagMLCPF(32i32);
pub const MLDETECTF_FILTER_SPECIALCHAR: tagMLCPF = tagMLCPF(64i32);
pub const MLDETECTF_EURO_UTF8: tagMLCPF = tagMLCPF(128i32);
#[repr(C)]
pub struct tagSCRIPFONTINFO(i32);
#[repr(C)]
pub struct textrange_properties(i32);