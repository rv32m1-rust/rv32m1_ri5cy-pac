#[doc = "Reader of register WAR"]
pub type R = crate::R<u32, super::WAR>;
#[doc = "Writer for register WAR"]
pub type W = crate::W<u32, super::WAR>;
#[doc = "Register WAR `reset()`'s with value 0x000f_ffff"]
impl crate::ResetValue for super::WAR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x000f_ffff
  }
}
#[doc = "Time Seconds Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRW_A {
  #[doc = "0: Writes to the Time Seconds Register are ignored."]
  TSRW_0,
  #[doc = "1: Writes to the Time Seconds Register complete as normal."]
  TSRW_1,
}
impl From<TSRW_A> for bool {
  #[inline(always)]
  fn from(variant: TSRW_A) -> Self {
    match variant {
      TSRW_A::TSRW_0 => false,
      TSRW_A::TSRW_1 => true,
    }
  }
}
#[doc = "Reader of field `TSRW`"]
pub type TSRW_R = crate::R<bool, TSRW_A>;
impl TSRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TSRW_A {
    match self.bits {
      false => TSRW_A::TSRW_0,
      true => TSRW_A::TSRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TSRW_0`"]
  #[inline(always)]
  pub fn is_tsrw_0(&self) -> bool {
    *self == TSRW_A::TSRW_0
  }
  #[doc = "Checks if the value of the field is `TSRW_1`"]
  #[inline(always)]
  pub fn is_tsrw_1(&self) -> bool {
    *self == TSRW_A::TSRW_1
  }
}
#[doc = "Write proxy for field `TSRW`"]
pub struct TSRW_W<'a> {
  w: &'a mut W,
}
impl<'a> TSRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TSRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Time Seconds Register are ignored."]
  #[inline(always)]
  pub fn tsrw_0(self) -> &'a mut W {
    self.variant(TSRW_A::TSRW_0)
  }
  #[doc = "Writes to the Time Seconds Register complete as normal."]
  #[inline(always)]
  pub fn tsrw_1(self) -> &'a mut W {
    self.variant(TSRW_A::TSRW_1)
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
#[doc = "Time Prescaler Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRW_A {
  #[doc = "0: Writes to the Time Prescaler Register are ignored."]
  TPRW_0,
  #[doc = "1: Writes to the Time Prescaler Register complete as normal."]
  TPRW_1,
}
impl From<TPRW_A> for bool {
  #[inline(always)]
  fn from(variant: TPRW_A) -> Self {
    match variant {
      TPRW_A::TPRW_0 => false,
      TPRW_A::TPRW_1 => true,
    }
  }
}
#[doc = "Reader of field `TPRW`"]
pub type TPRW_R = crate::R<bool, TPRW_A>;
impl TPRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPRW_A {
    match self.bits {
      false => TPRW_A::TPRW_0,
      true => TPRW_A::TPRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPRW_0`"]
  #[inline(always)]
  pub fn is_tprw_0(&self) -> bool {
    *self == TPRW_A::TPRW_0
  }
  #[doc = "Checks if the value of the field is `TPRW_1`"]
  #[inline(always)]
  pub fn is_tprw_1(&self) -> bool {
    *self == TPRW_A::TPRW_1
  }
}
#[doc = "Write proxy for field `TPRW`"]
pub struct TPRW_W<'a> {
  w: &'a mut W,
}
impl<'a> TPRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Time Prescaler Register are ignored."]
  #[inline(always)]
  pub fn tprw_0(self) -> &'a mut W {
    self.variant(TPRW_A::TPRW_0)
  }
  #[doc = "Writes to the Time Prescaler Register complete as normal."]
  #[inline(always)]
  pub fn tprw_1(self) -> &'a mut W {
    self.variant(TPRW_A::TPRW_1)
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
#[doc = "Time Alarm Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARW_A {
  #[doc = "0: Writes to the Time Alarm Register are ignored."]
  TARW_0,
  #[doc = "1: Writes to the Time Alarm Register complete as normal."]
  TARW_1,
}
impl From<TARW_A> for bool {
  #[inline(always)]
  fn from(variant: TARW_A) -> Self {
    match variant {
      TARW_A::TARW_0 => false,
      TARW_A::TARW_1 => true,
    }
  }
}
#[doc = "Reader of field `TARW`"]
pub type TARW_R = crate::R<bool, TARW_A>;
impl TARW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TARW_A {
    match self.bits {
      false => TARW_A::TARW_0,
      true => TARW_A::TARW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TARW_0`"]
  #[inline(always)]
  pub fn is_tarw_0(&self) -> bool {
    *self == TARW_A::TARW_0
  }
  #[doc = "Checks if the value of the field is `TARW_1`"]
  #[inline(always)]
  pub fn is_tarw_1(&self) -> bool {
    *self == TARW_A::TARW_1
  }
}
#[doc = "Write proxy for field `TARW`"]
pub struct TARW_W<'a> {
  w: &'a mut W,
}
impl<'a> TARW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TARW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Time Alarm Register are ignored."]
  #[inline(always)]
  pub fn tarw_0(self) -> &'a mut W {
    self.variant(TARW_A::TARW_0)
  }
  #[doc = "Writes to the Time Alarm Register complete as normal."]
  #[inline(always)]
  pub fn tarw_1(self) -> &'a mut W {
    self.variant(TARW_A::TARW_1)
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
#[doc = "Time Compensation Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRW_A {
  #[doc = "0: Writes to the Time Compensation Register are ignored."]
  TCRW_0,
  #[doc = "1: Writes to the Time Compensation Register complete as normal."]
  TCRW_1,
}
impl From<TCRW_A> for bool {
  #[inline(always)]
  fn from(variant: TCRW_A) -> Self {
    match variant {
      TCRW_A::TCRW_0 => false,
      TCRW_A::TCRW_1 => true,
    }
  }
}
#[doc = "Reader of field `TCRW`"]
pub type TCRW_R = crate::R<bool, TCRW_A>;
impl TCRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCRW_A {
    match self.bits {
      false => TCRW_A::TCRW_0,
      true => TCRW_A::TCRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCRW_0`"]
  #[inline(always)]
  pub fn is_tcrw_0(&self) -> bool {
    *self == TCRW_A::TCRW_0
  }
  #[doc = "Checks if the value of the field is `TCRW_1`"]
  #[inline(always)]
  pub fn is_tcrw_1(&self) -> bool {
    *self == TCRW_A::TCRW_1
  }
}
#[doc = "Write proxy for field `TCRW`"]
pub struct TCRW_W<'a> {
  w: &'a mut W,
}
impl<'a> TCRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Time Compensation Register are ignored."]
  #[inline(always)]
  pub fn tcrw_0(self) -> &'a mut W {
    self.variant(TCRW_A::TCRW_0)
  }
  #[doc = "Writes to the Time Compensation Register complete as normal."]
  #[inline(always)]
  pub fn tcrw_1(self) -> &'a mut W {
    self.variant(TCRW_A::TCRW_1)
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
#[doc = "Control Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRW_A {
  #[doc = "0: Writes to the Control Register are ignored."]
  CRW_0,
  #[doc = "1: Writes to the Control Register complete as normal."]
  CRW_1,
}
impl From<CRW_A> for bool {
  #[inline(always)]
  fn from(variant: CRW_A) -> Self {
    match variant {
      CRW_A::CRW_0 => false,
      CRW_A::CRW_1 => true,
    }
  }
}
#[doc = "Reader of field `CRW`"]
pub type CRW_R = crate::R<bool, CRW_A>;
impl CRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CRW_A {
    match self.bits {
      false => CRW_A::CRW_0,
      true => CRW_A::CRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `CRW_0`"]
  #[inline(always)]
  pub fn is_crw_0(&self) -> bool {
    *self == CRW_A::CRW_0
  }
  #[doc = "Checks if the value of the field is `CRW_1`"]
  #[inline(always)]
  pub fn is_crw_1(&self) -> bool {
    *self == CRW_A::CRW_1
  }
}
#[doc = "Write proxy for field `CRW`"]
pub struct CRW_W<'a> {
  w: &'a mut W,
}
impl<'a> CRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Control Register are ignored."]
  #[inline(always)]
  pub fn crw_0(self) -> &'a mut W {
    self.variant(CRW_A::CRW_0)
  }
  #[doc = "Writes to the Control Register complete as normal."]
  #[inline(always)]
  pub fn crw_1(self) -> &'a mut W {
    self.variant(CRW_A::CRW_1)
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
#[doc = "Status Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRW_A {
  #[doc = "0: Writes to the Status Register are ignored."]
  SRW_0,
  #[doc = "1: Writes to the Status Register complete as normal."]
  SRW_1,
}
impl From<SRW_A> for bool {
  #[inline(always)]
  fn from(variant: SRW_A) -> Self {
    match variant {
      SRW_A::SRW_0 => false,
      SRW_A::SRW_1 => true,
    }
  }
}
#[doc = "Reader of field `SRW`"]
pub type SRW_R = crate::R<bool, SRW_A>;
impl SRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SRW_A {
    match self.bits {
      false => SRW_A::SRW_0,
      true => SRW_A::SRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `SRW_0`"]
  #[inline(always)]
  pub fn is_srw_0(&self) -> bool {
    *self == SRW_A::SRW_0
  }
  #[doc = "Checks if the value of the field is `SRW_1`"]
  #[inline(always)]
  pub fn is_srw_1(&self) -> bool {
    *self == SRW_A::SRW_1
  }
}
#[doc = "Write proxy for field `SRW`"]
pub struct SRW_W<'a> {
  w: &'a mut W,
}
impl<'a> SRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Status Register are ignored."]
  #[inline(always)]
  pub fn srw_0(self) -> &'a mut W {
    self.variant(SRW_A::SRW_0)
  }
  #[doc = "Writes to the Status Register complete as normal."]
  #[inline(always)]
  pub fn srw_1(self) -> &'a mut W {
    self.variant(SRW_A::SRW_1)
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
#[doc = "Lock Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRW_A {
  #[doc = "0: Writes to the Lock Register are ignored."]
  LRW_0,
  #[doc = "1: Writes to the Lock Register complete as normal."]
  LRW_1,
}
impl From<LRW_A> for bool {
  #[inline(always)]
  fn from(variant: LRW_A) -> Self {
    match variant {
      LRW_A::LRW_0 => false,
      LRW_A::LRW_1 => true,
    }
  }
}
#[doc = "Reader of field `LRW`"]
pub type LRW_R = crate::R<bool, LRW_A>;
impl LRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LRW_A {
    match self.bits {
      false => LRW_A::LRW_0,
      true => LRW_A::LRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `LRW_0`"]
  #[inline(always)]
  pub fn is_lrw_0(&self) -> bool {
    *self == LRW_A::LRW_0
  }
  #[doc = "Checks if the value of the field is `LRW_1`"]
  #[inline(always)]
  pub fn is_lrw_1(&self) -> bool {
    *self == LRW_A::LRW_1
  }
}
#[doc = "Write proxy for field `LRW`"]
pub struct LRW_W<'a> {
  w: &'a mut W,
}
impl<'a> LRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Lock Register are ignored."]
  #[inline(always)]
  pub fn lrw_0(self) -> &'a mut W {
    self.variant(LRW_A::LRW_0)
  }
  #[doc = "Writes to the Lock Register complete as normal."]
  #[inline(always)]
  pub fn lrw_1(self) -> &'a mut W {
    self.variant(LRW_A::LRW_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "Interrupt Enable Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERW_A {
  #[doc = "0: Writes to the Interupt Enable Register are ignored."]
  IERW_0,
  #[doc = "1: Writes to the Interrupt Enable Register complete as normal."]
  IERW_1,
}
impl From<IERW_A> for bool {
  #[inline(always)]
  fn from(variant: IERW_A) -> Self {
    match variant {
      IERW_A::IERW_0 => false,
      IERW_A::IERW_1 => true,
    }
  }
}
#[doc = "Reader of field `IERW`"]
pub type IERW_R = crate::R<bool, IERW_A>;
impl IERW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IERW_A {
    match self.bits {
      false => IERW_A::IERW_0,
      true => IERW_A::IERW_1,
    }
  }
  #[doc = "Checks if the value of the field is `IERW_0`"]
  #[inline(always)]
  pub fn is_ierw_0(&self) -> bool {
    *self == IERW_A::IERW_0
  }
  #[doc = "Checks if the value of the field is `IERW_1`"]
  #[inline(always)]
  pub fn is_ierw_1(&self) -> bool {
    *self == IERW_A::IERW_1
  }
}
#[doc = "Write proxy for field `IERW`"]
pub struct IERW_W<'a> {
  w: &'a mut W,
}
impl<'a> IERW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IERW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Interupt Enable Register are ignored."]
  #[inline(always)]
  pub fn ierw_0(self) -> &'a mut W {
    self.variant(IERW_A::IERW_0)
  }
  #[doc = "Writes to the Interrupt Enable Register complete as normal."]
  #[inline(always)]
  pub fn ierw_1(self) -> &'a mut W {
    self.variant(IERW_A::IERW_1)
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
#[doc = "Tamper Time Seconds Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTSW_A {
  #[doc = "0: Writes to the Tamper Time Seconds Register are ignored."]
  TTSW_0,
  #[doc = "1: Writes to the Tamper Time Seconds Register complete as normal."]
  TTSW_1,
}
impl From<TTSW_A> for bool {
  #[inline(always)]
  fn from(variant: TTSW_A) -> Self {
    match variant {
      TTSW_A::TTSW_0 => false,
      TTSW_A::TTSW_1 => true,
    }
  }
}
#[doc = "Reader of field `TTSW`"]
pub type TTSW_R = crate::R<bool, TTSW_A>;
impl TTSW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TTSW_A {
    match self.bits {
      false => TTSW_A::TTSW_0,
      true => TTSW_A::TTSW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TTSW_0`"]
  #[inline(always)]
  pub fn is_ttsw_0(&self) -> bool {
    *self == TTSW_A::TTSW_0
  }
  #[doc = "Checks if the value of the field is `TTSW_1`"]
  #[inline(always)]
  pub fn is_ttsw_1(&self) -> bool {
    *self == TTSW_A::TTSW_1
  }
}
#[doc = "Write proxy for field `TTSW`"]
pub struct TTSW_W<'a> {
  w: &'a mut W,
}
impl<'a> TTSW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TTSW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Tamper Time Seconds Register are ignored."]
  #[inline(always)]
  pub fn ttsw_0(self) -> &'a mut W {
    self.variant(TTSW_A::TTSW_0)
  }
  #[doc = "Writes to the Tamper Time Seconds Register complete as normal."]
  #[inline(always)]
  pub fn ttsw_1(self) -> &'a mut W {
    self.variant(TTSW_A::TTSW_1)
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
#[doc = "Monotonic Enable Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MERW_A {
  #[doc = "0: Writes to the Monotonic Enable Register are ignored."]
  MERW_0,
  #[doc = "1: Writes to the Monotonic Enable Register complete as normal."]
  MERW_1,
}
impl From<MERW_A> for bool {
  #[inline(always)]
  fn from(variant: MERW_A) -> Self {
    match variant {
      MERW_A::MERW_0 => false,
      MERW_A::MERW_1 => true,
    }
  }
}
#[doc = "Reader of field `MERW`"]
pub type MERW_R = crate::R<bool, MERW_A>;
impl MERW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MERW_A {
    match self.bits {
      false => MERW_A::MERW_0,
      true => MERW_A::MERW_1,
    }
  }
  #[doc = "Checks if the value of the field is `MERW_0`"]
  #[inline(always)]
  pub fn is_merw_0(&self) -> bool {
    *self == MERW_A::MERW_0
  }
  #[doc = "Checks if the value of the field is `MERW_1`"]
  #[inline(always)]
  pub fn is_merw_1(&self) -> bool {
    *self == MERW_A::MERW_1
  }
}
#[doc = "Write proxy for field `MERW`"]
pub struct MERW_W<'a> {
  w: &'a mut W,
}
impl<'a> MERW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MERW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Monotonic Enable Register are ignored."]
  #[inline(always)]
  pub fn merw_0(self) -> &'a mut W {
    self.variant(MERW_A::MERW_0)
  }
  #[doc = "Writes to the Monotonic Enable Register complete as normal."]
  #[inline(always)]
  pub fn merw_1(self) -> &'a mut W {
    self.variant(MERW_A::MERW_1)
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
#[doc = "Monotonic Counter Low Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLW_A {
  #[doc = "0: Writes to the Monotonic Counter Low Register are ignored."]
  MCLW_0,
  #[doc = "1: Writes to the Monotonic Counter Low Register complete as normal."]
  MCLW_1,
}
impl From<MCLW_A> for bool {
  #[inline(always)]
  fn from(variant: MCLW_A) -> Self {
    match variant {
      MCLW_A::MCLW_0 => false,
      MCLW_A::MCLW_1 => true,
    }
  }
}
#[doc = "Reader of field `MCLW`"]
pub type MCLW_R = crate::R<bool, MCLW_A>;
impl MCLW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MCLW_A {
    match self.bits {
      false => MCLW_A::MCLW_0,
      true => MCLW_A::MCLW_1,
    }
  }
  #[doc = "Checks if the value of the field is `MCLW_0`"]
  #[inline(always)]
  pub fn is_mclw_0(&self) -> bool {
    *self == MCLW_A::MCLW_0
  }
  #[doc = "Checks if the value of the field is `MCLW_1`"]
  #[inline(always)]
  pub fn is_mclw_1(&self) -> bool {
    *self == MCLW_A::MCLW_1
  }
}
#[doc = "Write proxy for field `MCLW`"]
pub struct MCLW_W<'a> {
  w: &'a mut W,
}
impl<'a> MCLW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MCLW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Monotonic Counter Low Register are ignored."]
  #[inline(always)]
  pub fn mclw_0(self) -> &'a mut W {
    self.variant(MCLW_A::MCLW_0)
  }
  #[doc = "Writes to the Monotonic Counter Low Register complete as normal."]
  #[inline(always)]
  pub fn mclw_1(self) -> &'a mut W {
    self.variant(MCLW_A::MCLW_1)
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
#[doc = "Monotonic Counter High Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCHW_A {
  #[doc = "0: Writes to the Monotonic Counter High Register are ignored."]
  MCHW_0,
  #[doc = "1: Writes to the Monotonic Counter High Register complete as normal."]
  MCHW_1,
}
impl From<MCHW_A> for bool {
  #[inline(always)]
  fn from(variant: MCHW_A) -> Self {
    match variant {
      MCHW_A::MCHW_0 => false,
      MCHW_A::MCHW_1 => true,
    }
  }
}
#[doc = "Reader of field `MCHW`"]
pub type MCHW_R = crate::R<bool, MCHW_A>;
impl MCHW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MCHW_A {
    match self.bits {
      false => MCHW_A::MCHW_0,
      true => MCHW_A::MCHW_1,
    }
  }
  #[doc = "Checks if the value of the field is `MCHW_0`"]
  #[inline(always)]
  pub fn is_mchw_0(&self) -> bool {
    *self == MCHW_A::MCHW_0
  }
  #[doc = "Checks if the value of the field is `MCHW_1`"]
  #[inline(always)]
  pub fn is_mchw_1(&self) -> bool {
    *self == MCHW_A::MCHW_1
  }
}
#[doc = "Write proxy for field `MCHW`"]
pub struct MCHW_W<'a> {
  w: &'a mut W,
}
impl<'a> MCHW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MCHW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Monotonic Counter High Register are ignored."]
  #[inline(always)]
  pub fn mchw_0(self) -> &'a mut W {
    self.variant(MCHW_A::MCHW_0)
  }
  #[doc = "Writes to the Monotonic Counter High Register complete as normal."]
  #[inline(always)]
  pub fn mchw_1(self) -> &'a mut W {
    self.variant(MCHW_A::MCHW_1)
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
#[doc = "Tamper Detect Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRW_A {
  #[doc = "0: Writes to the Tamper Detect Register are ignored."]
  TDRW_0,
  #[doc = "1: Writes to the Tamper Detect Register complete as normal."]
  TDRW_1,
}
impl From<TDRW_A> for bool {
  #[inline(always)]
  fn from(variant: TDRW_A) -> Self {
    match variant {
      TDRW_A::TDRW_0 => false,
      TDRW_A::TDRW_1 => true,
    }
  }
}
#[doc = "Reader of field `TDRW`"]
pub type TDRW_R = crate::R<bool, TDRW_A>;
impl TDRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TDRW_A {
    match self.bits {
      false => TDRW_A::TDRW_0,
      true => TDRW_A::TDRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TDRW_0`"]
  #[inline(always)]
  pub fn is_tdrw_0(&self) -> bool {
    *self == TDRW_A::TDRW_0
  }
  #[doc = "Checks if the value of the field is `TDRW_1`"]
  #[inline(always)]
  pub fn is_tdrw_1(&self) -> bool {
    *self == TDRW_A::TDRW_1
  }
}
#[doc = "Write proxy for field `TDRW`"]
pub struct TDRW_W<'a> {
  w: &'a mut W,
}
impl<'a> TDRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TDRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Tamper Detect Register are ignored."]
  #[inline(always)]
  pub fn tdrw_0(self) -> &'a mut W {
    self.variant(TDRW_A::TDRW_0)
  }
  #[doc = "Writes to the Tamper Detect Register complete as normal."]
  #[inline(always)]
  pub fn tdrw_1(self) -> &'a mut W {
    self.variant(TDRW_A::TDRW_1)
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
#[doc = "Tamper Interrupt Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIRW_A {
  #[doc = "0: Writes to the Tamper Interrupt Register are ignored."]
  TIRW_0,
  #[doc = "1: Writes to the Tamper Interrupt Register complete as normal."]
  TIRW_1,
}
impl From<TIRW_A> for bool {
  #[inline(always)]
  fn from(variant: TIRW_A) -> Self {
    match variant {
      TIRW_A::TIRW_0 => false,
      TIRW_A::TIRW_1 => true,
    }
  }
}
#[doc = "Reader of field `TIRW`"]
pub type TIRW_R = crate::R<bool, TIRW_A>;
impl TIRW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TIRW_A {
    match self.bits {
      false => TIRW_A::TIRW_0,
      true => TIRW_A::TIRW_1,
    }
  }
  #[doc = "Checks if the value of the field is `TIRW_0`"]
  #[inline(always)]
  pub fn is_tirw_0(&self) -> bool {
    *self == TIRW_A::TIRW_0
  }
  #[doc = "Checks if the value of the field is `TIRW_1`"]
  #[inline(always)]
  pub fn is_tirw_1(&self) -> bool {
    *self == TIRW_A::TIRW_1
  }
}
#[doc = "Write proxy for field `TIRW`"]
pub struct TIRW_W<'a> {
  w: &'a mut W,
}
impl<'a> TIRW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TIRW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the Tamper Interrupt Register are ignored."]
  #[inline(always)]
  pub fn tirw_0(self) -> &'a mut W {
    self.variant(TIRW_A::TIRW_0)
  }
  #[doc = "Writes to the Tamper Interrupt Register complete as normal."]
  #[inline(always)]
  pub fn tirw_1(self) -> &'a mut W {
    self.variant(TIRW_A::TIRW_1)
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
#[doc = "Reader of field `PCRW`"]
pub type PCRW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCRW`"]
pub struct PCRW_W<'a> {
  w: &'a mut W,
}
impl<'a> PCRW_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Time Seconds Register Write"]
  #[inline(always)]
  pub fn tsrw(&self) -> TSRW_R {
    TSRW_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Time Prescaler Register Write"]
  #[inline(always)]
  pub fn tprw(&self) -> TPRW_R {
    TPRW_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Time Alarm Register Write"]
  #[inline(always)]
  pub fn tarw(&self) -> TARW_R {
    TARW_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Time Compensation Register Write"]
  #[inline(always)]
  pub fn tcrw(&self) -> TCRW_R {
    TCRW_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Control Register Write"]
  #[inline(always)]
  pub fn crw(&self) -> CRW_R {
    CRW_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Status Register Write"]
  #[inline(always)]
  pub fn srw(&self) -> SRW_R {
    SRW_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Lock Register Write"]
  #[inline(always)]
  pub fn lrw(&self) -> LRW_R {
    LRW_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Interrupt Enable Register Write"]
  #[inline(always)]
  pub fn ierw(&self) -> IERW_R {
    IERW_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Tamper Time Seconds Write"]
  #[inline(always)]
  pub fn ttsw(&self) -> TTSW_R {
    TTSW_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Monotonic Enable Register Write"]
  #[inline(always)]
  pub fn merw(&self) -> MERW_R {
    MERW_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Monotonic Counter Low Write"]
  #[inline(always)]
  pub fn mclw(&self) -> MCLW_R {
    MCLW_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Monotonic Counter High Write"]
  #[inline(always)]
  pub fn mchw(&self) -> MCHW_R {
    MCHW_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Tamper Detect Register Write"]
  #[inline(always)]
  pub fn tdrw(&self) -> TDRW_R {
    TDRW_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Tamper Interrupt Register Write"]
  #[inline(always)]
  pub fn tirw(&self) -> TIRW_R {
    TIRW_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - Pin Configuration Register Write"]
  #[inline(always)]
  pub fn pcrw(&self) -> PCRW_R {
    PCRW_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Time Seconds Register Write"]
  #[inline(always)]
  pub fn tsrw(&mut self) -> TSRW_W {
    TSRW_W { w: self }
  }
  #[doc = "Bit 1 - Time Prescaler Register Write"]
  #[inline(always)]
  pub fn tprw(&mut self) -> TPRW_W {
    TPRW_W { w: self }
  }
  #[doc = "Bit 2 - Time Alarm Register Write"]
  #[inline(always)]
  pub fn tarw(&mut self) -> TARW_W {
    TARW_W { w: self }
  }
  #[doc = "Bit 3 - Time Compensation Register Write"]
  #[inline(always)]
  pub fn tcrw(&mut self) -> TCRW_W {
    TCRW_W { w: self }
  }
  #[doc = "Bit 4 - Control Register Write"]
  #[inline(always)]
  pub fn crw(&mut self) -> CRW_W {
    CRW_W { w: self }
  }
  #[doc = "Bit 5 - Status Register Write"]
  #[inline(always)]
  pub fn srw(&mut self) -> SRW_W {
    SRW_W { w: self }
  }
  #[doc = "Bit 6 - Lock Register Write"]
  #[inline(always)]
  pub fn lrw(&mut self) -> LRW_W {
    LRW_W { w: self }
  }
  #[doc = "Bit 7 - Interrupt Enable Register Write"]
  #[inline(always)]
  pub fn ierw(&mut self) -> IERW_W {
    IERW_W { w: self }
  }
  #[doc = "Bit 8 - Tamper Time Seconds Write"]
  #[inline(always)]
  pub fn ttsw(&mut self) -> TTSW_W {
    TTSW_W { w: self }
  }
  #[doc = "Bit 9 - Monotonic Enable Register Write"]
  #[inline(always)]
  pub fn merw(&mut self) -> MERW_W {
    MERW_W { w: self }
  }
  #[doc = "Bit 10 - Monotonic Counter Low Write"]
  #[inline(always)]
  pub fn mclw(&mut self) -> MCLW_W {
    MCLW_W { w: self }
  }
  #[doc = "Bit 11 - Monotonic Counter High Write"]
  #[inline(always)]
  pub fn mchw(&mut self) -> MCHW_W {
    MCHW_W { w: self }
  }
  #[doc = "Bit 13 - Tamper Detect Register Write"]
  #[inline(always)]
  pub fn tdrw(&mut self) -> TDRW_W {
    TDRW_W { w: self }
  }
  #[doc = "Bit 15 - Tamper Interrupt Register Write"]
  #[inline(always)]
  pub fn tirw(&mut self) -> TIRW_W {
    TIRW_W { w: self }
  }
  #[doc = "Bits 16:19 - Pin Configuration Register Write"]
  #[inline(always)]
  pub fn pcrw(&mut self) -> PCRW_W {
    PCRW_W { w: self }
  }
}
