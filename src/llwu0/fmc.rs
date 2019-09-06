#[doc = "Reader of register FMC"]
pub type R = crate::R<u32, super::FMC>;
#[doc = "Writer for register FMC"]
pub type W = crate::W<u32, super::FMC>;
#[doc = "Register FMC `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Filter Mode for FILT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTM1_A {
  #[doc = "0: External input pin filter detection active only during LLS/VLLS mode"]
  FILTM1_0,
  #[doc = "1: External input pin filter detection active during all power modes"]
  FILTM1_1,
}
impl From<FILTM1_A> for bool {
  #[inline(always)]
  fn from(variant: FILTM1_A) -> Self {
    match variant {
      FILTM1_A::FILTM1_0 => false,
      FILTM1_A::FILTM1_1 => true,
    }
  }
}
#[doc = "Reader of field `FILTM1`"]
pub type FILTM1_R = crate::R<bool, FILTM1_A>;
impl FILTM1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTM1_A {
    match self.bits {
      false => FILTM1_A::FILTM1_0,
      true => FILTM1_A::FILTM1_1,
    }
  }
  #[doc = "Checks if the value of the field is `FILTM1_0`"]
  #[inline(always)]
  pub fn is_filtm1_0(&self) -> bool {
    *self == FILTM1_A::FILTM1_0
  }
  #[doc = "Checks if the value of the field is `FILTM1_1`"]
  #[inline(always)]
  pub fn is_filtm1_1(&self) -> bool {
    *self == FILTM1_A::FILTM1_1
  }
}
#[doc = "Write proxy for field `FILTM1`"]
pub struct FILTM1_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTM1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTM1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin filter detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn filtm1_0(self) -> &'a mut W {
    self.variant(FILTM1_A::FILTM1_0)
  }
  #[doc = "External input pin filter detection active during all power modes"]
  #[inline(always)]
  pub fn filtm1_1(self) -> &'a mut W {
    self.variant(FILTM1_A::FILTM1_1)
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
#[doc = "Filter Mode for FILT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTM2_A {
  #[doc = "0: External input pin filter detection active only during LLS/VLLS mode"]
  FILTM2_0,
  #[doc = "1: External input pin filter detection active during all power modes"]
  FILTM2_1,
}
impl From<FILTM2_A> for bool {
  #[inline(always)]
  fn from(variant: FILTM2_A) -> Self {
    match variant {
      FILTM2_A::FILTM2_0 => false,
      FILTM2_A::FILTM2_1 => true,
    }
  }
}
#[doc = "Reader of field `FILTM2`"]
pub type FILTM2_R = crate::R<bool, FILTM2_A>;
impl FILTM2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTM2_A {
    match self.bits {
      false => FILTM2_A::FILTM2_0,
      true => FILTM2_A::FILTM2_1,
    }
  }
  #[doc = "Checks if the value of the field is `FILTM2_0`"]
  #[inline(always)]
  pub fn is_filtm2_0(&self) -> bool {
    *self == FILTM2_A::FILTM2_0
  }
  #[doc = "Checks if the value of the field is `FILTM2_1`"]
  #[inline(always)]
  pub fn is_filtm2_1(&self) -> bool {
    *self == FILTM2_A::FILTM2_1
  }
}
#[doc = "Write proxy for field `FILTM2`"]
pub struct FILTM2_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTM2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTM2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "External input pin filter detection active only during LLS/VLLS mode"]
  #[inline(always)]
  pub fn filtm2_0(self) -> &'a mut W {
    self.variant(FILTM2_A::FILTM2_0)
  }
  #[doc = "External input pin filter detection active during all power modes"]
  #[inline(always)]
  pub fn filtm2_1(self) -> &'a mut W {
    self.variant(FILTM2_A::FILTM2_1)
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
impl R {
  #[doc = "Bit 0 - Filter Mode for FILT1"]
  #[inline(always)]
  pub fn filtm1(&self) -> FILTM1_R {
    FILTM1_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Filter Mode for FILT2"]
  #[inline(always)]
  pub fn filtm2(&self) -> FILTM2_R {
    FILTM2_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Filter Mode for FILT1"]
  #[inline(always)]
  pub fn filtm1(&mut self) -> FILTM1_W {
    FILTM1_W { w: self }
  }
  #[doc = "Bit 1 - Filter Mode for FILT2"]
  #[inline(always)]
  pub fn filtm2(&mut self) -> FILTM2_W {
    FILTM2_W { w: self }
  }
}
