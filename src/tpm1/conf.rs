#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
  #[doc = "0: no description available"]
  DOZEEN_0,
  #[doc = "1: no description available"]
  DOZEEN_1,
}
impl From<DOZEEN_A> for bool {
  #[inline(always)]
  fn from(variant: DOZEEN_A) -> Self {
    match variant {
      DOZEEN_A::DOZEEN_0 => false,
      DOZEEN_A::DOZEEN_1 => true,
    }
  }
}
#[doc = "Reader of field `DOZEEN`"]
pub type DOZEEN_R = crate::R<bool, DOZEEN_A>;
impl DOZEEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DOZEEN_A {
    match self.bits {
      false => DOZEEN_A::DOZEEN_0,
      true => DOZEEN_A::DOZEEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DOZEEN_0`"]
  #[inline(always)]
  pub fn is_dozeen_0(&self) -> bool {
    *self == DOZEEN_A::DOZEEN_0
  }
  #[doc = "Checks if the value of the field is `DOZEEN_1`"]
  #[inline(always)]
  pub fn is_dozeen_1(&self) -> bool {
    *self == DOZEEN_A::DOZEEN_1
  }
}
#[doc = "Write proxy for field `DOZEEN`"]
pub struct DOZEEN_W<'a> {
  w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn dozeen_0(self) -> &'a mut W {
    self.variant(DOZEEN_A::DOZEEN_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn dozeen_1(self) -> &'a mut W {
    self.variant(DOZEEN_A::DOZEEN_1)
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
#[doc = "Debug Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGMODE_A {
  #[doc = "0: no description available"]
  DBGMODE_0,
  #[doc = "3: no description available"]
  DBGMODE_3,
}
impl From<DBGMODE_A> for u8 {
  #[inline(always)]
  fn from(variant: DBGMODE_A) -> Self {
    match variant {
      DBGMODE_A::DBGMODE_0 => 0,
      DBGMODE_A::DBGMODE_3 => 3,
    }
  }
}
#[doc = "Reader of field `DBGMODE`"]
pub type DBGMODE_R = crate::R<u8, DBGMODE_A>;
impl DBGMODE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DBGMODE_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DBGMODE_A::DBGMODE_0),
      3 => Val(DBGMODE_A::DBGMODE_3),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DBGMODE_0`"]
  #[inline(always)]
  pub fn is_dbgmode_0(&self) -> bool {
    *self == DBGMODE_A::DBGMODE_0
  }
  #[doc = "Checks if the value of the field is `DBGMODE_3`"]
  #[inline(always)]
  pub fn is_dbgmode_3(&self) -> bool {
    *self == DBGMODE_A::DBGMODE_3
  }
}
#[doc = "Write proxy for field `DBGMODE`"]
pub struct DBGMODE_W<'a> {
  w: &'a mut W,
}
impl<'a> DBGMODE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DBGMODE_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn dbgmode_0(self) -> &'a mut W {
    self.variant(DBGMODE_A::DBGMODE_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn dbgmode_3(self) -> &'a mut W {
    self.variant(DBGMODE_A::DBGMODE_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Global Time Base Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBSYNC_A {
  #[doc = "0: Global timebase synchronization disabled."]
  GTBSYNC_0,
  #[doc = "1: Global timebase synchronization enabled."]
  GTBSYNC_1,
}
impl From<GTBSYNC_A> for bool {
  #[inline(always)]
  fn from(variant: GTBSYNC_A) -> Self {
    match variant {
      GTBSYNC_A::GTBSYNC_0 => false,
      GTBSYNC_A::GTBSYNC_1 => true,
    }
  }
}
#[doc = "Reader of field `GTBSYNC`"]
pub type GTBSYNC_R = crate::R<bool, GTBSYNC_A>;
impl GTBSYNC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GTBSYNC_A {
    match self.bits {
      false => GTBSYNC_A::GTBSYNC_0,
      true => GTBSYNC_A::GTBSYNC_1,
    }
  }
  #[doc = "Checks if the value of the field is `GTBSYNC_0`"]
  #[inline(always)]
  pub fn is_gtbsync_0(&self) -> bool {
    *self == GTBSYNC_A::GTBSYNC_0
  }
  #[doc = "Checks if the value of the field is `GTBSYNC_1`"]
  #[inline(always)]
  pub fn is_gtbsync_1(&self) -> bool {
    *self == GTBSYNC_A::GTBSYNC_1
  }
}
#[doc = "Write proxy for field `GTBSYNC`"]
pub struct GTBSYNC_W<'a> {
  w: &'a mut W,
}
impl<'a> GTBSYNC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GTBSYNC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Global timebase synchronization disabled."]
  #[inline(always)]
  pub fn gtbsync_0(self) -> &'a mut W {
    self.variant(GTBSYNC_A::GTBSYNC_0)
  }
  #[doc = "Global timebase synchronization enabled."]
  #[inline(always)]
  pub fn gtbsync_1(self) -> &'a mut W {
    self.variant(GTBSYNC_A::GTBSYNC_1)
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
#[doc = "Global time base enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEEN_A {
  #[doc = "0: no description available"]
  GTBEEN_0,
  #[doc = "1: All channels use an externally generated global timebase as their timebase"]
  GTBEEN_1,
}
impl From<GTBEEN_A> for bool {
  #[inline(always)]
  fn from(variant: GTBEEN_A) -> Self {
    match variant {
      GTBEEN_A::GTBEEN_0 => false,
      GTBEEN_A::GTBEEN_1 => true,
    }
  }
}
#[doc = "Reader of field `GTBEEN`"]
pub type GTBEEN_R = crate::R<bool, GTBEEN_A>;
impl GTBEEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GTBEEN_A {
    match self.bits {
      false => GTBEEN_A::GTBEEN_0,
      true => GTBEEN_A::GTBEEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `GTBEEN_0`"]
  #[inline(always)]
  pub fn is_gtbeen_0(&self) -> bool {
    *self == GTBEEN_A::GTBEEN_0
  }
  #[doc = "Checks if the value of the field is `GTBEEN_1`"]
  #[inline(always)]
  pub fn is_gtbeen_1(&self) -> bool {
    *self == GTBEEN_A::GTBEEN_1
  }
}
#[doc = "Write proxy for field `GTBEEN`"]
pub struct GTBEEN_W<'a> {
  w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GTBEEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn gtbeen_0(self) -> &'a mut W {
    self.variant(GTBEEN_A::GTBEEN_0)
  }
  #[doc = "All channels use an externally generated global timebase as their timebase"]
  #[inline(always)]
  pub fn gtbeen_1(self) -> &'a mut W {
    self.variant(GTBEEN_A::GTBEEN_1)
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
#[doc = "Counter Start on Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOT_A {
  #[doc = "0: no description available"]
  CSOT_0,
  #[doc = "1: no description available"]
  CSOT_1,
}
impl From<CSOT_A> for bool {
  #[inline(always)]
  fn from(variant: CSOT_A) -> Self {
    match variant {
      CSOT_A::CSOT_0 => false,
      CSOT_A::CSOT_1 => true,
    }
  }
}
#[doc = "Reader of field `CSOT`"]
pub type CSOT_R = crate::R<bool, CSOT_A>;
impl CSOT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CSOT_A {
    match self.bits {
      false => CSOT_A::CSOT_0,
      true => CSOT_A::CSOT_1,
    }
  }
  #[doc = "Checks if the value of the field is `CSOT_0`"]
  #[inline(always)]
  pub fn is_csot_0(&self) -> bool {
    *self == CSOT_A::CSOT_0
  }
  #[doc = "Checks if the value of the field is `CSOT_1`"]
  #[inline(always)]
  pub fn is_csot_1(&self) -> bool {
    *self == CSOT_A::CSOT_1
  }
}
#[doc = "Write proxy for field `CSOT`"]
pub struct CSOT_W<'a> {
  w: &'a mut W,
}
impl<'a> CSOT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CSOT_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn csot_0(self) -> &'a mut W {
    self.variant(CSOT_A::CSOT_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn csot_1(self) -> &'a mut W {
    self.variant(CSOT_A::CSOT_1)
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
#[doc = "Counter Stop On Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOO_A {
  #[doc = "0: no description available"]
  CSOO_0,
  #[doc = "1: no description available"]
  CSOO_1,
}
impl From<CSOO_A> for bool {
  #[inline(always)]
  fn from(variant: CSOO_A) -> Self {
    match variant {
      CSOO_A::CSOO_0 => false,
      CSOO_A::CSOO_1 => true,
    }
  }
}
#[doc = "Reader of field `CSOO`"]
pub type CSOO_R = crate::R<bool, CSOO_A>;
impl CSOO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CSOO_A {
    match self.bits {
      false => CSOO_A::CSOO_0,
      true => CSOO_A::CSOO_1,
    }
  }
  #[doc = "Checks if the value of the field is `CSOO_0`"]
  #[inline(always)]
  pub fn is_csoo_0(&self) -> bool {
    *self == CSOO_A::CSOO_0
  }
  #[doc = "Checks if the value of the field is `CSOO_1`"]
  #[inline(always)]
  pub fn is_csoo_1(&self) -> bool {
    *self == CSOO_A::CSOO_1
  }
}
#[doc = "Write proxy for field `CSOO`"]
pub struct CSOO_W<'a> {
  w: &'a mut W,
}
impl<'a> CSOO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CSOO_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn csoo_0(self) -> &'a mut W {
    self.variant(CSOO_A::CSOO_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn csoo_1(self) -> &'a mut W {
    self.variant(CSOO_A::CSOO_1)
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
#[doc = "Counter Reload On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROT_A {
  #[doc = "0: Counter is not reloaded due to a rising edge on the selected input trigger"]
  CROT_0,
  #[doc = "1: Counter is reloaded when a rising edge is detected on the selected input trigger"]
  CROT_1,
}
impl From<CROT_A> for bool {
  #[inline(always)]
  fn from(variant: CROT_A) -> Self {
    match variant {
      CROT_A::CROT_0 => false,
      CROT_A::CROT_1 => true,
    }
  }
}
#[doc = "Reader of field `CROT`"]
pub type CROT_R = crate::R<bool, CROT_A>;
impl CROT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CROT_A {
    match self.bits {
      false => CROT_A::CROT_0,
      true => CROT_A::CROT_1,
    }
  }
  #[doc = "Checks if the value of the field is `CROT_0`"]
  #[inline(always)]
  pub fn is_crot_0(&self) -> bool {
    *self == CROT_A::CROT_0
  }
  #[doc = "Checks if the value of the field is `CROT_1`"]
  #[inline(always)]
  pub fn is_crot_1(&self) -> bool {
    *self == CROT_A::CROT_1
  }
}
#[doc = "Write proxy for field `CROT`"]
pub struct CROT_W<'a> {
  w: &'a mut W,
}
impl<'a> CROT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CROT_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
  #[inline(always)]
  pub fn crot_0(self) -> &'a mut W {
    self.variant(CROT_A::CROT_0)
  }
  #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
  #[inline(always)]
  pub fn crot_1(self) -> &'a mut W {
    self.variant(CROT_A::CROT_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
#[doc = "Reader of field `CPOT`"]
pub type CPOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOT`"]
pub struct CPOT_W<'a> {
  w: &'a mut W,
}
impl<'a> CPOT_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
    self.w
  }
}
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOL_A {
  #[doc = "0: Trigger is active high."]
  TRGPOL_0,
  #[doc = "1: Trigger is active low."]
  TRGPOL_1,
}
impl From<TRGPOL_A> for bool {
  #[inline(always)]
  fn from(variant: TRGPOL_A) -> Self {
    match variant {
      TRGPOL_A::TRGPOL_0 => false,
      TRGPOL_A::TRGPOL_1 => true,
    }
  }
}
#[doc = "Reader of field `TRGPOL`"]
pub type TRGPOL_R = crate::R<bool, TRGPOL_A>;
impl TRGPOL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRGPOL_A {
    match self.bits {
      false => TRGPOL_A::TRGPOL_0,
      true => TRGPOL_A::TRGPOL_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRGPOL_0`"]
  #[inline(always)]
  pub fn is_trgpol_0(&self) -> bool {
    *self == TRGPOL_A::TRGPOL_0
  }
  #[doc = "Checks if the value of the field is `TRGPOL_1`"]
  #[inline(always)]
  pub fn is_trgpol_1(&self) -> bool {
    *self == TRGPOL_A::TRGPOL_1
  }
}
#[doc = "Write proxy for field `TRGPOL`"]
pub struct TRGPOL_W<'a> {
  w: &'a mut W,
}
impl<'a> TRGPOL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRGPOL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Trigger is active high."]
  #[inline(always)]
  pub fn trgpol_0(self) -> &'a mut W {
    self.variant(TRGPOL_A::TRGPOL_0)
  }
  #[doc = "Trigger is active low."]
  #[inline(always)]
  pub fn trgpol_1(self) -> &'a mut W {
    self.variant(TRGPOL_A::TRGPOL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
    self.w
  }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRC_A {
  #[doc = "0: Trigger source selected by TRGSEL is external."]
  TRGSRC_0,
  #[doc = "1: Trigger source selected by TRGSEL is internal (channel pin input capture)."]
  TRGSRC_1,
}
impl From<TRGSRC_A> for bool {
  #[inline(always)]
  fn from(variant: TRGSRC_A) -> Self {
    match variant {
      TRGSRC_A::TRGSRC_0 => false,
      TRGSRC_A::TRGSRC_1 => true,
    }
  }
}
#[doc = "Reader of field `TRGSRC`"]
pub type TRGSRC_R = crate::R<bool, TRGSRC_A>;
impl TRGSRC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRGSRC_A {
    match self.bits {
      false => TRGSRC_A::TRGSRC_0,
      true => TRGSRC_A::TRGSRC_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRGSRC_0`"]
  #[inline(always)]
  pub fn is_trgsrc_0(&self) -> bool {
    *self == TRGSRC_A::TRGSRC_0
  }
  #[doc = "Checks if the value of the field is `TRGSRC_1`"]
  #[inline(always)]
  pub fn is_trgsrc_1(&self) -> bool {
    *self == TRGSRC_A::TRGSRC_1
  }
}
#[doc = "Write proxy for field `TRGSRC`"]
pub struct TRGSRC_W<'a> {
  w: &'a mut W,
}
impl<'a> TRGSRC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRGSRC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Trigger source selected by TRGSEL is external."]
  #[inline(always)]
  pub fn trgsrc_0(self) -> &'a mut W {
    self.variant(TRGSRC_A::TRGSRC_0)
  }
  #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
  #[inline(always)]
  pub fn trgsrc_1(self) -> &'a mut W {
    self.variant(TRGSRC_A::TRGSRC_1)
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
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL_A {
  #[doc = "1: Channel 0 pin input capture"]
  TRGSEL_1,
  #[doc = "2: Channel 1 pin input capture"]
  TRGSEL_2,
  #[doc = "3: Channel 0 or Channel 1 pin input capture"]
  TRGSEL_3,
}
impl From<TRGSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: TRGSEL_A) -> Self {
    match variant {
      TRGSEL_A::TRGSEL_1 => 1,
      TRGSEL_A::TRGSEL_2 => 2,
      TRGSEL_A::TRGSEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, TRGSEL_A>;
impl TRGSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TRGSEL_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(TRGSEL_A::TRGSEL_1),
      2 => Val(TRGSEL_A::TRGSEL_2),
      3 => Val(TRGSEL_A::TRGSEL_3),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TRGSEL_1`"]
  #[inline(always)]
  pub fn is_trgsel_1(&self) -> bool {
    *self == TRGSEL_A::TRGSEL_1
  }
  #[doc = "Checks if the value of the field is `TRGSEL_2`"]
  #[inline(always)]
  pub fn is_trgsel_2(&self) -> bool {
    *self == TRGSEL_A::TRGSEL_2
  }
  #[doc = "Checks if the value of the field is `TRGSEL_3`"]
  #[inline(always)]
  pub fn is_trgsel_3(&self) -> bool {
    *self == TRGSEL_A::TRGSEL_3
  }
}
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Channel 0 pin input capture"]
  #[inline(always)]
  pub fn trgsel_1(self) -> &'a mut W {
    self.variant(TRGSEL_A::TRGSEL_1)
  }
  #[doc = "Channel 1 pin input capture"]
  #[inline(always)]
  pub fn trgsel_2(self) -> &'a mut W {
    self.variant(TRGSEL_A::TRGSEL_2)
  }
  #[doc = "Channel 0 or Channel 1 pin input capture"]
  #[inline(always)]
  pub fn trgsel_3(self) -> &'a mut W {
    self.variant(TRGSEL_A::TRGSEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bit 5 - Doze Enable"]
  #[inline(always)]
  pub fn dozeen(&self) -> DOZEEN_R {
    DOZEEN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bits 6:7 - Debug Mode"]
  #[inline(always)]
  pub fn dbgmode(&self) -> DBGMODE_R {
    DBGMODE_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bit 8 - Global Time Base Synchronization"]
  #[inline(always)]
  pub fn gtbsync(&self) -> GTBSYNC_R {
    GTBSYNC_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Global time base enable"]
  #[inline(always)]
  pub fn gtbeen(&self) -> GTBEEN_R {
    GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Counter Start on Trigger"]
  #[inline(always)]
  pub fn csot(&self) -> CSOT_R {
    CSOT_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Counter Stop On Overflow"]
  #[inline(always)]
  pub fn csoo(&self) -> CSOO_R {
    CSOO_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - Counter Reload On Trigger"]
  #[inline(always)]
  pub fn crot(&self) -> CROT_R {
    CROT_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - Counter Pause On Trigger"]
  #[inline(always)]
  pub fn cpot(&self) -> CPOT_R {
    CPOT_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 22 - Trigger Polarity"]
  #[inline(always)]
  pub fn trgpol(&self) -> TRGPOL_R {
    TRGPOL_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Trigger Source"]
  #[inline(always)]
  pub fn trgsrc(&self) -> TRGSRC_R {
    TRGSRC_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bits 24:25 - Trigger Select"]
  #[inline(always)]
  pub fn trgsel(&self) -> TRGSEL_R {
    TRGSEL_R::new(((self.bits >> 24) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bit 5 - Doze Enable"]
  #[inline(always)]
  pub fn dozeen(&mut self) -> DOZEEN_W {
    DOZEEN_W { w: self }
  }
  #[doc = "Bits 6:7 - Debug Mode"]
  #[inline(always)]
  pub fn dbgmode(&mut self) -> DBGMODE_W {
    DBGMODE_W { w: self }
  }
  #[doc = "Bit 8 - Global Time Base Synchronization"]
  #[inline(always)]
  pub fn gtbsync(&mut self) -> GTBSYNC_W {
    GTBSYNC_W { w: self }
  }
  #[doc = "Bit 9 - Global time base enable"]
  #[inline(always)]
  pub fn gtbeen(&mut self) -> GTBEEN_W {
    GTBEEN_W { w: self }
  }
  #[doc = "Bit 16 - Counter Start on Trigger"]
  #[inline(always)]
  pub fn csot(&mut self) -> CSOT_W {
    CSOT_W { w: self }
  }
  #[doc = "Bit 17 - Counter Stop On Overflow"]
  #[inline(always)]
  pub fn csoo(&mut self) -> CSOO_W {
    CSOO_W { w: self }
  }
  #[doc = "Bit 18 - Counter Reload On Trigger"]
  #[inline(always)]
  pub fn crot(&mut self) -> CROT_W {
    CROT_W { w: self }
  }
  #[doc = "Bit 19 - Counter Pause On Trigger"]
  #[inline(always)]
  pub fn cpot(&mut self) -> CPOT_W {
    CPOT_W { w: self }
  }
  #[doc = "Bit 22 - Trigger Polarity"]
  #[inline(always)]
  pub fn trgpol(&mut self) -> TRGPOL_W {
    TRGPOL_W { w: self }
  }
  #[doc = "Bit 23 - Trigger Source"]
  #[inline(always)]
  pub fn trgsrc(&mut self) -> TRGSRC_W {
    TRGSRC_W { w: self }
  }
  #[doc = "Bits 24:25 - Trigger Select"]
  #[inline(always)]
  pub fn trgsel(&mut self) -> TRGSEL_W {
    TRGSEL_W { w: self }
  }
}
