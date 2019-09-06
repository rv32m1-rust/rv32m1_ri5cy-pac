#[doc = "Reader of register RAR"]
pub type R = crate::R<u32, super::RAR>;
#[doc = "Writer for register RAR"]
pub type W = crate::W<u32, super::RAR>;
#[doc = "Register RAR `reset()`'s with value 0x000f_ffff"]
impl crate::ResetValue for super::RAR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x000f_ffff
  }
}
#[doc = "Time Seconds Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRR_A {
  #[doc = "0: Reads to the Time Seconds Register are ignored."]
  TSRR_0,
  #[doc = "1: Reads to the Time Seconds Register complete as normal."]
  TSRR_1,
}
impl From<TSRR_A> for bool {
  #[inline(always)]
  fn from(variant: TSRR_A) -> Self {
    match variant {
      TSRR_A::TSRR_0 => false,
      TSRR_A::TSRR_1 => true,
    }
  }
}
#[doc = "Reader of field `TSRR`"]
pub type TSRR_R = crate::R<bool, TSRR_A>;
impl TSRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TSRR_A {
    match self.bits {
      false => TSRR_A::TSRR_0,
      true => TSRR_A::TSRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TSRR_0`"]
  #[inline(always)]
  pub fn is_tsrr_0(&self) -> bool {
    *self == TSRR_A::TSRR_0
  }
  #[doc = "Checks if the value of the field is `TSRR_1`"]
  #[inline(always)]
  pub fn is_tsrr_1(&self) -> bool {
    *self == TSRR_A::TSRR_1
  }
}
#[doc = "Write proxy for field `TSRR`"]
pub struct TSRR_W<'a> {
  w: &'a mut W,
}
impl<'a> TSRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TSRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Time Seconds Register are ignored."]
  #[inline(always)]
  pub fn tsrr_0(self) -> &'a mut W {
    self.variant(TSRR_A::TSRR_0)
  }
  #[doc = "Reads to the Time Seconds Register complete as normal."]
  #[inline(always)]
  pub fn tsrr_1(self) -> &'a mut W {
    self.variant(TSRR_A::TSRR_1)
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
#[doc = "Time Prescaler Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRR_A {
  #[doc = "0: Reads to the Time Pprescaler Register are ignored."]
  TPRR_0,
  #[doc = "1: Reads to the Time Prescaler Register complete as normal."]
  TPRR_1,
}
impl From<TPRR_A> for bool {
  #[inline(always)]
  fn from(variant: TPRR_A) -> Self {
    match variant {
      TPRR_A::TPRR_0 => false,
      TPRR_A::TPRR_1 => true,
    }
  }
}
#[doc = "Reader of field `TPRR`"]
pub type TPRR_R = crate::R<bool, TPRR_A>;
impl TPRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPRR_A {
    match self.bits {
      false => TPRR_A::TPRR_0,
      true => TPRR_A::TPRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPRR_0`"]
  #[inline(always)]
  pub fn is_tprr_0(&self) -> bool {
    *self == TPRR_A::TPRR_0
  }
  #[doc = "Checks if the value of the field is `TPRR_1`"]
  #[inline(always)]
  pub fn is_tprr_1(&self) -> bool {
    *self == TPRR_A::TPRR_1
  }
}
#[doc = "Write proxy for field `TPRR`"]
pub struct TPRR_W<'a> {
  w: &'a mut W,
}
impl<'a> TPRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Time Pprescaler Register are ignored."]
  #[inline(always)]
  pub fn tprr_0(self) -> &'a mut W {
    self.variant(TPRR_A::TPRR_0)
  }
  #[doc = "Reads to the Time Prescaler Register complete as normal."]
  #[inline(always)]
  pub fn tprr_1(self) -> &'a mut W {
    self.variant(TPRR_A::TPRR_1)
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
#[doc = "Time Alarm Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARR_A {
  #[doc = "0: Reads to the Time Alarm Register are ignored."]
  TARR_0,
  #[doc = "1: Reads to the Time Alarm Register complete as normal."]
  TARR_1,
}
impl From<TARR_A> for bool {
  #[inline(always)]
  fn from(variant: TARR_A) -> Self {
    match variant {
      TARR_A::TARR_0 => false,
      TARR_A::TARR_1 => true,
    }
  }
}
#[doc = "Reader of field `TARR`"]
pub type TARR_R = crate::R<bool, TARR_A>;
impl TARR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TARR_A {
    match self.bits {
      false => TARR_A::TARR_0,
      true => TARR_A::TARR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TARR_0`"]
  #[inline(always)]
  pub fn is_tarr_0(&self) -> bool {
    *self == TARR_A::TARR_0
  }
  #[doc = "Checks if the value of the field is `TARR_1`"]
  #[inline(always)]
  pub fn is_tarr_1(&self) -> bool {
    *self == TARR_A::TARR_1
  }
}
#[doc = "Write proxy for field `TARR`"]
pub struct TARR_W<'a> {
  w: &'a mut W,
}
impl<'a> TARR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TARR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Time Alarm Register are ignored."]
  #[inline(always)]
  pub fn tarr_0(self) -> &'a mut W {
    self.variant(TARR_A::TARR_0)
  }
  #[doc = "Reads to the Time Alarm Register complete as normal."]
  #[inline(always)]
  pub fn tarr_1(self) -> &'a mut W {
    self.variant(TARR_A::TARR_1)
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
#[doc = "Time Compensation Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRR_A {
  #[doc = "0: Reads to the Time Compensation Register are ignored."]
  TCRR_0,
  #[doc = "1: Reads to the Time Compensation Register complete as normal."]
  TCRR_1,
}
impl From<TCRR_A> for bool {
  #[inline(always)]
  fn from(variant: TCRR_A) -> Self {
    match variant {
      TCRR_A::TCRR_0 => false,
      TCRR_A::TCRR_1 => true,
    }
  }
}
#[doc = "Reader of field `TCRR`"]
pub type TCRR_R = crate::R<bool, TCRR_A>;
impl TCRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCRR_A {
    match self.bits {
      false => TCRR_A::TCRR_0,
      true => TCRR_A::TCRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCRR_0`"]
  #[inline(always)]
  pub fn is_tcrr_0(&self) -> bool {
    *self == TCRR_A::TCRR_0
  }
  #[doc = "Checks if the value of the field is `TCRR_1`"]
  #[inline(always)]
  pub fn is_tcrr_1(&self) -> bool {
    *self == TCRR_A::TCRR_1
  }
}
#[doc = "Write proxy for field `TCRR`"]
pub struct TCRR_W<'a> {
  w: &'a mut W,
}
impl<'a> TCRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Time Compensation Register are ignored."]
  #[inline(always)]
  pub fn tcrr_0(self) -> &'a mut W {
    self.variant(TCRR_A::TCRR_0)
  }
  #[doc = "Reads to the Time Compensation Register complete as normal."]
  #[inline(always)]
  pub fn tcrr_1(self) -> &'a mut W {
    self.variant(TCRR_A::TCRR_1)
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
#[doc = "Control Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR_A {
  #[doc = "0: Reads to the Control Register are ignored."]
  CRR_0,
  #[doc = "1: Reads to the Control Register complete as normal."]
  CRR_1,
}
impl From<CRR_A> for bool {
  #[inline(always)]
  fn from(variant: CRR_A) -> Self {
    match variant {
      CRR_A::CRR_0 => false,
      CRR_A::CRR_1 => true,
    }
  }
}
#[doc = "Reader of field `CRR`"]
pub type CRR_R = crate::R<bool, CRR_A>;
impl CRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CRR_A {
    match self.bits {
      false => CRR_A::CRR_0,
      true => CRR_A::CRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `CRR_0`"]
  #[inline(always)]
  pub fn is_crr_0(&self) -> bool {
    *self == CRR_A::CRR_0
  }
  #[doc = "Checks if the value of the field is `CRR_1`"]
  #[inline(always)]
  pub fn is_crr_1(&self) -> bool {
    *self == CRR_A::CRR_1
  }
}
#[doc = "Write proxy for field `CRR`"]
pub struct CRR_W<'a> {
  w: &'a mut W,
}
impl<'a> CRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Control Register are ignored."]
  #[inline(always)]
  pub fn crr_0(self) -> &'a mut W {
    self.variant(CRR_A::CRR_0)
  }
  #[doc = "Reads to the Control Register complete as normal."]
  #[inline(always)]
  pub fn crr_1(self) -> &'a mut W {
    self.variant(CRR_A::CRR_1)
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
#[doc = "Status Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR_A {
  #[doc = "0: Reads to the Status Register are ignored."]
  SRR_0,
  #[doc = "1: Reads to the Status Register complete as normal."]
  SRR_1,
}
impl From<SRR_A> for bool {
  #[inline(always)]
  fn from(variant: SRR_A) -> Self {
    match variant {
      SRR_A::SRR_0 => false,
      SRR_A::SRR_1 => true,
    }
  }
}
#[doc = "Reader of field `SRR`"]
pub type SRR_R = crate::R<bool, SRR_A>;
impl SRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SRR_A {
    match self.bits {
      false => SRR_A::SRR_0,
      true => SRR_A::SRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `SRR_0`"]
  #[inline(always)]
  pub fn is_srr_0(&self) -> bool {
    *self == SRR_A::SRR_0
  }
  #[doc = "Checks if the value of the field is `SRR_1`"]
  #[inline(always)]
  pub fn is_srr_1(&self) -> bool {
    *self == SRR_A::SRR_1
  }
}
#[doc = "Write proxy for field `SRR`"]
pub struct SRR_W<'a> {
  w: &'a mut W,
}
impl<'a> SRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Status Register are ignored."]
  #[inline(always)]
  pub fn srr_0(self) -> &'a mut W {
    self.variant(SRR_A::SRR_0)
  }
  #[doc = "Reads to the Status Register complete as normal."]
  #[inline(always)]
  pub fn srr_1(self) -> &'a mut W {
    self.variant(SRR_A::SRR_1)
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
#[doc = "Lock Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRR_A {
  #[doc = "0: Reads to the Lock Register are ignored."]
  LRR_0,
  #[doc = "1: Reads to the Lock Register complete as normal."]
  LRR_1,
}
impl From<LRR_A> for bool {
  #[inline(always)]
  fn from(variant: LRR_A) -> Self {
    match variant {
      LRR_A::LRR_0 => false,
      LRR_A::LRR_1 => true,
    }
  }
}
#[doc = "Reader of field `LRR`"]
pub type LRR_R = crate::R<bool, LRR_A>;
impl LRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LRR_A {
    match self.bits {
      false => LRR_A::LRR_0,
      true => LRR_A::LRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `LRR_0`"]
  #[inline(always)]
  pub fn is_lrr_0(&self) -> bool {
    *self == LRR_A::LRR_0
  }
  #[doc = "Checks if the value of the field is `LRR_1`"]
  #[inline(always)]
  pub fn is_lrr_1(&self) -> bool {
    *self == LRR_A::LRR_1
  }
}
#[doc = "Write proxy for field `LRR`"]
pub struct LRR_W<'a> {
  w: &'a mut W,
}
impl<'a> LRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Lock Register are ignored."]
  #[inline(always)]
  pub fn lrr_0(self) -> &'a mut W {
    self.variant(LRR_A::LRR_0)
  }
  #[doc = "Reads to the Lock Register complete as normal."]
  #[inline(always)]
  pub fn lrr_1(self) -> &'a mut W {
    self.variant(LRR_A::LRR_1)
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
#[doc = "Interrupt Enable Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERR_A {
  #[doc = "0: Reads to the Interrupt Enable Register are ignored."]
  IERR_0,
  #[doc = "1: Reads to the Interrupt Enable Register complete as normal."]
  IERR_1,
}
impl From<IERR_A> for bool {
  #[inline(always)]
  fn from(variant: IERR_A) -> Self {
    match variant {
      IERR_A::IERR_0 => false,
      IERR_A::IERR_1 => true,
    }
  }
}
#[doc = "Reader of field `IERR`"]
pub type IERR_R = crate::R<bool, IERR_A>;
impl IERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IERR_A {
    match self.bits {
      false => IERR_A::IERR_0,
      true => IERR_A::IERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `IERR_0`"]
  #[inline(always)]
  pub fn is_ierr_0(&self) -> bool {
    *self == IERR_A::IERR_0
  }
  #[doc = "Checks if the value of the field is `IERR_1`"]
  #[inline(always)]
  pub fn is_ierr_1(&self) -> bool {
    *self == IERR_A::IERR_1
  }
}
#[doc = "Write proxy for field `IERR`"]
pub struct IERR_W<'a> {
  w: &'a mut W,
}
impl<'a> IERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Interrupt Enable Register are ignored."]
  #[inline(always)]
  pub fn ierr_0(self) -> &'a mut W {
    self.variant(IERR_A::IERR_0)
  }
  #[doc = "Reads to the Interrupt Enable Register complete as normal."]
  #[inline(always)]
  pub fn ierr_1(self) -> &'a mut W {
    self.variant(IERR_A::IERR_1)
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
#[doc = "Tamper Time Seconds Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTSR_A {
  #[doc = "0: Reads to the Tamper Time Seconds Register are ignored."]
  TTSR_0,
  #[doc = "1: Reads to the Tamper Time Seconds Register complete as normal."]
  TTSR_1,
}
impl From<TTSR_A> for bool {
  #[inline(always)]
  fn from(variant: TTSR_A) -> Self {
    match variant {
      TTSR_A::TTSR_0 => false,
      TTSR_A::TTSR_1 => true,
    }
  }
}
#[doc = "Reader of field `TTSR`"]
pub type TTSR_R = crate::R<bool, TTSR_A>;
impl TTSR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TTSR_A {
    match self.bits {
      false => TTSR_A::TTSR_0,
      true => TTSR_A::TTSR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TTSR_0`"]
  #[inline(always)]
  pub fn is_ttsr_0(&self) -> bool {
    *self == TTSR_A::TTSR_0
  }
  #[doc = "Checks if the value of the field is `TTSR_1`"]
  #[inline(always)]
  pub fn is_ttsr_1(&self) -> bool {
    *self == TTSR_A::TTSR_1
  }
}
#[doc = "Write proxy for field `TTSR`"]
pub struct TTSR_W<'a> {
  w: &'a mut W,
}
impl<'a> TTSR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TTSR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Tamper Time Seconds Register are ignored."]
  #[inline(always)]
  pub fn ttsr_0(self) -> &'a mut W {
    self.variant(TTSR_A::TTSR_0)
  }
  #[doc = "Reads to the Tamper Time Seconds Register complete as normal."]
  #[inline(always)]
  pub fn ttsr_1(self) -> &'a mut W {
    self.variant(TTSR_A::TTSR_1)
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
#[doc = "Monotonic Enable Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MERR_A {
  #[doc = "0: Reads to the Monotonic Enable Register are ignored."]
  MERR_0,
  #[doc = "1: Reads to the Monotonic Enable Register complete as normal."]
  MERR_1,
}
impl From<MERR_A> for bool {
  #[inline(always)]
  fn from(variant: MERR_A) -> Self {
    match variant {
      MERR_A::MERR_0 => false,
      MERR_A::MERR_1 => true,
    }
  }
}
#[doc = "Reader of field `MERR`"]
pub type MERR_R = crate::R<bool, MERR_A>;
impl MERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MERR_A {
    match self.bits {
      false => MERR_A::MERR_0,
      true => MERR_A::MERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `MERR_0`"]
  #[inline(always)]
  pub fn is_merr_0(&self) -> bool {
    *self == MERR_A::MERR_0
  }
  #[doc = "Checks if the value of the field is `MERR_1`"]
  #[inline(always)]
  pub fn is_merr_1(&self) -> bool {
    *self == MERR_A::MERR_1
  }
}
#[doc = "Write proxy for field `MERR`"]
pub struct MERR_W<'a> {
  w: &'a mut W,
}
impl<'a> MERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Monotonic Enable Register are ignored."]
  #[inline(always)]
  pub fn merr_0(self) -> &'a mut W {
    self.variant(MERR_A::MERR_0)
  }
  #[doc = "Reads to the Monotonic Enable Register complete as normal."]
  #[inline(always)]
  pub fn merr_1(self) -> &'a mut W {
    self.variant(MERR_A::MERR_1)
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
#[doc = "Monotonic Counter Low Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLR_A {
  #[doc = "0: Reads to the Monotonic Counter Low Register are ignored."]
  MCLR_0,
  #[doc = "1: Reads to the Monotonic Counter Low Register complete as normal."]
  MCLR_1,
}
impl From<MCLR_A> for bool {
  #[inline(always)]
  fn from(variant: MCLR_A) -> Self {
    match variant {
      MCLR_A::MCLR_0 => false,
      MCLR_A::MCLR_1 => true,
    }
  }
}
#[doc = "Reader of field `MCLR`"]
pub type MCLR_R = crate::R<bool, MCLR_A>;
impl MCLR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MCLR_A {
    match self.bits {
      false => MCLR_A::MCLR_0,
      true => MCLR_A::MCLR_1,
    }
  }
  #[doc = "Checks if the value of the field is `MCLR_0`"]
  #[inline(always)]
  pub fn is_mclr_0(&self) -> bool {
    *self == MCLR_A::MCLR_0
  }
  #[doc = "Checks if the value of the field is `MCLR_1`"]
  #[inline(always)]
  pub fn is_mclr_1(&self) -> bool {
    *self == MCLR_A::MCLR_1
  }
}
#[doc = "Write proxy for field `MCLR`"]
pub struct MCLR_W<'a> {
  w: &'a mut W,
}
impl<'a> MCLR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MCLR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Monotonic Counter Low Register are ignored."]
  #[inline(always)]
  pub fn mclr_0(self) -> &'a mut W {
    self.variant(MCLR_A::MCLR_0)
  }
  #[doc = "Reads to the Monotonic Counter Low Register complete as normal."]
  #[inline(always)]
  pub fn mclr_1(self) -> &'a mut W {
    self.variant(MCLR_A::MCLR_1)
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
#[doc = "Monotonic Counter High Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCHR_A {
  #[doc = "0: Reads to the Monotonic Counter High Register are ignored."]
  MCHR_0,
  #[doc = "1: Reads to the Monotonic Counter High Register complete as normal."]
  MCHR_1,
}
impl From<MCHR_A> for bool {
  #[inline(always)]
  fn from(variant: MCHR_A) -> Self {
    match variant {
      MCHR_A::MCHR_0 => false,
      MCHR_A::MCHR_1 => true,
    }
  }
}
#[doc = "Reader of field `MCHR`"]
pub type MCHR_R = crate::R<bool, MCHR_A>;
impl MCHR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MCHR_A {
    match self.bits {
      false => MCHR_A::MCHR_0,
      true => MCHR_A::MCHR_1,
    }
  }
  #[doc = "Checks if the value of the field is `MCHR_0`"]
  #[inline(always)]
  pub fn is_mchr_0(&self) -> bool {
    *self == MCHR_A::MCHR_0
  }
  #[doc = "Checks if the value of the field is `MCHR_1`"]
  #[inline(always)]
  pub fn is_mchr_1(&self) -> bool {
    *self == MCHR_A::MCHR_1
  }
}
#[doc = "Write proxy for field `MCHR`"]
pub struct MCHR_W<'a> {
  w: &'a mut W,
}
impl<'a> MCHR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MCHR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Monotonic Counter High Register are ignored."]
  #[inline(always)]
  pub fn mchr_0(self) -> &'a mut W {
    self.variant(MCHR_A::MCHR_0)
  }
  #[doc = "Reads to the Monotonic Counter High Register complete as normal."]
  #[inline(always)]
  pub fn mchr_1(self) -> &'a mut W {
    self.variant(MCHR_A::MCHR_1)
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
#[doc = "Tamper Detect Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRR_A {
  #[doc = "0: Reads to the Tamper Detect Register are ignored."]
  TDRR_0,
  #[doc = "1: Reads to the Tamper Detect Register complete as normal."]
  TDRR_1,
}
impl From<TDRR_A> for bool {
  #[inline(always)]
  fn from(variant: TDRR_A) -> Self {
    match variant {
      TDRR_A::TDRR_0 => false,
      TDRR_A::TDRR_1 => true,
    }
  }
}
#[doc = "Reader of field `TDRR`"]
pub type TDRR_R = crate::R<bool, TDRR_A>;
impl TDRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TDRR_A {
    match self.bits {
      false => TDRR_A::TDRR_0,
      true => TDRR_A::TDRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TDRR_0`"]
  #[inline(always)]
  pub fn is_tdrr_0(&self) -> bool {
    *self == TDRR_A::TDRR_0
  }
  #[doc = "Checks if the value of the field is `TDRR_1`"]
  #[inline(always)]
  pub fn is_tdrr_1(&self) -> bool {
    *self == TDRR_A::TDRR_1
  }
}
#[doc = "Write proxy for field `TDRR`"]
pub struct TDRR_W<'a> {
  w: &'a mut W,
}
impl<'a> TDRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TDRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Tamper Detect Register are ignored."]
  #[inline(always)]
  pub fn tdrr_0(self) -> &'a mut W {
    self.variant(TDRR_A::TDRR_0)
  }
  #[doc = "Reads to the Tamper Detect Register complete as normal."]
  #[inline(always)]
  pub fn tdrr_1(self) -> &'a mut W {
    self.variant(TDRR_A::TDRR_1)
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
#[doc = "Tamper Interrupt Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIRR_A {
  #[doc = "0: Reads to the Tamper Interrupt Register are ignored."]
  TIRR_0,
  #[doc = "1: Reads to the Tamper Interrupt Register complete as normal."]
  TIRR_1,
}
impl From<TIRR_A> for bool {
  #[inline(always)]
  fn from(variant: TIRR_A) -> Self {
    match variant {
      TIRR_A::TIRR_0 => false,
      TIRR_A::TIRR_1 => true,
    }
  }
}
#[doc = "Reader of field `TIRR`"]
pub type TIRR_R = crate::R<bool, TIRR_A>;
impl TIRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TIRR_A {
    match self.bits {
      false => TIRR_A::TIRR_0,
      true => TIRR_A::TIRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `TIRR_0`"]
  #[inline(always)]
  pub fn is_tirr_0(&self) -> bool {
    *self == TIRR_A::TIRR_0
  }
  #[doc = "Checks if the value of the field is `TIRR_1`"]
  #[inline(always)]
  pub fn is_tirr_1(&self) -> bool {
    *self == TIRR_A::TIRR_1
  }
}
#[doc = "Write proxy for field `TIRR`"]
pub struct TIRR_W<'a> {
  w: &'a mut W,
}
impl<'a> TIRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TIRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reads to the Tamper Interrupt Register are ignored."]
  #[inline(always)]
  pub fn tirr_0(self) -> &'a mut W {
    self.variant(TIRR_A::TIRR_0)
  }
  #[doc = "Reads to the Tamper Interrupt Register complete as normal."]
  #[inline(always)]
  pub fn tirr_1(self) -> &'a mut W {
    self.variant(TIRR_A::TIRR_1)
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
#[doc = "Reader of field `PCRR`"]
pub type PCRR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCRR`"]
pub struct PCRR_W<'a> {
  w: &'a mut W,
}
impl<'a> PCRR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Time Seconds Register Read"]
  #[inline(always)]
  pub fn tsrr(&self) -> TSRR_R {
    TSRR_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Time Prescaler Register Read"]
  #[inline(always)]
  pub fn tprr(&self) -> TPRR_R {
    TPRR_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Time Alarm Register Read"]
  #[inline(always)]
  pub fn tarr(&self) -> TARR_R {
    TARR_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Time Compensation Register Read"]
  #[inline(always)]
  pub fn tcrr(&self) -> TCRR_R {
    TCRR_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Control Register Read"]
  #[inline(always)]
  pub fn crr(&self) -> CRR_R {
    CRR_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Status Register Read"]
  #[inline(always)]
  pub fn srr(&self) -> SRR_R {
    SRR_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Lock Register Read"]
  #[inline(always)]
  pub fn lrr(&self) -> LRR_R {
    LRR_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Interrupt Enable Register Read"]
  #[inline(always)]
  pub fn ierr(&self) -> IERR_R {
    IERR_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Tamper Time Seconds Read"]
  #[inline(always)]
  pub fn ttsr(&self) -> TTSR_R {
    TTSR_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Monotonic Enable Register Read"]
  #[inline(always)]
  pub fn merr(&self) -> MERR_R {
    MERR_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Monotonic Counter Low Read"]
  #[inline(always)]
  pub fn mclr(&self) -> MCLR_R {
    MCLR_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Monotonic Counter High Read"]
  #[inline(always)]
  pub fn mchr(&self) -> MCHR_R {
    MCHR_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Tamper Detect Register Read"]
  #[inline(always)]
  pub fn tdrr(&self) -> TDRR_R {
    TDRR_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Tamper Interrupt Register Read"]
  #[inline(always)]
  pub fn tirr(&self) -> TIRR_R {
    TIRR_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - Pin Configuration Register Read"]
  #[inline(always)]
  pub fn pcrr(&self) -> PCRR_R {
    PCRR_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Time Seconds Register Read"]
  #[inline(always)]
  pub fn tsrr(&mut self) -> TSRR_W {
    TSRR_W { w: self }
  }
  #[doc = "Bit 1 - Time Prescaler Register Read"]
  #[inline(always)]
  pub fn tprr(&mut self) -> TPRR_W {
    TPRR_W { w: self }
  }
  #[doc = "Bit 2 - Time Alarm Register Read"]
  #[inline(always)]
  pub fn tarr(&mut self) -> TARR_W {
    TARR_W { w: self }
  }
  #[doc = "Bit 3 - Time Compensation Register Read"]
  #[inline(always)]
  pub fn tcrr(&mut self) -> TCRR_W {
    TCRR_W { w: self }
  }
  #[doc = "Bit 4 - Control Register Read"]
  #[inline(always)]
  pub fn crr(&mut self) -> CRR_W {
    CRR_W { w: self }
  }
  #[doc = "Bit 5 - Status Register Read"]
  #[inline(always)]
  pub fn srr(&mut self) -> SRR_W {
    SRR_W { w: self }
  }
  #[doc = "Bit 6 - Lock Register Read"]
  #[inline(always)]
  pub fn lrr(&mut self) -> LRR_W {
    LRR_W { w: self }
  }
  #[doc = "Bit 7 - Interrupt Enable Register Read"]
  #[inline(always)]
  pub fn ierr(&mut self) -> IERR_W {
    IERR_W { w: self }
  }
  #[doc = "Bit 8 - Tamper Time Seconds Read"]
  #[inline(always)]
  pub fn ttsr(&mut self) -> TTSR_W {
    TTSR_W { w: self }
  }
  #[doc = "Bit 9 - Monotonic Enable Register Read"]
  #[inline(always)]
  pub fn merr(&mut self) -> MERR_W {
    MERR_W { w: self }
  }
  #[doc = "Bit 10 - Monotonic Counter Low Read"]
  #[inline(always)]
  pub fn mclr(&mut self) -> MCLR_W {
    MCLR_W { w: self }
  }
  #[doc = "Bit 11 - Monotonic Counter High Read"]
  #[inline(always)]
  pub fn mchr(&mut self) -> MCHR_W {
    MCHR_W { w: self }
  }
  #[doc = "Bit 13 - Tamper Detect Register Read"]
  #[inline(always)]
  pub fn tdrr(&mut self) -> TDRR_W {
    TDRR_W { w: self }
  }
  #[doc = "Bit 15 - Tamper Interrupt Register Read"]
  #[inline(always)]
  pub fn tirr(&mut self) -> TIRR_W {
    TIRR_W { w: self }
  }
  #[doc = "Bits 16:19 - Pin Configuration Register Read"]
  #[inline(always)]
  pub fn pcrr(&mut self) -> PCRR_W {
    PCRR_W { w: self }
  }
}
