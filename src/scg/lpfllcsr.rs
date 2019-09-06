#[doc = "Reader of register LPFLLCSR"]
pub type R = crate::R<u32, super::LPFLLCSR>;
#[doc = "Writer for register LPFLLCSR"]
pub type W = crate::W<u32, super::LPFLLCSR>;
#[doc = "Register LPFLLCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPFLLCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "LPFLL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLEN_A {
  #[doc = "0: LPFLL is disabled"]
  LPFLLEN_0,
  #[doc = "1: LPFLL is enabled"]
  LPFLLEN_1,
}
impl From<LPFLLEN_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLEN_A) -> Self {
    match variant {
      LPFLLEN_A::LPFLLEN_0 => false,
      LPFLLEN_A::LPFLLEN_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLEN`"]
pub type LPFLLEN_R = crate::R<bool, LPFLLEN_A>;
impl LPFLLEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLEN_A {
    match self.bits {
      false => LPFLLEN_A::LPFLLEN_0,
      true => LPFLLEN_A::LPFLLEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLEN_0`"]
  #[inline(always)]
  pub fn is_lpfllen_0(&self) -> bool {
    *self == LPFLLEN_A::LPFLLEN_0
  }
  #[doc = "Checks if the value of the field is `LPFLLEN_1`"]
  #[inline(always)]
  pub fn is_lpfllen_1(&self) -> bool {
    *self == LPFLLEN_A::LPFLLEN_1
  }
}
#[doc = "Write proxy for field `LPFLLEN`"]
pub struct LPFLLEN_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPFLL is disabled"]
  #[inline(always)]
  pub fn lpfllen_0(self) -> &'a mut W {
    self.variant(LPFLLEN_A::LPFLLEN_0)
  }
  #[doc = "LPFLL is enabled"]
  #[inline(always)]
  pub fn lpfllen_1(self) -> &'a mut W {
    self.variant(LPFLLEN_A::LPFLLEN_1)
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
#[doc = "LPFLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLSTEN_A {
  #[doc = "0: LPFLL is disabled in Stop modes."]
  LPFLLSTEN_0,
  #[doc = "1: LPFLL is enabled in Stop modes"]
  LPFLLSTEN_1,
}
impl From<LPFLLSTEN_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLSTEN_A) -> Self {
    match variant {
      LPFLLSTEN_A::LPFLLSTEN_0 => false,
      LPFLLSTEN_A::LPFLLSTEN_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLSTEN`"]
pub type LPFLLSTEN_R = crate::R<bool, LPFLLSTEN_A>;
impl LPFLLSTEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLSTEN_A {
    match self.bits {
      false => LPFLLSTEN_A::LPFLLSTEN_0,
      true => LPFLLSTEN_A::LPFLLSTEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLSTEN_0`"]
  #[inline(always)]
  pub fn is_lpfllsten_0(&self) -> bool {
    *self == LPFLLSTEN_A::LPFLLSTEN_0
  }
  #[doc = "Checks if the value of the field is `LPFLLSTEN_1`"]
  #[inline(always)]
  pub fn is_lpfllsten_1(&self) -> bool {
    *self == LPFLLSTEN_A::LPFLLSTEN_1
  }
}
#[doc = "Write proxy for field `LPFLLSTEN`"]
pub struct LPFLLSTEN_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLSTEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLSTEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPFLL is disabled in Stop modes."]
  #[inline(always)]
  pub fn lpfllsten_0(self) -> &'a mut W {
    self.variant(LPFLLSTEN_A::LPFLLSTEN_0)
  }
  #[doc = "LPFLL is enabled in Stop modes"]
  #[inline(always)]
  pub fn lpfllsten_1(self) -> &'a mut W {
    self.variant(LPFLLSTEN_A::LPFLLSTEN_1)
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
#[doc = "LPFLL Trim Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLTREN_A {
  #[doc = "0: Disable trimming LPFLL to an reference clock source"]
  LPFLLTREN_0,
  #[doc = "1: Enable trimming LPFLL to an reference clock source"]
  LPFLLTREN_1,
}
impl From<LPFLLTREN_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLTREN_A) -> Self {
    match variant {
      LPFLLTREN_A::LPFLLTREN_0 => false,
      LPFLLTREN_A::LPFLLTREN_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLTREN`"]
pub type LPFLLTREN_R = crate::R<bool, LPFLLTREN_A>;
impl LPFLLTREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLTREN_A {
    match self.bits {
      false => LPFLLTREN_A::LPFLLTREN_0,
      true => LPFLLTREN_A::LPFLLTREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLTREN_0`"]
  #[inline(always)]
  pub fn is_lpflltren_0(&self) -> bool {
    *self == LPFLLTREN_A::LPFLLTREN_0
  }
  #[doc = "Checks if the value of the field is `LPFLLTREN_1`"]
  #[inline(always)]
  pub fn is_lpflltren_1(&self) -> bool {
    *self == LPFLLTREN_A::LPFLLTREN_1
  }
}
#[doc = "Write proxy for field `LPFLLTREN`"]
pub struct LPFLLTREN_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLTREN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLTREN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable trimming LPFLL to an reference clock source"]
  #[inline(always)]
  pub fn lpflltren_0(self) -> &'a mut W {
    self.variant(LPFLLTREN_A::LPFLLTREN_0)
  }
  #[doc = "Enable trimming LPFLL to an reference clock source"]
  #[inline(always)]
  pub fn lpflltren_1(self) -> &'a mut W {
    self.variant(LPFLLTREN_A::LPFLLTREN_1)
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
#[doc = "LPFLL Trim Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLTRUP_A {
  #[doc = "0: Disable LPFLL trimming updates. LPFLL frequency determined by AUTOTRIM written value."]
  LPFLLTRUP_0,
  #[doc = "1: Enable LPFLL trimming updates. LPFLL frequency determined by reference clock multiplication"]
  LPFLLTRUP_1,
}
impl From<LPFLLTRUP_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLTRUP_A) -> Self {
    match variant {
      LPFLLTRUP_A::LPFLLTRUP_0 => false,
      LPFLLTRUP_A::LPFLLTRUP_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLTRUP`"]
pub type LPFLLTRUP_R = crate::R<bool, LPFLLTRUP_A>;
impl LPFLLTRUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLTRUP_A {
    match self.bits {
      false => LPFLLTRUP_A::LPFLLTRUP_0,
      true => LPFLLTRUP_A::LPFLLTRUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLTRUP_0`"]
  #[inline(always)]
  pub fn is_lpflltrup_0(&self) -> bool {
    *self == LPFLLTRUP_A::LPFLLTRUP_0
  }
  #[doc = "Checks if the value of the field is `LPFLLTRUP_1`"]
  #[inline(always)]
  pub fn is_lpflltrup_1(&self) -> bool {
    *self == LPFLLTRUP_A::LPFLLTRUP_1
  }
}
#[doc = "Write proxy for field `LPFLLTRUP`"]
pub struct LPFLLTRUP_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLTRUP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLTRUP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable LPFLL trimming updates. LPFLL frequency determined by AUTOTRIM written value."]
  #[inline(always)]
  pub fn lpflltrup_0(self) -> &'a mut W {
    self.variant(LPFLLTRUP_A::LPFLLTRUP_0)
  }
  #[doc = "Enable LPFLL trimming updates. LPFLL frequency determined by reference clock multiplication"]
  #[inline(always)]
  pub fn lpflltrup_1(self) -> &'a mut W {
    self.variant(LPFLLTRUP_A::LPFLLTRUP_1)
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
#[doc = "LPFLL Trim LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLTRMLOCK_A {
  #[doc = "0: LPFLL not Locked"]
  LPFLLTRMLOCK_0,
  #[doc = "1: LPFLL trimmed and Locked"]
  LPFLLTRMLOCK_1,
}
impl From<LPFLLTRMLOCK_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLTRMLOCK_A) -> Self {
    match variant {
      LPFLLTRMLOCK_A::LPFLLTRMLOCK_0 => false,
      LPFLLTRMLOCK_A::LPFLLTRMLOCK_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLTRMLOCK`"]
pub type LPFLLTRMLOCK_R = crate::R<bool, LPFLLTRMLOCK_A>;
impl LPFLLTRMLOCK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLTRMLOCK_A {
    match self.bits {
      false => LPFLLTRMLOCK_A::LPFLLTRMLOCK_0,
      true => LPFLLTRMLOCK_A::LPFLLTRMLOCK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLTRMLOCK_0`"]
  #[inline(always)]
  pub fn is_lpflltrmlock_0(&self) -> bool {
    *self == LPFLLTRMLOCK_A::LPFLLTRMLOCK_0
  }
  #[doc = "Checks if the value of the field is `LPFLLTRMLOCK_1`"]
  #[inline(always)]
  pub fn is_lpflltrmlock_1(&self) -> bool {
    *self == LPFLLTRMLOCK_A::LPFLLTRMLOCK_1
  }
}
#[doc = "LPFLL Clock Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLCM_A {
  #[doc = "0: LPFLL Clock Monitor is disabled"]
  LPFLLCM_0,
  #[doc = "1: LPFLL Clock Monitor is enabled"]
  LPFLLCM_1,
}
impl From<LPFLLCM_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLCM_A) -> Self {
    match variant {
      LPFLLCM_A::LPFLLCM_0 => false,
      LPFLLCM_A::LPFLLCM_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLCM`"]
pub type LPFLLCM_R = crate::R<bool, LPFLLCM_A>;
impl LPFLLCM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLCM_A {
    match self.bits {
      false => LPFLLCM_A::LPFLLCM_0,
      true => LPFLLCM_A::LPFLLCM_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLCM_0`"]
  #[inline(always)]
  pub fn is_lpfllcm_0(&self) -> bool {
    *self == LPFLLCM_A::LPFLLCM_0
  }
  #[doc = "Checks if the value of the field is `LPFLLCM_1`"]
  #[inline(always)]
  pub fn is_lpfllcm_1(&self) -> bool {
    *self == LPFLLCM_A::LPFLLCM_1
  }
}
#[doc = "Write proxy for field `LPFLLCM`"]
pub struct LPFLLCM_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLCM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLCM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPFLL Clock Monitor is disabled"]
  #[inline(always)]
  pub fn lpfllcm_0(self) -> &'a mut W {
    self.variant(LPFLLCM_A::LPFLLCM_0)
  }
  #[doc = "LPFLL Clock Monitor is enabled"]
  #[inline(always)]
  pub fn lpfllcm_1(self) -> &'a mut W {
    self.variant(LPFLLCM_A::LPFLLCM_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "LPFLL Clock Monitor ResetEnable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLCMRE_A {
  #[doc = "0: Clock Monitor generates interrupt when error detected"]
  LPFLLCMRE_0,
  #[doc = "1: Clock Monitor generates reset when error detected"]
  LPFLLCMRE_1,
}
impl From<LPFLLCMRE_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLCMRE_A) -> Self {
    match variant {
      LPFLLCMRE_A::LPFLLCMRE_0 => false,
      LPFLLCMRE_A::LPFLLCMRE_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLCMRE`"]
pub type LPFLLCMRE_R = crate::R<bool, LPFLLCMRE_A>;
impl LPFLLCMRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLCMRE_A {
    match self.bits {
      false => LPFLLCMRE_A::LPFLLCMRE_0,
      true => LPFLLCMRE_A::LPFLLCMRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLCMRE_0`"]
  #[inline(always)]
  pub fn is_lpfllcmre_0(&self) -> bool {
    *self == LPFLLCMRE_A::LPFLLCMRE_0
  }
  #[doc = "Checks if the value of the field is `LPFLLCMRE_1`"]
  #[inline(always)]
  pub fn is_lpfllcmre_1(&self) -> bool {
    *self == LPFLLCMRE_A::LPFLLCMRE_1
  }
}
#[doc = "Write proxy for field `LPFLLCMRE`"]
pub struct LPFLLCMRE_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLCMRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLCMRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock Monitor generates interrupt when error detected"]
  #[inline(always)]
  pub fn lpfllcmre_0(self) -> &'a mut W {
    self.variant(LPFLLCMRE_A::LPFLLCMRE_0)
  }
  #[doc = "Clock Monitor generates reset when error detected"]
  #[inline(always)]
  pub fn lpfllcmre_1(self) -> &'a mut W {
    self.variant(LPFLLCMRE_A::LPFLLCMRE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: Control Status Register can be written."]
  LK_0,
  #[doc = "1: Control Status Register cannot be written."]
  LK_1,
}
impl From<LK_A> for bool {
  #[inline(always)]
  fn from(variant: LK_A) -> Self {
    match variant {
      LK_A::LK_0 => false,
      LK_A::LK_1 => true,
    }
  }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK_A {
    match self.bits {
      false => LK_A::LK_0,
      true => LK_A::LK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK_0`"]
  #[inline(always)]
  pub fn is_lk_0(&self) -> bool {
    *self == LK_A::LK_0
  }
  #[doc = "Checks if the value of the field is `LK_1`"]
  #[inline(always)]
  pub fn is_lk_1(&self) -> bool {
    *self == LK_A::LK_1
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Control Status Register can be written."]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_A::LK_0)
  }
  #[doc = "Control Status Register cannot be written."]
  #[inline(always)]
  pub fn lk_1(self) -> &'a mut W {
    self.variant(LK_A::LK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "LPFLL Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLVLD_A {
  #[doc = "0: LPFLL is not enabled or clock is not valid."]
  LPFLLVLD_0,
  #[doc = "1: LPFLL is enabled and output clock is valid."]
  LPFLLVLD_1,
}
impl From<LPFLLVLD_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLVLD_A) -> Self {
    match variant {
      LPFLLVLD_A::LPFLLVLD_0 => false,
      LPFLLVLD_A::LPFLLVLD_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLVLD`"]
pub type LPFLLVLD_R = crate::R<bool, LPFLLVLD_A>;
impl LPFLLVLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLVLD_A {
    match self.bits {
      false => LPFLLVLD_A::LPFLLVLD_0,
      true => LPFLLVLD_A::LPFLLVLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLVLD_0`"]
  #[inline(always)]
  pub fn is_lpfllvld_0(&self) -> bool {
    *self == LPFLLVLD_A::LPFLLVLD_0
  }
  #[doc = "Checks if the value of the field is `LPFLLVLD_1`"]
  #[inline(always)]
  pub fn is_lpfllvld_1(&self) -> bool {
    *self == LPFLLVLD_A::LPFLLVLD_1
  }
}
#[doc = "LPFLL Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLSEL_A {
  #[doc = "0: LPFLL is not the system clock source"]
  LPFLLSEL_0,
  #[doc = "1: LPFLL is the system clock source"]
  LPFLLSEL_1,
}
impl From<LPFLLSEL_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLSEL_A) -> Self {
    match variant {
      LPFLLSEL_A::LPFLLSEL_0 => false,
      LPFLLSEL_A::LPFLLSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLSEL`"]
pub type LPFLLSEL_R = crate::R<bool, LPFLLSEL_A>;
impl LPFLLSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLSEL_A {
    match self.bits {
      false => LPFLLSEL_A::LPFLLSEL_0,
      true => LPFLLSEL_A::LPFLLSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLSEL_0`"]
  #[inline(always)]
  pub fn is_lpfllsel_0(&self) -> bool {
    *self == LPFLLSEL_A::LPFLLSEL_0
  }
  #[doc = "Checks if the value of the field is `LPFLLSEL_1`"]
  #[inline(always)]
  pub fn is_lpfllsel_1(&self) -> bool {
    *self == LPFLLSEL_A::LPFLLSEL_1
  }
}
#[doc = "LPFLL Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLERR_A {
  #[doc = "0: Error not detected with the LPFLL trimming."]
  LPFLLERR_0,
  #[doc = "1: Error detected with the LPFLL trimming."]
  LPFLLERR_1,
}
impl From<LPFLLERR_A> for bool {
  #[inline(always)]
  fn from(variant: LPFLLERR_A) -> Self {
    match variant {
      LPFLLERR_A::LPFLLERR_0 => false,
      LPFLLERR_A::LPFLLERR_1 => true,
    }
  }
}
#[doc = "Reader of field `LPFLLERR`"]
pub type LPFLLERR_R = crate::R<bool, LPFLLERR_A>;
impl LPFLLERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLERR_A {
    match self.bits {
      false => LPFLLERR_A::LPFLLERR_0,
      true => LPFLLERR_A::LPFLLERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLERR_0`"]
  #[inline(always)]
  pub fn is_lpfllerr_0(&self) -> bool {
    *self == LPFLLERR_A::LPFLLERR_0
  }
  #[doc = "Checks if the value of the field is `LPFLLERR_1`"]
  #[inline(always)]
  pub fn is_lpfllerr_1(&self) -> bool {
    *self == LPFLLERR_A::LPFLLERR_1
  }
}
#[doc = "Write proxy for field `LPFLLERR`"]
pub struct LPFLLERR_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Error not detected with the LPFLL trimming."]
  #[inline(always)]
  pub fn lpfllerr_0(self) -> &'a mut W {
    self.variant(LPFLLERR_A::LPFLLERR_0)
  }
  #[doc = "Error detected with the LPFLL trimming."]
  #[inline(always)]
  pub fn lpfllerr_1(self) -> &'a mut W {
    self.variant(LPFLLERR_A::LPFLLERR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - LPFLL Enable"]
  #[inline(always)]
  pub fn lpfllen(&self) -> LPFLLEN_R {
    LPFLLEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - LPFLL Stop Enable"]
  #[inline(always)]
  pub fn lpfllsten(&self) -> LPFLLSTEN_R {
    LPFLLSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 8 - LPFLL Trim Enable"]
  #[inline(always)]
  pub fn lpflltren(&self) -> LPFLLTREN_R {
    LPFLLTREN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - LPFLL Trim Update"]
  #[inline(always)]
  pub fn lpflltrup(&self) -> LPFLLTRUP_R {
    LPFLLTRUP_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - LPFLL Trim LOCK"]
  #[inline(always)]
  pub fn lpflltrmlock(&self) -> LPFLLTRMLOCK_R {
    LPFLLTRMLOCK_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 16 - LPFLL Clock Monitor"]
  #[inline(always)]
  pub fn lpfllcm(&self) -> LPFLLCM_R {
    LPFLLCM_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - LPFLL Clock Monitor ResetEnable"]
  #[inline(always)]
  pub fn lpfllcmre(&self) -> LPFLLCMRE_R {
    LPFLLCMRE_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - LPFLL Valid"]
  #[inline(always)]
  pub fn lpfllvld(&self) -> LPFLLVLD_R {
    LPFLLVLD_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - LPFLL Selected"]
  #[inline(always)]
  pub fn lpfllsel(&self) -> LPFLLSEL_R {
    LPFLLSEL_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - LPFLL Clock Error"]
  #[inline(always)]
  pub fn lpfllerr(&self) -> LPFLLERR_R {
    LPFLLERR_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - LPFLL Enable"]
  #[inline(always)]
  pub fn lpfllen(&mut self) -> LPFLLEN_W {
    LPFLLEN_W { w: self }
  }
  #[doc = "Bit 1 - LPFLL Stop Enable"]
  #[inline(always)]
  pub fn lpfllsten(&mut self) -> LPFLLSTEN_W {
    LPFLLSTEN_W { w: self }
  }
  #[doc = "Bit 8 - LPFLL Trim Enable"]
  #[inline(always)]
  pub fn lpflltren(&mut self) -> LPFLLTREN_W {
    LPFLLTREN_W { w: self }
  }
  #[doc = "Bit 9 - LPFLL Trim Update"]
  #[inline(always)]
  pub fn lpflltrup(&mut self) -> LPFLLTRUP_W {
    LPFLLTRUP_W { w: self }
  }
  #[doc = "Bit 16 - LPFLL Clock Monitor"]
  #[inline(always)]
  pub fn lpfllcm(&mut self) -> LPFLLCM_W {
    LPFLLCM_W { w: self }
  }
  #[doc = "Bit 17 - LPFLL Clock Monitor ResetEnable"]
  #[inline(always)]
  pub fn lpfllcmre(&mut self) -> LPFLLCMRE_W {
    LPFLLCMRE_W { w: self }
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
  #[doc = "Bit 26 - LPFLL Clock Error"]
  #[inline(always)]
  pub fn lpfllerr(&mut self) -> LPFLLERR_W {
    LPFLLERR_W { w: self }
  }
}
