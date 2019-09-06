#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Radio System Control"]
  pub control: CONTROL,
  _reserved1: [u8; 12usize],
  #[doc = "0x10 - Radio Miscellaneous"]
  pub misc: MISC,
  #[doc = "0x14 - RSIM Power Control"]
  pub power: POWER,
  #[doc = "0x18 - Radio Software Configuration"]
  pub sw_config: SW_CONFIG,
  _reserved4: [u8; 228usize],
  #[doc = "0x100 - Deep Sleep Timer"]
  pub dsm_timer: DSM_TIMER,
  #[doc = "0x104 - Deep Sleep Timer Control"]
  pub dsm_control: DSM_CONTROL,
  #[doc = "0x108 - Deep Sleep Wakeup Sequence"]
  pub dsm_wakeup: DSM_WAKEUP,
  #[doc = "0x10c - WOR Deep Sleep Duration"]
  pub wor_duration: WOR_DURATION,
  #[doc = "0x110 - WOR Deep Sleep Wake Time"]
  pub wor_wake: WOR_WAKE,
  _reserved9: [u8; 8usize],
  #[doc = "0x11c - MAN Deep Sleep Time"]
  pub man_sleep: MAN_SLEEP,
  #[doc = "0x120 - MAN Deep Sleep Wake Time"]
  pub man_wake: MAN_WAKE,
  #[doc = "0x124 - Radio Oscillator Control"]
  pub rf_osc_ctrl: RF_OSC_CTRL,
  #[doc = "0x128 - Radio Analog Test Registers"]
  pub ana_test: ANA_TEST,
  #[doc = "0x12c - Radio Analog Trim Registers"]
  pub ana_trim: ANA_TRIM,
}
#[doc = "Radio System Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Radio System Control"]
pub mod control;
#[doc = "Radio Miscellaneous\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "Radio Miscellaneous"]
pub mod misc;
#[doc = "RSIM Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "RSIM Power Control"]
pub mod power;
#[doc = "Radio Software Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_config](sw_config) module"]
pub type SW_CONFIG = crate::Reg<u32, _SW_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CONFIG;
#[doc = "`read()` method returns [sw_config::R](sw_config::R) reader structure"]
impl crate::Readable for SW_CONFIG {}
#[doc = "`write(|w| ..)` method takes [sw_config::W](sw_config::W) writer structure"]
impl crate::Writable for SW_CONFIG {}
#[doc = "Radio Software Configuration"]
pub mod sw_config;
#[doc = "Deep Sleep Timer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsm_timer](dsm_timer) module"]
pub type DSM_TIMER = crate::Reg<u32, _DSM_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSM_TIMER;
#[doc = "`read()` method returns [dsm_timer::R](dsm_timer::R) reader structure"]
impl crate::Readable for DSM_TIMER {}
#[doc = "Deep Sleep Timer"]
pub mod dsm_timer;
#[doc = "Deep Sleep Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsm_control](dsm_control) module"]
pub type DSM_CONTROL = crate::Reg<u32, _DSM_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSM_CONTROL;
#[doc = "`read()` method returns [dsm_control::R](dsm_control::R) reader structure"]
impl crate::Readable for DSM_CONTROL {}
#[doc = "`write(|w| ..)` method takes [dsm_control::W](dsm_control::W) writer structure"]
impl crate::Writable for DSM_CONTROL {}
#[doc = "Deep Sleep Timer Control"]
pub mod dsm_control;
#[doc = "Deep Sleep Wakeup Sequence\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsm_wakeup](dsm_wakeup) module"]
pub type DSM_WAKEUP = crate::Reg<u32, _DSM_WAKEUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSM_WAKEUP;
#[doc = "`read()` method returns [dsm_wakeup::R](dsm_wakeup::R) reader structure"]
impl crate::Readable for DSM_WAKEUP {}
#[doc = "`write(|w| ..)` method takes [dsm_wakeup::W](dsm_wakeup::W) writer structure"]
impl crate::Writable for DSM_WAKEUP {}
#[doc = "Deep Sleep Wakeup Sequence"]
pub mod dsm_wakeup;
#[doc = "WOR Deep Sleep Duration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wor_duration](wor_duration) module"]
pub type WOR_DURATION = crate::Reg<u32, _WOR_DURATION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WOR_DURATION;
#[doc = "`read()` method returns [wor_duration::R](wor_duration::R) reader structure"]
impl crate::Readable for WOR_DURATION {}
#[doc = "WOR Deep Sleep Duration"]
pub mod wor_duration;
#[doc = "WOR Deep Sleep Wake Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wor_wake](wor_wake) module"]
pub type WOR_WAKE = crate::Reg<u32, _WOR_WAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WOR_WAKE;
#[doc = "`read()` method returns [wor_wake::R](wor_wake::R) reader structure"]
impl crate::Readable for WOR_WAKE {}
#[doc = "`write(|w| ..)` method takes [wor_wake::W](wor_wake::W) writer structure"]
impl crate::Writable for WOR_WAKE {}
#[doc = "WOR Deep Sleep Wake Time"]
pub mod wor_wake;
#[doc = "MAN Deep Sleep Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [man_sleep](man_sleep) module"]
pub type MAN_SLEEP = crate::Reg<u32, _MAN_SLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN_SLEEP;
#[doc = "`read()` method returns [man_sleep::R](man_sleep::R) reader structure"]
impl crate::Readable for MAN_SLEEP {}
#[doc = "`write(|w| ..)` method takes [man_sleep::W](man_sleep::W) writer structure"]
impl crate::Writable for MAN_SLEEP {}
#[doc = "MAN Deep Sleep Time"]
pub mod man_sleep;
#[doc = "MAN Deep Sleep Wake Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [man_wake](man_wake) module"]
pub type MAN_WAKE = crate::Reg<u32, _MAN_WAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN_WAKE;
#[doc = "`read()` method returns [man_wake::R](man_wake::R) reader structure"]
impl crate::Readable for MAN_WAKE {}
#[doc = "`write(|w| ..)` method takes [man_wake::W](man_wake::W) writer structure"]
impl crate::Writable for MAN_WAKE {}
#[doc = "MAN Deep Sleep Wake Time"]
pub mod man_wake;
#[doc = "Radio Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rf_osc_ctrl](rf_osc_ctrl) module"]
pub type RF_OSC_CTRL = crate::Reg<u32, _RF_OSC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_OSC_CTRL;
#[doc = "`read()` method returns [rf_osc_ctrl::R](rf_osc_ctrl::R) reader structure"]
impl crate::Readable for RF_OSC_CTRL {}
#[doc = "`write(|w| ..)` method takes [rf_osc_ctrl::W](rf_osc_ctrl::W) writer structure"]
impl crate::Writable for RF_OSC_CTRL {}
#[doc = "Radio Oscillator Control"]
pub mod rf_osc_ctrl;
#[doc = "Radio Analog Test Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ana_test](ana_test) module"]
pub type ANA_TEST = crate::Reg<u32, _ANA_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_TEST;
#[doc = "`read()` method returns [ana_test::R](ana_test::R) reader structure"]
impl crate::Readable for ANA_TEST {}
#[doc = "`write(|w| ..)` method takes [ana_test::W](ana_test::W) writer structure"]
impl crate::Writable for ANA_TEST {}
#[doc = "Radio Analog Test Registers"]
pub mod ana_test;
#[doc = "Radio Analog Trim Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ana_trim](ana_trim) module"]
pub type ANA_TRIM = crate::Reg<u32, _ANA_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_TRIM;
#[doc = "`read()` method returns [ana_trim::R](ana_trim::R) reader structure"]
impl crate::Readable for ANA_TRIM {}
#[doc = "`write(|w| ..)` method takes [ana_trim::W](ana_trim::W) writer structure"]
impl crate::Writable for ANA_TRIM {}
#[doc = "Radio Analog Trim Registers"]
pub mod ana_trim;
