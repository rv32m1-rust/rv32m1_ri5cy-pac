#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - no description available"]
  pub cs: [CS; 6],
  _reserved1: [u8; 24usize],
  #[doc = "0x60 - Chip Select Port Multiplexing Control Register"]
  pub cspmcr: CSPMCR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CS {
  #[doc = "0x00 - Chip Select Address Register"]
  pub csar: self::cs::CSAR,
  #[doc = "0x04 - Chip Select Mask Register"]
  pub csmr: self::cs::CSMR,
  #[doc = "0x08 - Chip Select Control Register"]
  pub cscr: self::cs::CSCR,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod cs;
#[doc = "Chip Select Port Multiplexing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspmcr](cspmcr) module"]
pub type CSPMCR = crate::Reg<u32, _CSPMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPMCR;
#[doc = "`read()` method returns [cspmcr::R](cspmcr::R) reader structure"]
impl crate::Readable for CSPMCR {}
#[doc = "`write(|w| ..)` method takes [cspmcr::W](cspmcr::W) writer structure"]
impl crate::Writable for CSPMCR {}
#[doc = "Chip Select Port Multiplexing Control Register"]
pub mod cspmcr;
