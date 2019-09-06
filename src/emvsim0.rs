#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub ver_id: VER_ID,
  #[doc = "0x04 - Parameter Register"]
  pub param: PARAM,
  #[doc = "0x08 - Clock Configuration Register"]
  pub clkcfg: CLKCFG,
  #[doc = "0x0c - Baud Rate Divisor Register"]
  pub divisor: DIVISOR,
  #[doc = "0x10 - Control Register"]
  pub ctrl: CTRL,
  #[doc = "0x14 - Interrupt Mask Register"]
  pub int_mask: INT_MASK,
  #[doc = "0x18 - Receiver Threshold Register"]
  pub rx_thd: RX_THD,
  #[doc = "0x1c - Transmitter Threshold Register"]
  pub tx_thd: TX_THD,
  #[doc = "0x20 - Receive Status Register"]
  pub rx_status: RX_STATUS,
  #[doc = "0x24 - Transmitter Status Register"]
  pub tx_status: TX_STATUS,
  #[doc = "0x28 - Port Control and Status Register"]
  pub pcsr: PCSR,
  #[doc = "0x2c - Receive Data Read Buffer"]
  pub rx_buf: RX_BUF,
  #[doc = "0x30 - Transmit Data Buffer"]
  pub tx_buf: TX_BUF,
  #[doc = "0x34 - Transmitter Guard ETU Value Register"]
  pub tx_getu: TX_GETU,
  #[doc = "0x38 - Character Wait Time Value Register"]
  pub cwt_val: CWT_VAL,
  #[doc = "0x3c - Block Wait Time Value Register"]
  pub bwt_val: BWT_VAL,
  #[doc = "0x40 - Block Guard Time Value Register"]
  pub bgt_val: BGT_VAL,
  #[doc = "0x44 - General Purpose Counter 0 Timeout Value Register"]
  pub gpcnt0_val: GPCNT0_VAL,
  #[doc = "0x48 - General Purpose Counter 1 Timeout Value"]
  pub gpcnt1_val: GPCNT1_VAL,
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ver_id](ver_id) module"]
pub type VER_ID = crate::Reg<u32, _VER_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VER_ID;
#[doc = "`read()` method returns [ver_id::R](ver_id::R) reader structure"]
impl crate::Readable for VER_ID {}
#[doc = "Version ID Register"]
pub mod ver_id;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkcfg](clkcfg) module"]
pub type CLKCFG = crate::Reg<u32, _CLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCFG;
#[doc = "`read()` method returns [clkcfg::R](clkcfg::R) reader structure"]
impl crate::Readable for CLKCFG {}
#[doc = "`write(|w| ..)` method takes [clkcfg::W](clkcfg::W) writer structure"]
impl crate::Writable for CLKCFG {}
#[doc = "Clock Configuration Register"]
pub mod clkcfg;
#[doc = "Baud Rate Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [divisor](divisor) module"]
pub type DIVISOR = crate::Reg<u32, _DIVISOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVISOR;
#[doc = "`read()` method returns [divisor::R](divisor::R) reader structure"]
impl crate::Readable for DIVISOR {}
#[doc = "`write(|w| ..)` method takes [divisor::W](divisor::W) writer structure"]
impl crate::Writable for DIVISOR {}
#[doc = "Baud Rate Divisor Register"]
pub mod divisor;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_mask](int_mask) module"]
pub type INT_MASK = crate::Reg<u32, _INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MASK;
#[doc = "`read()` method returns [int_mask::R](int_mask::R) reader structure"]
impl crate::Readable for INT_MASK {}
#[doc = "`write(|w| ..)` method takes [int_mask::W](int_mask::W) writer structure"]
impl crate::Writable for INT_MASK {}
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "Receiver Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_thd](rx_thd) module"]
pub type RX_THD = crate::Reg<u32, _RX_THD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_THD;
#[doc = "`read()` method returns [rx_thd::R](rx_thd::R) reader structure"]
impl crate::Readable for RX_THD {}
#[doc = "`write(|w| ..)` method takes [rx_thd::W](rx_thd::W) writer structure"]
impl crate::Writable for RX_THD {}
#[doc = "Receiver Threshold Register"]
pub mod rx_thd;
#[doc = "Transmitter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_thd](tx_thd) module"]
pub type TX_THD = crate::Reg<u32, _TX_THD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_THD;
#[doc = "`read()` method returns [tx_thd::R](tx_thd::R) reader structure"]
impl crate::Readable for TX_THD {}
#[doc = "`write(|w| ..)` method takes [tx_thd::W](tx_thd::W) writer structure"]
impl crate::Writable for TX_THD {}
#[doc = "Transmitter Threshold Register"]
pub mod tx_thd;
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_status](rx_status) module"]
pub type RX_STATUS = crate::Reg<u32, _RX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_STATUS;
#[doc = "`read()` method returns [rx_status::R](rx_status::R) reader structure"]
impl crate::Readable for RX_STATUS {}
#[doc = "`write(|w| ..)` method takes [rx_status::W](rx_status::W) writer structure"]
impl crate::Writable for RX_STATUS {}
#[doc = "Receive Status Register"]
pub mod rx_status;
#[doc = "Transmitter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_status](tx_status) module"]
pub type TX_STATUS = crate::Reg<u32, _TX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_STATUS;
#[doc = "`read()` method returns [tx_status::R](tx_status::R) reader structure"]
impl crate::Readable for TX_STATUS {}
#[doc = "`write(|w| ..)` method takes [tx_status::W](tx_status::W) writer structure"]
impl crate::Writable for TX_STATUS {}
#[doc = "Transmitter Status Register"]
pub mod tx_status;
#[doc = "Port Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcsr](pcsr) module"]
pub type PCSR = crate::Reg<u32, _PCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSR;
#[doc = "`read()` method returns [pcsr::R](pcsr::R) reader structure"]
impl crate::Readable for PCSR {}
#[doc = "`write(|w| ..)` method takes [pcsr::W](pcsr::W) writer structure"]
impl crate::Writable for PCSR {}
#[doc = "Port Control and Status Register"]
pub mod pcsr;
#[doc = "Receive Data Read Buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_buf](rx_buf) module"]
pub type RX_BUF = crate::Reg<u32, _RX_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_BUF;
#[doc = "`read()` method returns [rx_buf::R](rx_buf::R) reader structure"]
impl crate::Readable for RX_BUF {}
#[doc = "Receive Data Read Buffer"]
pub mod rx_buf;
#[doc = "Transmit Data Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_buf](tx_buf) module"]
pub type TX_BUF = crate::Reg<u32, _TX_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_BUF;
#[doc = "`read()` method returns [tx_buf::R](tx_buf::R) reader structure"]
impl crate::Readable for TX_BUF {}
#[doc = "`write(|w| ..)` method takes [tx_buf::W](tx_buf::W) writer structure"]
impl crate::Writable for TX_BUF {}
#[doc = "Transmit Data Buffer"]
pub mod tx_buf;
#[doc = "Transmitter Guard ETU Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_getu](tx_getu) module"]
pub type TX_GETU = crate::Reg<u32, _TX_GETU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_GETU;
#[doc = "`read()` method returns [tx_getu::R](tx_getu::R) reader structure"]
impl crate::Readable for TX_GETU {}
#[doc = "`write(|w| ..)` method takes [tx_getu::W](tx_getu::W) writer structure"]
impl crate::Writable for TX_GETU {}
#[doc = "Transmitter Guard ETU Value Register"]
pub mod tx_getu;
#[doc = "Character Wait Time Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cwt_val](cwt_val) module"]
pub type CWT_VAL = crate::Reg<u32, _CWT_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWT_VAL;
#[doc = "`read()` method returns [cwt_val::R](cwt_val::R) reader structure"]
impl crate::Readable for CWT_VAL {}
#[doc = "`write(|w| ..)` method takes [cwt_val::W](cwt_val::W) writer structure"]
impl crate::Writable for CWT_VAL {}
#[doc = "Character Wait Time Value Register"]
pub mod cwt_val;
#[doc = "Block Wait Time Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bwt_val](bwt_val) module"]
pub type BWT_VAL = crate::Reg<u32, _BWT_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWT_VAL;
#[doc = "`read()` method returns [bwt_val::R](bwt_val::R) reader structure"]
impl crate::Readable for BWT_VAL {}
#[doc = "`write(|w| ..)` method takes [bwt_val::W](bwt_val::W) writer structure"]
impl crate::Writable for BWT_VAL {}
#[doc = "Block Wait Time Value Register"]
pub mod bwt_val;
#[doc = "Block Guard Time Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bgt_val](bgt_val) module"]
pub type BGT_VAL = crate::Reg<u32, _BGT_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGT_VAL;
#[doc = "`read()` method returns [bgt_val::R](bgt_val::R) reader structure"]
impl crate::Readable for BGT_VAL {}
#[doc = "`write(|w| ..)` method takes [bgt_val::W](bgt_val::W) writer structure"]
impl crate::Writable for BGT_VAL {}
#[doc = "Block Guard Time Value Register"]
pub mod bgt_val;
#[doc = "General Purpose Counter 0 Timeout Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpcnt0_val](gpcnt0_val) module"]
pub type GPCNT0_VAL = crate::Reg<u32, _GPCNT0_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCNT0_VAL;
#[doc = "`read()` method returns [gpcnt0_val::R](gpcnt0_val::R) reader structure"]
impl crate::Readable for GPCNT0_VAL {}
#[doc = "`write(|w| ..)` method takes [gpcnt0_val::W](gpcnt0_val::W) writer structure"]
impl crate::Writable for GPCNT0_VAL {}
#[doc = "General Purpose Counter 0 Timeout Value Register"]
pub mod gpcnt0_val;
#[doc = "General Purpose Counter 1 Timeout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpcnt1_val](gpcnt1_val) module"]
pub type GPCNT1_VAL = crate::Reg<u32, _GPCNT1_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCNT1_VAL;
#[doc = "`read()` method returns [gpcnt1_val::R](gpcnt1_val::R) reader structure"]
impl crate::Readable for GPCNT1_VAL {}
#[doc = "`write(|w| ..)` method takes [gpcnt1_val::W](gpcnt1_val::W) writer structure"]
impl crate::Writable for GPCNT1_VAL {}
#[doc = "General Purpose Counter 1 Timeout Value"]
pub mod gpcnt1_val;
