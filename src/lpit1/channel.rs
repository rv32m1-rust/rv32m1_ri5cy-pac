#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tval](tval) module"]
pub type TVAL = crate::Reg<u32, _TVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TVAL;
#[doc = "`read()` method returns [tval::R](tval::R) reader structure"]
impl crate::Readable for TVAL {}
#[doc = "`write(|w| ..)` method takes [tval::W](tval::W) writer structure"]
impl crate::Writable for TVAL {}
#[doc = "Timer Value Register"]
pub mod tval;
#[doc = "Current Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cval](cval) module"]
pub type CVAL = crate::Reg<u32, _CVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL;
#[doc = "`read()` method returns [cval::R](cval::R) reader structure"]
impl crate::Readable for CVAL {}
#[doc = "Current Timer Value"]
pub mod cval;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tctrl](tctrl) module"]
pub type TCTRL = crate::Reg<u32, _TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL;
#[doc = "`read()` method returns [tctrl::R](tctrl::R) reader structure"]
impl crate::Readable for TCTRL {}
#[doc = "`write(|w| ..)` method takes [tctrl::W](tctrl::W) writer structure"]
impl crate::Writable for TCTRL {}
#[doc = "Timer Control Register"]
pub mod tctrl;
