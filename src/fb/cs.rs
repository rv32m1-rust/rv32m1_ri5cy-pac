#[doc = "Chip Select Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csar](csar) module"]
pub type CSAR = crate::Reg<u32, _CSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSAR;
#[doc = "`read()` method returns [csar::R](csar::R) reader structure"]
impl crate::Readable for CSAR {}
#[doc = "`write(|w| ..)` method takes [csar::W](csar::W) writer structure"]
impl crate::Writable for CSAR {}
#[doc = "Chip Select Address Register"]
pub mod csar;
#[doc = "Chip Select Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csmr](csmr) module"]
pub type CSMR = crate::Reg<u32, _CSMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSMR;
#[doc = "`read()` method returns [csmr::R](csmr::R) reader structure"]
impl crate::Readable for CSMR {}
#[doc = "`write(|w| ..)` method takes [csmr::W](csmr::W) writer structure"]
impl crate::Writable for CSMR {}
#[doc = "Chip Select Mask Register"]
pub mod csmr;
#[doc = "Chip Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscr](cscr) module"]
pub type CSCR = crate::Reg<u32, _CSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCR;
#[doc = "`read()` method returns [cscr::R](cscr::R) reader structure"]
impl crate::Readable for CSCR {}
#[doc = "`write(|w| ..)` method takes [cscr::W](cscr::W) writer structure"]
impl crate::Writable for CSCR {}
#[doc = "Chip Select Control Register"]
pub mod cscr;
