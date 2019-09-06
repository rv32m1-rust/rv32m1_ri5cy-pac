#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x0080_0000"]
impl crate::ResetValue for super::CFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0080_0000
  }
}
#[doc = "ADC trigger priority control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRICTRL_A {
  #[doc = "0: If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
  TPRICTRL_0,
  #[doc = "1: If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated. Note that compare until true commands can be interrupted prior to resulting in a true conversion."]
  TPRICTRL_1,
}
impl From<TPRICTRL_A> for bool {
  #[inline(always)]
  fn from(variant: TPRICTRL_A) -> Self {
    match variant {
      TPRICTRL_A::TPRICTRL_0 => false,
      TPRICTRL_A::TPRICTRL_1 => true,
    }
  }
}
#[doc = "Reader of field `TPRICTRL`"]
pub type TPRICTRL_R = crate::R<bool, TPRICTRL_A>;
impl TPRICTRL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPRICTRL_A {
    match self.bits {
      false => TPRICTRL_A::TPRICTRL_0,
      true => TPRICTRL_A::TPRICTRL_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPRICTRL_0`"]
  #[inline(always)]
  pub fn is_tprictrl_0(&self) -> bool {
    *self == TPRICTRL_A::TPRICTRL_0
  }
  #[doc = "Checks if the value of the field is `TPRICTRL_1`"]
  #[inline(always)]
  pub fn is_tprictrl_1(&self) -> bool {
    *self == TPRICTRL_A::TPRICTRL_1
  }
}
#[doc = "Write proxy for field `TPRICTRL`"]
pub struct TPRICTRL_W<'a> {
  w: &'a mut W,
}
impl<'a> TPRICTRL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPRICTRL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
  #[inline(always)]
  pub fn tprictrl_0(self) -> &'a mut W {
    self.variant(TPRICTRL_A::TPRICTRL_0)
  }
  #[doc = "If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated. Note that compare until true commands can be interrupted prior to resulting in a true conversion."]
  #[inline(always)]
  pub fn tprictrl_1(self) -> &'a mut W {
    self.variant(TPRICTRL_A::TPRICTRL_1)
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
#[doc = "Power Configuration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSEL_A {
  #[doc = "0: Level 1 (Lowest power setting)"]
  PWRSEL_0,
  #[doc = "1: Level 2"]
  PWRSEL_1,
  #[doc = "2: Level 3"]
  PWRSEL_2,
  #[doc = "3: Level 4 (Highest power setting)"]
  PWRSEL_3,
}
impl From<PWRSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: PWRSEL_A) -> Self {
    match variant {
      PWRSEL_A::PWRSEL_0 => 0,
      PWRSEL_A::PWRSEL_1 => 1,
      PWRSEL_A::PWRSEL_2 => 2,
      PWRSEL_A::PWRSEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `PWRSEL`"]
pub type PWRSEL_R = crate::R<u8, PWRSEL_A>;
impl PWRSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PWRSEL_A {
    match self.bits {
      0 => PWRSEL_A::PWRSEL_0,
      1 => PWRSEL_A::PWRSEL_1,
      2 => PWRSEL_A::PWRSEL_2,
      3 => PWRSEL_A::PWRSEL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PWRSEL_0`"]
  #[inline(always)]
  pub fn is_pwrsel_0(&self) -> bool {
    *self == PWRSEL_A::PWRSEL_0
  }
  #[doc = "Checks if the value of the field is `PWRSEL_1`"]
  #[inline(always)]
  pub fn is_pwrsel_1(&self) -> bool {
    *self == PWRSEL_A::PWRSEL_1
  }
  #[doc = "Checks if the value of the field is `PWRSEL_2`"]
  #[inline(always)]
  pub fn is_pwrsel_2(&self) -> bool {
    *self == PWRSEL_A::PWRSEL_2
  }
  #[doc = "Checks if the value of the field is `PWRSEL_3`"]
  #[inline(always)]
  pub fn is_pwrsel_3(&self) -> bool {
    *self == PWRSEL_A::PWRSEL_3
  }
}
#[doc = "Write proxy for field `PWRSEL`"]
pub struct PWRSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> PWRSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PWRSEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Level 1 (Lowest power setting)"]
  #[inline(always)]
  pub fn pwrsel_0(self) -> &'a mut W {
    self.variant(PWRSEL_A::PWRSEL_0)
  }
  #[doc = "Level 2"]
  #[inline(always)]
  pub fn pwrsel_1(self) -> &'a mut W {
    self.variant(PWRSEL_A::PWRSEL_1)
  }
  #[doc = "Level 3"]
  #[inline(always)]
  pub fn pwrsel_2(self) -> &'a mut W {
    self.variant(PWRSEL_A::PWRSEL_2)
  }
  #[doc = "Level 4 (Highest power setting)"]
  #[inline(always)]
  pub fn pwrsel_3(self) -> &'a mut W {
    self.variant(PWRSEL_A::PWRSEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
  #[doc = "0: (Default) Option 1 setting."]
  REFSEL_0,
  #[doc = "1: Option 2 setting."]
  REFSEL_1,
  #[doc = "2: Option 3 setting."]
  REFSEL_2,
}
impl From<REFSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: REFSEL_A) -> Self {
    match variant {
      REFSEL_A::REFSEL_0 => 0,
      REFSEL_A::REFSEL_1 => 1,
      REFSEL_A::REFSEL_2 => 2,
    }
  }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(REFSEL_A::REFSEL_0),
      1 => Val(REFSEL_A::REFSEL_1),
      2 => Val(REFSEL_A::REFSEL_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `REFSEL_0`"]
  #[inline(always)]
  pub fn is_refsel_0(&self) -> bool {
    *self == REFSEL_A::REFSEL_0
  }
  #[doc = "Checks if the value of the field is `REFSEL_1`"]
  #[inline(always)]
  pub fn is_refsel_1(&self) -> bool {
    *self == REFSEL_A::REFSEL_1
  }
  #[doc = "Checks if the value of the field is `REFSEL_2`"]
  #[inline(always)]
  pub fn is_refsel_2(&self) -> bool {
    *self == REFSEL_A::REFSEL_2
  }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "(Default) Option 1 setting."]
  #[inline(always)]
  pub fn refsel_0(self) -> &'a mut W {
    self.variant(REFSEL_A::REFSEL_0)
  }
  #[doc = "Option 2 setting."]
  #[inline(always)]
  pub fn refsel_1(self) -> &'a mut W {
    self.variant(REFSEL_A::REFSEL_1)
  }
  #[doc = "Option 3 setting."]
  #[inline(always)]
  pub fn refsel_2(self) -> &'a mut W {
    self.variant(REFSEL_A::REFSEL_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Configure for offset calibration function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALOFS_A {
  #[doc = "0: Calibration function disabled"]
  CALOFS_0,
  #[doc = "1: Configure for offset calibration function"]
  CALOFS_1,
}
impl From<CALOFS_A> for bool {
  #[inline(always)]
  fn from(variant: CALOFS_A) -> Self {
    match variant {
      CALOFS_A::CALOFS_0 => false,
      CALOFS_A::CALOFS_1 => true,
    }
  }
}
#[doc = "Reader of field `CALOFS`"]
pub type CALOFS_R = crate::R<bool, CALOFS_A>;
impl CALOFS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CALOFS_A {
    match self.bits {
      false => CALOFS_A::CALOFS_0,
      true => CALOFS_A::CALOFS_1,
    }
  }
  #[doc = "Checks if the value of the field is `CALOFS_0`"]
  #[inline(always)]
  pub fn is_calofs_0(&self) -> bool {
    *self == CALOFS_A::CALOFS_0
  }
  #[doc = "Checks if the value of the field is `CALOFS_1`"]
  #[inline(always)]
  pub fn is_calofs_1(&self) -> bool {
    *self == CALOFS_A::CALOFS_1
  }
}
#[doc = "Write proxy for field `CALOFS`"]
pub struct CALOFS_W<'a> {
  w: &'a mut W,
}
impl<'a> CALOFS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CALOFS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Calibration function disabled"]
  #[inline(always)]
  pub fn calofs_0(self) -> &'a mut W {
    self.variant(CALOFS_A::CALOFS_0)
  }
  #[doc = "Configure for offset calibration function"]
  #[inline(always)]
  pub fn calofs_1(self) -> &'a mut W {
    self.variant(CALOFS_A::CALOFS_1)
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
#[doc = "Reader of field `PUDLY`"]
pub type PUDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUDLY`"]
pub struct PUDLY_W<'a> {
  w: &'a mut W,
}
impl<'a> PUDLY_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
    self.w
  }
}
#[doc = "ADC Analog Pre-Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWREN_A {
  #[doc = "0: ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
  PWREN_0,
  #[doc = "1: ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
  PWREN_1,
}
impl From<PWREN_A> for bool {
  #[inline(always)]
  fn from(variant: PWREN_A) -> Self {
    match variant {
      PWREN_A::PWREN_0 => false,
      PWREN_A::PWREN_1 => true,
    }
  }
}
#[doc = "Reader of field `PWREN`"]
pub type PWREN_R = crate::R<bool, PWREN_A>;
impl PWREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PWREN_A {
    match self.bits {
      false => PWREN_A::PWREN_0,
      true => PWREN_A::PWREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `PWREN_0`"]
  #[inline(always)]
  pub fn is_pwren_0(&self) -> bool {
    *self == PWREN_A::PWREN_0
  }
  #[doc = "Checks if the value of the field is `PWREN_1`"]
  #[inline(always)]
  pub fn is_pwren_1(&self) -> bool {
    *self == PWREN_A::PWREN_1
  }
}
#[doc = "Write proxy for field `PWREN`"]
pub struct PWREN_W<'a> {
  w: &'a mut W,
}
impl<'a> PWREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PWREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
  #[inline(always)]
  pub fn pwren_0(self) -> &'a mut W {
    self.variant(PWREN_A::PWREN_0)
  }
  #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
  #[inline(always)]
  pub fn pwren_1(self) -> &'a mut W {
    self.variant(PWREN_A::PWREN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
    self.w
  }
}
#[doc = "Enable support for low voltage reference on Option 1 Reference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF1RNG_A {
  #[doc = "0: Configuration required when Voltage Reference Option 1 input is in high voltage range"]
  VREF1RNG_0,
  #[doc = "1: Configuration required when Voltage Reference Option 1 input is in low voltage range"]
  VREF1RNG_1,
}
impl From<VREF1RNG_A> for bool {
  #[inline(always)]
  fn from(variant: VREF1RNG_A) -> Self {
    match variant {
      VREF1RNG_A::VREF1RNG_0 => false,
      VREF1RNG_A::VREF1RNG_1 => true,
    }
  }
}
#[doc = "Reader of field `VREF1RNG`"]
pub type VREF1RNG_R = crate::R<bool, VREF1RNG_A>;
impl VREF1RNG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VREF1RNG_A {
    match self.bits {
      false => VREF1RNG_A::VREF1RNG_0,
      true => VREF1RNG_A::VREF1RNG_1,
    }
  }
  #[doc = "Checks if the value of the field is `VREF1RNG_0`"]
  #[inline(always)]
  pub fn is_vref1rng_0(&self) -> bool {
    *self == VREF1RNG_A::VREF1RNG_0
  }
  #[doc = "Checks if the value of the field is `VREF1RNG_1`"]
  #[inline(always)]
  pub fn is_vref1rng_1(&self) -> bool {
    *self == VREF1RNG_A::VREF1RNG_1
  }
}
#[doc = "Write proxy for field `VREF1RNG`"]
pub struct VREF1RNG_W<'a> {
  w: &'a mut W,
}
impl<'a> VREF1RNG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VREF1RNG_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Configuration required when Voltage Reference Option 1 input is in high voltage range"]
  #[inline(always)]
  pub fn vref1rng_0(self) -> &'a mut W {
    self.variant(VREF1RNG_A::VREF1RNG_0)
  }
  #[doc = "Configuration required when Voltage Reference Option 1 input is in low voltage range"]
  #[inline(always)]
  pub fn vref1rng_1(self) -> &'a mut W {
    self.variant(VREF1RNG_A::VREF1RNG_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "ADC asynchronous clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCKEN_A {
  #[doc = "0: ADC internal clock is disabled"]
  ADCKEN_0,
  #[doc = "1: ADC internal clock is enabled"]
  ADCKEN_1,
}
impl From<ADCKEN_A> for bool {
  #[inline(always)]
  fn from(variant: ADCKEN_A) -> Self {
    match variant {
      ADCKEN_A::ADCKEN_0 => false,
      ADCKEN_A::ADCKEN_1 => true,
    }
  }
}
#[doc = "Reader of field `ADCKEN`"]
pub type ADCKEN_R = crate::R<bool, ADCKEN_A>;
impl ADCKEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ADCKEN_A {
    match self.bits {
      false => ADCKEN_A::ADCKEN_0,
      true => ADCKEN_A::ADCKEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `ADCKEN_0`"]
  #[inline(always)]
  pub fn is_adcken_0(&self) -> bool {
    *self == ADCKEN_A::ADCKEN_0
  }
  #[doc = "Checks if the value of the field is `ADCKEN_1`"]
  #[inline(always)]
  pub fn is_adcken_1(&self) -> bool {
    *self == ADCKEN_A::ADCKEN_1
  }
}
#[doc = "Write proxy for field `ADCKEN`"]
pub struct ADCKEN_W<'a> {
  w: &'a mut W,
}
impl<'a> ADCKEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ADCKEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "ADC internal clock is disabled"]
  #[inline(always)]
  pub fn adcken_0(self) -> &'a mut W {
    self.variant(ADCKEN_A::ADCKEN_0)
  }
  #[doc = "ADC internal clock is enabled"]
  #[inline(always)]
  pub fn adcken_1(self) -> &'a mut W {
    self.variant(ADCKEN_A::ADCKEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - ADC trigger priority control"]
  #[inline(always)]
  pub fn tprictrl(&self) -> TPRICTRL_R {
    TPRICTRL_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bits 4:5 - Power Configuration Select"]
  #[inline(always)]
  pub fn pwrsel(&self) -> PWRSEL_R {
    PWRSEL_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Voltage Reference Selection"]
  #[inline(always)]
  pub fn refsel(&self) -> REFSEL_R {
    REFSEL_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bit 15 - Configure for offset calibration function"]
  #[inline(always)]
  pub fn calofs(&self) -> CALOFS_R {
    CALOFS_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bits 16:23 - Power Up Delay"]
  #[inline(always)]
  pub fn pudly(&self) -> PUDLY_R {
    PUDLY_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bit 28 - ADC Analog Pre-Enable"]
  #[inline(always)]
  pub fn pwren(&self) -> PWREN_R {
    PWREN_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Enable support for low voltage reference on Option 1 Reference"]
  #[inline(always)]
  pub fn vref1rng(&self) -> VREF1RNG_R {
    VREF1RNG_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 31 - ADC asynchronous clock enable"]
  #[inline(always)]
  pub fn adcken(&self) -> ADCKEN_R {
    ADCKEN_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - ADC trigger priority control"]
  #[inline(always)]
  pub fn tprictrl(&mut self) -> TPRICTRL_W {
    TPRICTRL_W { w: self }
  }
  #[doc = "Bits 4:5 - Power Configuration Select"]
  #[inline(always)]
  pub fn pwrsel(&mut self) -> PWRSEL_W {
    PWRSEL_W { w: self }
  }
  #[doc = "Bits 6:7 - Voltage Reference Selection"]
  #[inline(always)]
  pub fn refsel(&mut self) -> REFSEL_W {
    REFSEL_W { w: self }
  }
  #[doc = "Bit 15 - Configure for offset calibration function"]
  #[inline(always)]
  pub fn calofs(&mut self) -> CALOFS_W {
    CALOFS_W { w: self }
  }
  #[doc = "Bits 16:23 - Power Up Delay"]
  #[inline(always)]
  pub fn pudly(&mut self) -> PUDLY_W {
    PUDLY_W { w: self }
  }
  #[doc = "Bit 28 - ADC Analog Pre-Enable"]
  #[inline(always)]
  pub fn pwren(&mut self) -> PWREN_W {
    PWREN_W { w: self }
  }
  #[doc = "Bit 29 - Enable support for low voltage reference on Option 1 Reference"]
  #[inline(always)]
  pub fn vref1rng(&mut self) -> VREF1RNG_W {
    VREF1RNG_W { w: self }
  }
  #[doc = "Bit 31 - ADC asynchronous clock enable"]
  #[inline(always)]
  pub fn adcken(&mut self) -> ADCKEN_W {
    ADCKEN_W { w: self }
  }
}
