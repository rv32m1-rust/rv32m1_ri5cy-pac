#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  _reserved0: [u8; 32usize],
  #[doc = "0x20 - PCC DMA1 Register"]
  pub pcc_dma1: PCC_DMA1,
  _reserved1: [u8; 24usize],
  #[doc = "0x3c - PCC GPIOE Register"]
  pub pcc_gpioe: PCC_GPIOE,
  _reserved2: [u8; 24usize],
  #[doc = "0x58 - PCC XRDC_PAC Register"]
  pub pcc_xrdc_pac: PCC_XRDC_PAC,
  #[doc = "0x5c - PCC XRDC_MRC Register"]
  pub pcc_xrdc_mrc: PCC_XRDC_MRC,
  _reserved4: [u8; 12usize],
  #[doc = "0x6c - PCC SEMA42_1 Register"]
  pub pcc_sema42_1: PCC_SEMA42_1,
  _reserved5: [u8; 20usize],
  #[doc = "0x84 - PCC DMAMUX1 Register"]
  pub pcc_dmamux1: PCC_DMAMUX1,
  #[doc = "0x88 - PCC INTMUX1 Register"]
  pub pcc_intmux1: PCC_INTMUX1,
  _reserved7: [u8; 4usize],
  #[doc = "0x90 - PCC MUB Register"]
  pub pcc_mub: PCC_MUB,
  _reserved8: [u8; 12usize],
  #[doc = "0xa0 - PCC CAU3 Register"]
  pub pcc_cau3: PCC_CAU3,
  #[doc = "0xa4 - PCC TRNG Register"]
  pub pcc_trng: PCC_TRNG,
  #[doc = "0xa8 - PCC LPIT1 Register"]
  pub pcc_lpit1: PCC_LPIT1,
  _reserved11: [u8; 8usize],
  #[doc = "0xb4 - PCC TPM3 Register"]
  pub pcc_tpm3: PCC_TPM3,
  #[doc = "0xb8 - PCC LPI2C3 Register"]
  pub pcc_lpi2c3: PCC_LPI2C3,
  _reserved13: [u8; 24usize],
  #[doc = "0xd4 - PCC LPSPI3 Register"]
  pub pcc_lpspi3: PCC_LPSPI3,
  #[doc = "0xd8 - PCC LPUART3 Register"]
  pub pcc_lpuart3: PCC_LPUART3,
  #[doc = "0xdc - PCC PORTE Register"]
  pub pcc_porte: PCC_PORTE,
  _reserved16: [u8; 288usize],
  #[doc = "0x200 - PCC MTB Register"]
  pub pcc_mtb: PCC_MTB,
  #[doc = "0x204 - PCC EXT_CLK Register"]
  pub pcc_ext_clk: PCC_EXT_CLK,
}
#[doc = "PCC DMA1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_dma1](pcc_dma1) module"]
pub type PCC_DMA1 = crate::Reg<u32, _PCC_DMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_DMA1;
#[doc = "`read()` method returns [pcc_dma1::R](pcc_dma1::R) reader structure"]
impl crate::Readable for PCC_DMA1 {}
#[doc = "`write(|w| ..)` method takes [pcc_dma1::W](pcc_dma1::W) writer structure"]
impl crate::Writable for PCC_DMA1 {}
#[doc = "PCC DMA1 Register"]
pub mod pcc_dma1;
#[doc = "PCC GPIOE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_gpioe](pcc_gpioe) module"]
pub type PCC_GPIOE = crate::Reg<u32, _PCC_GPIOE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_GPIOE;
#[doc = "`read()` method returns [pcc_gpioe::R](pcc_gpioe::R) reader structure"]
impl crate::Readable for PCC_GPIOE {}
#[doc = "`write(|w| ..)` method takes [pcc_gpioe::W](pcc_gpioe::W) writer structure"]
impl crate::Writable for PCC_GPIOE {}
#[doc = "PCC GPIOE Register"]
pub mod pcc_gpioe;
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
#[doc = "PCC SEMA42_1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_sema42_1](pcc_sema42_1) module"]
pub type PCC_SEMA42_1 = crate::Reg<u32, _PCC_SEMA42_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_SEMA42_1;
#[doc = "`read()` method returns [pcc_sema42_1::R](pcc_sema42_1::R) reader structure"]
impl crate::Readable for PCC_SEMA42_1 {}
#[doc = "`write(|w| ..)` method takes [pcc_sema42_1::W](pcc_sema42_1::W) writer structure"]
impl crate::Writable for PCC_SEMA42_1 {}
#[doc = "PCC SEMA42_1 Register"]
pub mod pcc_sema42_1;
#[doc = "PCC DMAMUX1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_dmamux1](pcc_dmamux1) module"]
pub type PCC_DMAMUX1 = crate::Reg<u32, _PCC_DMAMUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_DMAMUX1;
#[doc = "`read()` method returns [pcc_dmamux1::R](pcc_dmamux1::R) reader structure"]
impl crate::Readable for PCC_DMAMUX1 {}
#[doc = "`write(|w| ..)` method takes [pcc_dmamux1::W](pcc_dmamux1::W) writer structure"]
impl crate::Writable for PCC_DMAMUX1 {}
#[doc = "PCC DMAMUX1 Register"]
pub mod pcc_dmamux1;
#[doc = "PCC INTMUX1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_intmux1](pcc_intmux1) module"]
pub type PCC_INTMUX1 = crate::Reg<u32, _PCC_INTMUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_INTMUX1;
#[doc = "`read()` method returns [pcc_intmux1::R](pcc_intmux1::R) reader structure"]
impl crate::Readable for PCC_INTMUX1 {}
#[doc = "`write(|w| ..)` method takes [pcc_intmux1::W](pcc_intmux1::W) writer structure"]
impl crate::Writable for PCC_INTMUX1 {}
#[doc = "PCC INTMUX1 Register"]
pub mod pcc_intmux1;
#[doc = "PCC MUB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_mub](pcc_mub) module"]
pub type PCC_MUB = crate::Reg<u32, _PCC_MUB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_MUB;
#[doc = "`read()` method returns [pcc_mub::R](pcc_mub::R) reader structure"]
impl crate::Readable for PCC_MUB {}
#[doc = "`write(|w| ..)` method takes [pcc_mub::W](pcc_mub::W) writer structure"]
impl crate::Writable for PCC_MUB {}
#[doc = "PCC MUB Register"]
pub mod pcc_mub;
#[doc = "PCC CAU3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_cau3](pcc_cau3) module"]
pub type PCC_CAU3 = crate::Reg<u32, _PCC_CAU3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_CAU3;
#[doc = "`read()` method returns [pcc_cau3::R](pcc_cau3::R) reader structure"]
impl crate::Readable for PCC_CAU3 {}
#[doc = "`write(|w| ..)` method takes [pcc_cau3::W](pcc_cau3::W) writer structure"]
impl crate::Writable for PCC_CAU3 {}
#[doc = "PCC CAU3 Register"]
pub mod pcc_cau3;
#[doc = "PCC TRNG Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_trng](pcc_trng) module"]
pub type PCC_TRNG = crate::Reg<u32, _PCC_TRNG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_TRNG;
#[doc = "`read()` method returns [pcc_trng::R](pcc_trng::R) reader structure"]
impl crate::Readable for PCC_TRNG {}
#[doc = "`write(|w| ..)` method takes [pcc_trng::W](pcc_trng::W) writer structure"]
impl crate::Writable for PCC_TRNG {}
#[doc = "PCC TRNG Register"]
pub mod pcc_trng;
#[doc = "PCC LPIT1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpit1](pcc_lpit1) module"]
pub type PCC_LPIT1 = crate::Reg<u32, _PCC_LPIT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPIT1;
#[doc = "`read()` method returns [pcc_lpit1::R](pcc_lpit1::R) reader structure"]
impl crate::Readable for PCC_LPIT1 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpit1::W](pcc_lpit1::W) writer structure"]
impl crate::Writable for PCC_LPIT1 {}
#[doc = "PCC LPIT1 Register"]
pub mod pcc_lpit1;
#[doc = "PCC TPM3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_tpm3](pcc_tpm3) module"]
pub type PCC_TPM3 = crate::Reg<u32, _PCC_TPM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_TPM3;
#[doc = "`read()` method returns [pcc_tpm3::R](pcc_tpm3::R) reader structure"]
impl crate::Readable for PCC_TPM3 {}
#[doc = "`write(|w| ..)` method takes [pcc_tpm3::W](pcc_tpm3::W) writer structure"]
impl crate::Writable for PCC_TPM3 {}
#[doc = "PCC TPM3 Register"]
pub mod pcc_tpm3;
#[doc = "PCC LPI2C3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpi2c3](pcc_lpi2c3) module"]
pub type PCC_LPI2C3 = crate::Reg<u32, _PCC_LPI2C3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPI2C3;
#[doc = "`read()` method returns [pcc_lpi2c3::R](pcc_lpi2c3::R) reader structure"]
impl crate::Readable for PCC_LPI2C3 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpi2c3::W](pcc_lpi2c3::W) writer structure"]
impl crate::Writable for PCC_LPI2C3 {}
#[doc = "PCC LPI2C3 Register"]
pub mod pcc_lpi2c3;
#[doc = "PCC LPSPI3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpspi3](pcc_lpspi3) module"]
pub type PCC_LPSPI3 = crate::Reg<u32, _PCC_LPSPI3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI3;
#[doc = "`read()` method returns [pcc_lpspi3::R](pcc_lpspi3::R) reader structure"]
impl crate::Readable for PCC_LPSPI3 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi3::W](pcc_lpspi3::W) writer structure"]
impl crate::Writable for PCC_LPSPI3 {}
#[doc = "PCC LPSPI3 Register"]
pub mod pcc_lpspi3;
#[doc = "PCC LPUART3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_lpuart3](pcc_lpuart3) module"]
pub type PCC_LPUART3 = crate::Reg<u32, _PCC_LPUART3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART3;
#[doc = "`read()` method returns [pcc_lpuart3::R](pcc_lpuart3::R) reader structure"]
impl crate::Readable for PCC_LPUART3 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart3::W](pcc_lpuart3::W) writer structure"]
impl crate::Writable for PCC_LPUART3 {}
#[doc = "PCC LPUART3 Register"]
pub mod pcc_lpuart3;
#[doc = "PCC PORTE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_porte](pcc_porte) module"]
pub type PCC_PORTE = crate::Reg<u32, _PCC_PORTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTE;
#[doc = "`read()` method returns [pcc_porte::R](pcc_porte::R) reader structure"]
impl crate::Readable for PCC_PORTE {}
#[doc = "`write(|w| ..)` method takes [pcc_porte::W](pcc_porte::W) writer structure"]
impl crate::Writable for PCC_PORTE {}
#[doc = "PCC PORTE Register"]
pub mod pcc_porte;
#[doc = "PCC MTB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_mtb](pcc_mtb) module"]
pub type PCC_MTB = crate::Reg<u32, _PCC_MTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_MTB;
#[doc = "`read()` method returns [pcc_mtb::R](pcc_mtb::R) reader structure"]
impl crate::Readable for PCC_MTB {}
#[doc = "`write(|w| ..)` method takes [pcc_mtb::W](pcc_mtb::W) writer structure"]
impl crate::Writable for PCC_MTB {}
#[doc = "PCC MTB Register"]
pub mod pcc_mtb;
#[doc = "PCC EXT_CLK Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcc_ext_clk](pcc_ext_clk) module"]
pub type PCC_EXT_CLK = crate::Reg<u32, _PCC_EXT_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_EXT_CLK;
#[doc = "`read()` method returns [pcc_ext_clk::R](pcc_ext_clk::R) reader structure"]
impl crate::Readable for PCC_EXT_CLK {}
#[doc = "`write(|w| ..)` method takes [pcc_ext_clk::W](pcc_ext_clk::W) writer structure"]
impl crate::Writable for PCC_EXT_CLK {}
#[doc = "PCC EXT_CLK Register"]
pub mod pcc_ext_clk;
