#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  _reserved0: [u8; 4usize],
  #[doc = "0x04 - PCC MSCM Register"]
  pub pcc_mscm: PCC_MSCM,
  _reserved1: [u8; 8usize],
  #[doc = "0x10 - PCC AXBS0 Register"]
  pub pcc_axbs0: PCC_AXBS0,
  _reserved2: [u8; 12usize],
  #[doc = "0x20 - PCC DMA0 Register"]
  pub pcc_dma0: PCC_DMA0,
  _reserved3: [u8; 12usize],
  #[doc = "0x30 - PCC FLEXBUS Register"]
  pub pcc_flexbus: PCC_FLEXBUS,
  _reserved4: [u8; 28usize],
  #[doc = "0x50 - PCC XRDC_MGR Register"]
  pub pcc_xrdc_mgr: PCC_XRDC_MGR,
  _reserved5: [u8; 4usize],
  #[doc = "0x58 - PCC XRDC_PAC Register"]
  pub pcc_xrdc_pac: PCC_XRDC_PAC,
  #[doc = "0x5c - PCC XRDC_MRC Register"]
  pub pcc_xrdc_mrc: PCC_XRDC_MRC,
  _reserved7: [u8; 12usize],
  #[doc = "0x6c - PCC SEMA42_0 Register"]
  pub pcc_sema42_0: PCC_SEMA42_0,
  _reserved8: [u8; 20usize],
  #[doc = "0x84 - PCC DMAMUX0 Register"]
  pub pcc_dmamux0: PCC_DMAMUX0,
  #[doc = "0x88 - PCC EWM Register"]
  pub pcc_ewm: PCC_EWM,
  _reserved10: [u8; 8usize],
  #[doc = "0x94 - PCC MUA Register"]
  pub pcc_mua: PCC_MUA,
  _reserved11: [u8; 36usize],
  #[doc = "0xbc - PCC CRC0 Register"]
  pub pcc_crc0: PCC_CRC0,
  #[doc = "0xc0 - PCC LPIT0 Register"]
  pub pcc_lpit0: PCC_LPIT0,
  _reserved13: [u8; 16usize],
  #[doc = "0xd4 - PCC TPM0 Register"]
  pub pcc_tpm0: PCC_TPM0,
  #[doc = "0xd8 - PCC TPM1 Register"]
  pub pcc_tpm1: PCC_TPM1,
  #[doc = "0xdc - PCC TPM2 Register"]
  pub pcc_tpm2: PCC_TPM2,
  #[doc = "0xe0 - PCC EMVSIM0 Register"]
  pub pcc_emvsim0: PCC_EMVSIM0,
  #[doc = "0xe4 - PCC FLEXIO0 Register"]
  pub pcc_flexio0: PCC_FLEXIO0,
  #[doc = "0xe8 - PCC LPI2C0 Register"]
  pub pcc_lpi2c0: PCC_LPI2C0,
  #[doc = "0xec - PCC LPI2C1 Register"]
  pub pcc_lpi2c1: PCC_LPI2C1,
  #[doc = "0xf0 - PCC LPI2C2 Register"]
  pub pcc_lpi2c2: PCC_LPI2C2,
  #[doc = "0xf4 - PCC I2S0 Register"]
  pub pcc_i2s0: PCC_I2S0,
  #[doc = "0xf8 - PCC USDHC0 Register"]
  pub pcc_usdhc0: PCC_USDHC0,
  #[doc = "0xfc - PCC LPSPI0 Register"]
  pub pcc_lpspi0: PCC_LPSPI0,
  #[doc = "0x100 - PCC LPSPI1 Register"]
  pub pcc_lpspi1: PCC_LPSPI1,
  #[doc = "0x104 - PCC LPSPI2 Register"]
  pub pcc_lpspi2: PCC_LPSPI2,
  #[doc = "0x108 - PCC LPUART0 Register"]
  pub pcc_lpuart0: PCC_LPUART0,
  #[doc = "0x10c - PCC LPUART1 Register"]
  pub pcc_lpuart1: PCC_LPUART1,
  #[doc = "0x110 - PCC LPUART2 Register"]
  pub pcc_lpuart2: PCC_LPUART2,
  #[doc = "0x114 - PCC USB0 Register"]
  pub pcc_usb0: PCC_USB0,
  #[doc = "0x118 - PCC PORTA Register"]
  pub pcc_porta: PCC_PORTA,
  #[doc = "0x11c - PCC PORTB Register"]
  pub pcc_portb: PCC_PORTB,
  #[doc = "0x120 - PCC PORTC Register"]
  pub pcc_portc: PCC_PORTC,
  #[doc = "0x124 - PCC PORTD Register"]
  pub pcc_portd: PCC_PORTD,
  #[doc = "0x128 - PCC ADC0 Register"]
  pub pcc_adc0: PCC_ADC0,
  _reserved35: [u8; 4usize],
  #[doc = "0x130 - PCC LPDAC0 Register"]
  pub pcc_lpdac0: PCC_LPDAC0,
  #[doc = "0x134 - PCC VREF Register"]
  pub pcc_vref: PCC_VREF,
  _reserved37: [u8; 200usize],
  #[doc = "0x200 - PCC TRACE Register"]
  pub pcc_trace: PCC_TRACE,
}
#[doc = "PCC MSCM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_mscm](pcc_mscm) module"]
pub type PCC_MSCM = crate::Reg<u32, _PCC_MSCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_MSCM;
#[doc = "`read()` method returns [pcc_mscm::R](pcc_mscm::R) reader structure"]
impl crate::Readable for PCC_MSCM {}
#[doc = "`write(|w| ..)` method takes [pcc_mscm::W](pcc_mscm::W) writer structure"]
impl crate::Writable for PCC_MSCM {}
#[doc = "PCC MSCM Register"]
pub mod pcc_mscm;
#[doc = "PCC AXBS0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_axbs0](pcc_axbs0) module"]
pub type PCC_AXBS0 = crate::Reg<u32, _PCC_AXBS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_AXBS0;
#[doc = "`read()` method returns [pcc_axbs0::R](pcc_axbs0::R) reader structure"]
impl crate::Readable for PCC_AXBS0 {}
#[doc = "`write(|w| ..)` method takes [pcc_axbs0::W](pcc_axbs0::W) writer structure"]
impl crate::Writable for PCC_AXBS0 {}
#[doc = "PCC AXBS0 Register"]
pub mod pcc_axbs0;
#[doc = "PCC DMA0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_dma0](pcc_dma0) module"]
pub type PCC_DMA0 = crate::Reg<u32, _PCC_DMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_DMA0;
#[doc = "`read()` method returns [pcc_dma0::R](pcc_dma0::R) reader structure"]
impl crate::Readable for PCC_DMA0 {}
#[doc = "`write(|w| ..)` method takes [pcc_dma0::W](pcc_dma0::W) writer structure"]
impl crate::Writable for PCC_DMA0 {}
#[doc = "PCC DMA0 Register"]
pub mod pcc_dma0;
#[doc = "PCC FLEXBUS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_flexbus](pcc_flexbus) module"]
pub type PCC_FLEXBUS = crate::Reg<u32, _PCC_FLEXBUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FLEXBUS;
#[doc = "`read()` method returns [pcc_flexbus::R](pcc_flexbus::R) reader structure"]
impl crate::Readable for PCC_FLEXBUS {}
#[doc = "`write(|w| ..)` method takes [pcc_flexbus::W](pcc_flexbus::W) writer structure"]
impl crate::Writable for PCC_FLEXBUS {}
#[doc = "PCC FLEXBUS Register"]
pub mod pcc_flexbus;
#[doc = "PCC XRDC_MGR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_xrdc_mgr](pcc_xrdc_mgr) module"]
pub type PCC_XRDC_MGR = crate::Reg<u32, _PCC_XRDC_MGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_XRDC_MGR;
#[doc = "`read()` method returns [pcc_xrdc_mgr::R](pcc_xrdc_mgr::R) reader structure"]
impl crate::Readable for PCC_XRDC_MGR {}
#[doc = "`write(|w| ..)` method takes [pcc_xrdc_mgr::W](pcc_xrdc_mgr::W) writer structure"]
impl crate::Writable for PCC_XRDC_MGR {}
#[doc = "PCC XRDC_MGR Register"]
pub mod pcc_xrdc_mgr;
#[doc = "PCC XRDC_PAC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_xrdc_pac](pcc_xrdc_pac) module"]
pub type PCC_XRDC_PAC = crate::Reg<u32, _PCC_XRDC_PAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_XRDC_PAC;
#[doc = "`read()` method returns [pcc_xrdc_pac::R](pcc_xrdc_pac::R) reader structure"]
impl crate::Readable for PCC_XRDC_PAC {}
#[doc = "`write(|w| ..)` method takes [pcc_xrdc_pac::W](pcc_xrdc_pac::W) writer structure"]
impl crate::Writable for PCC_XRDC_PAC {}
#[doc = "PCC XRDC_PAC Register"]
pub mod pcc_xrdc_pac;
#[doc = "PCC XRDC_MRC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_xrdc_mrc](pcc_xrdc_mrc) module"]
pub type PCC_XRDC_MRC = crate::Reg<u32, _PCC_XRDC_MRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_XRDC_MRC;
#[doc = "`read()` method returns [pcc_xrdc_mrc::R](pcc_xrdc_mrc::R) reader structure"]
impl crate::Readable for PCC_XRDC_MRC {}
#[doc = "`write(|w| ..)` method takes [pcc_xrdc_mrc::W](pcc_xrdc_mrc::W) writer structure"]
impl crate::Writable for PCC_XRDC_MRC {}
#[doc = "PCC XRDC_MRC Register"]
pub mod pcc_xrdc_mrc;
#[doc = "PCC SEMA42_0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_sema42_0](pcc_sema42_0) module"]
pub type PCC_SEMA42_0 = crate::Reg<u32, _PCC_SEMA42_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_SEMA42_0;
#[doc = "`read()` method returns [pcc_sema42_0::R](pcc_sema42_0::R) reader structure"]
impl crate::Readable for PCC_SEMA42_0 {}
#[doc = "`write(|w| ..)` method takes [pcc_sema42_0::W](pcc_sema42_0::W) writer structure"]
impl crate::Writable for PCC_SEMA42_0 {}
#[doc = "PCC SEMA42_0 Register"]
pub mod pcc_sema42_0;
#[doc = "PCC DMAMUX0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_dmamux0](pcc_dmamux0) module"]
pub type PCC_DMAMUX0 = crate::Reg<u32, _PCC_DMAMUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_DMAMUX0;
#[doc = "`read()` method returns [pcc_dmamux0::R](pcc_dmamux0::R) reader structure"]
impl crate::Readable for PCC_DMAMUX0 {}
#[doc = "`write(|w| ..)` method takes [pcc_dmamux0::W](pcc_dmamux0::W) writer structure"]
impl crate::Writable for PCC_DMAMUX0 {}
#[doc = "PCC DMAMUX0 Register"]
pub mod pcc_dmamux0;
#[doc = "PCC EWM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_ewm](pcc_ewm) module"]
pub type PCC_EWM = crate::Reg<u32, _PCC_EWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_EWM;
#[doc = "`read()` method returns [pcc_ewm::R](pcc_ewm::R) reader structure"]
impl crate::Readable for PCC_EWM {}
#[doc = "`write(|w| ..)` method takes [pcc_ewm::W](pcc_ewm::W) writer structure"]
impl crate::Writable for PCC_EWM {}
#[doc = "PCC EWM Register"]
pub mod pcc_ewm;
#[doc = "PCC MUA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_mua](pcc_mua) module"]
pub type PCC_MUA = crate::Reg<u32, _PCC_MUA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_MUA;
#[doc = "`read()` method returns [pcc_mua::R](pcc_mua::R) reader structure"]
impl crate::Readable for PCC_MUA {}
#[doc = "`write(|w| ..)` method takes [pcc_mua::W](pcc_mua::W) writer structure"]
impl crate::Writable for PCC_MUA {}
#[doc = "PCC MUA Register"]
pub mod pcc_mua;
#[doc = "PCC CRC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_crc0](pcc_crc0) module"]
pub type PCC_CRC0 = crate::Reg<u32, _PCC_CRC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_CRC0;
#[doc = "`read()` method returns [pcc_crc0::R](pcc_crc0::R) reader structure"]
impl crate::Readable for PCC_CRC0 {}
#[doc = "`write(|w| ..)` method takes [pcc_crc0::W](pcc_crc0::W) writer structure"]
impl crate::Writable for PCC_CRC0 {}
#[doc = "PCC CRC0 Register"]
pub mod pcc_crc0;
#[doc = "PCC LPIT0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpit0](pcc_lpit0) module"]
pub type PCC_LPIT0 = crate::Reg<u32, _PCC_LPIT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPIT0;
#[doc = "`read()` method returns [pcc_lpit0::R](pcc_lpit0::R) reader structure"]
impl crate::Readable for PCC_LPIT0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpit0::W](pcc_lpit0::W) writer structure"]
impl crate::Writable for PCC_LPIT0 {}
#[doc = "PCC LPIT0 Register"]
pub mod pcc_lpit0;
#[doc = "PCC TPM0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_tpm0](pcc_tpm0) module"]
pub type PCC_TPM0 = crate::Reg<u32, _PCC_TPM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_TPM0;
#[doc = "`read()` method returns [pcc_tpm0::R](pcc_tpm0::R) reader structure"]
impl crate::Readable for PCC_TPM0 {}
#[doc = "`write(|w| ..)` method takes [pcc_tpm0::W](pcc_tpm0::W) writer structure"]
impl crate::Writable for PCC_TPM0 {}
#[doc = "PCC TPM0 Register"]
pub mod pcc_tpm0;
#[doc = "PCC TPM1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_tpm1](pcc_tpm1) module"]
pub type PCC_TPM1 = crate::Reg<u32, _PCC_TPM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_TPM1;
#[doc = "`read()` method returns [pcc_tpm1::R](pcc_tpm1::R) reader structure"]
impl crate::Readable for PCC_TPM1 {}
#[doc = "`write(|w| ..)` method takes [pcc_tpm1::W](pcc_tpm1::W) writer structure"]
impl crate::Writable for PCC_TPM1 {}
#[doc = "PCC TPM1 Register"]
pub mod pcc_tpm1;
#[doc = "PCC TPM2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_tpm2](pcc_tpm2) module"]
pub type PCC_TPM2 = crate::Reg<u32, _PCC_TPM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_TPM2;
#[doc = "`read()` method returns [pcc_tpm2::R](pcc_tpm2::R) reader structure"]
impl crate::Readable for PCC_TPM2 {}
#[doc = "`write(|w| ..)` method takes [pcc_tpm2::W](pcc_tpm2::W) writer structure"]
impl crate::Writable for PCC_TPM2 {}
#[doc = "PCC TPM2 Register"]
pub mod pcc_tpm2;
#[doc = "PCC EMVSIM0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_emvsim0](pcc_emvsim0) module"]
pub type PCC_EMVSIM0 = crate::Reg<u32, _PCC_EMVSIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_EMVSIM0;
#[doc = "`read()` method returns [pcc_emvsim0::R](pcc_emvsim0::R) reader structure"]
impl crate::Readable for PCC_EMVSIM0 {}
#[doc = "`write(|w| ..)` method takes [pcc_emvsim0::W](pcc_emvsim0::W) writer structure"]
impl crate::Writable for PCC_EMVSIM0 {}
#[doc = "PCC EMVSIM0 Register"]
pub mod pcc_emvsim0;
#[doc = "PCC FLEXIO0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_flexio0](pcc_flexio0) module"]
pub type PCC_FLEXIO0 = crate::Reg<u32, _PCC_FLEXIO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FLEXIO0;
#[doc = "`read()` method returns [pcc_flexio0::R](pcc_flexio0::R) reader structure"]
impl crate::Readable for PCC_FLEXIO0 {}
#[doc = "`write(|w| ..)` method takes [pcc_flexio0::W](pcc_flexio0::W) writer structure"]
impl crate::Writable for PCC_FLEXIO0 {}
#[doc = "PCC FLEXIO0 Register"]
pub mod pcc_flexio0;
#[doc = "PCC LPI2C0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpi2c0](pcc_lpi2c0) module"]
pub type PCC_LPI2C0 = crate::Reg<u32, _PCC_LPI2C0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPI2C0;
#[doc = "`read()` method returns [pcc_lpi2c0::R](pcc_lpi2c0::R) reader structure"]
impl crate::Readable for PCC_LPI2C0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpi2c0::W](pcc_lpi2c0::W) writer structure"]
impl crate::Writable for PCC_LPI2C0 {}
#[doc = "PCC LPI2C0 Register"]
pub mod pcc_lpi2c0;
#[doc = "PCC LPI2C1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpi2c1](pcc_lpi2c1) module"]
pub type PCC_LPI2C1 = crate::Reg<u32, _PCC_LPI2C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPI2C1;
#[doc = "`read()` method returns [pcc_lpi2c1::R](pcc_lpi2c1::R) reader structure"]
impl crate::Readable for PCC_LPI2C1 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpi2c1::W](pcc_lpi2c1::W) writer structure"]
impl crate::Writable for PCC_LPI2C1 {}
#[doc = "PCC LPI2C1 Register"]
pub mod pcc_lpi2c1;
#[doc = "PCC LPI2C2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpi2c2](pcc_lpi2c2) module"]
pub type PCC_LPI2C2 = crate::Reg<u32, _PCC_LPI2C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPI2C2;
#[doc = "`read()` method returns [pcc_lpi2c2::R](pcc_lpi2c2::R) reader structure"]
impl crate::Readable for PCC_LPI2C2 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpi2c2::W](pcc_lpi2c2::W) writer structure"]
impl crate::Writable for PCC_LPI2C2 {}
#[doc = "PCC LPI2C2 Register"]
pub mod pcc_lpi2c2;
#[doc = "PCC I2S0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_i2s0](pcc_i2s0) module"]
pub type PCC_I2S0 = crate::Reg<u32, _PCC_I2S0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_I2S0;
#[doc = "`read()` method returns [pcc_i2s0::R](pcc_i2s0::R) reader structure"]
impl crate::Readable for PCC_I2S0 {}
#[doc = "`write(|w| ..)` method takes [pcc_i2s0::W](pcc_i2s0::W) writer structure"]
impl crate::Writable for PCC_I2S0 {}
#[doc = "PCC I2S0 Register"]
pub mod pcc_i2s0;
#[doc = "PCC USDHC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_usdhc0](pcc_usdhc0) module"]
pub type PCC_USDHC0 = crate::Reg<u32, _PCC_USDHC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_USDHC0;
#[doc = "`read()` method returns [pcc_usdhc0::R](pcc_usdhc0::R) reader structure"]
impl crate::Readable for PCC_USDHC0 {}
#[doc = "`write(|w| ..)` method takes [pcc_usdhc0::W](pcc_usdhc0::W) writer structure"]
impl crate::Writable for PCC_USDHC0 {}
#[doc = "PCC USDHC0 Register"]
pub mod pcc_usdhc0;
#[doc = "PCC LPSPI0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpspi0](pcc_lpspi0) module"]
pub type PCC_LPSPI0 = crate::Reg<u32, _PCC_LPSPI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI0;
#[doc = "`read()` method returns [pcc_lpspi0::R](pcc_lpspi0::R) reader structure"]
impl crate::Readable for PCC_LPSPI0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi0::W](pcc_lpspi0::W) writer structure"]
impl crate::Writable for PCC_LPSPI0 {}
#[doc = "PCC LPSPI0 Register"]
pub mod pcc_lpspi0;
#[doc = "PCC LPSPI1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpspi1](pcc_lpspi1) module"]
pub type PCC_LPSPI1 = crate::Reg<u32, _PCC_LPSPI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI1;
#[doc = "`read()` method returns [pcc_lpspi1::R](pcc_lpspi1::R) reader structure"]
impl crate::Readable for PCC_LPSPI1 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi1::W](pcc_lpspi1::W) writer structure"]
impl crate::Writable for PCC_LPSPI1 {}
#[doc = "PCC LPSPI1 Register"]
pub mod pcc_lpspi1;
#[doc = "PCC LPSPI2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpspi2](pcc_lpspi2) module"]
pub type PCC_LPSPI2 = crate::Reg<u32, _PCC_LPSPI2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI2;
#[doc = "`read()` method returns [pcc_lpspi2::R](pcc_lpspi2::R) reader structure"]
impl crate::Readable for PCC_LPSPI2 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi2::W](pcc_lpspi2::W) writer structure"]
impl crate::Writable for PCC_LPSPI2 {}
#[doc = "PCC LPSPI2 Register"]
pub mod pcc_lpspi2;
#[doc = "PCC LPUART0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpuart0](pcc_lpuart0) module"]
pub type PCC_LPUART0 = crate::Reg<u32, _PCC_LPUART0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART0;
#[doc = "`read()` method returns [pcc_lpuart0::R](pcc_lpuart0::R) reader structure"]
impl crate::Readable for PCC_LPUART0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart0::W](pcc_lpuart0::W) writer structure"]
impl crate::Writable for PCC_LPUART0 {}
#[doc = "PCC LPUART0 Register"]
pub mod pcc_lpuart0;
#[doc = "PCC LPUART1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpuart1](pcc_lpuart1) module"]
pub type PCC_LPUART1 = crate::Reg<u32, _PCC_LPUART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART1;
#[doc = "`read()` method returns [pcc_lpuart1::R](pcc_lpuart1::R) reader structure"]
impl crate::Readable for PCC_LPUART1 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart1::W](pcc_lpuart1::W) writer structure"]
impl crate::Writable for PCC_LPUART1 {}
#[doc = "PCC LPUART1 Register"]
pub mod pcc_lpuart1;
#[doc = "PCC LPUART2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpuart2](pcc_lpuart2) module"]
pub type PCC_LPUART2 = crate::Reg<u32, _PCC_LPUART2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART2;
#[doc = "`read()` method returns [pcc_lpuart2::R](pcc_lpuart2::R) reader structure"]
impl crate::Readable for PCC_LPUART2 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart2::W](pcc_lpuart2::W) writer structure"]
impl crate::Writable for PCC_LPUART2 {}
#[doc = "PCC LPUART2 Register"]
pub mod pcc_lpuart2;
#[doc = "PCC USB0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_usb0](pcc_usb0) module"]
pub type PCC_USB0 = crate::Reg<u32, _PCC_USB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_USB0;
#[doc = "`read()` method returns [pcc_usb0::R](pcc_usb0::R) reader structure"]
impl crate::Readable for PCC_USB0 {}
#[doc = "`write(|w| ..)` method takes [pcc_usb0::W](pcc_usb0::W) writer structure"]
impl crate::Writable for PCC_USB0 {}
#[doc = "PCC USB0 Register"]
pub mod pcc_usb0;
#[doc = "PCC PORTA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_porta](pcc_porta) module"]
pub type PCC_PORTA = crate::Reg<u32, _PCC_PORTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTA;
#[doc = "`read()` method returns [pcc_porta::R](pcc_porta::R) reader structure"]
impl crate::Readable for PCC_PORTA {}
#[doc = "`write(|w| ..)` method takes [pcc_porta::W](pcc_porta::W) writer structure"]
impl crate::Writable for PCC_PORTA {}
#[doc = "PCC PORTA Register"]
pub mod pcc_porta;
#[doc = "PCC PORTB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_portb](pcc_portb) module"]
pub type PCC_PORTB = crate::Reg<u32, _PCC_PORTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTB;
#[doc = "`read()` method returns [pcc_portb::R](pcc_portb::R) reader structure"]
impl crate::Readable for PCC_PORTB {}
#[doc = "`write(|w| ..)` method takes [pcc_portb::W](pcc_portb::W) writer structure"]
impl crate::Writable for PCC_PORTB {}
#[doc = "PCC PORTB Register"]
pub mod pcc_portb;
#[doc = "PCC PORTC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_portc](pcc_portc) module"]
pub type PCC_PORTC = crate::Reg<u32, _PCC_PORTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTC;
#[doc = "`read()` method returns [pcc_portc::R](pcc_portc::R) reader structure"]
impl crate::Readable for PCC_PORTC {}
#[doc = "`write(|w| ..)` method takes [pcc_portc::W](pcc_portc::W) writer structure"]
impl crate::Writable for PCC_PORTC {}
#[doc = "PCC PORTC Register"]
pub mod pcc_portc;
#[doc = "PCC PORTD Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_portd](pcc_portd) module"]
pub type PCC_PORTD = crate::Reg<u32, _PCC_PORTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTD;
#[doc = "`read()` method returns [pcc_portd::R](pcc_portd::R) reader structure"]
impl crate::Readable for PCC_PORTD {}
#[doc = "`write(|w| ..)` method takes [pcc_portd::W](pcc_portd::W) writer structure"]
impl crate::Writable for PCC_PORTD {}
#[doc = "PCC PORTD Register"]
pub mod pcc_portd;
#[doc = "PCC ADC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_adc0](pcc_adc0) module"]
pub type PCC_ADC0 = crate::Reg<u32, _PCC_ADC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_ADC0;
#[doc = "`read()` method returns [pcc_adc0::R](pcc_adc0::R) reader structure"]
impl crate::Readable for PCC_ADC0 {}
#[doc = "`write(|w| ..)` method takes [pcc_adc0::W](pcc_adc0::W) writer structure"]
impl crate::Writable for PCC_ADC0 {}
#[doc = "PCC ADC0 Register"]
pub mod pcc_adc0;
#[doc = "PCC LPDAC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpdac0](pcc_lpdac0) module"]
pub type PCC_LPDAC0 = crate::Reg<u32, _PCC_LPDAC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPDAC0;
#[doc = "`read()` method returns [pcc_lpdac0::R](pcc_lpdac0::R) reader structure"]
impl crate::Readable for PCC_LPDAC0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpdac0::W](pcc_lpdac0::W) writer structure"]
impl crate::Writable for PCC_LPDAC0 {}
#[doc = "PCC LPDAC0 Register"]
pub mod pcc_lpdac0;
#[doc = "PCC VREF Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_vref](pcc_vref) module"]
pub type PCC_VREF = crate::Reg<u32, _PCC_VREF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_VREF;
#[doc = "`read()` method returns [pcc_vref::R](pcc_vref::R) reader structure"]
impl crate::Readable for PCC_VREF {}
#[doc = "`write(|w| ..)` method takes [pcc_vref::W](pcc_vref::W) writer structure"]
impl crate::Writable for PCC_VREF {}
#[doc = "PCC VREF Register"]
pub mod pcc_vref;
#[doc = "PCC TRACE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_trace](pcc_trace) module"]
pub type PCC_TRACE = crate::Reg<u32, _PCC_TRACE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_TRACE;
#[doc = "`read()` method returns [pcc_trace::R](pcc_trace::R) reader structure"]
impl crate::Readable for PCC_TRACE {}
#[doc = "`write(|w| ..)` method takes [pcc_trace::W](pcc_trace::W) writer structure"]
impl crate::Writable for PCC_TRACE {}
#[doc = "PCC TRACE Register"]
pub mod pcc_trace;
