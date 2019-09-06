#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_A {
  #[doc = "0: No effect."]
  SWR_0,
  #[doc = "1: no description available"]
  SWR_1,
}
impl From<SWR_A> for bool {
  #[inline(always)]
  fn from(variant: SWR_A) -> Self {
    match variant {
      SWR_A::SWR_0 => false,
      SWR_A::SWR_1 => true,
    }
  }
}
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, SWR_A>;
impl SWR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWR_A {
    match self.bits {
      false => SWR_A::SWR_0,
      true => SWR_A::SWR_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWR_0`"]
  #[inline(always)]
  pub fn is_swr_0(&self) -> bool {
    *self == SWR_A::SWR_0
  }
  #[doc = "Checks if the value of the field is `SWR_1`"]
  #[inline(always)]
  pub fn is_swr_1(&self) -> bool {
    *self == SWR_A::SWR_1
  }
}
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
  w: &'a mut W,
}
impl<'a> SWR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn swr_0(self) -> &'a mut W {
    self.variant(SWR_A::SWR_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn swr_1(self) -> &'a mut W {
    self.variant(SWR_A::SWR_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Wakeup Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPE_A {
  #[doc = "0: RTC_WAKEUP pin is disabled."]
  WPE_0,
  #[doc = "1: RTC_WAKEUP pin is enabled and asserts if the RTC interrupt asserts or if the wakeup pin is forced on."]
  WPE_1,
}
impl From<WPE_A> for bool {
  #[inline(always)]
  fn from(variant: WPE_A) -> Self {
    match variant {
      WPE_A::WPE_0 => false,
      WPE_A::WPE_1 => true,
    }
  }
}
#[doc = "Reader of field `WPE`"]
pub type WPE_R = crate::R<bool, WPE_A>;
impl WPE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WPE_A {
    match self.bits {
      false => WPE_A::WPE_0,
      true => WPE_A::WPE_1,
    }
  }
  #[doc = "Checks if the value of the field is `WPE_0`"]
  #[inline(always)]
  pub fn is_wpe_0(&self) -> bool {
    *self == WPE_A::WPE_0
  }
  #[doc = "Checks if the value of the field is `WPE_1`"]
  #[inline(always)]
  pub fn is_wpe_1(&self) -> bool {
    *self == WPE_A::WPE_1
  }
}
#[doc = "Write proxy for field `WPE`"]
pub struct WPE_W<'a> {
  w: &'a mut W,
}
impl<'a> WPE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WPE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC_WAKEUP pin is disabled."]
  #[inline(always)]
  pub fn wpe_0(self) -> &'a mut W {
    self.variant(WPE_A::WPE_0)
  }
  #[doc = "RTC_WAKEUP pin is enabled and asserts if the RTC interrupt asserts or if the wakeup pin is forced on."]
  #[inline(always)]
  pub fn wpe_1(self) -> &'a mut W {
    self.variant(WPE_A::WPE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
#[doc = "Supervisor Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUP_A {
  #[doc = "0: Non-supervisor mode write accesses are not supported and generate a bus error."]
  SUP_0,
  #[doc = "1: Non-supervisor mode write accesses are supported."]
  SUP_1,
}
impl From<SUP_A> for bool {
  #[inline(always)]
  fn from(variant: SUP_A) -> Self {
    match variant {
      SUP_A::SUP_0 => false,
      SUP_A::SUP_1 => true,
    }
  }
}
#[doc = "Reader of field `SUP`"]
pub type SUP_R = crate::R<bool, SUP_A>;
impl SUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SUP_A {
    match self.bits {
      false => SUP_A::SUP_0,
      true => SUP_A::SUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `SUP_0`"]
  #[inline(always)]
  pub fn is_sup_0(&self) -> bool {
    *self == SUP_A::SUP_0
  }
  #[doc = "Checks if the value of the field is `SUP_1`"]
  #[inline(always)]
  pub fn is_sup_1(&self) -> bool {
    *self == SUP_A::SUP_1
  }
}
#[doc = "Write proxy for field `SUP`"]
pub struct SUP_W<'a> {
  w: &'a mut W,
}
impl<'a> SUP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SUP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
  #[inline(always)]
  pub fn sup_0(self) -> &'a mut W {
    self.variant(SUP_A::SUP_0)
  }
  #[doc = "Non-supervisor mode write accesses are supported."]
  #[inline(always)]
  pub fn sup_1(self) -> &'a mut W {
    self.variant(SUP_A::SUP_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UM_A {
  #[doc = "0: Registers cannot be written when locked."]
  UM_0,
  #[doc = "1: Registers can be written when locked under limited conditions."]
  UM_1,
}
impl From<UM_A> for bool {
  #[inline(always)]
  fn from(variant: UM_A) -> Self {
    match variant {
      UM_A::UM_0 => false,
      UM_A::UM_1 => true,
    }
  }
}
#[doc = "Reader of field `UM`"]
pub type UM_R = crate::R<bool, UM_A>;
impl UM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> UM_A {
    match self.bits {
      false => UM_A::UM_0,
      true => UM_A::UM_1,
    }
  }
  #[doc = "Checks if the value of the field is `UM_0`"]
  #[inline(always)]
  pub fn is_um_0(&self) -> bool {
    *self == UM_A::UM_0
  }
  #[doc = "Checks if the value of the field is `UM_1`"]
  #[inline(always)]
  pub fn is_um_1(&self) -> bool {
    *self == UM_A::UM_1
  }
}
#[doc = "Write proxy for field `UM`"]
pub struct UM_W<'a> {
  w: &'a mut W,
}
impl<'a> UM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: UM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Registers cannot be written when locked."]
  #[inline(always)]
  pub fn um_0(self) -> &'a mut W {
    self.variant(UM_A::UM_0)
  }
  #[doc = "Registers can be written when locked under limited conditions."]
  #[inline(always)]
  pub fn um_1(self) -> &'a mut W {
    self.variant(UM_A::UM_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "Wakeup Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPS_A {
  #[doc = "0: RTC_WAKEUP pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
  WPS_0,
  #[doc = "1: RTC_WAKEUP pin outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
  WPS_1,
}
impl From<WPS_A> for bool {
  #[inline(always)]
  fn from(variant: WPS_A) -> Self {
    match variant {
      WPS_A::WPS_0 => false,
      WPS_A::WPS_1 => true,
    }
  }
}
#[doc = "Reader of field `WPS`"]
pub type WPS_R = crate::R<bool, WPS_A>;
impl WPS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WPS_A {
    match self.bits {
      false => WPS_A::WPS_0,
      true => WPS_A::WPS_1,
    }
  }
  #[doc = "Checks if the value of the field is `WPS_0`"]
  #[inline(always)]
  pub fn is_wps_0(&self) -> bool {
    *self == WPS_A::WPS_0
  }
  #[doc = "Checks if the value of the field is `WPS_1`"]
  #[inline(always)]
  pub fn is_wps_1(&self) -> bool {
    *self == WPS_A::WPS_1
  }
}
#[doc = "Write proxy for field `WPS`"]
pub struct WPS_W<'a> {
  w: &'a mut W,
}
impl<'a> WPS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WPS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC_WAKEUP pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
  #[inline(always)]
  pub fn wps_0(self) -> &'a mut W {
    self.variant(WPS_A::WPS_0)
  }
  #[doc = "RTC_WAKEUP pin outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
  #[inline(always)]
  pub fn wps_1(self) -> &'a mut W {
    self.variant(WPS_A::WPS_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPS_A {
  #[doc = "0: The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
  CPS_0,
  #[doc = "1: The RTC 32.768 kHz clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
  CPS_1,
}
impl From<CPS_A> for bool {
  #[inline(always)]
  fn from(variant: CPS_A) -> Self {
    match variant {
      CPS_A::CPS_0 => false,
      CPS_A::CPS_1 => true,
    }
  }
}
#[doc = "Reader of field `CPS`"]
pub type CPS_R = crate::R<bool, CPS_A>;
impl CPS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CPS_A {
    match self.bits {
      false => CPS_A::CPS_0,
      true => CPS_A::CPS_1,
    }
  }
  #[doc = "Checks if the value of the field is `CPS_0`"]
  #[inline(always)]
  pub fn is_cps_0(&self) -> bool {
    *self == CPS_A::CPS_0
  }
  #[doc = "Checks if the value of the field is `CPS_1`"]
  #[inline(always)]
  pub fn is_cps_1(&self) -> bool {
    *self == CPS_A::CPS_1
  }
}
#[doc = "Write proxy for field `CPS`"]
pub struct CPS_W<'a> {
  w: &'a mut W,
}
impl<'a> CPS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CPS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
  #[inline(always)]
  pub fn cps_0(self) -> &'a mut W {
    self.variant(CPS_A::CPS_0)
  }
  #[doc = "The RTC 32.768 kHz clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
  #[inline(always)]
  pub fn cps_1(self) -> &'a mut W {
    self.variant(CPS_A::CPS_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "LPO Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOS_A {
  #[doc = "0: RTC prescaler increments using 32.768 kHz clock."]
  LPOS_0,
  #[doc = "1: RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\] of the prescaler are ignored."]
  LPOS_1,
}
impl From<LPOS_A> for bool {
  #[inline(always)]
  fn from(variant: LPOS_A) -> Self {
    match variant {
      LPOS_A::LPOS_0 => false,
      LPOS_A::LPOS_1 => true,
    }
  }
}
#[doc = "Reader of field `LPOS`"]
pub type LPOS_R = crate::R<bool, LPOS_A>;
impl LPOS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPOS_A {
    match self.bits {
      false => LPOS_A::LPOS_0,
      true => LPOS_A::LPOS_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPOS_0`"]
  #[inline(always)]
  pub fn is_lpos_0(&self) -> bool {
    *self == LPOS_A::LPOS_0
  }
  #[doc = "Checks if the value of the field is `LPOS_1`"]
  #[inline(always)]
  pub fn is_lpos_1(&self) -> bool {
    *self == LPOS_A::LPOS_1
  }
}
#[doc = "Write proxy for field `LPOS`"]
pub struct LPOS_W<'a> {
  w: &'a mut W,
}
impl<'a> LPOS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPOS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC prescaler increments using 32.768 kHz clock."]
  #[inline(always)]
  pub fn lpos_0(self) -> &'a mut W {
    self.variant(LPOS_A::LPOS_0)
  }
  #[doc = "RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\] of the prescaler are ignored."]
  #[inline(always)]
  pub fn lpos_1(self) -> &'a mut W {
    self.variant(LPOS_A::LPOS_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
#[doc = "Oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCE_A {
  #[doc = "0: 32.768 kHz oscillator is disabled."]
  OSCE_0,
  #[doc = "1: 32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
  OSCE_1,
}
impl From<OSCE_A> for bool {
  #[inline(always)]
  fn from(variant: OSCE_A) -> Self {
    match variant {
      OSCE_A::OSCE_0 => false,
      OSCE_A::OSCE_1 => true,
    }
  }
}
#[doc = "Reader of field `OSCE`"]
pub type OSCE_R = crate::R<bool, OSCE_A>;
impl OSCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OSCE_A {
    match self.bits {
      false => OSCE_A::OSCE_0,
      true => OSCE_A::OSCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `OSCE_0`"]
  #[inline(always)]
  pub fn is_osce_0(&self) -> bool {
    *self == OSCE_A::OSCE_0
  }
  #[doc = "Checks if the value of the field is `OSCE_1`"]
  #[inline(always)]
  pub fn is_osce_1(&self) -> bool {
    *self == OSCE_A::OSCE_1
  }
}
#[doc = "Write proxy for field `OSCE`"]
pub struct OSCE_W<'a> {
  w: &'a mut W,
}
impl<'a> OSCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OSCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "32.768 kHz oscillator is disabled."]
  #[inline(always)]
  pub fn osce_0(self) -> &'a mut W {
    self.variant(OSCE_A::OSCE_0)
  }
  #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
  #[inline(always)]
  pub fn osce_1(self) -> &'a mut W {
    self.variant(OSCE_A::OSCE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
    self.w
  }
}
#[doc = "Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO_A {
  #[doc = "0: The 32 kHz clock is output to other peripherals."]
  CLKO_0,
  #[doc = "1: The 32 kHz clock is not output to other peripherals."]
  CLKO_1,
}
impl From<CLKO_A> for bool {
  #[inline(always)]
  fn from(variant: CLKO_A) -> Self {
    match variant {
      CLKO_A::CLKO_0 => false,
      CLKO_A::CLKO_1 => true,
    }
  }
}
#[doc = "Reader of field `CLKO`"]
pub type CLKO_R = crate::R<bool, CLKO_A>;
impl CLKO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CLKO_A {
    match self.bits {
      false => CLKO_A::CLKO_0,
      true => CLKO_A::CLKO_1,
    }
  }
  #[doc = "Checks if the value of the field is `CLKO_0`"]
  #[inline(always)]
  pub fn is_clko_0(&self) -> bool {
    *self == CLKO_A::CLKO_0
  }
  #[doc = "Checks if the value of the field is `CLKO_1`"]
  #[inline(always)]
  pub fn is_clko_1(&self) -> bool {
    *self == CLKO_A::CLKO_1
  }
}
#[doc = "Write proxy for field `CLKO`"]
pub struct CLKO_W<'a> {
  w: &'a mut W,
}
impl<'a> CLKO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CLKO_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The 32 kHz clock is output to other peripherals."]
  #[inline(always)]
  pub fn clko_0(self) -> &'a mut W {
    self.variant(CLKO_A::CLKO_0)
  }
  #[doc = "The 32 kHz clock is not output to other peripherals."]
  #[inline(always)]
  pub fn clko_1(self) -> &'a mut W {
    self.variant(CLKO_A::CLKO_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
#[doc = "Oscillator 16pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC16P_A {
  #[doc = "0: Disable the load."]
  SC16P_0,
  #[doc = "1: Enable the additional load."]
  SC16P_1,
}
impl From<SC16P_A> for bool {
  #[inline(always)]
  fn from(variant: SC16P_A) -> Self {
    match variant {
      SC16P_A::SC16P_0 => false,
      SC16P_A::SC16P_1 => true,
    }
  }
}
#[doc = "Reader of field `SC16P`"]
pub type SC16P_R = crate::R<bool, SC16P_A>;
impl SC16P_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SC16P_A {
    match self.bits {
      false => SC16P_A::SC16P_0,
      true => SC16P_A::SC16P_1,
    }
  }
  #[doc = "Checks if the value of the field is `SC16P_0`"]
  #[inline(always)]
  pub fn is_sc16p_0(&self) -> bool {
    *self == SC16P_A::SC16P_0
  }
  #[doc = "Checks if the value of the field is `SC16P_1`"]
  #[inline(always)]
  pub fn is_sc16p_1(&self) -> bool {
    *self == SC16P_A::SC16P_1
  }
}
#[doc = "Write proxy for field `SC16P`"]
pub struct SC16P_W<'a> {
  w: &'a mut W,
}
impl<'a> SC16P_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SC16P_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable the load."]
  #[inline(always)]
  pub fn sc16p_0(self) -> &'a mut W {
    self.variant(SC16P_A::SC16P_0)
  }
  #[doc = "Enable the additional load."]
  #[inline(always)]
  pub fn sc16p_1(self) -> &'a mut W {
    self.variant(SC16P_A::SC16P_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
    self.w
  }
}
#[doc = "Oscillator 8pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC8P_A {
  #[doc = "0: Disable the load."]
  SC8P_0,
  #[doc = "1: Enable the additional load."]
  SC8P_1,
}
impl From<SC8P_A> for bool {
  #[inline(always)]
  fn from(variant: SC8P_A) -> Self {
    match variant {
      SC8P_A::SC8P_0 => false,
      SC8P_A::SC8P_1 => true,
    }
  }
}
#[doc = "Reader of field `SC8P`"]
pub type SC8P_R = crate::R<bool, SC8P_A>;
impl SC8P_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SC8P_A {
    match self.bits {
      false => SC8P_A::SC8P_0,
      true => SC8P_A::SC8P_1,
    }
  }
  #[doc = "Checks if the value of the field is `SC8P_0`"]
  #[inline(always)]
  pub fn is_sc8p_0(&self) -> bool {
    *self == SC8P_A::SC8P_0
  }
  #[doc = "Checks if the value of the field is `SC8P_1`"]
  #[inline(always)]
  pub fn is_sc8p_1(&self) -> bool {
    *self == SC8P_A::SC8P_1
  }
}
#[doc = "Write proxy for field `SC8P`"]
pub struct SC8P_W<'a> {
  w: &'a mut W,
}
impl<'a> SC8P_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SC8P_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable the load."]
  #[inline(always)]
  pub fn sc8p_0(self) -> &'a mut W {
    self.variant(SC8P_A::SC8P_0)
  }
  #[doc = "Enable the additional load."]
  #[inline(always)]
  pub fn sc8p_1(self) -> &'a mut W {
    self.variant(SC8P_A::SC8P_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
    self.w
  }
}
#[doc = "Oscillator 4pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC4P_A {
  #[doc = "0: Disable the load."]
  SC4P_0,
  #[doc = "1: Enable the additional load."]
  SC4P_1,
}
impl From<SC4P_A> for bool {
  #[inline(always)]
  fn from(variant: SC4P_A) -> Self {
    match variant {
      SC4P_A::SC4P_0 => false,
      SC4P_A::SC4P_1 => true,
    }
  }
}
#[doc = "Reader of field `SC4P`"]
pub type SC4P_R = crate::R<bool, SC4P_A>;
impl SC4P_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SC4P_A {
    match self.bits {
      false => SC4P_A::SC4P_0,
      true => SC4P_A::SC4P_1,
    }
  }
  #[doc = "Checks if the value of the field is `SC4P_0`"]
  #[inline(always)]
  pub fn is_sc4p_0(&self) -> bool {
    *self == SC4P_A::SC4P_0
  }
  #[doc = "Checks if the value of the field is `SC4P_1`"]
  #[inline(always)]
  pub fn is_sc4p_1(&self) -> bool {
    *self == SC4P_A::SC4P_1
  }
}
#[doc = "Write proxy for field `SC4P`"]
pub struct SC4P_W<'a> {
  w: &'a mut W,
}
impl<'a> SC4P_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SC4P_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable the load."]
  #[inline(always)]
  pub fn sc4p_0(self) -> &'a mut W {
    self.variant(SC4P_A::SC4P_0)
  }
  #[doc = "Enable the additional load."]
  #[inline(always)]
  pub fn sc4p_1(self) -> &'a mut W {
    self.variant(SC4P_A::SC4P_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "Oscillator 2pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC2P_A {
  #[doc = "0: Disable the load."]
  SC2P_0,
  #[doc = "1: Enable the additional load."]
  SC2P_1,
}
impl From<SC2P_A> for bool {
  #[inline(always)]
  fn from(variant: SC2P_A) -> Self {
    match variant {
      SC2P_A::SC2P_0 => false,
      SC2P_A::SC2P_1 => true,
    }
  }
}
#[doc = "Reader of field `SC2P`"]
pub type SC2P_R = crate::R<bool, SC2P_A>;
impl SC2P_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SC2P_A {
    match self.bits {
      false => SC2P_A::SC2P_0,
      true => SC2P_A::SC2P_1,
    }
  }
  #[doc = "Checks if the value of the field is `SC2P_0`"]
  #[inline(always)]
  pub fn is_sc2p_0(&self) -> bool {
    *self == SC2P_A::SC2P_0
  }
  #[doc = "Checks if the value of the field is `SC2P_1`"]
  #[inline(always)]
  pub fn is_sc2p_1(&self) -> bool {
    *self == SC2P_A::SC2P_1
  }
}
#[doc = "Write proxy for field `SC2P`"]
pub struct SC2P_W<'a> {
  w: &'a mut W,
}
impl<'a> SC2P_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SC2P_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable the load."]
  #[inline(always)]
  pub fn sc2p_0(self) -> &'a mut W {
    self.variant(SC2P_A::SC2P_0)
  }
  #[doc = "Enable the additional load."]
  #[inline(always)]
  pub fn sc2p_1(self) -> &'a mut W {
    self.variant(SC2P_A::SC2P_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
    self.w
  }
}
#[doc = "Oscillator Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCM_A {
  #[doc = "0: Configures the 32.768kHz crystal oscillator for robust operation supporting a wide range of crystals."]
  OSCM_0,
  #[doc = "1: Configures the 32.768kHz crystal oscillator for low power operation supporting a more limited range of crystals."]
  OSCM_1,
}
impl From<OSCM_A> for bool {
  #[inline(always)]
  fn from(variant: OSCM_A) -> Self {
    match variant {
      OSCM_A::OSCM_0 => false,
      OSCM_A::OSCM_1 => true,
    }
  }
}
#[doc = "Reader of field `OSCM`"]
pub type OSCM_R = crate::R<bool, OSCM_A>;
impl OSCM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OSCM_A {
    match self.bits {
      false => OSCM_A::OSCM_0,
      true => OSCM_A::OSCM_1,
    }
  }
  #[doc = "Checks if the value of the field is `OSCM_0`"]
  #[inline(always)]
  pub fn is_oscm_0(&self) -> bool {
    *self == OSCM_A::OSCM_0
  }
  #[doc = "Checks if the value of the field is `OSCM_1`"]
  #[inline(always)]
  pub fn is_oscm_1(&self) -> bool {
    *self == OSCM_A::OSCM_1
  }
}
#[doc = "Write proxy for field `OSCM`"]
pub struct OSCM_W<'a> {
  w: &'a mut W,
}
impl<'a> OSCM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OSCM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Configures the 32.768kHz crystal oscillator for robust operation supporting a wide range of crystals."]
  #[inline(always)]
  pub fn oscm_0(self) -> &'a mut W {
    self.variant(OSCM_A::OSCM_0)
  }
  #[doc = "Configures the 32.768kHz crystal oscillator for low power operation supporting a more limited range of crystals."]
  #[inline(always)]
  pub fn oscm_1(self) -> &'a mut W {
    self.variant(OSCM_A::OSCM_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
#[doc = "POR Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORS_A {
  #[doc = "0: POR brownout enabled for 120us every 128ms."]
  PORS_0,
  #[doc = "1: POR brownout enabled for 120us every 64ms."]
  PORS_1,
  #[doc = "2: POR brownout enabled for 120us every 32ms."]
  PORS_2,
  #[doc = "3: POR brownout always enabled."]
  PORS_3,
}
impl From<PORS_A> for u8 {
  #[inline(always)]
  fn from(variant: PORS_A) -> Self {
    match variant {
      PORS_A::PORS_0 => 0,
      PORS_A::PORS_1 => 1,
      PORS_A::PORS_2 => 2,
      PORS_A::PORS_3 => 3,
    }
  }
}
#[doc = "Reader of field `PORS`"]
pub type PORS_R = crate::R<u8, PORS_A>;
impl PORS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PORS_A {
    match self.bits {
      0 => PORS_A::PORS_0,
      1 => PORS_A::PORS_1,
      2 => PORS_A::PORS_2,
      3 => PORS_A::PORS_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PORS_0`"]
  #[inline(always)]
  pub fn is_pors_0(&self) -> bool {
    *self == PORS_A::PORS_0
  }
  #[doc = "Checks if the value of the field is `PORS_1`"]
  #[inline(always)]
  pub fn is_pors_1(&self) -> bool {
    *self == PORS_A::PORS_1
  }
  #[doc = "Checks if the value of the field is `PORS_2`"]
  #[inline(always)]
  pub fn is_pors_2(&self) -> bool {
    *self == PORS_A::PORS_2
  }
  #[doc = "Checks if the value of the field is `PORS_3`"]
  #[inline(always)]
  pub fn is_pors_3(&self) -> bool {
    *self == PORS_A::PORS_3
  }
}
#[doc = "Write proxy for field `PORS`"]
pub struct PORS_W<'a> {
  w: &'a mut W,
}
impl<'a> PORS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PORS_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "POR brownout enabled for 120us every 128ms."]
  #[inline(always)]
  pub fn pors_0(self) -> &'a mut W {
    self.variant(PORS_A::PORS_0)
  }
  #[doc = "POR brownout enabled for 120us every 64ms."]
  #[inline(always)]
  pub fn pors_1(self) -> &'a mut W {
    self.variant(PORS_A::PORS_1)
  }
  #[doc = "POR brownout enabled for 120us every 32ms."]
  #[inline(always)]
  pub fn pors_2(self) -> &'a mut W {
    self.variant(PORS_A::PORS_2)
  }
  #[doc = "POR brownout always enabled."]
  #[inline(always)]
  pub fn pors_3(self) -> &'a mut W {
    self.variant(PORS_A::PORS_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "Clock Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPE_A {
  #[doc = "0: The RTC_CLKOUT function is disabled."]
  CPE_0,
  #[doc = "1: Enable RTC_CLKOUT pin on pin 1."]
  CPE_1,
  #[doc = "2: Enable RTC_CLKOUT pin on pin 2."]
  CPE_2,
  #[doc = "3: Enable RTC_CLKOUT pin on pin 3."]
  CPE_3,
}
impl From<CPE_A> for u8 {
  #[inline(always)]
  fn from(variant: CPE_A) -> Self {
    match variant {
      CPE_A::CPE_0 => 0,
      CPE_A::CPE_1 => 1,
      CPE_A::CPE_2 => 2,
      CPE_A::CPE_3 => 3,
    }
  }
}
#[doc = "Reader of field `CPE`"]
pub type CPE_R = crate::R<u8, CPE_A>;
impl CPE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CPE_A {
    match self.bits {
      0 => CPE_A::CPE_0,
      1 => CPE_A::CPE_1,
      2 => CPE_A::CPE_2,
      3 => CPE_A::CPE_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `CPE_0`"]
  #[inline(always)]
  pub fn is_cpe_0(&self) -> bool {
    *self == CPE_A::CPE_0
  }
  #[doc = "Checks if the value of the field is `CPE_1`"]
  #[inline(always)]
  pub fn is_cpe_1(&self) -> bool {
    *self == CPE_A::CPE_1
  }
  #[doc = "Checks if the value of the field is `CPE_2`"]
  #[inline(always)]
  pub fn is_cpe_2(&self) -> bool {
    *self == CPE_A::CPE_2
  }
  #[doc = "Checks if the value of the field is `CPE_3`"]
  #[inline(always)]
  pub fn is_cpe_3(&self) -> bool {
    *self == CPE_A::CPE_3
  }
}
#[doc = "Write proxy for field `CPE`"]
pub struct CPE_W<'a> {
  w: &'a mut W,
}
impl<'a> CPE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CPE_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "The RTC_CLKOUT function is disabled."]
  #[inline(always)]
  pub fn cpe_0(self) -> &'a mut W {
    self.variant(CPE_A::CPE_0)
  }
  #[doc = "Enable RTC_CLKOUT pin on pin 1."]
  #[inline(always)]
  pub fn cpe_1(self) -> &'a mut W {
    self.variant(CPE_A::CPE_1)
  }
  #[doc = "Enable RTC_CLKOUT pin on pin 2."]
  #[inline(always)]
  pub fn cpe_2(self) -> &'a mut W {
    self.variant(CPE_A::CPE_2)
  }
  #[doc = "Enable RTC_CLKOUT pin on pin 3."]
  #[inline(always)]
  pub fn cpe_3(self) -> &'a mut W {
    self.variant(CPE_A::CPE_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Software Reset"]
  #[inline(always)]
  pub fn swr(&self) -> SWR_R {
    SWR_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Wakeup Pin Enable"]
  #[inline(always)]
  pub fn wpe(&self) -> WPE_R {
    WPE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Supervisor Access"]
  #[inline(always)]
  pub fn sup(&self) -> SUP_R {
    SUP_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Update Mode"]
  #[inline(always)]
  pub fn um(&self) -> UM_R {
    UM_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Wakeup Pin Select"]
  #[inline(always)]
  pub fn wps(&self) -> WPS_R {
    WPS_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Clock Pin Select"]
  #[inline(always)]
  pub fn cps(&self) -> CPS_R {
    CPS_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 7 - LPO Select"]
  #[inline(always)]
  pub fn lpos(&self) -> LPOS_R {
    LPOS_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Oscillator Enable"]
  #[inline(always)]
  pub fn osce(&self) -> OSCE_R {
    OSCE_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Clock Output"]
  #[inline(always)]
  pub fn clko(&self) -> CLKO_R {
    CLKO_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
  #[inline(always)]
  pub fn sc16p(&self) -> SC16P_R {
    SC16P_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
  #[inline(always)]
  pub fn sc8p(&self) -> SC8P_R {
    SC8P_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
  #[inline(always)]
  pub fn sc4p(&self) -> SC4P_R {
    SC4P_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
  #[inline(always)]
  pub fn sc2p(&self) -> SC2P_R {
    SC2P_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Oscillator Mode Select"]
  #[inline(always)]
  pub fn oscm(&self) -> OSCM_R {
    OSCM_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bits 16:17 - POR Select"]
  #[inline(always)]
  pub fn pors(&self) -> PORS_R {
    PORS_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 24:25 - Clock Pin Enable"]
  #[inline(always)]
  pub fn cpe(&self) -> CPE_R {
    CPE_R::new(((self.bits >> 24) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Software Reset"]
  #[inline(always)]
  pub fn swr(&mut self) -> SWR_W {
    SWR_W { w: self }
  }
  #[doc = "Bit 1 - Wakeup Pin Enable"]
  #[inline(always)]
  pub fn wpe(&mut self) -> WPE_W {
    WPE_W { w: self }
  }
  #[doc = "Bit 2 - Supervisor Access"]
  #[inline(always)]
  pub fn sup(&mut self) -> SUP_W {
    SUP_W { w: self }
  }
  #[doc = "Bit 3 - Update Mode"]
  #[inline(always)]
  pub fn um(&mut self) -> UM_W {
    UM_W { w: self }
  }
  #[doc = "Bit 4 - Wakeup Pin Select"]
  #[inline(always)]
  pub fn wps(&mut self) -> WPS_W {
    WPS_W { w: self }
  }
  #[doc = "Bit 5 - Clock Pin Select"]
  #[inline(always)]
  pub fn cps(&mut self) -> CPS_W {
    CPS_W { w: self }
  }
  #[doc = "Bit 7 - LPO Select"]
  #[inline(always)]
  pub fn lpos(&mut self) -> LPOS_W {
    LPOS_W { w: self }
  }
  #[doc = "Bit 8 - Oscillator Enable"]
  #[inline(always)]
  pub fn osce(&mut self) -> OSCE_W {
    OSCE_W { w: self }
  }
  #[doc = "Bit 9 - Clock Output"]
  #[inline(always)]
  pub fn clko(&mut self) -> CLKO_W {
    CLKO_W { w: self }
  }
  #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
  #[inline(always)]
  pub fn sc16p(&mut self) -> SC16P_W {
    SC16P_W { w: self }
  }
  #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
  #[inline(always)]
  pub fn sc8p(&mut self) -> SC8P_W {
    SC8P_W { w: self }
  }
  #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
  #[inline(always)]
  pub fn sc4p(&mut self) -> SC4P_W {
    SC4P_W { w: self }
  }
  #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
  #[inline(always)]
  pub fn sc2p(&mut self) -> SC2P_W {
    SC2P_W { w: self }
  }
  #[doc = "Bit 15 - Oscillator Mode Select"]
  #[inline(always)]
  pub fn oscm(&mut self) -> OSCM_W {
    OSCM_W { w: self }
  }
  #[doc = "Bits 16:17 - POR Select"]
  #[inline(always)]
  pub fn pors(&mut self) -> PORS_W {
    PORS_W { w: self }
  }
  #[doc = "Bits 24:25 - Clock Pin Enable"]
  #[inline(always)]
  pub fn cpe(&mut self) -> CPE_W {
    CPE_W { w: self }
  }
}
