#[doc = "Reader of register SWTRIG"]
pub type R = crate::R<u32, super::SWTRIG>;
#[doc = "Writer for register SWTRIG"]
pub type W = crate::W<u32, super::SWTRIG>;
#[doc = "Register SWTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::SWTRIG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Software trigger 0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT0_AW {
  #[doc = "0: No trigger 0 event generated."]
  SWT0_0,
  #[doc = "1: Trigger 0 event generated."]
  SWT0_1,
}
impl From<SWT0_AW> for bool {
  #[inline(always)]
  fn from(variant: SWT0_AW) -> Self {
    match variant {
      SWT0_AW::SWT0_0 => false,
      SWT0_AW::SWT0_1 => true,
    }
  }
}
#[doc = "Write proxy for field `SWT0`"]
pub struct SWT0_W<'a> {
  w: &'a mut W,
}
impl<'a> SWT0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWT0_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No trigger 0 event generated."]
  #[inline(always)]
  pub fn swt0_0(self) -> &'a mut W {
    self.variant(SWT0_AW::SWT0_0)
  }
  #[doc = "Trigger 0 event generated."]
  #[inline(always)]
  pub fn swt0_1(self) -> &'a mut W {
    self.variant(SWT0_AW::SWT0_1)
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
#[doc = "Software trigger 1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT1_AW {
  #[doc = "0: No trigger 1 event generated."]
  SWT1_0,
  #[doc = "1: Trigger 1 event generated."]
  SWT1_1,
}
impl From<SWT1_AW> for bool {
  #[inline(always)]
  fn from(variant: SWT1_AW) -> Self {
    match variant {
      SWT1_AW::SWT1_0 => false,
      SWT1_AW::SWT1_1 => true,
    }
  }
}
#[doc = "Write proxy for field `SWT1`"]
pub struct SWT1_W<'a> {
  w: &'a mut W,
}
impl<'a> SWT1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWT1_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No trigger 1 event generated."]
  #[inline(always)]
  pub fn swt1_0(self) -> &'a mut W {
    self.variant(SWT1_AW::SWT1_0)
  }
  #[doc = "Trigger 1 event generated."]
  #[inline(always)]
  pub fn swt1_1(self) -> &'a mut W {
    self.variant(SWT1_AW::SWT1_1)
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
#[doc = "Software trigger 2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT2_AW {
  #[doc = "0: No trigger 2 event generated."]
  SWT2_0,
  #[doc = "1: Trigger 2 event generated."]
  SWT2_1,
}
impl From<SWT2_AW> for bool {
  #[inline(always)]
  fn from(variant: SWT2_AW) -> Self {
    match variant {
      SWT2_AW::SWT2_0 => false,
      SWT2_AW::SWT2_1 => true,
    }
  }
}
#[doc = "Write proxy for field `SWT2`"]
pub struct SWT2_W<'a> {
  w: &'a mut W,
}
impl<'a> SWT2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWT2_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No trigger 2 event generated."]
  #[inline(always)]
  pub fn swt2_0(self) -> &'a mut W {
    self.variant(SWT2_AW::SWT2_0)
  }
  #[doc = "Trigger 2 event generated."]
  #[inline(always)]
  pub fn swt2_1(self) -> &'a mut W {
    self.variant(SWT2_AW::SWT2_1)
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
#[doc = "Software trigger 3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT3_AW {
  #[doc = "0: No trigger 3 event generated."]
  SWT3_0,
  #[doc = "1: Trigger 3 event generated."]
  SWT3_1,
}
impl From<SWT3_AW> for bool {
  #[inline(always)]
  fn from(variant: SWT3_AW) -> Self {
    match variant {
      SWT3_AW::SWT3_0 => false,
      SWT3_AW::SWT3_1 => true,
    }
  }
}
#[doc = "Write proxy for field `SWT3`"]
pub struct SWT3_W<'a> {
  w: &'a mut W,
}
impl<'a> SWT3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWT3_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No trigger 3 event generated."]
  #[inline(always)]
  pub fn swt3_0(self) -> &'a mut W {
    self.variant(SWT3_AW::SWT3_0)
  }
  #[doc = "Trigger 3 event generated."]
  #[inline(always)]
  pub fn swt3_1(self) -> &'a mut W {
    self.variant(SWT3_AW::SWT3_1)
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
impl R {}
impl W {
  #[doc = "Bit 0 - Software trigger 0 event"]
  #[inline(always)]
  pub fn swt0(&mut self) -> SWT0_W {
    SWT0_W { w: self }
  }
  #[doc = "Bit 1 - Software trigger 1 event"]
  #[inline(always)]
  pub fn swt1(&mut self) -> SWT1_W {
    SWT1_W { w: self }
  }
  #[doc = "Bit 2 - Software trigger 2 event"]
  #[inline(always)]
  pub fn swt2(&mut self) -> SWT2_W {
    SWT2_W { w: self }
  }
  #[doc = "Bit 3 - Software trigger 3 event"]
  #[inline(always)]
  pub fn swt3(&mut self) -> SWT3_W {
    SWT3_W { w: self }
  }
}
