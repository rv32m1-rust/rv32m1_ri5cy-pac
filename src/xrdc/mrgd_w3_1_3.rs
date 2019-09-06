#[doc = "Reader of register MRGD_W3_1_3"]
pub type R = crate::R<u32, super::MRGD_W3_1_3>;
#[doc = "Writer for register MRGD_W3_1_3"]
pub type W = crate::W<u32, super::MRGD_W3_1_3>;
#[doc = "Register MRGD_W3_1_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MRGD_W3_1_3 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Exclusive Access Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EAL_A {
  #[doc = "0: Lock disabled"]
  EAL_0,
  #[doc = "1: Lock disabled until next reset"]
  EAL_1,
  #[doc = "2: Lock enabled, lock state = available"]
  EAL_2,
  #[doc = "3: Lock enabled, lock state = not available"]
  EAL_3,
}
impl From<EAL_A> for u8 {
  #[inline(always)]
  fn from(variant: EAL_A) -> Self {
    match variant {
      EAL_A::EAL_0 => 0,
      EAL_A::EAL_1 => 1,
      EAL_A::EAL_2 => 2,
      EAL_A::EAL_3 => 3,
    }
  }
}
#[doc = "Reader of field `EAL`"]
pub type EAL_R = crate::R<u8, EAL_A>;
impl EAL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EAL_A {
    match self.bits {
      0 => EAL_A::EAL_0,
      1 => EAL_A::EAL_1,
      2 => EAL_A::EAL_2,
      3 => EAL_A::EAL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `EAL_0`"]
  #[inline(always)]
  pub fn is_eal_0(&self) -> bool {
    *self == EAL_A::EAL_0
  }
  #[doc = "Checks if the value of the field is `EAL_1`"]
  #[inline(always)]
  pub fn is_eal_1(&self) -> bool {
    *self == EAL_A::EAL_1
  }
  #[doc = "Checks if the value of the field is `EAL_2`"]
  #[inline(always)]
  pub fn is_eal_2(&self) -> bool {
    *self == EAL_A::EAL_2
  }
  #[doc = "Checks if the value of the field is `EAL_3`"]
  #[inline(always)]
  pub fn is_eal_3(&self) -> bool {
    *self == EAL_A::EAL_3
  }
}
#[doc = "Write proxy for field `EAL`"]
pub struct EAL_W<'a> {
  w: &'a mut W,
}
impl<'a> EAL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EAL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Lock disabled"]
  #[inline(always)]
  pub fn eal_0(self) -> &'a mut W {
    self.variant(EAL_A::EAL_0)
  }
  #[doc = "Lock disabled until next reset"]
  #[inline(always)]
  pub fn eal_1(self) -> &'a mut W {
    self.variant(EAL_A::EAL_1)
  }
  #[doc = "Lock enabled, lock state = available"]
  #[inline(always)]
  pub fn eal_2(self) -> &'a mut W {
    self.variant(EAL_A::EAL_2)
  }
  #[doc = "Lock enabled, lock state = not available"]
  #[inline(always)]
  pub fn eal_3(self) -> &'a mut W {
    self.variant(EAL_A::EAL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR`"]
pub struct CR_W<'a> {
  w: &'a mut W,
}
impl<'a> CR_W<'a> {
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
  #[doc = "Bits 24:25 - Exclusive Access Lock"]
  #[inline(always)]
  pub fn eal(&self) -> EAL_R {
    EAL_R::new(((self.bits >> 24) & 0x03) as u8)
  }
  #[doc = "Bit 31 - Code Region Indicator"]
  #[inline(always)]
  pub fn cr(&self) -> CR_R {
    CR_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 24:25 - Exclusive Access Lock"]
  #[inline(always)]
  pub fn eal(&mut self) -> EAL_W {
    EAL_W { w: self }
  }
  #[doc = "Bit 31 - Code Region Indicator"]
  #[inline(always)]
  pub fn cr(&mut self) -> CR_W {
    CR_W { w: self }
  }
}
