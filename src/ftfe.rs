#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Flash Status Register"]
  pub fstat: FSTAT,
  #[doc = "0x01 - Flash Configuration Register"]
  pub fcnfg: FCNFG,
  #[doc = "0x02 - Flash Security Register"]
  pub fsec: FSEC,
  _reserved3: [u8; 1usize],
  #[doc = "0x04 - Flash Common Command Object Registers"]
  pub fccob3: FCCOB,
  #[doc = "0x05 - Flash Common Command Object Registers"]
  pub fccob2: FCCOB,
  #[doc = "0x06 - Flash Common Command Object Registers"]
  pub fccob1: FCCOB,
  #[doc = "0x07 - Flash Common Command Object Registers"]
  pub fccob0: FCCOB,
  #[doc = "0x08 - Flash Common Command Object Registers"]
  pub fccob7: FCCOB,
  #[doc = "0x09 - Flash Common Command Object Registers"]
  pub fccob6: FCCOB,
  #[doc = "0x0a - Flash Common Command Object Registers"]
  pub fccob5: FCCOB,
  #[doc = "0x0b - Flash Common Command Object Registers"]
  pub fccob4: FCCOB,
  #[doc = "0x0c - Flash Common Command Object Registers"]
  pub fccobb: FCCOB,
  #[doc = "0x0d - Flash Common Command Object Registers"]
  pub fccoba: FCCOB,
  #[doc = "0x0e - Flash Common Command Object Registers"]
  pub fccob9: FCCOB,
  #[doc = "0x0f - Flash Common Command Object Registers"]
  pub fccob8: FCCOB,
  #[doc = "0x10 - Flash Option Registers"]
  pub fopt3: FOPT,
  #[doc = "0x11 - Flash Option Registers"]
  pub fopt2: FOPT,
  #[doc = "0x12 - Flash Option Registers"]
  pub fopt1: FOPT,
  #[doc = "0x13 - Flash Option Registers"]
  pub fopt0: FOPT,
  _reserved19: [u8; 4usize],
  #[doc = "0x18 - Primary Program Flash Protection Registers"]
  pub fproth3: FPROT,
  #[doc = "0x19 - Primary Program Flash Protection Registers"]
  pub fproth2: FPROT,
  #[doc = "0x1a - Primary Program Flash Protection Registers"]
  pub fproth1: FPROT,
  #[doc = "0x1b - Primary Program Flash Protection Registers"]
  pub fproth0: FPROT,
  #[doc = "0x1c - Primary Program Flash Protection Registers"]
  pub fprotl3: FPROT,
  #[doc = "0x1d - Primary Program Flash Protection Registers"]
  pub fprotl2: FPROT,
  #[doc = "0x1e - Primary Program Flash Protection Registers"]
  pub fprotl1: FPROT,
  #[doc = "0x1f - Primary Program Flash Protection Registers"]
  pub fprotl0: FPROT,
  _reserved27: [u8; 4usize],
  #[doc = "0x24 - Secondary Program Flash Protection Registers"]
  pub fprotsl: FPROTS,
  #[doc = "0x25 - Secondary Program Flash Protection Registers"]
  pub fprotsh: FPROTS,
  _reserved29: [u8; 6usize],
  #[doc = "0x2c - Primary Flash Access Segment Size Register"]
  pub facss: FACSS,
  #[doc = "0x2d - Primary Flash Access Segment Number Register"]
  pub facsn: FACSN,
  #[doc = "0x2e - Secondary Flash Access Segment Size Register"]
  pub facsss: FACSSS,
  #[doc = "0x2f - Secondary Flash Access Segment Number Register"]
  pub facsns: FACSNS,
  #[doc = "0x30 - Primary Execute-only Access Registers"]
  pub xacch3: XACC,
  #[doc = "0x31 - Primary Execute-only Access Registers"]
  pub xacch2: XACC,
  #[doc = "0x32 - Primary Execute-only Access Registers"]
  pub xacch1: XACC,
  #[doc = "0x33 - Primary Execute-only Access Registers"]
  pub xacch0: XACC,
  #[doc = "0x34 - Primary Execute-only Access Registers"]
  pub xaccl3: XACC,
  #[doc = "0x35 - Primary Execute-only Access Registers"]
  pub xaccl2: XACC,
  #[doc = "0x36 - Primary Execute-only Access Registers"]
  pub xaccl1: XACC,
  #[doc = "0x37 - Primary Execute-only Access Registers"]
  pub xaccl0: XACC,
  #[doc = "0x38 - Primary Supervisor-only Access Registers"]
  pub sacch3: SACC,
  #[doc = "0x39 - Primary Supervisor-only Access Registers"]
  pub sacch2: SACC,
  #[doc = "0x3a - Primary Supervisor-only Access Registers"]
  pub sacch1: SACC,
  #[doc = "0x3b - Primary Supervisor-only Access Registers"]
  pub sacch0: SACC,
  #[doc = "0x3c - Primary Supervisor-only Access Registers"]
  pub saccl3: SACC,
  #[doc = "0x3d - Primary Supervisor-only Access Registers"]
  pub saccl2: SACC,
  #[doc = "0x3e - Primary Supervisor-only Access Registers"]
  pub saccl1: SACC,
  #[doc = "0x3f - Primary Supervisor-only Access Registers"]
  pub saccl0: SACC,
  _reserved49: [u8; 4usize],
  #[doc = "0x44 - Secondary Execute-only Access Registers"]
  pub xaccsl: XACCS,
  #[doc = "0x45 - Secondary Execute-only Access Registers"]
  pub xaccsh: XACCS,
  _reserved51: [u8; 6usize],
  #[doc = "0x4c - Secondary Supervisor-only Access Registers"]
  pub saccsl: SACCS,
  #[doc = "0x4d - Secondary Supervisor-only Access Registers"]
  pub saccsh: SACCS,
  _reserved53: [u8; 4usize],
  #[doc = "0x52 - Flash Standby Control Register"]
  pub fstdbyctl: FSTDBYCTL,
  #[doc = "0x53 - Flash Standby Register"]
  pub fstdby: FSTDBY,
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u8, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "`write(|w| ..)` method takes [fstat::W](fstat::W) writer structure"]
impl crate::Writable for FSTAT {}
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcnfg](fcnfg) module"]
pub type FCNFG = crate::Reg<u8, _FCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCNFG;
#[doc = "`read()` method returns [fcnfg::R](fcnfg::R) reader structure"]
impl crate::Readable for FCNFG {}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](fcnfg::W) writer structure"]
impl crate::Writable for FCNFG {}
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsec](fsec) module"]
pub type FSEC = crate::Reg<u8, _FSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEC;
#[doc = "`read()` method returns [fsec::R](fsec::R) reader structure"]
impl crate::Readable for FSEC {}
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fccob](fccob) module"]
pub type FCCOB = crate::Reg<u8, _FCCOB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB;
#[doc = "`read()` method returns [fccob::R](fccob::R) reader structure"]
impl crate::Readable for FCCOB {}
#[doc = "`write(|w| ..)` method takes [fccob::W](fccob::W) writer structure"]
impl crate::Writable for FCCOB {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "Flash Option Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fopt](fopt) module"]
pub type FOPT = crate::Reg<u8, _FOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOPT;
#[doc = "`read()` method returns [fopt::R](fopt::R) reader structure"]
impl crate::Readable for FOPT {}
#[doc = "Flash Option Registers"]
pub mod fopt;
#[doc = "Primary Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fprot](fprot) module"]
pub type FPROT = crate::Reg<u8, _FPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT;
#[doc = "`read()` method returns [fprot::R](fprot::R) reader structure"]
impl crate::Readable for FPROT {}
#[doc = "`write(|w| ..)` method takes [fprot::W](fprot::W) writer structure"]
impl crate::Writable for FPROT {}
#[doc = "Primary Program Flash Protection Registers"]
pub mod fprot;
#[doc = "Secondary Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fprots](fprots) module"]
pub type FPROTS = crate::Reg<u8, _FPROTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROTS;
#[doc = "`read()` method returns [fprots::R](fprots::R) reader structure"]
impl crate::Readable for FPROTS {}
#[doc = "`write(|w| ..)` method takes [fprots::W](fprots::W) writer structure"]
impl crate::Writable for FPROTS {}
#[doc = "Secondary Program Flash Protection Registers"]
pub mod fprots;
#[doc = "Primary Flash Access Segment Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [facss](facss) module"]
pub type FACSS = crate::Reg<u8, _FACSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FACSS;
#[doc = "`read()` method returns [facss::R](facss::R) reader structure"]
impl crate::Readable for FACSS {}
#[doc = "Primary Flash Access Segment Size Register"]
pub mod facss;
#[doc = "Primary Flash Access Segment Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [facsn](facsn) module"]
pub type FACSN = crate::Reg<u8, _FACSN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FACSN;
#[doc = "`read()` method returns [facsn::R](facsn::R) reader structure"]
impl crate::Readable for FACSN {}
#[doc = "Primary Flash Access Segment Number Register"]
pub mod facsn;
#[doc = "Secondary Flash Access Segment Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [facsss](facsss) module"]
pub type FACSSS = crate::Reg<u8, _FACSSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FACSSS;
#[doc = "`read()` method returns [facsss::R](facsss::R) reader structure"]
impl crate::Readable for FACSSS {}
#[doc = "Secondary Flash Access Segment Size Register"]
pub mod facsss;
#[doc = "Secondary Flash Access Segment Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [facsns](facsns) module"]
pub type FACSNS = crate::Reg<u8, _FACSNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FACSNS;
#[doc = "`read()` method returns [facsns::R](facsns::R) reader structure"]
impl crate::Readable for FACSNS {}
#[doc = "Secondary Flash Access Segment Number Register"]
pub mod facsns;
#[doc = "Primary Execute-only Access Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xacc](xacc) module"]
pub type XACC = crate::Reg<u8, _XACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XACC;
#[doc = "`read()` method returns [xacc::R](xacc::R) reader structure"]
impl crate::Readable for XACC {}
#[doc = "Primary Execute-only Access Registers"]
pub mod xacc;
#[doc = "Primary Supervisor-only Access Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sacc](sacc) module"]
pub type SACC = crate::Reg<u8, _SACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SACC;
#[doc = "`read()` method returns [sacc::R](sacc::R) reader structure"]
impl crate::Readable for SACC {}
#[doc = "Primary Supervisor-only Access Registers"]
pub mod sacc;
#[doc = "Secondary Execute-only Access Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xaccs](xaccs) module"]
pub type XACCS = crate::Reg<u8, _XACCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XACCS;
#[doc = "`read()` method returns [xaccs::R](xaccs::R) reader structure"]
impl crate::Readable for XACCS {}
#[doc = "Secondary Execute-only Access Registers"]
pub mod xaccs;
#[doc = "Secondary Supervisor-only Access Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saccs](saccs) module"]
pub type SACCS = crate::Reg<u8, _SACCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SACCS;
#[doc = "`read()` method returns [saccs::R](saccs::R) reader structure"]
impl crate::Readable for SACCS {}
#[doc = "Secondary Supervisor-only Access Registers"]
pub mod saccs;
#[doc = "Flash Standby Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fstdbyctl](fstdbyctl) module"]
pub type FSTDBYCTL = crate::Reg<u8, _FSTDBYCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTDBYCTL;
#[doc = "`read()` method returns [fstdbyctl::R](fstdbyctl::R) reader structure"]
impl crate::Readable for FSTDBYCTL {}
#[doc = "Flash Standby Control Register"]
pub mod fstdbyctl;
#[doc = "Flash Standby Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fstdby](fstdby) module"]
pub type FSTDBY = crate::Reg<u8, _FSTDBY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTDBY;
#[doc = "`read()` method returns [fstdby::R](fstdby::R) reader structure"]
impl crate::Readable for FSTDBY {}
#[doc = "`write(|w| ..)` method takes [fstdby::W](fstdby::W) writer structure"]
impl crate::Writable for FSTDBY {}
#[doc = "Flash Standby Register"]
pub mod fstdby;
