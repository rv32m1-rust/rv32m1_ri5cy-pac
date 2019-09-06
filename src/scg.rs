#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub verid: VERID,
  #[doc = "0x04 - Parameter Register"]
  pub param: PARAM,
  _reserved2: [u8; 8usize],
  #[doc = "0x10 - Clock Status Register"]
  pub csr: CSR,
  #[doc = "0x14 - Run Clock Control Register"]
  pub rccr: RCCR,
  #[doc = "0x18 - VLPR Clock Control Register"]
  pub vccr: VCCR,
  #[doc = "0x1c - HSRUN Clock Control Register"]
  pub hccr: HCCR,
  #[doc = "0x20 - SCG CLKOUT Configuration Register"]
  pub clkoutcnfg: CLKOUTCNFG,
  _reserved7: [u8; 220usize],
  #[doc = "0x100 - System OSC Control Status Register"]
  pub sosccsr: SOSCCSR,
  #[doc = "0x104 - System OSC Divide Register"]
  pub soscdiv: SOSCDIV,
  _reserved9: [u8; 248usize],
  #[doc = "0x200 - Slow IRC Control Status Register"]
  pub sirccsr: SIRCCSR,
  #[doc = "0x204 - Slow IRC Divide Register"]
  pub sircdiv: SIRCDIV,
  #[doc = "0x208 - Slow IRC Configuration Register"]
  pub sirccfg: SIRCCFG,
  _reserved12: [u8; 244usize],
  #[doc = "0x300 - Fast IRC Control Status Register"]
  pub firccsr: FIRCCSR,
  #[doc = "0x304 - Fast IRC Divide Register"]
  pub fircdiv: FIRCDIV,
  #[doc = "0x308 - Fast IRC Configuration Register"]
  pub firccfg: FIRCCFG,
  #[doc = "0x30c - Fast IRC Trim Configuration Register"]
  pub firctcfg: FIRCTCFG,
  _reserved16: [u8; 8usize],
  #[doc = "0x318 - Fast IRC Status Register"]
  pub fircstat: FIRCSTAT,
  _reserved17: [u8; 228usize],
  #[doc = "0x400 - RTC OSC Control Status Register"]
  pub rosccsr: ROSCCSR,
  _reserved18: [u8; 252usize],
  #[doc = "0x500 - Low Power FLL Control Status Register"]
  pub lpfllcsr: LPFLLCSR,
  #[doc = "0x504 - Low Power FLL Divide Register"]
  pub lpflldiv: LPFLLDIV,
  #[doc = "0x508 - Low Power FLL Configuration Register"]
  pub lpfllcfg: LPFLLCFG,
  #[doc = "0x50c - Low Power FLL Trim Configuration Register"]
  pub lpflltcfg: LPFLLTCFG,
  _reserved22: [u8; 4usize],
  #[doc = "0x514 - Low Power FLL Status Register"]
  pub lpfllstat: LPFLLSTAT,
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "Clock Status Register"]
pub mod csr;
#[doc = "Run Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rccr](rccr) module"]
pub type RCCR = crate::Reg<u32, _RCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCCR;
#[doc = "`read()` method returns [rccr::R](rccr::R) reader structure"]
impl crate::Readable for RCCR {}
#[doc = "`write(|w| ..)` method takes [rccr::W](rccr::W) writer structure"]
impl crate::Writable for RCCR {}
#[doc = "Run Clock Control Register"]
pub mod rccr;
#[doc = "VLPR Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vccr](vccr) module"]
pub type VCCR = crate::Reg<u32, _VCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCCR;
#[doc = "`read()` method returns [vccr::R](vccr::R) reader structure"]
impl crate::Readable for VCCR {}
#[doc = "`write(|w| ..)` method takes [vccr::W](vccr::W) writer structure"]
impl crate::Writable for VCCR {}
#[doc = "VLPR Clock Control Register"]
pub mod vccr;
#[doc = "HSRUN Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hccr](hccr) module"]
pub type HCCR = crate::Reg<u32, _HCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCR;
#[doc = "`read()` method returns [hccr::R](hccr::R) reader structure"]
impl crate::Readable for HCCR {}
#[doc = "`write(|w| ..)` method takes [hccr::W](hccr::W) writer structure"]
impl crate::Writable for HCCR {}
#[doc = "HSRUN Clock Control Register"]
pub mod hccr;
#[doc = "SCG CLKOUT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkoutcnfg](clkoutcnfg) module"]
pub type CLKOUTCNFG = crate::Reg<u32, _CLKOUTCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTCNFG;
#[doc = "`read()` method returns [clkoutcnfg::R](clkoutcnfg::R) reader structure"]
impl crate::Readable for CLKOUTCNFG {}
#[doc = "`write(|w| ..)` method takes [clkoutcnfg::W](clkoutcnfg::W) writer structure"]
impl crate::Writable for CLKOUTCNFG {}
#[doc = "SCG CLKOUT Configuration Register"]
pub mod clkoutcnfg;
#[doc = "System OSC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sosccsr](sosccsr) module"]
pub type SOSCCSR = crate::Reg<u32, _SOSCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOSCCSR;
#[doc = "`read()` method returns [sosccsr::R](sosccsr::R) reader structure"]
impl crate::Readable for SOSCCSR {}
#[doc = "`write(|w| ..)` method takes [sosccsr::W](sosccsr::W) writer structure"]
impl crate::Writable for SOSCCSR {}
#[doc = "System OSC Control Status Register"]
pub mod sosccsr;
#[doc = "System OSC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [soscdiv](soscdiv) module"]
pub type SOSCDIV = crate::Reg<u32, _SOSCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOSCDIV;
#[doc = "`read()` method returns [soscdiv::R](soscdiv::R) reader structure"]
impl crate::Readable for SOSCDIV {}
#[doc = "`write(|w| ..)` method takes [soscdiv::W](soscdiv::W) writer structure"]
impl crate::Writable for SOSCDIV {}
#[doc = "System OSC Divide Register"]
pub mod soscdiv;
#[doc = "Slow IRC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sirccsr](sirccsr) module"]
pub type SIRCCSR = crate::Reg<u32, _SIRCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIRCCSR;
#[doc = "`read()` method returns [sirccsr::R](sirccsr::R) reader structure"]
impl crate::Readable for SIRCCSR {}
#[doc = "`write(|w| ..)` method takes [sirccsr::W](sirccsr::W) writer structure"]
impl crate::Writable for SIRCCSR {}
#[doc = "Slow IRC Control Status Register"]
pub mod sirccsr;
#[doc = "Slow IRC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sircdiv](sircdiv) module"]
pub type SIRCDIV = crate::Reg<u32, _SIRCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIRCDIV;
#[doc = "`read()` method returns [sircdiv::R](sircdiv::R) reader structure"]
impl crate::Readable for SIRCDIV {}
#[doc = "`write(|w| ..)` method takes [sircdiv::W](sircdiv::W) writer structure"]
impl crate::Writable for SIRCDIV {}
#[doc = "Slow IRC Divide Register"]
pub mod sircdiv;
#[doc = "Slow IRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sirccfg](sirccfg) module"]
pub type SIRCCFG = crate::Reg<u32, _SIRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIRCCFG;
#[doc = "`read()` method returns [sirccfg::R](sirccfg::R) reader structure"]
impl crate::Readable for SIRCCFG {}
#[doc = "`write(|w| ..)` method takes [sirccfg::W](sirccfg::W) writer structure"]
impl crate::Writable for SIRCCFG {}
#[doc = "Slow IRC Configuration Register"]
pub mod sirccfg;
#[doc = "Fast IRC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [firccsr](firccsr) module"]
pub type FIRCCSR = crate::Reg<u32, _FIRCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCCSR;
#[doc = "`read()` method returns [firccsr::R](firccsr::R) reader structure"]
impl crate::Readable for FIRCCSR {}
#[doc = "`write(|w| ..)` method takes [firccsr::W](firccsr::W) writer structure"]
impl crate::Writable for FIRCCSR {}
#[doc = "Fast IRC Control Status Register"]
pub mod firccsr;
#[doc = "Fast IRC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fircdiv](fircdiv) module"]
pub type FIRCDIV = crate::Reg<u32, _FIRCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCDIV;
#[doc = "`read()` method returns [fircdiv::R](fircdiv::R) reader structure"]
impl crate::Readable for FIRCDIV {}
#[doc = "`write(|w| ..)` method takes [fircdiv::W](fircdiv::W) writer structure"]
impl crate::Writable for FIRCDIV {}
#[doc = "Fast IRC Divide Register"]
pub mod fircdiv;
#[doc = "Fast IRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [firccfg](firccfg) module"]
pub type FIRCCFG = crate::Reg<u32, _FIRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCCFG;
#[doc = "`read()` method returns [firccfg::R](firccfg::R) reader structure"]
impl crate::Readable for FIRCCFG {}
#[doc = "`write(|w| ..)` method takes [firccfg::W](firccfg::W) writer structure"]
impl crate::Writable for FIRCCFG {}
#[doc = "Fast IRC Configuration Register"]
pub mod firccfg;
#[doc = "Fast IRC Trim Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [firctcfg](firctcfg) module"]
pub type FIRCTCFG = crate::Reg<u32, _FIRCTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCTCFG;
#[doc = "`read()` method returns [firctcfg::R](firctcfg::R) reader structure"]
impl crate::Readable for FIRCTCFG {}
#[doc = "`write(|w| ..)` method takes [firctcfg::W](firctcfg::W) writer structure"]
impl crate::Writable for FIRCTCFG {}
#[doc = "Fast IRC Trim Configuration Register"]
pub mod firctcfg;
#[doc = "Fast IRC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fircstat](fircstat) module"]
pub type FIRCSTAT = crate::Reg<u32, _FIRCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCSTAT;
#[doc = "`read()` method returns [fircstat::R](fircstat::R) reader structure"]
impl crate::Readable for FIRCSTAT {}
#[doc = "`write(|w| ..)` method takes [fircstat::W](fircstat::W) writer structure"]
impl crate::Writable for FIRCSTAT {}
#[doc = "Fast IRC Status Register"]
pub mod fircstat;
#[doc = "RTC OSC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rosccsr](rosccsr) module"]
pub type ROSCCSR = crate::Reg<u32, _ROSCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSCCSR;
#[doc = "`read()` method returns [rosccsr::R](rosccsr::R) reader structure"]
impl crate::Readable for ROSCCSR {}
#[doc = "`write(|w| ..)` method takes [rosccsr::W](rosccsr::W) writer structure"]
impl crate::Writable for ROSCCSR {}
#[doc = "RTC OSC Control Status Register"]
pub mod rosccsr;
#[doc = "Low Power FLL Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpfllcsr](lpfllcsr) module"]
pub type LPFLLCSR = crate::Reg<u32, _LPFLLCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPFLLCSR;
#[doc = "`read()` method returns [lpfllcsr::R](lpfllcsr::R) reader structure"]
impl crate::Readable for LPFLLCSR {}
#[doc = "`write(|w| ..)` method takes [lpfllcsr::W](lpfllcsr::W) writer structure"]
impl crate::Writable for LPFLLCSR {}
#[doc = "Low Power FLL Control Status Register"]
pub mod lpfllcsr;
#[doc = "Low Power FLL Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpflldiv](lpflldiv) module"]
pub type LPFLLDIV = crate::Reg<u32, _LPFLLDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPFLLDIV;
#[doc = "`read()` method returns [lpflldiv::R](lpflldiv::R) reader structure"]
impl crate::Readable for LPFLLDIV {}
#[doc = "`write(|w| ..)` method takes [lpflldiv::W](lpflldiv::W) writer structure"]
impl crate::Writable for LPFLLDIV {}
#[doc = "Low Power FLL Divide Register"]
pub mod lpflldiv;
#[doc = "Low Power FLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpfllcfg](lpfllcfg) module"]
pub type LPFLLCFG = crate::Reg<u32, _LPFLLCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPFLLCFG;
#[doc = "`read()` method returns [lpfllcfg::R](lpfllcfg::R) reader structure"]
impl crate::Readable for LPFLLCFG {}
#[doc = "`write(|w| ..)` method takes [lpfllcfg::W](lpfllcfg::W) writer structure"]
impl crate::Writable for LPFLLCFG {}
#[doc = "Low Power FLL Configuration Register"]
pub mod lpfllcfg;
#[doc = "Low Power FLL Trim Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpflltcfg](lpflltcfg) module"]
pub type LPFLLTCFG = crate::Reg<u32, _LPFLLTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPFLLTCFG;
#[doc = "`read()` method returns [lpflltcfg::R](lpflltcfg::R) reader structure"]
impl crate::Readable for LPFLLTCFG {}
#[doc = "`write(|w| ..)` method takes [lpflltcfg::W](lpflltcfg::W) writer structure"]
impl crate::Writable for LPFLLTCFG {}
#[doc = "Low Power FLL Trim Configuration Register"]
pub mod lpflltcfg;
#[doc = "Low Power FLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpfllstat](lpfllstat) module"]
pub type LPFLLSTAT = crate::Reg<u32, _LPFLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPFLLSTAT;
#[doc = "`read()` method returns [lpfllstat::R](lpfllstat::R) reader structure"]
impl crate::Readable for LPFLLSTAT {}
#[doc = "`write(|w| ..)` method takes [lpfllstat::W](lpfllstat::W) writer structure"]
impl crate::Writable for LPFLLSTAT {}
#[doc = "Low Power FLL Status Register"]
pub mod lpfllstat;
