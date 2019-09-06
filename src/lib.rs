#![doc = "Peripheral access API for RV32M1_RI5CY microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "MSCM"]
pub struct MSCM {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSCM {}
impl MSCM {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const mscm::RegisterBlock {
    0x4000_1000 as *const _
  }
}
impl Deref for MSCM {
  type Target = mscm::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*MSCM::ptr() }
  }
}
#[doc = "MSCM"]
pub mod mscm;
#[doc = "AXBS"]
pub struct AXBS0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXBS0 {}
impl AXBS0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const axbs0::RegisterBlock {
    0x4000_4000 as *const _
  }
}
impl Deref for AXBS0 {
  type Target = axbs0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*AXBS0::ptr() }
  }
}
#[doc = "AXBS"]
pub mod axbs0;
#[doc = "DMA"]
pub struct DMA0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const dma0::RegisterBlock {
    0x4000_8000 as *const _
  }
}
impl Deref for DMA0 {
  type Target = dma0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*DMA0::ptr() }
  }
}
#[doc = "DMA"]
pub mod dma0;
#[doc = "FB"]
pub struct FB {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for FB {}
impl FB {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const fb::RegisterBlock {
    0x4000_c000 as *const _
  }
}
impl Deref for FB {
  type Target = fb::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*FB::ptr() }
  }
}
#[doc = "FB"]
pub mod fb;
#[doc = "XRDC"]
pub struct XRDC {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for XRDC {}
impl XRDC {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const xrdc::RegisterBlock {
    0x4001_4000 as *const _
  }
}
impl Deref for XRDC {
  type Target = xrdc::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*XRDC::ptr() }
  }
}
#[doc = "XRDC"]
pub mod xrdc;
#[doc = "sema42_ips"]
pub struct SEMA420 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEMA420 {}
impl SEMA420 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const sema420::RegisterBlock {
    0x4001_b000 as *const _
  }
}
impl Deref for SEMA420 {
  type Target = sema420::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SEMA420::ptr() }
  }
}
#[doc = "sema42_ips"]
pub mod sema420;
#[doc = "crr_cmc0"]
pub struct SMC0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC0 {}
impl SMC0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const smc0::RegisterBlock {
    0x4002_0000 as *const _
  }
}
impl Deref for SMC0 {
  type Target = smc0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SMC0::ptr() }
  }
}
#[doc = "crr_cmc0"]
pub mod smc0;
#[doc = "DMA_CH_MUX"]
pub struct DMAMUX0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX0 {}
impl DMAMUX0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const dmamux0::RegisterBlock {
    0x4002_1000 as *const _
  }
}
impl Deref for DMAMUX0 {
  type Target = dmamux0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*DMAMUX0::ptr() }
  }
}
#[doc = "DMA_CH_MUX"]
pub mod dmamux0;
#[doc = "EWM"]
pub struct EWM {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const ewm::RegisterBlock {
    0x4002_2000 as *const _
  }
}
impl Deref for EWM {
  type Target = ewm::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*EWM::ptr() }
  }
}
#[doc = "EWM"]
pub mod ewm;
#[doc = "Flash"]
pub struct FTFE {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFE {}
impl FTFE {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const ftfe::RegisterBlock {
    0x4002_3000 as *const _
  }
}
impl Deref for FTFE {
  type Target = ftfe::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*FTFE::ptr() }
  }
}
#[doc = "Flash"]
pub mod ftfe;
#[doc = "LLWU"]
pub struct LLWU0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU0 {}
impl LLWU0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const llwu0::RegisterBlock {
    0x4002_4000 as *const _
  }
}
impl Deref for LLWU0 {
  type Target = llwu0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LLWU0::ptr() }
  }
}
#[doc = "LLWU"]
pub mod llwu0;
#[doc = "MUA"]
pub struct MUA {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for MUA {}
impl MUA {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const mua::RegisterBlock {
    0x4002_5000 as *const _
  }
}
impl Deref for MUA {
  type Target = mua::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*MUA::ptr() }
  }
}
#[doc = "MUA"]
pub mod mua;
#[doc = "SIM"]
pub struct SIM {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const sim::RegisterBlock {
    0x4002_6000 as *const _
  }
}
impl Deref for SIM {
  type Target = sim::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SIM::ptr() }
  }
}
#[doc = "SIM"]
pub mod sim;
#[doc = "USBVREG"]
pub struct USBVREG {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBVREG {}
impl USBVREG {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const usbvreg::RegisterBlock {
    0x4002_7000 as *const _
  }
}
impl Deref for USBVREG {
  type Target = usbvreg::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*USBVREG::ptr() }
  }
}
#[doc = "USBVREG"]
pub mod usbvreg;
#[doc = "SPM"]
pub struct SPM {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPM {}
impl SPM {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const spm::RegisterBlock {
    0x4002_8000 as *const _
  }
}
impl Deref for SPM {
  type Target = spm::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SPM::ptr() }
  }
}
#[doc = "SPM"]
pub mod spm;
#[doc = "TRGMUX"]
pub struct TRGMUX0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRGMUX0 {}
impl TRGMUX0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const trgmux0::RegisterBlock {
    0x4002_9000 as *const _
  }
}
impl Deref for TRGMUX0 {
  type Target = trgmux0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TRGMUX0::ptr() }
  }
}
#[doc = "TRGMUX"]
pub mod trgmux0;
#[doc = "WDOG"]
pub struct WDOG0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG0 {}
impl WDOG0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const wdog0::RegisterBlock {
    0x4002_a000 as *const _
  }
}
impl Deref for WDOG0 {
  type Target = wdog0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*WDOG0::ptr() }
  }
}
#[doc = "WDOG"]
pub mod wdog0;
#[doc = "PCC"]
pub struct PCC0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCC0 {}
impl PCC0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const pcc0::RegisterBlock {
    0x4002_b000 as *const _
  }
}
impl Deref for PCC0 {
  type Target = pcc0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PCC0::ptr() }
  }
}
#[doc = "PCC"]
pub mod pcc0;
#[doc = "SCG"]
pub struct SCG {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCG {}
impl SCG {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const scg::RegisterBlock {
    0x4002_c000 as *const _
  }
}
impl Deref for SCG {
  type Target = scg::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SCG::ptr() }
  }
}
#[doc = "SCG"]
pub mod scg;
#[doc = "CRC"]
pub struct CRC {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const crc::RegisterBlock {
    0x4002_f000 as *const _
  }
}
impl Deref for CRC {
  type Target = crc::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*CRC::ptr() }
  }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "LPIT"]
