#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Priority Slave Registers"]
  pub prs0: PRS0,
  _reserved1: [u8; 12usize],
  #[doc = "0x10 - Control Register"]
  pub crs0: CRS0,
  _reserved2: [u8; 236usize],
  #[doc = "0x100 - Priority Slave Registers"]
  pub prs1: PRS1,
  _reserved3: [u8; 12usize],
  #[doc = "0x110 - Control Register"]
  pub crs1: CRS1,
  _reserved4: [u8; 236usize],
  #[doc = "0x200 - Priority Slave Registers"]
  pub prs2: PRS2,
  _reserved5: [u8; 12usize],
  #[doc = "0x210 - Control Register"]
  pub crs2: CRS2,
  _reserved6: [u8; 236usize],
  #[doc = "0x300 - Priority Slave Registers"]
  pub prs3: PRS3,
  _reserved7: [u8; 12usize],
  #[doc = "0x310 - Control Register"]
  pub crs3: CRS3,
  _reserved8: [u8; 236usize],
  #[doc = "0x400 - Priority Slave Registers"]
  pub prs4: PRS4,
  _reserved9: [u8; 12usize],
  #[doc = "0x410 - Control Register"]
  pub crs4: CRS4,
  _reserved10: [u8; 1004usize],
  #[doc = "0x800 - Master General Purpose Control Register"]
  pub mgpcr0: MGPCR0,
  _reserved11: [u8; 252usize],
  #[doc = "0x900 - Master General Purpose Control Register"]
  pub mgpcr1: MGPCR1,
  _reserved12: [u8; 252usize],
  #[doc = "0xa00 - Master General Purpose Control Register"]
  pub mgpcr2: MGPCR2,
  _reserved13: [u8; 252usize],
  #[doc = "0xb00 - Master General Purpose Control Register"]
  pub mgpcr3: MGPCR3,
  _reserved14: [u8; 252usize],
  #[doc = "0xc00 - Master General Purpose Control Register"]
  pub mgpcr4: MGPCR4,
  _reserved15: [u8; 252usize],
  #[doc = "0xd00 - Master General Purpose Control Register"]
  pub mgpcr5: MGPCR5,
}
#[doc = "Priority Slave Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prs0](prs0) module"]
pub type PRS0 = crate::Reg<u32, _PRS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS0;
#[doc = "`read()` method returns [prs0::R](prs0::R) reader structure"]
impl crate::Readable for PRS0 {}
#[doc = "`write(|w| ..)` method takes [prs0::W](prs0::W) writer structure"]
impl crate::Writable for PRS0 {}
#[doc = "Priority Slave Registers"]
pub mod prs0;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crs0](crs0) module"]
pub type CRS0 = crate::Reg<u32, _CRS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRS0;
#[doc = "`read()` method returns [crs0::R](crs0::R) reader structure"]
impl crate::Readable for CRS0 {}
#[doc = "`write(|w| ..)` method takes [crs0::W](crs0::W) writer structure"]
impl crate::Writable for CRS0 {}
#[doc = "Control Register"]
pub mod crs0;
#[doc = "Priority Slave Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prs1](prs1) module"]
pub type PRS1 = crate::Reg<u32, _PRS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS1;
#[doc = "`read()` method returns [prs1::R](prs1::R) reader structure"]
impl crate::Readable for PRS1 {}
#[doc = "`write(|w| ..)` method takes [prs1::W](prs1::W) writer structure"]
impl crate::Writable for PRS1 {}
#[doc = "Priority Slave Registers"]
pub mod prs1;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crs1](crs1) module"]
pub type CRS1 = crate::Reg<u32, _CRS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRS1;
#[doc = "`read()` method returns [crs1::R](crs1::R) reader structure"]
impl crate::Readable for CRS1 {}
#[doc = "`write(|w| ..)` method takes [crs1::W](crs1::W) writer structure"]
impl crate::Writable for CRS1 {}
#[doc = "Control Register"]
pub mod crs1;
#[doc = "Priority Slave Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prs2](prs2) module"]
pub type PRS2 = crate::Reg<u32, _PRS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS2;
#[doc = "`read()` method returns [prs2::R](prs2::R) reader structure"]
impl crate::Readable for PRS2 {}
#[doc = "`write(|w| ..)` method takes [prs2::W](prs2::W) writer structure"]
impl crate::Writable for PRS2 {}
#[doc = "Priority Slave Registers"]
pub mod prs2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crs2](crs2) module"]
pub type CRS2 = crate::Reg<u32, _CRS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRS2;
#[doc = "`read()` method returns [crs2::R](crs2::R) reader structure"]
impl crate::Readable for CRS2 {}
#[doc = "`write(|w| ..)` method takes [crs2::W](crs2::W) writer structure"]
impl crate::Writable for CRS2 {}
#[doc = "Control Register"]
pub mod crs2;
#[doc = "Priority Slave Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prs3](prs3) module"]
pub type PRS3 = crate::Reg<u32, _PRS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS3;
#[doc = "`read()` method returns [prs3::R](prs3::R) reader structure"]
impl crate::Readable for PRS3 {}
#[doc = "`write(|w| ..)` method takes [prs3::W](prs3::W) writer structure"]
impl crate::Writable for PRS3 {}
#[doc = "Priority Slave Registers"]
pub mod prs3;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crs3](crs3) module"]
pub type CRS3 = crate::Reg<u32, _CRS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRS3;
#[doc = "`read()` method returns [crs3::R](crs3::R) reader structure"]
impl crate::Readable for CRS3 {}
#[doc = "`write(|w| ..)` method takes [crs3::W](crs3::W) writer structure"]
impl crate::Writable for CRS3 {}
#[doc = "Control Register"]
pub mod crs3;
#[doc = "Priority Slave Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prs4](prs4) module"]
pub type PRS4 = crate::Reg<u32, _PRS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS4;
#[doc = "`read()` method returns [prs4::R](prs4::R) reader structure"]
impl crate::Readable for PRS4 {}
#[doc = "`write(|w| ..)` method takes [prs4::W](prs4::W) writer structure"]
impl crate::Writable for PRS4 {}
#[doc = "Priority Slave Registers"]
pub mod prs4;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crs4](crs4) module"]
pub type CRS4 = crate::Reg<u32, _CRS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRS4;
#[doc = "`read()` method returns [crs4::R](crs4::R) reader structure"]
impl crate::Readable for CRS4 {}
#[doc = "`write(|w| ..)` method takes [crs4::W](crs4::W) writer structure"]
impl crate::Writable for CRS4 {}
#[doc = "Control Register"]
pub mod crs4;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mgpcr0](mgpcr0) module"]
pub type MGPCR0 = crate::Reg<u32, _MGPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR0;
#[doc = "`read()` method returns [mgpcr0::R](mgpcr0::R) reader structure"]
impl crate::Readable for MGPCR0 {}
#[doc = "`write(|w| ..)` method takes [mgpcr0::W](mgpcr0::W) writer structure"]
impl crate::Writable for MGPCR0 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr0;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mgpcr1](mgpcr1) module"]
pub type MGPCR1 = crate::Reg<u32, _MGPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR1;
#[doc = "`read()` method returns [mgpcr1::R](mgpcr1::R) reader structure"]
impl crate::Readable for MGPCR1 {}
#[doc = "`write(|w| ..)` method takes [mgpcr1::W](mgpcr1::W) writer structure"]
impl crate::Writable for MGPCR1 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr1;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mgpcr2](mgpcr2) module"]
pub type MGPCR2 = crate::Reg<u32, _MGPCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR2;
#[doc = "`read()` method returns [mgpcr2::R](mgpcr2::R) reader structure"]
impl crate::Readable for MGPCR2 {}
#[doc = "`write(|w| ..)` method takes [mgpcr2::W](mgpcr2::W) writer structure"]
impl crate::Writable for MGPCR2 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr2;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mgpcr3](mgpcr3) module"]
pub type MGPCR3 = crate::Reg<u32, _MGPCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR3;
#[doc = "`read()` method returns [mgpcr3::R](mgpcr3::R) reader structure"]
impl crate::Readable for MGPCR3 {}
#[doc = "`write(|w| ..)` method takes [mgpcr3::W](mgpcr3::W) writer structure"]
impl crate::Writable for MGPCR3 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr3;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mgpcr4](mgpcr4) module"]
pub type MGPCR4 = crate::Reg<u32, _MGPCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR4;
#[doc = "`read()` method returns [mgpcr4::R](mgpcr4::R) reader structure"]
impl crate::Readable for MGPCR4 {}
#[doc = "`write(|w| ..)` method takes [mgpcr4::W](mgpcr4::W) writer structure"]
impl crate::Writable for MGPCR4 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr4;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mgpcr5](mgpcr5) module"]
pub type MGPCR5 = crate::Reg<u32, _MGPCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR5;
#[doc = "`read()` method returns [mgpcr5::R](mgpcr5::R) reader structure"]
impl crate::Readable for MGPCR5 {}
#[doc = "`write(|w| ..)` method takes [mgpcr5::W](mgpcr5::W) writer structure"]
impl crate::Writable for MGPCR5 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr5;
