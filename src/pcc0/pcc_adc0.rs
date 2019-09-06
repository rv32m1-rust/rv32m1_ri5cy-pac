#[doc = "Reader of register PCC_ADC0"]
pub type R = crate::R<u32, super::PCC_ADC0>;
#[doc = "Writer for register PCC_ADC0"]
pub type W = crate::W<u32, super::PCC_ADC0>;
#[doc = "Register PCC_ADC0 `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::PCC_ADC0 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x8000_0000
  }
}
#[doc = "Peripheral Clock Divider Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCD_A {
  #[doc = "0: Divide by 1."]
  PCD_0,
  #[doc = "1: Divide by 2."]
  PCD_1,
  #[doc = "2: Divide by 3."]
  PCD_2,
  #[doc = "3: Divide by 4."]
  PCD_3,
  #[doc = "4: Divide by 5."]
  PCD_4,
  #[doc = "5: Divide by 6."]
  PCD_5,
  #[doc = "6: Divide by 7."]
  PCD_6,
  #[doc = "7: Divide by 8."]
  PCD_7,
}
impl From<PCD_A> for u8 {
  #[inline(always)]
  fn from(variant: PCD_A) -> Self {
    match variant {
      PCD_A::PCD_0 => 0,
      PCD_A::PCD_1 => 1,
      PCD_A::PCD_2 => 2,
      PCD_A::PCD_3 => 3,
      PCD_A::PCD_4 => 4,
      PCD_A::PCD_5 => 5,
      PCD_A::PCD_6 => 6,
      PCD_A::PCD_7 => 7,
    }
  }
}
#[doc = "Reader of field `PCD`"]
pub type PCD_R = crate::R<u8, PCD_A>;
impl PCD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PCD_A {
    match self.bits {
      0 => PCD_A::PCD_0,
      1 => PCD_A::PCD_1,
      2 => PCD_A::PCD_2,
      3 => PCD_A::PCD_3,
      4 => PCD_A::PCD_4,
      5 => PCD_A::PCD_5,
      6 => PCD_A::PCD_6,
      7 => PCD_A::PCD_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PCD_0`"]
  #[inline(always)]
  pub fn is_pcd_0(&self) -> bool {
    *self == PCD_A::PCD_0
  }
  #[doc = "Checks if the value of the field is `PCD_1`"]
  #[inline(always)]
  pub fn is_pcd_1(&self) -> bool {
    *self == PCD_A::PCD_1
  }
  #[doc = "Checks if the value of the field is `PCD_2`"]
  #[inline(always)]
  pub fn is_pcd_2(&self) -> bool {
    *self == PCD_A::PCD_2
  }
  #[doc = "Checks if the value of the field is `PCD_3`"]
  #[inline(always)]
  pub fn is_pcd_3(&self) -> bool {
    *self == PCD_A::PCD_3
  }
  #[doc = "Checks if the value of the field is `PCD_4`"]
  #[inline(always)]
  pub fn is_pcd_4(&self) -> bool {
    *self == PCD_A::PCD_4
  }
  #[doc = "Checks if the value of the field is `PCD_5`"]
  #[inline(always)]
  pub fn is_pcd_5(&self) -> bool {
    *self == PCD_A::PCD_5
  }
  #[doc = "Checks if the value of the field is `PCD_6`"]
  #[inline(always)]
  pub fn is_pcd_6(&self) -> bool {
    *self == PCD_A::PCD_6
  }
  #[doc = "Checks if the value of the field is `PCD_7`"]
  #[inline(always)]
  pub fn is_pcd_7(&self) -> bool {
    *self == PCD_A::PCD_7
  }
}
#[doc = "Write proxy for field `PCD`"]
pub struct PCD_W<'a> {
  w: &'a mut W,
}
impl<'a> PCD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PCD_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Divide by 1."]
  #[inline(always)]
  pub fn pcd_0(self) -> &'a mut W {
    self.variant(PCD_A::PCD_0)
  }
  #[doc = "Divide by 2."]
  #[inline(always)]
  pub fn pcd_1(self) -> &'a mut W {
    self.variant(PCD_A::PCD_1)
  }
  #[doc = "Divide by 3."]
  #[inline(always)]
  pub fn pcd_2(self) -> &'a mut W {
    self.variant(PCD_A::PCD_2)
  }
  #[doc = "Divide by 4."]
  #[inline(always)]
  pub fn pcd_3(self) -> &'a mut W {
    self.variant(PCD_A::PCD_3)
  }
  #[doc = "Divide by 5."]
  #[inline(always)]
  pub fn pcd_4(self) -> &'a mut W {
    self.variant(PCD_A::PCD_4)
  }
  #[doc = "Divide by 6."]
  #[inline(always)]
  pub fn pcd_5(self) -> &'a mut W {
    self.variant(PCD_A::PCD_5)
  }
  #[doc = "Divide by 7."]
  #[inline(always)]
  pub fn pcd_6(self) -> &'a mut W {
    self.variant(PCD_A::PCD_6)
  }
  #[doc = "Divide by 8."]
  #[inline(always)]
  pub fn pcd_7(self) -> &'a mut W {
    self.variant(PCD_A::PCD_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Peripheral Clock Divider Fraction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_A {
  #[doc = "0: Fractional value is 0."]
  FRAC_0,
  #[doc = "1: Fractional value is 1."]
  FRAC_1,
}
impl From<FRAC_A> for bool {
  #[inline(always)]
  fn from(variant: FRAC_A) -> Self {
    match variant {
      FRAC_A::FRAC_0 => false,
      FRAC_A::FRAC_1 => true,
    }
  }
}
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<bool, FRAC_A>;
impl FRAC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FRAC_A {
    match self.bits {
      false => FRAC_A::FRAC_0,
      true => FRAC_A::FRAC_1,
    }
  }
  #[doc = "Checks if the value of the field is `FRAC_0`"]
  #[inline(always)]
  pub fn is_frac_0(&self) -> bool {
    *self == FRAC_A::FRAC_0
  }
  #[doc = "Checks if the value of the field is `FRAC_1`"]
  #[inline(always)]
  pub fn is_frac_1(&self) -> bool {
    *self == FRAC_A::FRAC_1
  }
}
#[doc = "Write proxy for field `FRAC`"]
pub struct FRAC_W<'a> {
  w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FRAC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fractional value is 0."]
  #[inline(always)]
  pub fn frac_0(self) -> &'a mut W {
    self.variant(FRAC_A::FRAC_0)
  }
  #[doc = "Fractional value is 1."]
  #[inline(always)]
  pub fn frac_1(self) -> &'a mut W {
    self.variant(FRAC_A::FRAC_1)
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
#[doc = "Peripheral Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS_A {
  #[doc = "0: Clock is off."]
  PCS_0,
  #[doc = "1: Clock option 1"]
  PCS_1,
  #[doc = "2: Clock option 2"]
  PCS_2,
  #[doc = "3: Clock option 3"]
  PCS_3,
  #[doc = "4: Clock option 4"]
  PCS_4,
  #[doc = "5: Clock option 5"]
  PCS_5,
  #[doc = "6: Clock option 6"]
  PCS_6,
  #[doc = "7: Clock option 7"]
  PCS_7,
}
impl From<PCS_A> for u8 {
  #[inline(always)]
  fn from(variant: PCS_A) -> Self {
    match variant {
      PCS_A::PCS_0 => 0,
      PCS_A::PCS_1 => 1,
      PCS_A::PCS_2 => 2,
      PCS_A::PCS_3 => 3,
      PCS_A::PCS_4 => 4,
      PCS_A::PCS_5 => 5,
      PCS_A::PCS_6 => 6,
      PCS_A::PCS_7 => 7,
    }
  }
}
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, PCS_A>;
impl PCS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PCS_A {
    match self.bits {
      0 => PCS_A::PCS_0,
      1 => PCS_A::PCS_1,
      2 => PCS_A::PCS_2,
      3 => PCS_A::PCS_3,
      4 => PCS_A::PCS_4,
      5 => PCS_A::PCS_5,
      6 => PCS_A::PCS_6,
      7 => PCS_A::PCS_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PCS_0`"]
  #[inline(always)]
  pub fn is_pcs_0(&self) -> bool {
    *self == PCS_A::PCS_0
  }
  #[doc = "Checks if the value of the field is `PCS_1`"]
  #[inline(always)]
  pub fn is_pcs_1(&self) -> bool {
    *self == PCS_A::PCS_1
  }
  #[doc = "Checks if the value of the field is `PCS_2`"]
  #[inline(always)]
  pub fn is_pcs_2(&self) -> bool {
    *self == PCS_A::PCS_2
  }
  #[doc = "Checks if the value of the field is `PCS_3`"]
  #[inline(always)]
  pub fn is_pcs_3(&self) -> bool {
    *self == PCS_A::PCS_3
  }
  #[doc = "Checks if the value of the field is `PCS_4`"]
  #[inline(always)]
  pub fn is_pcs_4(&self) -> bool {
    *self == PCS_A::PCS_4
  }
  #[doc = "Checks if the value of the field is `PCS_5`"]
  #[inline(always)]
  pub fn is_pcs_5(&self) -> bool {
    *self == PCS_A::PCS_5
  }
  #[doc = "Checks if the value of the field is `PCS_6`"]
  #[inline(always)]
  pub fn is_pcs_6(&self) -> bool {
    *self == PCS_A::PCS_6
  }
  #[doc = "Checks if the value of the field is `PCS_7`"]
  #[inline(always)]
  pub fn is_pcs_7(&self) -> bool {
    *self == PCS_A::PCS_7
  }
}
#[doc = "Write proxy for field `PCS`"]
pub struct PCS_W<'a> {
  w: &'a mut W,
}
impl<'a> PCS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PCS_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Clock is off."]
  #[inline(always)]
  pub fn pcs_0(self) -> &'a mut W {
    self.variant(PCS_A::PCS_0)
  }
  #[doc = "Clock option 1"]
  #[inline(always)]
  pub fn pcs_1(self) -> &'a mut W {
    self.variant(PCS_A::PCS_1)
  }
  #[doc = "Clock option 2"]
  #[inline(always)]
  pub fn pcs_2(self) -> &'a mut W {
    self.variant(PCS_A::PCS_2)
  }
  #[doc = "Clock option 3"]
  #[inline(always)]
  pub fn pcs_3(self) -> &'a mut W {
    self.variant(PCS_A::PCS_3)
  }
  #[doc = "Clock option 4"]
  #[inline(always)]
  pub fn pcs_4(self) -> &'a mut W {
    self.variant(PCS_A::PCS_4)
  }
  #[doc = "Clock option 5"]
  #[inline(always)]
  pub fn pcs_5(self) -> &'a mut W {
    self.variant(PCS_A::PCS_5)
  }
  #[doc = "Clock option 6"]
  #[inline(always)]
  pub fn pcs_6(self) -> &'a mut W {
    self.variant(PCS_A::PCS_6)
  }
  #[doc = "Clock option 7"]
  #[inline(always)]
  pub fn pcs_7(self) -> &'a mut W {
    self.variant(PCS_A::PCS_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
    self.w
  }
}
#[doc = "In use flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INUSE_A {
  #[doc = "0: Peripheral is not being used."]
  INUSE_0,
  #[doc = "1: Peripheral is being used. Software cannot modify the existing clocking configuration."]
  INUSE_1,
}
impl From<INUSE_A> for bool {
  #[inline(always)]
  fn from(variant: INUSE_A) -> Self {
    match variant {
      INUSE_A::INUSE_0 => false,
      INUSE_A::INUSE_1 => true,
    }
  }
}
#[doc = "Reader of field `INUSE`"]
pub type INUSE_R = crate::R<bool, INUSE_A>;
impl INUSE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> INUSE_A {
    match self.bits {
      false => INUSE_A::INUSE_0,
      true => INUSE_A::INUSE_1,
    }
  }
  #[doc = "Checks if the value of the field is `INUSE_0`"]
  #[inline(always)]
  pub fn is_inuse_0(&self) -> bool {
    *self == INUSE_A::INUSE_0
  }
  #[doc = "Checks if the value of the field is `INUSE_1`"]
  #[inline(always)]
  pub fn is_inuse_1(&self) -> bool {
    *self == INUSE_A::INUSE_1
  }
}
#[doc = "Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGC_A {
  #[doc = "0: Clock disabled"]
  CGC_0,
  #[doc = "1: Clock enabled. The current clock selection and divider options are locked."]
  CGC_1,
}
impl From<CGC_A> for bool {
  #[inline(always)]
  fn from(variant: CGC_A) -> Self {
    match variant {
      CGC_A::CGC_0 => false,
      CGC_A::CGC_1 => true,
    }
  }
}
#[doc = "Reader of field `CGC`"]
pub type CGC_R = crate::R<bool, CGC_A>;
impl CGC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CGC_A {
    match self.bits {
      false => CGC_A::CGC_0,
      true => CGC_A::CGC_1,
    }
  }
  #[doc = "Checks if the value of the field is `CGC_0`"]
  #[inline(always)]
  pub fn is_cgc_0(&self) -> bool {
    *self == CGC_A::CGC_0
  }
  #[doc = "Checks if the value of the field is `CGC_1`"]
  #[inline(always)]
  pub fn is_cgc_1(&self) -> bool {
    *self == CGC_A::CGC_1
  }
}
#[doc = "Write proxy for field `CGC`"]
pub struct CGC_W<'a> {
  w: &'a mut W,
}
impl<'a> CGC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CGC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn cgc_0(self) -> &'a mut W {
    self.variant(CGC_A::CGC_0)
  }
  #[doc = "Clock enabled. The current clock selection and divider options are locked."]
  #[inline(always)]
  pub fn cgc_1(self) -> &'a mut W {
    self.variant(CGC_A::CGC_1)
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
#[doc = "Present\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR_A {
  #[doc = "0: Peripheral is not present."]
  PR_0,
  #[doc = "1: Peripheral is present."]
  PR_1,
}
impl From<PR_A> for bool {
  #[inline(always)]
  fn from(variant: PR_A) -> Self {
    match variant {
      PR_A::PR_0 => false,
      PR_A::PR_1 => true,
    }
  }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, PR_A>;
impl PR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PR_A {
    match self.bits {
      false => PR_A::PR_0,
      true => PR_A::PR_1,
    }
  }
  #[doc = "Checks if the value of the field is `PR_0`"]
  #[inline(always)]
  pub fn is_pr_0(&self) -> bool {
    *self == PR_A::PR_0
  }
  #[doc = "Checks if the value of the field is `PR_1`"]
  #[inline(always)]
  pub fn is_pr_1(&self) -> bool {
    *self == PR_A::PR_1
  }
}
impl R {
  #[doc = "Bits 0:2 - Peripheral Clock Divider Select"]
  #[inline(always)]
  pub fn pcd(&self) -> PCD_R {
    PCD_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bit 3 - Peripheral Clock Divider Fraction"]
  #[inline(always)]
  pub fn frac(&self) -> FRAC_R {
    FRAC_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 24:26 - Peripheral Clock Source Select"]
  #[inline(always)]
  pub fn pcs(&self) -> PCS_R {
    PCS_R::new(((self.bits >> 24) & 0x07) as u8)
  }
  #[doc = "Bit 29 - In use flag"]
  #[inline(always)]
  pub fn inuse(&self) -> INUSE_R {
    INUSE_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Clock Gate Control"]
  #[inline(always)]
  pub fn cgc(&self) -> CGC_R {
    CGC_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Present"]
  #[inline(always)]
  pub fn pr(&self) -> PR_R {
    PR_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:2 - Peripheral Clock Divider Select"]
  #[inline(always)]
  pub fn pcd(&mut self) -> PCD_W {
    PCD_W { w: self }
  }
  #[doc = "Bit 3 - Peripheral Clock Divider Fraction"]
  #[inline(always)]
  pub fn frac(&mut self) -> FRAC_W {
    FRAC_W { w: self }
  }
  #[doc = "Bits 24:26 - Peripheral Clock Source Select"]
  #[inline(always)]
  pub fn pcs(&mut self) -> PCS_W {
    PCS_W { w: self }
  }
  #[doc = "Bit 30 - Clock Gate Control"]
  #[inline(always)]
  pub fn cgc(&mut self) -> CGC_W {
    CGC_W { w: self }
  }
}
