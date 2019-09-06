#[doc = "Reader of register PKHA_CCR"]
pub type R = crate::R<u32, super::PKHA_CCR>;
#[doc = "Writer for register PKHA_CCR"]
pub type W = crate::W<u32, super::PKHA_CCR>;
#[doc = "Register PKHA_CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PKHA_CCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Clock Throttle selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKTHRT_A {
  #[doc = "0: PKHA clock division rate is 8/8 - full speed"]
  CKTHRT_0,
  #[doc = "1: PKHA clock division rate is 1/8"]
  CKTHRT_1,
  #[doc = "2: PKHA clock division rate is 2/8"]
  CKTHRT_2,
  #[doc = "3: PKHA clock division rate is 3/8"]
  CKTHRT_3,
  #[doc = "4: PKHA clock division rate is 4/8"]
  CKTHRT_4,
  #[doc = "5: PKHA clock division rate is 5/8"]
  CKTHRT_5,
  #[doc = "6: PKHA clock division rate is 6/8"]
  CKTHRT_6,
  #[doc = "7: PKHA clock division rate is 7/8"]
  CKTHRT_7,
}
impl From<CKTHRT_A> for u8 {
  #[inline(always)]
  fn from(variant: CKTHRT_A) -> Self {
    match variant {
      CKTHRT_A::CKTHRT_0 => 0,
      CKTHRT_A::CKTHRT_1 => 1,
      CKTHRT_A::CKTHRT_2 => 2,
      CKTHRT_A::CKTHRT_3 => 3,
      CKTHRT_A::CKTHRT_4 => 4,
      CKTHRT_A::CKTHRT_5 => 5,
      CKTHRT_A::CKTHRT_6 => 6,
      CKTHRT_A::CKTHRT_7 => 7,
    }
  }
}
#[doc = "Reader of field `CKTHRT`"]
pub type CKTHRT_R = crate::R<u8, CKTHRT_A>;
impl CKTHRT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CKTHRT_A {
    match self.bits {
      0 => CKTHRT_A::CKTHRT_0,
      1 => CKTHRT_A::CKTHRT_1,
      2 => CKTHRT_A::CKTHRT_2,
      3 => CKTHRT_A::CKTHRT_3,
      4 => CKTHRT_A::CKTHRT_4,
      5 => CKTHRT_A::CKTHRT_5,
      6 => CKTHRT_A::CKTHRT_6,
      7 => CKTHRT_A::CKTHRT_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `CKTHRT_0`"]
  #[inline(always)]
  pub fn is_ckthrt_0(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_0
  }
  #[doc = "Checks if the value of the field is `CKTHRT_1`"]
  #[inline(always)]
  pub fn is_ckthrt_1(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_1
  }
  #[doc = "Checks if the value of the field is `CKTHRT_2`"]
  #[inline(always)]
  pub fn is_ckthrt_2(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_2
  }
  #[doc = "Checks if the value of the field is `CKTHRT_3`"]
  #[inline(always)]
  pub fn is_ckthrt_3(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_3
  }
  #[doc = "Checks if the value of the field is `CKTHRT_4`"]
  #[inline(always)]
  pub fn is_ckthrt_4(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_4
  }
  #[doc = "Checks if the value of the field is `CKTHRT_5`"]
  #[inline(always)]
  pub fn is_ckthrt_5(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_5
  }
  #[doc = "Checks if the value of the field is `CKTHRT_6`"]
  #[inline(always)]
  pub fn is_ckthrt_6(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_6
  }
  #[doc = "Checks if the value of the field is `CKTHRT_7`"]
  #[inline(always)]
  pub fn is_ckthrt_7(&self) -> bool {
    *self == CKTHRT_A::CKTHRT_7
  }
}
#[doc = "Write proxy for field `CKTHRT`"]
pub struct CKTHRT_W<'a> {
  w: &'a mut W,
}
impl<'a> CKTHRT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CKTHRT_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "PKHA clock division rate is 8/8 - full speed"]
  #[inline(always)]
  pub fn ckthrt_0(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_0)
  }
  #[doc = "PKHA clock division rate is 1/8"]
  #[inline(always)]
  pub fn ckthrt_1(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_1)
  }
  #[doc = "PKHA clock division rate is 2/8"]
  #[inline(always)]
  pub fn ckthrt_2(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_2)
  }
  #[doc = "PKHA clock division rate is 3/8"]
  #[inline(always)]
  pub fn ckthrt_3(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_3)
  }
  #[doc = "PKHA clock division rate is 4/8"]
  #[inline(always)]
  pub fn ckthrt_4(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_4)
  }
  #[doc = "PKHA clock division rate is 5/8"]
  #[inline(always)]
  pub fn ckthrt_5(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_5)
  }
  #[doc = "PKHA clock division rate is 6/8"]
  #[inline(always)]
  pub fn ckthrt_6(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_6)
  }
  #[doc = "PKHA clock division rate is 7/8"]
  #[inline(always)]
  pub fn ckthrt_7(self) -> &'a mut W {
    self.variant(CKTHRT_A::CKTHRT_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Register Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: Register is unlocked"]
  LK_0,
  #[doc = "1: Register is locked"]
  LK_1,
}
impl From<LK_A> for bool {
  #[inline(always)]
  fn from(variant: LK_A) -> Self {
    match variant {
      LK_A::LK_0 => false,
      LK_A::LK_1 => true,
    }
  }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK_A {
    match self.bits {
      false => LK_A::LK_0,
      true => LK_A::LK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK_0`"]
  #[inline(always)]
  pub fn is_lk_0(&self) -> bool {
    *self == LK_A::LK_0
  }
  #[doc = "Checks if the value of the field is `LK_1`"]
  #[inline(always)]
  pub fn is_lk_1(&self) -> bool {
    *self == LK_A::LK_1
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Register is unlocked"]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_A::LK_0)
  }
  #[doc = "Register is locked"]
  #[inline(always)]
  pub fn lk_1(self) -> &'a mut W {
    self.variant(LK_A::LK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
    self.w
  }
}
#[doc = "Enable Linear Feedback Shift Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELFR_A {
  #[doc = "0: LFSR is only enabled if ECT = 1 and ECJ = 1"]
  ELFR_0,
  #[doc = "1: LFSR is enabled independently of ECT and ECJ"]
  ELFR_1,
}
impl From<ELFR_A> for bool {
  #[inline(always)]
  fn from(variant: ELFR_A) -> Self {
    match variant {
      ELFR_A::ELFR_0 => false,
      ELFR_A::ELFR_1 => true,
    }
  }
}
#[doc = "Reader of field `ELFR`"]
pub type ELFR_R = crate::R<bool, ELFR_A>;
impl ELFR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ELFR_A {
    match self.bits {
      false => ELFR_A::ELFR_0,
      true => ELFR_A::ELFR_1,
    }
  }
  #[doc = "Checks if the value of the field is `ELFR_0`"]
  #[inline(always)]
  pub fn is_elfr_0(&self) -> bool {
    *self == ELFR_A::ELFR_0
  }
  #[doc = "Checks if the value of the field is `ELFR_1`"]
  #[inline(always)]
  pub fn is_elfr_1(&self) -> bool {
    *self == ELFR_A::ELFR_1
  }
}
#[doc = "Write proxy for field `ELFR`"]
pub struct ELFR_W<'a> {
  w: &'a mut W,
}
impl<'a> ELFR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ELFR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LFSR is only enabled if ECT = 1 and ECJ = 1"]
  #[inline(always)]
  pub fn elfr_0(self) -> &'a mut W {
    self.variant(ELFR_A::ELFR_0)
  }
  #[doc = "LFSR is enabled independently of ECT and ECJ"]
  #[inline(always)]
  pub fn elfr_1(self) -> &'a mut W {
    self.variant(ELFR_A::ELFR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "Enable Clock Jitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECJ_A {
  #[doc = "0: Clock Jitter is disabled"]
  ECJ_0,
  #[doc = "1: Clock jitter is enabled"]
  ECJ_1,
}
impl From<ECJ_A> for bool {
  #[inline(always)]
  fn from(variant: ECJ_A) -> Self {
    match variant {
      ECJ_A::ECJ_0 => false,
      ECJ_A::ECJ_1 => true,
    }
  }
}
#[doc = "Reader of field `ECJ`"]
pub type ECJ_R = crate::R<bool, ECJ_A>;
impl ECJ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ECJ_A {
    match self.bits {
      false => ECJ_A::ECJ_0,
      true => ECJ_A::ECJ_1,
    }
  }
  #[doc = "Checks if the value of the field is `ECJ_0`"]
  #[inline(always)]
  pub fn is_ecj_0(&self) -> bool {
    *self == ECJ_A::ECJ_0
  }
  #[doc = "Checks if the value of the field is `ECJ_1`"]
  #[inline(always)]
  pub fn is_ecj_1(&self) -> bool {
    *self == ECJ_A::ECJ_1
  }
}
#[doc = "Write proxy for field `ECJ`"]
pub struct ECJ_W<'a> {
  w: &'a mut W,
}
impl<'a> ECJ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ECJ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock Jitter is disabled"]
  #[inline(always)]
  pub fn ecj_0(self) -> &'a mut W {
    self.variant(ECJ_A::ECJ_0)
  }
  #[doc = "Clock jitter is enabled"]
  #[inline(always)]
  pub fn ecj_1(self) -> &'a mut W {
    self.variant(ECJ_A::ECJ_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
#[doc = "Enable Clock Throttle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECT_A {
  #[doc = "0: PKHA clock throttle disabled meaning that PKHA is operatiing at full speed"]
  ECT_0,
  #[doc = "1: PKHA clock throttle enabled"]
  ECT_1,
}
impl From<ECT_A> for bool {
  #[inline(always)]
  fn from(variant: ECT_A) -> Self {
    match variant {
      ECT_A::ECT_0 => false,
      ECT_A::ECT_1 => true,
    }
  }
}
#[doc = "Reader of field `ECT`"]
pub type ECT_R = crate::R<bool, ECT_A>;
impl ECT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ECT_A {
    match self.bits {
      false => ECT_A::ECT_0,
      true => ECT_A::ECT_1,
    }
  }
  #[doc = "Checks if the value of the field is `ECT_0`"]
  #[inline(always)]
  pub fn is_ect_0(&self) -> bool {
    *self == ECT_A::ECT_0
  }
  #[doc = "Checks if the value of the field is `ECT_1`"]
  #[inline(always)]
  pub fn is_ect_1(&self) -> bool {
    *self == ECT_A::ECT_1
  }
}
#[doc = "Write proxy for field `ECT`"]
pub struct ECT_W<'a> {
  w: &'a mut W,
}
impl<'a> ECT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ECT_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "PKHA clock throttle disabled meaning that PKHA is operatiing at full speed"]
  #[inline(always)]
  pub fn ect_0(self) -> &'a mut W {
    self.variant(ECT_A::ECT_0)
  }
  #[doc = "PKHA clock throttle enabled"]
  #[inline(always)]
  pub fn ect_1(self) -> &'a mut W {
    self.variant(ECT_A::ECT_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Clock Throttle selection"]
  #[inline(always)]
  pub fn ckthrt(&self) -> CKTHRT_R {
    CKTHRT_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bit 24 - Register Lock"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Enable Linear Feedback Shift Register"]
  #[inline(always)]
  pub fn elfr(&self) -> ELFR_R {
    ELFR_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Enable Clock Jitter"]
  #[inline(always)]
  pub fn ecj(&self) -> ECJ_R {
    ECJ_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Enable Clock Throttle"]
  #[inline(always)]
  pub fn ect(&self) -> ECT_R {
    ECT_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:2 - Clock Throttle selection"]
  #[inline(always)]
  pub fn ckthrt(&mut self) -> CKTHRT_W {
    CKTHRT_W { w: self }
  }
  #[doc = "Bit 24 - Register Lock"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
  #[doc = "Bit 29 - Enable Linear Feedback Shift Register"]
  #[inline(always)]
  pub fn elfr(&mut self) -> ELFR_W {
    ELFR_W { w: self }
  }
  #[doc = "Bit 30 - Enable Clock Jitter"]
  #[inline(always)]
  pub fn ecj(&mut self) -> ECJ_W {
    ECJ_W { w: self }
  }
  #[doc = "Bit 31 - Enable Clock Throttle"]
  #[inline(always)]
  pub fn ect(&mut self) -> ECT_W {
    ECT_W { w: self }
  }
}
