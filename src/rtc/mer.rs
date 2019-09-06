#[doc = "Reader of register MER"]
pub type R = crate::R<u32, super::MER>;
#[doc = "Writer for register MER"]
pub type W = crate::W<u32, super::MER>;
#[doc = "Register MER `reset()`'s with value 0"]
impl crate::ResetValue for super::MER {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Monotonic Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCE_A {
  #[doc = "0: Writes to the monotonic counter load the counter with the value written."]
  MCE_0,
  #[doc = "1: Writes to the monotonic counter increment the counter."]
  MCE_1,
}
impl From<MCE_A> for bool {
  #[inline(always)]
  fn from(variant: MCE_A) -> Self {
    match variant {
      MCE_A::MCE_0 => false,
      MCE_A::MCE_1 => true,
    }
  }
}
#[doc = "Reader of field `MCE`"]
pub type MCE_R = crate::R<bool, MCE_A>;
impl MCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MCE_A {
    match self.bits {
      false => MCE_A::MCE_0,
      true => MCE_A::MCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `MCE_0`"]
  #[inline(always)]
  pub fn is_mce_0(&self) -> bool {
    *self == MCE_A::MCE_0
  }
  #[doc = "Checks if the value of the field is `MCE_1`"]
  #[inline(always)]
  pub fn is_mce_1(&self) -> bool {
    *self == MCE_A::MCE_1
  }
}
#[doc = "Write proxy for field `MCE`"]
pub struct MCE_W<'a> {
  w: &'a mut W,
}
impl<'a> MCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Writes to the monotonic counter load the counter with the value written."]
  #[inline(always)]
  pub fn mce_0(self) -> &'a mut W {
    self.variant(MCE_A::MCE_0)
  }
  #[doc = "Writes to the monotonic counter increment the counter."]
  #[inline(always)]
  pub fn mce_1(self) -> &'a mut W {
    self.variant(MCE_A::MCE_1)
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
impl R {
  #[doc = "Bit 4 - Monotonic Counter Enable"]
  #[inline(always)]
  pub fn mce(&self) -> MCE_R {
    MCE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - Monotonic Counter Enable"]
  #[inline(always)]
  pub fn mce(&mut self) -> MCE_W {
    MCE_W { w: self }
  }
}
