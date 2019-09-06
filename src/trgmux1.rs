#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - TRGMUX DMAMUX1 Register"]
  pub dmamux1: DMAMUX1,
  #[doc = "0x04 - TRGMUX LPIT1 Register"]
  pub lpit1: LPIT1,
  #[doc = "0x08 - TRGMUX TPM3 Register"]
  pub tpm3: TPM3,
  #[doc = "0x0c - TRGMUX LPI2C3 Register"]
  pub lpi2c3: LPI2C3,
  #[doc = "0x10 - TRGMUX LPSPI3 Register"]
  pub lpspi3: LPSPI3,
  #[doc = "0x14 - TRGMUX LPUART3 Register"]
  pub lpuart3: LPUART3,
  #[doc = "0x18 - TRGMUX LPCMP1 Register"]
  pub lpcmp1: LPCMP1,
  #[doc = "0x1c - TRGMUX DMAMUX0 Register"]
  pub dmamux0: DMAMUX0,
  #[doc = "0x20 - TRGMUX LPIT0 Register"]
  pub lpit0: LPIT0,
  #[doc = "0x24 - TRGMUX TPM0 Register"]
  pub tpm0: TPM0,
  #[doc = "0x28 - TRGMUX TPM1 Register"]
  pub tpm1: TPM1,
  #[doc = "0x2c - TRGMUX TPM2 Register"]
  pub tpm2: TPM2,
  #[doc = "0x30 - TRGMUX FLEXIO0 Register"]
  pub flexio0: FLEXIO0,
  #[doc = "0x34 - TRGMUX LPI2C0 Register"]
  pub lpi2c0: LPI2C0,
  #[doc = "0x38 - TRGMUX LPI2C1 Register"]
  pub lpi2c1: LPI2C1,
  #[doc = "0x3c - TRGMUX LPI2C2 Register"]
  pub lpi2c2: LPI2C2,
  #[doc = "0x40 - TRGMUX LPSPI0 Register"]
  pub lpspi0: LPSPI0,
  #[doc = "0x44 - TRGMUX LPSPI1 Register"]
  pub lpspi1: LPSPI1,
  #[doc = "0x48 - TRGMUX LPSPI2 Register"]
  pub lpspi2: LPSPI2,
  #[doc = "0x4c - TRGMUX LPUART0 Register"]
  pub lpuart0: LPUART0,
  #[doc = "0x50 - TRGMUX LPUART1 Register"]
  pub lpuart1: LPUART1,
  #[doc = "0x54 - TRGMUX LPUART2 Register"]
  pub lpuart2: LPUART2,
  #[doc = "0x58 - TRGMUX ADC0 Register"]
  pub adc0: ADC0,
  #[doc = "0x5c - TRGMUX LPCMP0 Register"]
  pub lpcmp0: LPCMP0,
  #[doc = "0x60 - TRGMUX LPDAC0 Register"]
  pub lpdac0: LPDAC0,
}
#[doc = "TRGMUX DMAMUX1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmamux1](dmamux1) module"]
pub type DMAMUX1 = crate::Reg<u32, _DMAMUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1;
#[doc = "`read()` method returns [dmamux1::R](dmamux1::R) reader structure"]
impl crate::Readable for DMAMUX1 {}
#[doc = "`write(|w| ..)` method takes [dmamux1::W](dmamux1::W) writer structure"]
impl crate::Writable for DMAMUX1 {}
#[doc = "TRGMUX DMAMUX1 Register"]
pub mod dmamux1;
#[doc = "TRGMUX LPIT1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpit1](lpit1) module"]
pub type LPIT1 = crate::Reg<u32, _LPIT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPIT1;
#[doc = "`read()` method returns [lpit1::R](lpit1::R) reader structure"]
impl crate::Readable for LPIT1 {}
#[doc = "`write(|w| ..)` method takes [lpit1::W](lpit1::W) writer structure"]
impl crate::Writable for LPIT1 {}
#[doc = "TRGMUX LPIT1 Register"]
pub mod lpit1;
#[doc = "TRGMUX TPM3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpm3](tpm3) module"]
pub type TPM3 = crate::Reg<u32, _TPM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPM3;
#[doc = "`read()` method returns [tpm3::R](tpm3::R) reader structure"]
impl crate::Readable for TPM3 {}
#[doc = "`write(|w| ..)` method takes [tpm3::W](tpm3::W) writer structure"]
impl crate::Writable for TPM3 {}
#[doc = "TRGMUX TPM3 Register"]
pub mod tpm3;
#[doc = "TRGMUX LPI2C3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c3](lpi2c3) module"]
pub type LPI2C3 = crate::Reg<u32, _LPI2C3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C3;
#[doc = "`read()` method returns [lpi2c3::R](lpi2c3::R) reader structure"]
impl crate::Readable for LPI2C3 {}
#[doc = "`write(|w| ..)` method takes [lpi2c3::W](lpi2c3::W) writer structure"]
impl crate::Writable for LPI2C3 {}
#[doc = "TRGMUX LPI2C3 Register"]
pub mod lpi2c3;
#[doc = "TRGMUX LPSPI3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi3](lpspi3) module"]
pub type LPSPI3 = crate::Reg<u32, _LPSPI3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI3;
#[doc = "`read()` method returns [lpspi3::R](lpspi3::R) reader structure"]
impl crate::Readable for LPSPI3 {}
#[doc = "`write(|w| ..)` method takes [lpspi3::W](lpspi3::W) writer structure"]
impl crate::Writable for LPSPI3 {}
#[doc = "TRGMUX LPSPI3 Register"]
pub mod lpspi3;
#[doc = "TRGMUX LPUART3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart3](lpuart3) module"]
pub type LPUART3 = crate::Reg<u32, _LPUART3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART3;
#[doc = "`read()` method returns [lpuart3::R](lpuart3::R) reader structure"]
impl crate::Readable for LPUART3 {}
#[doc = "`write(|w| ..)` method takes [lpuart3::W](lpuart3::W) writer structure"]
impl crate::Writable for LPUART3 {}
#[doc = "TRGMUX LPUART3 Register"]
pub mod lpuart3;
#[doc = "TRGMUX LPCMP1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpcmp1](lpcmp1) module"]
pub type LPCMP1 = crate::Reg<u32, _LPCMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCMP1;
#[doc = "`read()` method returns [lpcmp1::R](lpcmp1::R) reader structure"]
impl crate::Readable for LPCMP1 {}
#[doc = "`write(|w| ..)` method takes [lpcmp1::W](lpcmp1::W) writer structure"]
impl crate::Writable for LPCMP1 {}
#[doc = "TRGMUX LPCMP1 Register"]
pub mod lpcmp1;
#[doc = "TRGMUX DMAMUX0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmamux0](dmamux0) module"]
pub type DMAMUX0 = crate::Reg<u32, _DMAMUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX0;
#[doc = "`read()` method returns [dmamux0::R](dmamux0::R) reader structure"]
impl crate::Readable for DMAMUX0 {}
#[doc = "`write(|w| ..)` method takes [dmamux0::W](dmamux0::W) writer structure"]
impl crate::Writable for DMAMUX0 {}
#[doc = "TRGMUX DMAMUX0 Register"]
pub mod dmamux0;
#[doc = "TRGMUX LPIT0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpit0](lpit0) module"]
pub type LPIT0 = crate::Reg<u32, _LPIT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPIT0;
#[doc = "`read()` method returns [lpit0::R](lpit0::R) reader structure"]
impl crate::Readable for LPIT0 {}
#[doc = "`write(|w| ..)` method takes [lpit0::W](lpit0::W) writer structure"]
impl crate::Writable for LPIT0 {}
#[doc = "TRGMUX LPIT0 Register"]
pub mod lpit0;
#[doc = "TRGMUX TPM0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpm0](tpm0) module"]
pub type TPM0 = crate::Reg<u32, _TPM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPM0;
#[doc = "`read()` method returns [tpm0::R](tpm0::R) reader structure"]
impl crate::Readable for TPM0 {}
#[doc = "`write(|w| ..)` method takes [tpm0::W](tpm0::W) writer structure"]
impl crate::Writable for TPM0 {}
#[doc = "TRGMUX TPM0 Register"]
pub mod tpm0;
#[doc = "TRGMUX TPM1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpm1](tpm1) module"]
pub type TPM1 = crate::Reg<u32, _TPM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPM1;
#[doc = "`read()` method returns [tpm1::R](tpm1::R) reader structure"]
impl crate::Readable for TPM1 {}
#[doc = "`write(|w| ..)` method takes [tpm1::W](tpm1::W) writer structure"]
impl crate::Writable for TPM1 {}
#[doc = "TRGMUX TPM1 Register"]
pub mod tpm1;
#[doc = "TRGMUX TPM2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpm2](tpm2) module"]
pub type TPM2 = crate::Reg<u32, _TPM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPM2;
#[doc = "`read()` method returns [tpm2::R](tpm2::R) reader structure"]
impl crate::Readable for TPM2 {}
#[doc = "`write(|w| ..)` method takes [tpm2::W](tpm2::W) writer structure"]
impl crate::Writable for TPM2 {}
#[doc = "TRGMUX TPM2 Register"]
pub mod tpm2;
#[doc = "TRGMUX FLEXIO0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexio0](flexio0) module"]
pub type FLEXIO0 = crate::Reg<u32, _FLEXIO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXIO0;
#[doc = "`read()` method returns [flexio0::R](flexio0::R) reader structure"]
impl crate::Readable for FLEXIO0 {}
#[doc = "`write(|w| ..)` method takes [flexio0::W](flexio0::W) writer structure"]
impl crate::Writable for FLEXIO0 {}
#[doc = "TRGMUX FLEXIO0 Register"]
pub mod flexio0;
#[doc = "TRGMUX LPI2C0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c0](lpi2c0) module"]
pub type LPI2C0 = crate::Reg<u32, _LPI2C0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C0;
#[doc = "`read()` method returns [lpi2c0::R](lpi2c0::R) reader structure"]
impl crate::Readable for LPI2C0 {}
#[doc = "`write(|w| ..)` method takes [lpi2c0::W](lpi2c0::W) writer structure"]
impl crate::Writable for LPI2C0 {}
#[doc = "TRGMUX LPI2C0 Register"]
pub mod lpi2c0;
#[doc = "TRGMUX LPI2C1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c1](lpi2c1) module"]
pub type LPI2C1 = crate::Reg<u32, _LPI2C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C1;
#[doc = "`read()` method returns [lpi2c1::R](lpi2c1::R) reader structure"]
impl crate::Readable for LPI2C1 {}
#[doc = "`write(|w| ..)` method takes [lpi2c1::W](lpi2c1::W) writer structure"]
impl crate::Writable for LPI2C1 {}
#[doc = "TRGMUX LPI2C1 Register"]
pub mod lpi2c1;
#[doc = "TRGMUX LPI2C2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c2](lpi2c2) module"]
pub type LPI2C2 = crate::Reg<u32, _LPI2C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C2;
#[doc = "`read()` method returns [lpi2c2::R](lpi2c2::R) reader structure"]
impl crate::Readable for LPI2C2 {}
#[doc = "`write(|w| ..)` method takes [lpi2c2::W](lpi2c2::W) writer structure"]
impl crate::Writable for LPI2C2 {}
#[doc = "TRGMUX LPI2C2 Register"]
pub mod lpi2c2;
#[doc = "TRGMUX LPSPI0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi0](lpspi0) module"]
pub type LPSPI0 = crate::Reg<u32, _LPSPI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI0;
#[doc = "`read()` method returns [lpspi0::R](lpspi0::R) reader structure"]
impl crate::Readable for LPSPI0 {}
#[doc = "`write(|w| ..)` method takes [lpspi0::W](lpspi0::W) writer structure"]
impl crate::Writable for LPSPI0 {}
#[doc = "TRGMUX LPSPI0 Register"]
pub mod lpspi0;
#[doc = "TRGMUX LPSPI1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi1](lpspi1) module"]
pub type LPSPI1 = crate::Reg<u32, _LPSPI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI1;
#[doc = "`read()` method returns [lpspi1::R](lpspi1::R) reader structure"]
impl crate::Readable for LPSPI1 {}
#[doc = "`write(|w| ..)` method takes [lpspi1::W](lpspi1::W) writer structure"]
impl crate::Writable for LPSPI1 {}
#[doc = "TRGMUX LPSPI1 Register"]
pub mod lpspi1;
#[doc = "TRGMUX LPSPI2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi2](lpspi2) module"]
pub type LPSPI2 = crate::Reg<u32, _LPSPI2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI2;
#[doc = "`read()` method returns [lpspi2::R](lpspi2::R) reader structure"]
impl crate::Readable for LPSPI2 {}
#[doc = "`write(|w| ..)` method takes [lpspi2::W](lpspi2::W) writer structure"]
impl crate::Writable for LPSPI2 {}
#[doc = "TRGMUX LPSPI2 Register"]
pub mod lpspi2;
#[doc = "TRGMUX LPUART0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart0](lpuart0) module"]
pub type LPUART0 = crate::Reg<u32, _LPUART0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART0;
#[doc = "`read()` method returns [lpuart0::R](lpuart0::R) reader structure"]
impl crate::Readable for LPUART0 {}
#[doc = "`write(|w| ..)` method takes [lpuart0::W](lpuart0::W) writer structure"]
impl crate::Writable for LPUART0 {}
#[doc = "TRGMUX LPUART0 Register"]
pub mod lpuart0;
#[doc = "TRGMUX LPUART1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart1](lpuart1) module"]
pub type LPUART1 = crate::Reg<u32, _LPUART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART1;
#[doc = "`read()` method returns [lpuart1::R](lpuart1::R) reader structure"]
impl crate::Readable for LPUART1 {}
#[doc = "`write(|w| ..)` method takes [lpuart1::W](lpuart1::W) writer structure"]
impl crate::Writable for LPUART1 {}
#[doc = "TRGMUX LPUART1 Register"]
pub mod lpuart1;
#[doc = "TRGMUX LPUART2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart2](lpuart2) module"]
pub type LPUART2 = crate::Reg<u32, _LPUART2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART2;
#[doc = "`read()` method returns [lpuart2::R](lpuart2::R) reader structure"]
impl crate::Readable for LPUART2 {}
#[doc = "`write(|w| ..)` method takes [lpuart2::W](lpuart2::W) writer structure"]
impl crate::Writable for LPUART2 {}
#[doc = "TRGMUX LPUART2 Register"]
pub mod lpuart2;
#[doc = "TRGMUX ADC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc0](adc0) module"]
pub type ADC0 = crate::Reg<u32, _ADC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC0;
#[doc = "`read()` method returns [adc0::R](adc0::R) reader structure"]
impl crate::Readable for ADC0 {}
#[doc = "`write(|w| ..)` method takes [adc0::W](adc0::W) writer structure"]
impl crate::Writable for ADC0 {}
#[doc = "TRGMUX ADC0 Register"]
pub mod adc0;
#[doc = "TRGMUX LPCMP0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpcmp0](lpcmp0) module"]
pub type LPCMP0 = crate::Reg<u32, _LPCMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCMP0;
#[doc = "`read()` method returns [lpcmp0::R](lpcmp0::R) reader structure"]
impl crate::Readable for LPCMP0 {}
#[doc = "`write(|w| ..)` method takes [lpcmp0::W](lpcmp0::W) writer structure"]
impl crate::Writable for LPCMP0 {}
#[doc = "TRGMUX LPCMP0 Register"]
pub mod lpcmp0;
#[doc = "TRGMUX LPDAC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpdac0](lpdac0) module"]
pub type LPDAC0 = crate::Reg<u32, _LPDAC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPDAC0;
#[doc = "`read()` method returns [lpdac0::R](lpdac0::R) reader structure"]
impl crate::Readable for LPDAC0 {}
#[doc = "`write(|w| ..)` method takes [lpdac0::W](lpdac0::W) writer structure"]
impl crate::Writable for LPDAC0 {}
#[doc = "TRGMUX LPDAC0 Register"]
pub mod lpdac0;
