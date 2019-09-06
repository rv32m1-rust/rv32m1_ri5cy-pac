#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub verid: VERID,
  _reserved1: [u8; 4usize],
  #[doc = "0x08 - Regulator Status Register"]
  pub rsr: RSR,
  _reserved2: [u8; 4usize],
  #[doc = "0x10 - Run Control Register"]
  pub rctrl: RCTRL,
  #[doc = "0x14 - Low Power Control Register"]
  pub lpctrl: LPCTRL,
  _reserved4: [u8; 232usize],
  #[doc = "0x100 - CORE LDO RUN Configuration Register"]
  pub corercnfg: CORERCNFG,
  #[doc = "0x104 - CORE LDO Low Power Configuration register"]
  pub corelpcnfg: CORELPCNFG,
  #[doc = "0x108 - Core LDO Status And Control register"]
  pub coresc: CORESC,
  #[doc = "0x10c - Low Voltage Detect Status and Control 1 register"]
  pub lvdsc1: LVDSC1,
  #[doc = "0x110 - Low Voltage Detect Status and Control 2 register"]
  pub lvdsc2: LVDSC2,
  #[doc = "0x114 - High Voltage Detect Status And Control 1 register"]
  pub hvdsc1: HVDSC1,
  _reserved10: [u8; 232usize],
  #[doc = "0x200 - RF LDO Low Power Configuration register"]
  pub rfldolpcnfg: RFLDOLPCNFG,
  #[doc = "0x204 - RF LDO Status And Control register"]
  pub rfldosc: RFLDOSC,
  _reserved12: [u8; 252usize],
  #[doc = "0x304 - DCDC Status Control Register"]
  pub dcdcsc: DCDCSC,
  _reserved13: [u8; 4usize],
  #[doc = "0x30c - DCDC Control Register 1"]
  pub dcdcc1: DCDCC1,
  #[doc = "0x310 - DCDC Control Register 2"]
  pub dcdcc2: DCDCC2,
  #[doc = "0x314 - DCDC Control Register 3"]
  pub dcdcc3: DCDCC3,
  #[doc = "0x318 - DCDC Control Register 4"]
  pub dcdcc4: DCDCC4,
  _reserved17: [u8; 4usize],
  #[doc = "0x320 - DCDC Control Register 6"]
  pub dcdcc6: DCDCC6,
  _reserved18: [u8; 232usize],
  #[doc = "0x40c - LP Request Pin Control Register"]
  pub lpreqpincntrl: LPREQPINCNTRL,
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
#[doc = "Regulator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "Regulator Status Register"]
pub mod rsr;
#[doc = "Run Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rctrl](rctrl) module"]
pub type RCTRL = crate::Reg<u32, _RCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCTRL;
#[doc = "`read()` method returns [rctrl::R](rctrl::R) reader structure"]
impl crate::Readable for RCTRL {}
#[doc = "`write(|w| ..)` method takes [rctrl::W](rctrl::W) writer structure"]
impl crate::Writable for RCTRL {}
#[doc = "Run Control Register"]
pub mod rctrl;
#[doc = "Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpctrl](lpctrl) module"]
pub type LPCTRL = crate::Reg<u32, _LPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCTRL;
#[doc = "`read()` method returns [lpctrl::R](lpctrl::R) reader structure"]
impl crate::Readable for LPCTRL {}
#[doc = "`write(|w| ..)` method takes [lpctrl::W](lpctrl::W) writer structure"]
impl crate::Writable for LPCTRL {}
#[doc = "Low Power Control Register"]
pub mod lpctrl;
#[doc = "CORE LDO RUN Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [corercnfg](corercnfg) module"]
pub type CORERCNFG = crate::Reg<u32, _CORERCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORERCNFG;
#[doc = "`read()` method returns [corercnfg::R](corercnfg::R) reader structure"]
impl crate::Readable for CORERCNFG {}
#[doc = "`write(|w| ..)` method takes [corercnfg::W](corercnfg::W) writer structure"]
impl crate::Writable for CORERCNFG {}
#[doc = "CORE LDO RUN Configuration Register"]
pub mod corercnfg;
#[doc = "CORE LDO Low Power Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [corelpcnfg](corelpcnfg) module"]
pub type CORELPCNFG = crate::Reg<u32, _CORELPCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORELPCNFG;
#[doc = "`read()` method returns [corelpcnfg::R](corelpcnfg::R) reader structure"]
impl crate::Readable for CORELPCNFG {}
#[doc = "`write(|w| ..)` method takes [corelpcnfg::W](corelpcnfg::W) writer structure"]
impl crate::Writable for CORELPCNFG {}
#[doc = "CORE LDO Low Power Configuration register"]
pub mod corelpcnfg;
#[doc = "Core LDO Status And Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [coresc](coresc) module"]
pub type CORESC = crate::Reg<u32, _CORESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORESC;
#[doc = "`read()` method returns [coresc::R](coresc::R) reader structure"]
impl crate::Readable for CORESC {}
#[doc = "`write(|w| ..)` method takes [coresc::W](coresc::W) writer structure"]
impl crate::Writable for CORESC {}
#[doc = "Core LDO Status And Control register"]
pub mod coresc;
#[doc = "Low Voltage Detect Status and Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lvdsc1](lvdsc1) module"]
pub type LVDSC1 = crate::Reg<u32, _LVDSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LVDSC1;
#[doc = "`read()` method returns [lvdsc1::R](lvdsc1::R) reader structure"]
impl crate::Readable for LVDSC1 {}
#[doc = "`write(|w| ..)` method takes [lvdsc1::W](lvdsc1::W) writer structure"]
impl crate::Writable for LVDSC1 {}
#[doc = "Low Voltage Detect Status and Control 1 register"]
pub mod lvdsc1;
#[doc = "Low Voltage Detect Status and Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lvdsc2](lvdsc2) module"]
pub type LVDSC2 = crate::Reg<u32, _LVDSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LVDSC2;
#[doc = "`read()` method returns [lvdsc2::R](lvdsc2::R) reader structure"]
impl crate::Readable for LVDSC2 {}
#[doc = "`write(|w| ..)` method takes [lvdsc2::W](lvdsc2::W) writer structure"]
impl crate::Writable for LVDSC2 {}
#[doc = "Low Voltage Detect Status and Control 2 register"]
pub mod lvdsc2;
#[doc = "High Voltage Detect Status And Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hvdsc1](hvdsc1) module"]
pub type HVDSC1 = crate::Reg<u32, _HVDSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HVDSC1;
#[doc = "`read()` method returns [hvdsc1::R](hvdsc1::R) reader structure"]
impl crate::Readable for HVDSC1 {}
#[doc = "`write(|w| ..)` method takes [hvdsc1::W](hvdsc1::W) writer structure"]
impl crate::Writable for HVDSC1 {}
#[doc = "High Voltage Detect Status And Control 1 register"]
pub mod hvdsc1;
#[doc = "RF LDO Low Power Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfldolpcnfg](rfldolpcnfg) module"]
pub type RFLDOLPCNFG = crate::Reg<u32, _RFLDOLPCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFLDOLPCNFG;
#[doc = "`read()` method returns [rfldolpcnfg::R](rfldolpcnfg::R) reader structure"]
impl crate::Readable for RFLDOLPCNFG {}
#[doc = "`write(|w| ..)` method takes [rfldolpcnfg::W](rfldolpcnfg::W) writer structure"]
impl crate::Writable for RFLDOLPCNFG {}
#[doc = "RF LDO Low Power Configuration register"]
pub mod rfldolpcnfg;
#[doc = "RF LDO Status And Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfldosc](rfldosc) module"]
pub type RFLDOSC = crate::Reg<u32, _RFLDOSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFLDOSC;
#[doc = "`read()` method returns [rfldosc::R](rfldosc::R) reader structure"]
impl crate::Readable for RFLDOSC {}
#[doc = "`write(|w| ..)` method takes [rfldosc::W](rfldosc::W) writer structure"]
impl crate::Writable for RFLDOSC {}
#[doc = "RF LDO Status And Control register"]
pub mod rfldosc;
#[doc = "DCDC Status Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcsc](dcdcsc) module"]
pub type DCDCSC = crate::Reg<u32, _DCDCSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCSC;
#[doc = "`read()` method returns [dcdcsc::R](dcdcsc::R) reader structure"]
impl crate::Readable for DCDCSC {}
#[doc = "`write(|w| ..)` method takes [dcdcsc::W](dcdcsc::W) writer structure"]
impl crate::Writable for DCDCSC {}
#[doc = "DCDC Status Control Register"]
pub mod dcdcsc;
#[doc = "DCDC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcc1](dcdcc1) module"]
pub type DCDCC1 = crate::Reg<u32, _DCDCC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCC1;
#[doc = "`read()` method returns [dcdcc1::R](dcdcc1::R) reader structure"]
impl crate::Readable for DCDCC1 {}
#[doc = "`write(|w| ..)` method takes [dcdcc1::W](dcdcc1::W) writer structure"]
impl crate::Writable for DCDCC1 {}
#[doc = "DCDC Control Register 1"]
pub mod dcdcc1;
#[doc = "DCDC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcc2](dcdcc2) module"]
pub type DCDCC2 = crate::Reg<u32, _DCDCC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCC2;
#[doc = "`read()` method returns [dcdcc2::R](dcdcc2::R) reader structure"]
impl crate::Readable for DCDCC2 {}
#[doc = "`write(|w| ..)` method takes [dcdcc2::W](dcdcc2::W) writer structure"]
impl crate::Writable for DCDCC2 {}
#[doc = "DCDC Control Register 2"]
pub mod dcdcc2;
#[doc = "DCDC Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcc3](dcdcc3) module"]
pub type DCDCC3 = crate::Reg<u32, _DCDCC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCC3;
#[doc = "`read()` method returns [dcdcc3::R](dcdcc3::R) reader structure"]
impl crate::Readable for DCDCC3 {}
#[doc = "`write(|w| ..)` method takes [dcdcc3::W](dcdcc3::W) writer structure"]
impl crate::Writable for DCDCC3 {}
#[doc = "DCDC Control Register 3"]
pub mod dcdcc3;
#[doc = "DCDC Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcc4](dcdcc4) module"]
pub type DCDCC4 = crate::Reg<u32, _DCDCC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCC4;
#[doc = "`read()` method returns [dcdcc4::R](dcdcc4::R) reader structure"]
impl crate::Readable for DCDCC4 {}
#[doc = "`write(|w| ..)` method takes [dcdcc4::W](dcdcc4::W) writer structure"]
impl crate::Writable for DCDCC4 {}
#[doc = "DCDC Control Register 4"]
pub mod dcdcc4;
#[doc = "DCDC Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcc6](dcdcc6) module"]
pub type DCDCC6 = crate::Reg<u32, _DCDCC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCC6;
#[doc = "`read()` method returns [dcdcc6::R](dcdcc6::R) reader structure"]
impl crate::Readable for DCDCC6 {}
#[doc = "`write(|w| ..)` method takes [dcdcc6::W](dcdcc6::W) writer structure"]
impl crate::Writable for DCDCC6 {}
#[doc = "DCDC Control Register 6"]
pub mod dcdcc6;
#[doc = "LP Request Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpreqpincntrl](lpreqpincntrl) module"]
pub type LPREQPINCNTRL = crate::Reg<u32, _LPREQPINCNTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPREQPINCNTRL;
#[doc = "`read()` method returns [lpreqpincntrl::R](lpreqpincntrl::R) reader structure"]
impl crate::Readable for LPREQPINCNTRL {}
#[doc = "`write(|w| ..)` method takes [lpreqpincntrl::W](lpreqpincntrl::W) writer structure"]
impl crate::Writable for LPREQPINCNTRL {}
#[doc = "LP Request Pin Control Register"]
pub mod lpreqpincntrl;
