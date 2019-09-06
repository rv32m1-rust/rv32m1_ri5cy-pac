#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub verid: VERID,
  #[doc = "0x04 - Parameter Register"]
  pub param: PARAM,
  #[doc = "0x08 - TPM Global Register"]
  pub global: GLOBAL,
  _reserved3: [u8; 4usize],
  #[doc = "0x10 - Status and Control"]
  pub sc: SC,
  #[doc = "0x14 - Counter"]
  pub cnt: CNT,
  #[doc = "0x18 - Modulo"]
  pub mod_: MOD,
  #[doc = "0x1c - Capture and Compare Status"]
  pub status: STATUS,
  #[doc = "0x20 - no description available"]
  pub channel: [CHANNEL; 2],
  _reserved8: [u8; 52usize],
  #[doc = "0x64 - Combine Channel Register"]
  pub combine: COMBINE,
  _reserved9: [u8; 4usize],
  #[doc = "0x6c - Channel Trigger"]
  pub trig: TRIG,
  #[doc = "0x70 - Channel Polarity"]
  pub pol: POL,
  _reserved11: [u8; 4usize],
  #[doc = "0x78 - Filter Control"]
  pub filter: FILTER,
  _reserved12: [u8; 4usize],
  #[doc = "0x80 - Quadrature Decoder Control and Status"]
  pub qdctrl: QDCTRL,
  #[doc = "0x84 - Configuration"]
  pub conf: CONF,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
  #[doc = "0x00 - Channel (n) Status and Control"]
  pub csc: self::channel::CSC,
  #[doc = "0x04 - Channel (n) Value"]
  pub cv: self::channel::CV,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
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
#[doc = "TPM Global Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [global](global) module"]
pub type GLOBAL = crate::Reg<u32, _GLOBAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBAL;
#[doc = "`read()` method returns [global::R](global::R) reader structure"]
impl crate::Readable for GLOBAL {}
#[doc = "`write(|w| ..)` method takes [global::W](global::W) writer structure"]
impl crate::Writable for GLOBAL {}
#[doc = "TPM Global Register"]
pub mod global;
#[doc = "Status and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "Status and Control"]
pub mod sc;
#[doc = "Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter"]
pub mod cnt;
#[doc = "Modulo\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Modulo"]
pub mod mod_;
#[doc = "Capture and Compare Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Capture and Compare Status"]
pub mod status;
#[doc = "Combine Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [combine](combine) module"]
pub type COMBINE = crate::Reg<u32, _COMBINE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBINE;
#[doc = "`read()` method returns [combine::R](combine::R) reader structure"]
impl crate::Readable for COMBINE {}
#[doc = "`write(|w| ..)` method takes [combine::W](combine::W) writer structure"]
impl crate::Writable for COMBINE {}
#[doc = "Combine Channel Register"]
pub mod combine;
#[doc = "Channel Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trig](trig) module"]
pub type TRIG = crate::Reg<u32, _TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG;
#[doc = "`read()` method returns [trig::R](trig::R) reader structure"]
impl crate::Readable for TRIG {}
#[doc = "`write(|w| ..)` method takes [trig::W](trig::W) writer structure"]
impl crate::Writable for TRIG {}
#[doc = "Channel Trigger"]
pub mod trig;
#[doc = "Channel Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pol](pol) module"]
pub type POL = crate::Reg<u32, _POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POL;
#[doc = "`read()` method returns [pol::R](pol::R) reader structure"]
impl crate::Readable for POL {}
#[doc = "`write(|w| ..)` method takes [pol::W](pol::W) writer structure"]
impl crate::Writable for POL {}
#[doc = "Channel Polarity"]
pub mod pol;
#[doc = "Filter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filter](filter) module"]
pub type FILTER = crate::Reg<u32, _FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER;
#[doc = "`read()` method returns [filter::R](filter::R) reader structure"]
impl crate::Readable for FILTER {}
#[doc = "`write(|w| ..)` method takes [filter::W](filter::W) writer structure"]
impl crate::Writable for FILTER {}
#[doc = "Filter Control"]
pub mod filter;
#[doc = "Quadrature Decoder Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdctrl](qdctrl) module"]
pub type QDCTRL = crate::Reg<u32, _QDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDCTRL;
#[doc = "`read()` method returns [qdctrl::R](qdctrl::R) reader structure"]
impl crate::Readable for QDCTRL {}
#[doc = "`write(|w| ..)` method takes [qdctrl::W](qdctrl::W) writer structure"]
impl crate::Writable for QDCTRL {}
#[doc = "Quadrature Decoder Control and Status"]
pub mod qdctrl;
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "Configuration"]
pub mod conf;
