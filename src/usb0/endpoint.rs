#[doc = "Endpoint Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [endpt](endpt) module"]
pub type ENDPT = crate::Reg<u8, _ENDPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPT;
#[doc = "`read()` method returns [endpt::R](endpt::R) reader structure"]
impl crate::Readable for ENDPT {}
#[doc = "`write(|w| ..)` method takes [endpt::W](endpt::W) writer structure"]
impl crate::Writable for ENDPT {}
#[doc = "Endpoint Control register"]
pub mod endpt;
