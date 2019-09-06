#[doc = "Reader of register PID1"]
pub type R = crate::R<u32, super::PID1>;
#[doc = "Writer for register PID1"]
pub type W = crate::W<u32, super::PID1>;
#[doc = "Register PID1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PID1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
  w: &'a mut W,
}
impl<'a> PID_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
    self.w
  }
}
#[doc = "Reader of field `SP4SM`"]
pub type SP4SM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SP4SM`"]
pub struct SP4SM_W<'a> {
  w: &'a mut W,
}
impl<'a> SP4SM_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
    self.w
  }
}
#[doc = "Reader of field `TSM`"]
pub type TSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSM`"]
pub struct TSM_W<'a> {
  w: &'a mut W,
}
impl<'a> TSM_W<'a> {
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
#[doc = "Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK2_A {
  #[doc = "0: Register can be written by any secure privileged write."]
  LK2_0,
  #[doc = "1: Register can be written by any secure privileged write."]
  LK2_1,
  #[doc = "2: Register can only be written by a secure privileged write from bus master m."]
  LK2_2,
  #[doc = "3: Register is locked (read-only) until the next reset."]
  LK2_3,
}
impl From<LK2_A> for u8 {
  #[inline(always)]
  fn from(variant: LK2_A) -> Self {
    match variant {
      LK2_A::LK2_0 => 0,
      LK2_A::LK2_1 => 1,
      LK2_A::LK2_2 => 2,
      LK2_A::LK2_3 => 3,
    }
  }
}
#[doc = "Reader of field `LK2`"]
pub type LK2_R = crate::R<u8, LK2_A>;
impl LK2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK2_A {
    match self.bits {
      0 => LK2_A::LK2_0,
      1 => LK2_A::LK2_1,
      2 => LK2_A::LK2_2,
      3 => LK2_A::LK2_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `LK2_0`"]
  #[inline(always)]
  pub fn is_lk2_0(&self) -> bool {
    *self == LK2_A::LK2_0
  }
  #[doc = "Checks if the value of the field is `LK2_1`"]
  #[inline(always)]
  pub fn is_lk2_1(&self) -> bool {
    *self == LK2_A::LK2_1
  }
  #[doc = "Checks if the value of the field is `LK2_2`"]
  #[inline(always)]
  pub fn is_lk2_2(&self) -> bool {
    *self == LK2_A::LK2_2
  }
  #[doc = "Checks if the value of the field is `LK2_3`"]
  #[inline(always)]
  pub fn is_lk2_3(&self) -> bool {
    *self == LK2_A::LK2_3
  }
}
#[doc = "Write proxy for field `LK2`"]
pub struct LK2_W<'a> {
  w: &'a mut W,
}
impl<'a> LK2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Register can be written by any secure privileged write."]
  #[inline(always)]
  pub fn lk2_0(self) -> &'a mut W {
    self.variant(LK2_A::LK2_0)
  }
  #[doc = "Register can be written by any secure privileged write."]
  #[inline(always)]
  pub fn lk2_1(self) -> &'a mut W {
    self.variant(LK2_A::LK2_1)
  }
  #[doc = "Register can only be written by a secure privileged write from bus master m."]
  #[inline(always)]
  pub fn lk2_2(self) -> &'a mut W {
    self.variant(LK2_A::LK2_2)
  }
  #[doc = "Register is locked (read-only) until the next reset."]
  #[inline(always)]
  pub fn lk2_3(self) -> &'a mut W {
    self.variant(LK2_A::LK2_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:5 - Process identifier"]
  #[inline(always)]
  pub fn pid(&self) -> PID_R {
    PID_R::new((self.bits & 0x3f) as u8)
  }
  #[doc = "Bit 27 - Special 4-state model"]
  #[inline(always)]
  pub fn sp4sm(&self) -> SP4SM_R {
    SP4SM_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Three-state model"]
  #[inline(always)]
  pub fn tsm(&self) -> TSM_R {
    TSM_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bits 29:30 - Lock"]
  #[inline(always)]
  pub fn lk2(&self) -> LK2_R {
    LK2_R::new(((self.bits >> 29) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:5 - Process identifier"]
  #[inline(always)]
  pub fn pid(&mut self) -> PID_W {
    PID_W { w: self }
  }
  #[doc = "Bit 27 - Special 4-state model"]
  #[inline(always)]
  pub fn sp4sm(&mut self) -> SP4SM_W {
    SP4SM_W { w: self }
  }
  #[doc = "Bit 28 - Three-state model"]
  #[inline(always)]
  pub fn tsm(&mut self) -> TSM_W {
    TSM_W { w: self }
  }
  #[doc = "Bits 29:30 - Lock"]
  #[inline(always)]
  pub fn lk2(&mut self) -> LK2_W {
    LK2_W { w: self }
  }
}
