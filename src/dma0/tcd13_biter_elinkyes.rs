#[doc = "Reader of register TCD13_BITER_ELINKYES"]
pub type R = crate::R<u16, super::TCD13_BITER_ELINKYES>;
#[doc = "Writer for register TCD13_BITER_ELINKYES"]
pub type W = crate::W<u16, super::TCD13_BITER_ELINKYES>;
#[doc = "Register TCD13_BITER_ELINKYES `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD13_BITER_ELINKYES {
  type Type = u16;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `BITER`"]
pub type BITER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BITER`"]
pub struct BITER_W<'a> {
  w: &'a mut W,
}
impl<'a> BITER_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x01ff) | ((value as u16) & 0x01ff);
    self.w
  }
}
#[doc = "Reader of field `LINKCH`"]
pub type LINKCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINKCH`"]
pub struct LINKCH_W<'a> {
  w: &'a mut W,
}
impl<'a> LINKCH_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u16) & 0x0f) << 9);
    self.w
  }
}
#[doc = "Enables channel-to-channel linking on minor loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELINK_A {
  #[doc = "0: The channel-to-channel linking is disabled"]
  ELINK_0,
  #[doc = "1: The channel-to-channel linking is enabled"]
  ELINK_1,
}
impl From<ELINK_A> for bool {
  #[inline(always)]
  fn from(variant: ELINK_A) -> Self {
    match variant {
      ELINK_A::ELINK_0 => false,
      ELINK_A::ELINK_1 => true,
    }
  }
}
#[doc = "Reader of field `ELINK`"]
pub type ELINK_R = crate::R<bool, ELINK_A>;
impl ELINK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ELINK_A {
    match self.bits {
      false => ELINK_A::ELINK_0,
      true => ELINK_A::ELINK_1,
    }
  }
  #[doc = "Checks if the value of the field is `ELINK_0`"]
  #[inline(always)]
  pub fn is_elink_0(&self) -> bool {
    *self == ELINK_A::ELINK_0
  }
  #[doc = "Checks if the value of the field is `ELINK_1`"]
  #[inline(always)]
  pub fn is_elink_1(&self) -> bool {
    *self == ELINK_A::ELINK_1
  }
}
#[doc = "Write proxy for field `ELINK`"]
pub struct ELINK_W<'a> {
  w: &'a mut W,
}
impl<'a> ELINK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ELINK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel-to-channel linking is disabled"]
  #[inline(always)]
  pub fn elink_0(self) -> &'a mut W {
    self.variant(ELINK_A::ELINK_0)
  }
  #[doc = "The channel-to-channel linking is enabled"]
  #[inline(always)]
  pub fn elink_1(self) -> &'a mut W {
    self.variant(ELINK_A::ELINK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:8 - Starting major iteration count"]
  #[inline(always)]
  pub fn biter(&self) -> BITER_R {
    BITER_R::new((self.bits & 0x01ff) as u16)
  }
  #[doc = "Bits 9:12 - Link Channel Number"]
  #[inline(always)]
  pub fn linkch(&self) -> LINKCH_R {
    LINKCH_R::new(((self.bits >> 9) & 0x0f) as u8)
  }
  #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
  #[inline(always)]
  pub fn elink(&self) -> ELINK_R {
    ELINK_R::new(((self.bits >> 15) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:8 - Starting major iteration count"]
  #[inline(always)]
  pub fn biter(&mut self) -> BITER_W {
    BITER_W { w: self }
  }
  #[doc = "Bits 9:12 - Link Channel Number"]
  #[inline(always)]
  pub fn linkch(&mut self) -> LINKCH_W {
    LINKCH_W { w: self }
  }
  #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
  #[inline(always)]
  pub fn elink(&mut self) -> ELINK_W {
    ELINK_W { w: self }
  }
}
