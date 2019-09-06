#[doc = "Reader of register CLK_RECOVER_IRC_EN"]
pub type R = crate::R<u8, super::CLK_RECOVER_IRC_EN>;
#[doc = "Writer for register CLK_RECOVER_IRC_EN"]
pub type W = crate::W<u8, super::CLK_RECOVER_IRC_EN>;
#[doc = "Register CLK_RECOVER_IRC_EN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CLK_RECOVER_IRC_EN {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x01
  }
}
#[doc = "Regulator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_EN_A {
  #[doc = "0: no description available"]
  REG_EN_0,
  #[doc = "1: no description available"]
  REG_EN_1,
}
impl From<REG_EN_A> for bool {
  #[inline(always)]
  fn from(variant: REG_EN_A) -> Self {
    match variant {
      REG_EN_A::REG_EN_0 => false,
      REG_EN_A::REG_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `REG_EN`"]
pub type REG_EN_R = crate::R<bool, REG_EN_A>;
impl REG_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> REG_EN_A {
    match self.bits {
      false => REG_EN_A::REG_EN_0,
      true => REG_EN_A::REG_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `REG_EN_0`"]
  #[inline(always)]
  pub fn is_reg_en_0(&self) -> bool {
    *self == REG_EN_A::REG_EN_0
  }
  #[doc = "Checks if the value of the field is `REG_EN_1`"]
  #[inline(always)]
  pub fn is_reg_en_1(&self) -> bool {
    *self == REG_EN_A::REG_EN_1
  }
}
#[doc = "Write proxy for field `REG_EN`"]
pub struct REG_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> REG_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: REG_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn reg_en_0(self) -> &'a mut W {
    self.variant(REG_EN_A::REG_EN_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn reg_en_1(self) -> &'a mut W {
    self.variant(REG_EN_A::REG_EN_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
    self.w
  }
}
#[doc = "IRC_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_EN_A {
  #[doc = "0: no description available"]
  IRC_EN_0,
  #[doc = "1: no description available"]
  IRC_EN_1,
}
impl From<IRC_EN_A> for bool {
  #[inline(always)]
  fn from(variant: IRC_EN_A) -> Self {
    match variant {
      IRC_EN_A::IRC_EN_0 => false,
      IRC_EN_A::IRC_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `IRC_EN`"]
pub type IRC_EN_R = crate::R<bool, IRC_EN_A>;
impl IRC_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IRC_EN_A {
    match self.bits {
      false => IRC_EN_A::IRC_EN_0,
      true => IRC_EN_A::IRC_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `IRC_EN_0`"]
  #[inline(always)]
  pub fn is_irc_en_0(&self) -> bool {
    *self == IRC_EN_A::IRC_EN_0
  }
  #[doc = "Checks if the value of the field is `IRC_EN_1`"]
  #[inline(always)]
  pub fn is_irc_en_1(&self) -> bool {
    *self == IRC_EN_A::IRC_EN_1
  }
}
#[doc = "Write proxy for field `IRC_EN`"]
pub struct IRC_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> IRC_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IRC_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irc_en_0(self) -> &'a mut W {
    self.variant(IRC_EN_A::IRC_EN_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irc_en_1(self) -> &'a mut W {
    self.variant(IRC_EN_A::IRC_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Regulator enable"]
  #[inline(always)]
  pub fn reg_en(&self) -> REG_EN_R {
    REG_EN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - IRC_EN"]
  #[inline(always)]
  pub fn irc_en(&self) -> IRC_EN_R {
    IRC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Regulator enable"]
  #[inline(always)]
  pub fn reg_en(&mut self) -> REG_EN_W {
    REG_EN_W { w: self }
  }
  #[doc = "Bit 1 - IRC_EN"]
  #[inline(always)]
  pub fn irc_en(&mut self) -> IRC_EN_W {
    IRC_EN_W { w: self }
  }
}
