#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - USB VREG Control Register"]
  pub ctrl: CTRL,
  #[doc = "0x04 - USB VREG Configuration Control Register"]
  pub cfgctrl: CFGCTRL,
}
#[doc = "USB VREG Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "USB VREG Control Register"]
pub mod ctrl;
#[doc = "USB VREG Configuration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgctrl](cfgctrl) module"]
pub type CFGCTRL = crate::Reg<u32, _CFGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGCTRL;
#[doc = "`read()` method returns [cfgctrl::R](cfgctrl::R) reader structure"]
impl crate::Readable for CFGCTRL {}
#[doc = "`write(|w| ..)` method takes [cfgctrl::W](cfgctrl::W) writer structure"]
impl crate::Writable for CFGCTRL {}
#[doc = "USB VREG Configuration Control Register"]
pub mod cfgctrl;
