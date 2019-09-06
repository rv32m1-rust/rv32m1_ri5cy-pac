#[doc = "Reader of register MR"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Boot Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTCFG_A {
  #[doc = "0: Boot from Flash."]
  BOOTCFG_0,
  #[doc = "1: Boot from ROM due to BOOTCFG0 pin assertion."]
  BOOTCFG_1,
  #[doc = "2: Boot from ROM due to FOPT configuration."]
  BOOTCFG_2,
  #[doc = "3: Boot from ROM due to both BOOTCFG0 pin assertion and FOPT configuration."]
  BOOTCFG_3,
}
impl From<BOOTCFG_A> for u8 {
  #[inline(always)]
  fn from(variant: BOOTCFG_A) -> Self {
    match variant {
      BOOTCFG_A::BOOTCFG_0 => 0,
      BOOTCFG_A::BOOTCFG_1 => 1,
      BOOTCFG_A::BOOTCFG_2 => 2,
      BOOTCFG_A::BOOTCFG_3 => 3,
    }
  }
}
#[doc = "Reader of field `BOOTCFG`"]
pub type BOOTCFG_R = crate::R<u8, BOOTCFG_A>;
impl BOOTCFG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BOOTCFG_A {
    match self.bits {
      0 => BOOTCFG_A::BOOTCFG_0,
      1 => BOOTCFG_A::BOOTCFG_1,
      2 => BOOTCFG_A::BOOTCFG_2,
      3 => BOOTCFG_A::BOOTCFG_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `BOOTCFG_0`"]
  #[inline(always)]
  pub fn is_bootcfg_0(&self) -> bool {
    *self == BOOTCFG_A::BOOTCFG_0
  }
  #[doc = "Checks if the value of the field is `BOOTCFG_1`"]
  #[inline(always)]
  pub fn is_bootcfg_1(&self) -> bool {
    *self == BOOTCFG_A::BOOTCFG_1
  }
  #[doc = "Checks if the value of the field is `BOOTCFG_2`"]
  #[inline(always)]
  pub fn is_bootcfg_2(&self) -> bool {
    *self == BOOTCFG_A::BOOTCFG_2
  }
  #[doc = "Checks if the value of the field is `BOOTCFG_3`"]
  #[inline(always)]
  pub fn is_bootcfg_3(&self) -> bool {
    *self == BOOTCFG_A::BOOTCFG_3
  }
}
#[doc = "Write proxy for field `BOOTCFG`"]
pub struct BOOTCFG_W<'a> {
  w: &'a mut W,
}
impl<'a> BOOTCFG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BOOTCFG_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Boot from Flash."]
  #[inline(always)]
  pub fn bootcfg_0(self) -> &'a mut W {
    self.variant(BOOTCFG_A::BOOTCFG_0)
  }
  #[doc = "Boot from ROM due to BOOTCFG0 pin assertion."]
  #[inline(always)]
  pub fn bootcfg_1(self) -> &'a mut W {
    self.variant(BOOTCFG_A::BOOTCFG_1)
  }
  #[doc = "Boot from ROM due to FOPT configuration."]
  #[inline(always)]
  pub fn bootcfg_2(self) -> &'a mut W {
    self.variant(BOOTCFG_A::BOOTCFG_2)
  }
  #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT configuration."]
  #[inline(always)]
  pub fn bootcfg_3(self) -> &'a mut W {
    self.variant(BOOTCFG_A::BOOTCFG_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Boot Configuration"]
  #[inline(always)]
  pub fn bootcfg(&self) -> BOOTCFG_R {
    BOOTCFG_R::new((self.bits & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Boot Configuration"]
  #[inline(always)]
  pub fn bootcfg(&mut self) -> BOOTCFG_W {
    BOOTCFG_W { w: self }
  }
}