pub struct LPIT0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPIT0 {}
impl LPIT0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpit0::RegisterBlock {
    0x4003_0000 as *const _
  }
}
impl Deref for LPIT0 {
  type Target = lpit0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPIT0::ptr() }
  }
}
#[doc = "LPIT"]
pub mod lpit0;
#[doc = "RTC"]
pub struct RTC {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const rtc::RegisterBlock {
    0x4003_1000 as *const _
  }
}
impl Deref for RTC {
  type Target = rtc::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*RTC::ptr() }
  }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "LPTMR"]
pub struct LPTMR0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lptmr0::RegisterBlock {
    0x4003_2000 as *const _
  }
}
impl Deref for LPTMR0 {
  type Target = lptmr0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPTMR0::ptr() }
  }
}
#[doc = "LPTMR"]
pub mod lptmr0;
#[doc = "LPTMR"]
pub struct LPTMR1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR1 {}
impl LPTMR1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lptmr1::RegisterBlock {
    0x4003_3000 as *const _
  }
}
impl Deref for LPTMR1 {
  type Target = lptmr1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPTMR1::ptr() }
  }
}
#[doc = "LPTMR"]
pub mod lptmr1;
#[doc = "TSTMRA"]
pub struct TSTMRA {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSTMRA {}
impl TSTMRA {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const tstmra::RegisterBlock {
    0x4003_4000 as *const _
  }
}
impl Deref for TSTMRA {
  type Target = tstmra::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TSTMRA::ptr() }
  }
}
#[doc = "TSTMRA"]
pub mod tstmra;
#[doc = "TPM"]
pub struct TPM0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM0 {}
impl TPM0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const tpm0::RegisterBlock {
    0x4003_5000 as *const _
  }
}
impl Deref for TPM0 {
  type Target = tpm0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TPM0::ptr() }
  }
}
#[doc = "TPM"]
pub mod tpm0;
#[doc = "TPM"]
pub struct TPM1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM1 {}
impl TPM1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const tpm1::RegisterBlock {
    0x4003_6000 as *const _
  }
}
impl Deref for TPM1 {
  type Target = tpm1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TPM1::ptr() }
  }
}
#[doc = "TPM"]
pub mod tpm1;
#[doc = "TPM"]
pub struct TPM2 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM2 {}
impl TPM2 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const tpm2::RegisterBlock {
    0x4003_7000 as *const _
  }
}
impl Deref for TPM2 {
  type Target = tpm2::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TPM2::ptr() }
  }
}
#[doc = "TPM"]
pub mod tpm2;
#[doc = "EMVSIM"]
pub struct EMVSIM0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMVSIM0 {}
impl EMVSIM0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const emvsim0::RegisterBlock {
    0x4003_8000 as *const _
  }
}
impl Deref for EMVSIM0 {
  type Target = emvsim0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*EMVSIM0::ptr() }
  }
}
#[doc = "EMVSIM"]
pub mod emvsim0;
#[doc = "FLEXIO"]
pub struct FLEXIO0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXIO0 {}
impl FLEXIO0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const flexio0::RegisterBlock {
    0x4003_9000 as *const _
  }
}
impl Deref for FLEXIO0 {
  type Target = flexio0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*FLEXIO0::ptr() }
  }
}
#[doc = "FLEXIO"]
pub mod flexio0;
#[doc = "LPI2C"]
pub struct LPI2C0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C0 {}
impl LPI2C0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpi2c0::RegisterBlock {
    0x4003_a000 as *const _
  }
}
impl Deref for LPI2C0 {
  type Target = lpi2c0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPI2C0::ptr() }
  }
}
#[doc = "LPI2C"]
pub mod lpi2c0;
#[doc = "LPI2C"]
pub struct LPI2C1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C1 {}
impl LPI2C1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpi2c1::RegisterBlock {
    0x4003_b000 as *const _
  }
}
impl Deref for LPI2C1 {
  type Target = lpi2c1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPI2C1::ptr() }
  }
}
#[doc = "LPI2C"]
pub mod lpi2c1;
#[doc = "LPI2C"]
pub struct LPI2C2 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C2 {}
impl LPI2C2 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpi2c2::RegisterBlock {
    0x4003_c000 as *const _
  }
}
impl Deref for LPI2C2 {
  type Target = lpi2c2::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPI2C2::ptr() }
  }
}
#[doc = "LPI2C"]
pub mod lpi2c2;
#[doc = "I2S"]
pub struct I2S0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const i2s0::RegisterBlock {
    0x4003_d000 as *const _
  }
}
impl Deref for I2S0 {
  type Target = i2s0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*I2S0::ptr() }
  }
}
#[doc = "I2S"]
pub mod i2s0;
#[doc = "uSDHC"]
pub struct USDHC0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for USDHC0 {}
impl USDHC0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const usdhc0::RegisterBlock {
    0x4003_e000 as *const _
  }
}
impl Deref for USDHC0 {
  type Target = usdhc0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*USDHC0::ptr() }
  }
}
#[doc = "uSDHC"]
pub mod usdhc0;
#[doc = "LPSPI"]
pub struct LPSPI0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI0 {}
impl LPSPI0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpspi0::RegisterBlock {
    0x4003_f000 as *const _
  }
}
impl Deref for LPSPI0 {
  type Target = lpspi0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPSPI0::ptr() }
  }
}
#[doc = "LPSPI"]
pub mod lpspi0;
#[doc = "LPSPI"]
pub struct LPSPI1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI1 {}
impl LPSPI1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpspi1::RegisterBlock {
    0x4004_0000 as *const _
  }
}
impl Deref for LPSPI1 {
  type Target = lpspi1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPSPI1::ptr() }
  }
}
#[doc = "LPSPI"]
pub mod lpspi1;
#[doc = "LPSPI"]
pub struct LPSPI2 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI2 {}
impl LPSPI2 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpspi2::RegisterBlock {
    0x4004_1000 as *const _
  }
}
impl Deref for LPSPI2 {
  type Target = lpspi2::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPSPI2::ptr() }
  }
}
#[doc = "LPSPI"]
pub mod lpspi2;
#[doc = "LPUART"]
pub struct LPUART0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART0 {}
impl LPUART0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpuart0::RegisterBlock {
    0x4004_2000 as *const _
  }
}
impl Deref for LPUART0 {
  type Target = lpuart0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPUART0::ptr() }
  }
}
#[doc = "LPUART"]
pub mod lpuart0;
#[doc = "LPUART"]
pub struct LPUART1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpuart1::RegisterBlock {
    0x4004_3000 as *const _
  }
}
impl Deref for LPUART1 {
  type Target = lpuart1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPUART1::ptr() }
  }
}
#[doc = "LPUART"]
pub mod lpuart1;
#[doc = "LPUART"]
pub struct LPUART2 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART2 {}
impl LPUART2 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpuart2::RegisterBlock {
    0x4004_4000 as *const _
  }
}
impl Deref for LPUART2 {
  type Target = lpuart2::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPUART2::ptr() }
  }
}
#[doc = "LPUART"]
pub mod lpuart2;
#[doc = "USB"]
pub struct USB0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const usb0::RegisterBlock {
    0x4004_5000 as *const _
  }
}
impl Deref for USB0 {
  type Target = usb0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*USB0::ptr() }
  }
}
#[doc = "USB"]
pub mod usb0;
#[doc = "PORT"]
pub struct PORTA {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const porta::RegisterBlock {
    0x4004_6000 as *const _
  }
}
impl Deref for PORTA {
  type Target = porta::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PORTA::ptr() }
  }
}
#[doc = "PORT"]
pub mod porta;
#[doc = "PORT"]
pub struct PORTB {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const portb::RegisterBlock {
    0x4004_7000 as *const _
  }
}
impl Deref for PORTB {
  type Target = portb::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PORTB::ptr() }
  }
}
#[doc = "PORT"]
pub mod portb;
#[doc = "PORT"]
pub struct PORTC {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const portc::RegisterBlock {
    0x4004_8000 as *const _
  }
}
impl Deref for PORTC {
  type Target = portc::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PORTC::ptr() }
  }
}
#[doc = "PORT"]
pub mod portc;
#[doc = "PORT"]
pub struct PORTD {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const portd::RegisterBlock {
    0x4004_9000 as *const _
  }
}
impl Deref for PORTD {
  type Target = portd::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PORTD::ptr() }
  }
}
#[doc = "PORT"]
pub mod portd;
#[doc = "ADC"]
pub struct ADC0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const adc0::RegisterBlock {
    0x4004_a000 as *const _
  }
}
impl Deref for ADC0 {
  type Target = adc0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*ADC0::ptr() }
  }
}
#[doc = "ADC"]
pub mod adc0;
#[doc = "LPCMP"]
pub struct LPCMP0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCMP0 {}
impl LPCMP0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpcmp0::RegisterBlock {
    0x4004_b000 as *const _
  }
}
impl Deref for LPCMP0 {
  type Target = lpcmp0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPCMP0::ptr() }
  }
}
#[doc = "LPCMP"]
pub mod lpcmp0;
#[doc = "LPDAC"]
pub struct LPDAC0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPDAC0 {}
impl LPDAC0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpdac0::RegisterBlock {
    0x4004_c000 as *const _
  }
}
impl Deref for LPDAC0 {
  type Target = lpdac0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPDAC0::ptr() }
  }
}
#[doc = "LPDAC"]
pub mod lpdac0;
#[doc = "VREF"]
pub struct VREF {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const vref::RegisterBlock {
    0x4004_d000 as *const _
  }
}
impl Deref for VREF {
  type Target = vref::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*VREF::ptr() }
  }
}
#[doc = "VREF"]
pub mod vref;
#[doc = "sema42_ips"]
pub struct SEMA421 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEMA421 {}
impl SEMA421 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const sema421::RegisterBlock {
    0x4101_b000 as *const _
  }
}
impl Deref for SEMA421 {
  type Target = sema421::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SEMA421::ptr() }
  }
}
#[doc = "sema42_ips"]
pub mod sema421;
#[doc = "crr_cmc1"]
pub struct SMC1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC1 {}
impl SMC1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const smc1::RegisterBlock {
    0x4102_0000 as *const _
  }
}
impl Deref for SMC1 {
  type Target = smc1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*SMC1::ptr() }
  }
}
#[doc = "crr_cmc1"]
pub mod smc1;
#[doc = "LLWU"]
pub struct LLWU1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU1 {}
impl LLWU1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const llwu1::RegisterBlock {
    0x4102_3000 as *const _
  }
}
impl Deref for LLWU1 {
  type Target = llwu1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LLWU1::ptr() }
  }
}
#[doc = "LLWU"]
pub mod llwu1;
#[doc = "TRGMUX"]
pub struct TRGMUX1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRGMUX1 {}
impl TRGMUX1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const trgmux1::RegisterBlock {
    0x4102_5000 as *const _
  }
}
impl Deref for TRGMUX1 {
  type Target = trgmux1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TRGMUX1::ptr() }
  }
}
#[doc = "TRGMUX"]
pub mod trgmux1;
#[doc = "WDOG"]
pub struct WDOG1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG1 {}
impl WDOG1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const wdog1::RegisterBlock {
    0x4102_6000 as *const _
  }
}
impl Deref for WDOG1 {
  type Target = wdog1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*WDOG1::ptr() }
  }
}
#[doc = "WDOG"]
pub mod wdog1;
#[doc = "PCC"]
pub struct PCC1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCC1 {}
impl PCC1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const pcc1::RegisterBlock {
    0x4102_7000 as *const _
  }
}
impl Deref for PCC1 {
  type Target = pcc1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PCC1::ptr() }
  }
}
#[doc = "PCC"]
pub mod pcc1;
#[doc = "CAU3"]
pub struct CAU3 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAU3 {}
impl CAU3 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const cau3::RegisterBlock {
    0x4102_8000 as *const _
  }
}
impl Deref for CAU3 {
  type Target = cau3::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*CAU3::ptr() }
  }
}
#[doc = "CAU3"]
pub mod cau3;
#[doc = "TRNG"]
pub struct TRNG {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const trng::RegisterBlock {
    0x4102_9000 as *const _
  }
}
impl Deref for TRNG {
  type Target = trng::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TRNG::ptr() }
  }
}
#[doc = "TRNG"]
pub mod trng;
#[doc = "LPIT"]
pub struct LPIT1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPIT1 {}
impl LPIT1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpit1::RegisterBlock {
    0x4102_a000 as *const _
  }
}
impl Deref for LPIT1 {
  type Target = lpit1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPIT1::ptr() }
  }
}
#[doc = "LPIT"]
pub mod lpit1;
#[doc = "LPTMR"]
pub struct LPTMR2 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR2 {}
impl LPTMR2 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lptmr2::RegisterBlock {
    0x4102_b000 as *const _
  }
}
impl Deref for LPTMR2 {
  type Target = lptmr2::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPTMR2::ptr() }
  }
}
#[doc = "LPTMR"]
pub mod lptmr2;
#[doc = "TPM"]
pub struct TPM3 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM3 {}
impl TPM3 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const tpm3::RegisterBlock {
    0x4102_d000 as *const _
  }
}
impl Deref for TPM3 {
  type Target = tpm3::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*TPM3::ptr() }
  }
}
#[doc = "TPM"]
pub mod tpm3;
#[doc = "LPI2C"]
pub struct LPI2C3 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C3 {}
impl LPI2C3 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpi2c3::RegisterBlock {
    0x4102_e000 as *const _
  }
}
impl Deref for LPI2C3 {
  type Target = lpi2c3::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPI2C3::ptr() }
  }
}
#[doc = "LPI2C"]
pub mod lpi2c3;
#[doc = "RSIM"]
pub struct RSIM {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSIM {}
impl RSIM {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const rsim::RegisterBlock {
    0x4102_f000 as *const _
  }
}
impl Deref for RSIM {
  type Target = rsim::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*RSIM::ptr() }
  }
}
#[doc = "RSIM"]
pub mod rsim;
#[doc = "LPSPI"]
pub struct LPSPI3 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI3 {}
impl LPSPI3 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpspi3::RegisterBlock {
    0x4103_5000 as *const _
  }
}
impl Deref for LPSPI3 {
  type Target = lpspi3::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPSPI3::ptr() }
  }
}
#[doc = "LPSPI"]
pub mod lpspi3;
#[doc = "LPUART"]
pub struct LPUART3 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART3 {}
impl LPUART3 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpuart3::RegisterBlock {
    0x4103_6000 as *const _
  }
}
impl Deref for LPUART3 {
  type Target = lpuart3::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPUART3::ptr() }
  }
}
#[doc = "LPUART"]
pub mod lpuart3;
#[doc = "PORT"]
pub struct PORTE {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const porte::RegisterBlock {
    0x4103_7000 as *const _
  }
}
impl Deref for PORTE {
  type Target = porte::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*PORTE::ptr() }
  }
}
#[doc = "PORT"]
pub mod porte;
#[doc = "LPCMP"]
pub struct LPCMP1 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCMP1 {}
impl LPCMP1 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const lpcmp1::RegisterBlock {
    0x4103_8000 as *const _
  }
}
impl Deref for LPCMP1 {
  type Target = lpcmp1::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*LPCMP1::ptr() }
  }
}
#[doc = "LPCMP"]
pub mod lpcmp1;
#[doc = "GPIO"]
pub struct GPIOA {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const gpioa::RegisterBlock {
    0x4802_0000 as *const _
  }
}
impl Deref for GPIOA {
  type Target = gpioa::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*GPIOA::ptr() }
  }
}
#[doc = "GPIO"]
pub mod gpioa;
#[doc = "GPIO"]
pub struct GPIOE {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const gpioa::RegisterBlock {
    0x4100_f000 as *const _
  }
}
impl Deref for GPIOE {
  type Target = gpioa::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*GPIOE::ptr() }
  }
}
#[doc = "GPIO"]
pub struct GPIOB {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const gpioa::RegisterBlock {
    0x4802_0040 as *const _
  }
}
impl Deref for GPIOB {
  type Target = gpioa::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*GPIOB::ptr() }
  }
}
#[doc = "GPIO"]
pub struct GPIOC {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const gpioa::RegisterBlock {
    0x4802_0080 as *const _
  }
}
impl Deref for GPIOC {
  type Target = gpioa::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*GPIOC::ptr() }
  }
}
#[doc = "GPIO"]
pub struct GPIOD {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const gpioa::RegisterBlock {
    0x4802_00c0 as *const _
  }
}
impl Deref for GPIOD {
  type Target = gpioa::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*GPIOD::ptr() }
  }
}
#[doc = "MCM"]
pub struct MCM0 {
  _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM0 {}
impl MCM0 {
  #[doc = r"Returns a pointer to the register block"]
  #[inline(always)]
  pub const fn ptr() -> *const mcm0::RegisterBlock {
    0xe008_0000 as *const _
  }
}
impl Deref for MCM0 {
  type Target = mcm0::RegisterBlock;
  fn deref(&self) -> &Self::Target {
    unsafe { &*MCM0::ptr() }
  }
}
#[doc = "MCM"]
pub mod mcm0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
  #[doc = "MSCM"]
  pub MSCM: MSCM,
  #[doc = "AXBS0"]
  pub AXBS0: AXBS0,
  #[doc = "DMA0"]
  pub DMA0: DMA0,
  #[doc = "FB"]
  pub FB: FB,
  #[doc = "XRDC"]
  pub XRDC: XRDC,
  #[doc = "SEMA420"]
  pub SEMA420: SEMA420,
  #[doc = "SMC0"]
  pub SMC0: SMC0,
  #[doc = "DMAMUX0"]
  pub DMAMUX0: DMAMUX0,
  #[doc = "EWM"]
  pub EWM: EWM,
  #[doc = "FTFE"]
  pub FTFE: FTFE,
  #[doc = "LLWU0"]
  pub LLWU0: LLWU0,
  #[doc = "MUA"]
  pub MUA: MUA,
  #[doc = "SIM"]
  pub SIM: SIM,
  #[doc = "USBVREG"]
  pub USBVREG: USBVREG,
  #[doc = "SPM"]
  pub SPM: SPM,
  #[doc = "TRGMUX0"]
  pub TRGMUX0: TRGMUX0,
  #[doc = "WDOG0"]
  pub WDOG0: WDOG0,
  #[doc = "PCC0"]
  pub PCC0: PCC0,
  #[doc = "SCG"]
  pub SCG: SCG,
  #[doc = "CRC"]
  pub CRC: CRC,
  #[doc = "LPIT0"]
  pub LPIT0: LPIT0,
  #[doc = "RTC"]
  pub RTC: RTC,
  #[doc = "LPTMR0"]
  pub LPTMR0: LPTMR0,
  #[doc = "LPTMR1"]
  pub LPTMR1: LPTMR1,
  #[doc = "TSTMRA"]
  pub TSTMRA: TSTMRA,
  #[doc = "TPM0"]
  pub TPM0: TPM0,
  #[doc = "TPM1"]
  pub TPM1: TPM1,
  #[doc = "TPM2"]
  pub TPM2: TPM2,
  #[doc = "EMVSIM0"]
  pub EMVSIM0: EMVSIM0,
  #[doc = "FLEXIO0"]
  pub FLEXIO0: FLEXIO0,
  #[doc = "LPI2C0"]
  pub LPI2C0: LPI2C0,
  #[doc = "LPI2C1"]
  pub LPI2C1: LPI2C1,
  #[doc = "LPI2C2"]
  pub LPI2C2: LPI2C2,
  #[doc = "I2S0"]
  pub I2S0: I2S0,
  #[doc = "USDHC0"]
  pub USDHC0: USDHC0,
  #[doc = "LPSPI0"]
  pub LPSPI0: LPSPI0,
  #[doc = "LPSPI1"]
  pub LPSPI1: LPSPI1,
  #[doc = "LPSPI2"]
  pub LPSPI2: LPSPI2,
  #[doc = "LPUART0"]
  pub LPUART0: LPUART0,
  #[doc = "LPUART1"]
  pub LPUART1: LPUART1,
  #[doc = "LPUART2"]
  pub LPUART2: LPUART2,
  #[doc = "USB0"]
  pub USB0: USB0,
  #[doc = "PORTA"]
  pub PORTA: PORTA,
  #[doc = "PORTB"]
  pub PORTB: PORTB,
  #[doc = "PORTC"]
  pub PORTC: PORTC,
  #[doc = "PORTD"]
  pub PORTD: PORTD,
  #[doc = "ADC0"]
  pub ADC0: ADC0,
  #[doc = "LPCMP0"]
  pub LPCMP0: LPCMP0,
  #[doc = "LPDAC0"]
  pub LPDAC0: LPDAC0,
  #[doc = "VREF"]
  pub VREF: VREF,
  #[doc = "SEMA421"]
  pub SEMA421: SEMA421,
  #[doc = "SMC1"]
  pub SMC1: SMC1,
  #[doc = "LLWU1"]
  pub LLWU1: LLWU1,
  #[doc = "TRGMUX1"]
  pub TRGMUX1: TRGMUX1,
  #[doc = "WDOG1"]
  pub WDOG1: WDOG1,
  #[doc = "PCC1"]
  pub PCC1: PCC1,
  #[doc = "CAU3"]
  pub CAU3: CAU3,
  #[doc = "TRNG"]
  pub TRNG: TRNG,
  #[doc = "LPIT1"]
  pub LPIT1: LPIT1,
  #[doc = "LPTMR2"]
  pub LPTMR2: LPTMR2,
  #[doc = "TPM3"]
  pub TPM3: TPM3,
  #[doc = "LPI2C3"]
  pub LPI2C3: LPI2C3,
  #[doc = "RSIM"]
  pub RSIM: RSIM,
  #[doc = "LPSPI3"]
  pub LPSPI3: LPSPI3,
  #[doc = "LPUART3"]
  pub LPUART3: LPUART3,
  #[doc = "PORTE"]
  pub PORTE: PORTE,
  #[doc = "LPCMP1"]
  pub LPCMP1: LPCMP1,
  #[doc = "GPIOA"]
  pub GPIOA: GPIOA,
  #[doc = "GPIOE"]
  pub GPIOE: GPIOE,
  #[doc = "GPIOB"]
  pub GPIOB: GPIOB,
  #[doc = "GPIOC"]
  pub GPIOC: GPIOC,
  #[doc = "GPIOD"]
  pub GPIOD: GPIOD,
  #[doc = "MCM0"]
  pub MCM0: MCM0,
}
impl Peripherals {
  #[doc = r"Returns all the peripherals *once*"]
  #[inline]
  pub fn take() -> Option<Self> {
    riscv::interrupt::free(|_| {
      if unsafe { DEVICE_PERIPHERALS } {
        None
      } else {
        Some(unsafe { Peripherals::steal() })
      }
    })
  }
  #[doc = r"Unchecked version of `Peripherals::take`"]
  pub unsafe fn steal() -> Self {
    DEVICE_PERIPHERALS = true;
    Peripherals {
      MSCM: MSCM {
        _marker: PhantomData,
      },
      AXBS0: AXBS0 {
        _marker: PhantomData,
      },
      DMA0: DMA0 {
        _marker: PhantomData,
      },
      FB: FB {
        _marker: PhantomData,
      },
      XRDC: XRDC {
        _marker: PhantomData,
      },
      SEMA420: SEMA420 {
        _marker: PhantomData,
      },
      SMC0: SMC0 {
        _marker: PhantomData,
      },
      DMAMUX0: DMAMUX0 {
        _marker: PhantomData,
      },
      EWM: EWM {
        _marker: PhantomData,
      },
      FTFE: FTFE {
        _marker: PhantomData,
      },
      LLWU0: LLWU0 {
        _marker: PhantomData,
      },
      MUA: MUA {
        _marker: PhantomData,
      },
      SIM: SIM {
        _marker: PhantomData,
      },
      USBVREG: USBVREG {
        _marker: PhantomData,
      },
      SPM: SPM {
        _marker: PhantomData,
      },
      TRGMUX0: TRGMUX0 {
        _marker: PhantomData,
      },
      WDOG0: WDOG0 {
        _marker: PhantomData,
      },
      PCC0: PCC0 {
        _marker: PhantomData,
      },
      SCG: SCG {
        _marker: PhantomData,
      },
      CRC: CRC {
        _marker: PhantomData,
      },
      LPIT0: LPIT0 {
        _marker: PhantomData,
      },
      RTC: RTC {
        _marker: PhantomData,
      },
      LPTMR0: LPTMR0 {
        _marker: PhantomData,
      },
      LPTMR1: LPTMR1 {
        _marker: PhantomData,
      },
      TSTMRA: TSTMRA {
        _marker: PhantomData,
      },
      TPM0: TPM0 {
        _marker: PhantomData,
      },
      TPM1: TPM1 {
        _marker: PhantomData,
      },
      TPM2: TPM2 {
        _marker: PhantomData,
      },
      EMVSIM0: EMVSIM0 {
        _marker: PhantomData,
      },
      FLEXIO0: FLEXIO0 {
        _marker: PhantomData,
      },
      LPI2C0: LPI2C0 {
        _marker: PhantomData,
      },
      LPI2C1: LPI2C1 {
        _marker: PhantomData,
      },
      LPI2C2: LPI2C2 {
        _marker: PhantomData,
      },
      I2S0: I2S0 {
        _marker: PhantomData,
      },
      USDHC0: USDHC0 {
        _marker: PhantomData,
      },
      LPSPI0: LPSPI0 {
        _marker: PhantomData,
      },
      LPSPI1: LPSPI1 {
        _marker: PhantomData,
      },
      LPSPI2: LPSPI2 {
        _marker: PhantomData,
      },
      LPUART0: LPUART0 {
        _marker: PhantomData,
      },
      LPUART1: LPUART1 {
        _marker: PhantomData,
      },
      LPUART2: LPUART2 {
        _marker: PhantomData,
      },
      USB0: USB0 {
        _marker: PhantomData,
      },
      PORTA: PORTA {
        _marker: PhantomData,
      },
      PORTB: PORTB {
        _marker: PhantomData,
      },
      PORTC: PORTC {
        _marker: PhantomData,
      },
      PORTD: PORTD {
        _marker: PhantomData,
      },
      ADC0: ADC0 {
        _marker: PhantomData,
      },
      LPCMP0: LPCMP0 {
        _marker: PhantomData,
      },
      LPDAC0: LPDAC0 {
        _marker: PhantomData,
      },
      VREF: VREF {
        _marker: PhantomData,
      },
      SEMA421: SEMA421 {
        _marker: PhantomData,
      },
      SMC1: SMC1 {
        _marker: PhantomData,
      },
      LLWU1: LLWU1 {
        _marker: PhantomData,
      },
      TRGMUX1: TRGMUX1 {
        _marker: PhantomData,
      },
      WDOG1: WDOG1 {
        _marker: PhantomData,
      },
      PCC1: PCC1 {
        _marker: PhantomData,
      },
      CAU3: CAU3 {
        _marker: PhantomData,
      },
      TRNG: TRNG {
        _marker: PhantomData,
      },
      LPIT1: LPIT1 {
        _marker: PhantomData,
      },
      LPTMR2: LPTMR2 {
        _marker: PhantomData,
      },
      TPM3: TPM3 {
        _marker: PhantomData,
      },
      LPI2C3: LPI2C3 {
        _marker: PhantomData,
      },
      RSIM: RSIM {
        _marker: PhantomData,
      },
      LPSPI3: LPSPI3 {
        _marker: PhantomData,
      },
      LPUART3: LPUART3 {
        _marker: PhantomData,
      },
      PORTE: PORTE {
        _marker: PhantomData,
      },
      LPCMP1: LPCMP1 {
        _marker: PhantomData,
      },
      GPIOA: GPIOA {
        _marker: PhantomData,
      },
      GPIOE: GPIOE {
        _marker: PhantomData,
      },
      GPIOB: GPIOB {
        _marker: PhantomData,
      },
      GPIOC: GPIOC {
        _marker: PhantomData,
      },
      GPIOD: GPIOD {
        _marker: PhantomData,
      },
      MCM0: MCM0 {
        _marker: PhantomData,
      },
    }
  }
}
