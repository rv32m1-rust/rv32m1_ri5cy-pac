#[doc = "Channel (n) Status and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csc](csc) module"]
pub type CSC = crate::Reg<u32, _CSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSC;
#[doc = "`read()` method returns [csc::R](csc::R) reader structure"]
impl crate::Readable for CSC {}
#[doc = "`write(|w| ..)` method takes [csc::W](csc::W) writer structure"]
impl crate::Writable for CSC {}
#[doc = "Channel (n) Status and Control"]
pub mod csc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Channel (n) Value"]
pub mod cv;
