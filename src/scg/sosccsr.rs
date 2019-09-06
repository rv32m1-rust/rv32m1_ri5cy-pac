#[doc = "Reader of register SOSCCSR"]
pub type R = crate::R<u32, super::SOSCCSR>;
#[doc = "Writer for register SOSCCSR"]
pub type W = crate::W<u32, super::SOSCCSR>;
#[doc = "Register SOSCCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SOSCCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "System OSC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCEN_A {
  #[doc = "0: System OSC is disabled"]
  SOSCEN_0,
  #[doc = "1: System OSC is enabled"]
  SOSCEN_1,
}
impl From<SOSCEN_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCEN_A) -> Self {
    match variant {
      SOSCEN_A::SOSCEN_0 => false,
      SOSCEN_A::SOSCEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCEN`"]
pub type SOSCEN_R = crate::R<bool, SOSCEN_A>;
impl SOSCEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCEN_A {
    match self.bits {
      false => SOSCEN_A::SOSCEN_0,
      true => SOSCEN_A::SOSCEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCEN_0`"]
  #[inline(always)]
  pub fn is_soscen_0(&self) -> bool {
    *self == SOSCEN_A::SOSCEN_0
  }
  #[doc = "Checks if the value of the field is `SOSCEN_1`"]
  #[inline(always)]
  pub fn is_soscen_1(&self) -> bool {
    *self == SOSCEN_A::SOSCEN_1
  }
}
#[doc = "Write proxy for field `SOSCEN`"]
pub struct SOSCEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "System OSC is disabled"]
  #[inline(always)]
  pub fn soscen_0(self) -> &'a mut W {
    self.variant(SOSCEN_A::SOSCEN_0)
  }
  #[doc = "System OSC is enabled"]
  #[inline(always)]
  pub fn soscen_1(self) -> &'a mut W {
    self.variant(SOSCEN_A::SOSCEN_1)
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
#[doc = "System OSC Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCSTEN_A {
  #[doc = "0: System OSC is disabled in Stop modes"]
  SOSCSTEN_0,
  #[doc = "1: no description available"]
  SOSCSTEN_1,
}
impl From<SOSCSTEN_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCSTEN_A) -> Self {
    match variant {
      SOSCSTEN_A::SOSCSTEN_0 => false,
      SOSCSTEN_A::SOSCSTEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCSTEN`"]
pub type SOSCSTEN_R = crate::R<bool, SOSCSTEN_A>;
impl SOSCSTEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCSTEN_A {
    match self.bits {
      false => SOSCSTEN_A::SOSCSTEN_0,
      true => SOSCSTEN_A::SOSCSTEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCSTEN_0`"]
  #[inline(always)]
  pub fn is_soscsten_0(&self) -> bool {
    *self == SOSCSTEN_A::SOSCSTEN_0
  }
  #[doc = "Checks if the value of the field is `SOSCSTEN_1`"]
  #[inline(always)]
  pub fn is_soscsten_1(&self) -> bool {
    *self == SOSCSTEN_A::SOSCSTEN_1
  }
}
#[doc = "Write proxy for field `SOSCSTEN`"]
pub struct SOSCSTEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCSTEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCSTEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "System OSC is disabled in Stop modes"]
  #[inline(always)]
  pub fn soscsten_0(self) -> &'a mut W {
    self.variant(SOSCSTEN_A::SOSCSTEN_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn soscsten_1(self) -> &'a mut W {
    self.variant(SOSCSTEN_A::SOSCSTEN_1)
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
#[doc = "System OSC Low Power Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCLPEN_A {
  #[doc = "0: System OSC is disabled in VLP modes"]
  SOSCLPEN_0,
  #[doc = "1: System OSC is enabled in VLP modes"]
  SOSCLPEN_1,
}
impl From<SOSCLPEN_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCLPEN_A) -> Self {
    match variant {
      SOSCLPEN_A::SOSCLPEN_0 => false,
      SOSCLPEN_A::SOSCLPEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCLPEN`"]
pub type SOSCLPEN_R = crate::R<bool, SOSCLPEN_A>;
impl SOSCLPEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCLPEN_A {
    match self.bits {
      false => SOSCLPEN_A::SOSCLPEN_0,
      true => SOSCLPEN_A::SOSCLPEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCLPEN_0`"]
  #[inline(always)]
  pub fn is_sosclpen_0(&self) -> bool {
    *self == SOSCLPEN_A::SOSCLPEN_0
  }
  #[doc = "Checks if the value of the field is `SOSCLPEN_1`"]
  #[inline(always)]
  pub fn is_sosclpen_1(&self) -> bool {
    *self == SOSCLPEN_A::SOSCLPEN_1
  }
}
#[doc = "Write proxy for field `SOSCLPEN`"]
pub struct SOSCLPEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCLPEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCLPEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "System OSC is disabled in VLP modes"]
  #[inline(always)]
  pub fn sosclpen_0(self) -> &'a mut W {
    self.variant(SOSCLPEN_A::SOSCLPEN_0)
  }
  #[doc = "System OSC is enabled in VLP modes"]
  #[inline(always)]
  pub fn sosclpen_1(self) -> &'a mut W {
    self.variant(SOSCLPEN_A::SOSCLPEN_1)
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
#[doc = "System OSC Clock Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCM_A {
  #[doc = "0: System OSC Clock Monitor is disabled"]
  SOSCCM_0,
  #[doc = "1: System OSC Clock Monitor is enabled"]
  SOSCCM_1,
}
impl From<SOSCCM_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCCM_A) -> Self {
    match variant {
      SOSCCM_A::SOSCCM_0 => false,
      SOSCCM_A::SOSCCM_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCCM`"]
pub type SOSCCM_R = crate::R<bool, SOSCCM_A>;
impl SOSCCM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCCM_A {
    match self.bits {
      false => SOSCCM_A::SOSCCM_0,
      true => SOSCCM_A::SOSCCM_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCCM_0`"]
  #[inline(always)]
  pub fn is_sosccm_0(&self) -> bool {
    *self == SOSCCM_A::SOSCCM_0
  }
  #[doc = "Checks if the value of the field is `SOSCCM_1`"]
  #[inline(always)]
  pub fn is_sosccm_1(&self) -> bool {
    *self == SOSCCM_A::SOSCCM_1
  }
}
#[doc = "Write proxy for field `SOSCCM`"]
pub struct SOSCCM_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCCM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCCM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "System OSC Clock Monitor is disabled"]
  #[inline(always)]
  pub fn sosccm_0(self) -> &'a mut W {
    self.variant(SOSCCM_A::SOSCCM_0)
  }
  #[doc = "System OSC Clock Monitor is enabled"]
  #[inline(always)]
  pub fn sosccm_1(self) -> &'a mut W {
    self.variant(SOSCCM_A::SOSCCM_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "System OSC Clock Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCMRE_A {
  #[doc = "0: Clock Monitor generates interrupt when error detected"]
  SOSCCMRE_0,
  #[doc = "1: Clock Monitor generates reset when error detected"]
  SOSCCMRE_1,
}
impl From<SOSCCMRE_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCCMRE_A) -> Self {
    match variant {
      SOSCCMRE_A::SOSCCMRE_0 => false,
      SOSCCMRE_A::SOSCCMRE_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCCMRE`"]
pub type SOSCCMRE_R = crate::R<bool, SOSCCMRE_A>;
impl SOSCCMRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCCMRE_A {
    match self.bits {
      false => SOSCCMRE_A::SOSCCMRE_0,
      true => SOSCCMRE_A::SOSCCMRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCCMRE_0`"]
  #[inline(always)]
  pub fn is_sosccmre_0(&self) -> bool {
    *self == SOSCCMRE_A::SOSCCMRE_0
  }
  #[doc = "Checks if the value of the field is `SOSCCMRE_1`"]
  #[inline(always)]
  pub fn is_sosccmre_1(&self) -> bool {
    *self == SOSCCMRE_A::SOSCCMRE_1
  }
}
#[doc = "Write proxy for field `SOSCCMRE`"]
pub struct SOSCCMRE_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCCMRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCCMRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Clock Monitor generates interrupt when error detected"]
  #[inline(always)]
  pub fn sosccmre_0(self) -> &'a mut W {
    self.variant(SOSCCMRE_A::SOSCCMRE_0)
  }
  #[doc = "Clock Monitor generates reset when error detected"]
  #[inline(always)]
  pub fn sosccmre_1(self) -> &'a mut W {
    self.variant(SOSCCMRE_A::SOSCCMRE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: This Control Status Register can be written."]
  LK_0,
  #[doc = "1: This Control Status Register cannot be written."]
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
  #[doc = "This Control Status Register can be written."]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_A::LK_0)
  }
  #[doc = "This Control Status Register cannot be written."]
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "System OSC Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCVLD_A {
  #[doc = "0: System OSC is not enabled or clock is not valid"]
  SOSCVLD_0,
  #[doc = "1: System OSC is enabled and output clock is valid"]
  SOSCVLD_1,
}
impl From<SOSCVLD_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCVLD_A) -> Self {
    match variant {
      SOSCVLD_A::SOSCVLD_0 => false,
      SOSCVLD_A::SOSCVLD_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCVLD`"]
pub type SOSCVLD_R = crate::R<bool, SOSCVLD_A>;
impl SOSCVLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCVLD_A {
    match self.bits {
      false => SOSCVLD_A::SOSCVLD_0,
      true => SOSCVLD_A::SOSCVLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCVLD_0`"]
  #[inline(always)]
  pub fn is_soscvld_0(&self) -> bool {
    *self == SOSCVLD_A::SOSCVLD_0
  }
  #[doc = "Checks if the value of the field is `SOSCVLD_1`"]
  #[inline(always)]
  pub fn is_soscvld_1(&self) -> bool {
    *self == SOSCVLD_A::SOSCVLD_1
  }
}
#[doc = "System OSC Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCSEL_A {
  #[doc = "0: System OSC is not the system clock source"]
  SOSCSEL_0,
  #[doc = "1: System OSC is the system clock source"]
  SOSCSEL_1,
}
impl From<SOSCSEL_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCSEL_A) -> Self {
    match variant {
      SOSCSEL_A::SOSCSEL_0 => false,
      SOSCSEL_A::SOSCSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCSEL`"]
pub type SOSCSEL_R = crate::R<bool, SOSCSEL_A>;
impl SOSCSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCSEL_A {
    match self.bits {
      false => SOSCSEL_A::SOSCSEL_0,
      true => SOSCSEL_A::SOSCSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCSEL_0`"]
  #[inline(always)]
  pub fn is_soscsel_0(&self) -> bool {
    *self == SOSCSEL_A::SOSCSEL_0
  }
  #[doc = "Checks if the value of the field is `SOSCSEL_1`"]
  #[inline(always)]
  pub fn is_soscsel_1(&self) -> bool {
    *self == SOSCSEL_A::SOSCSEL_1
  }
}
#[doc = "System OSC Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCERR_A {
  #[doc = "0: System OSC Clock Monitor is disabled or has not detected an error"]
  SOSCERR_0,
  #[doc = "1: System OSC Clock Monitor is enabled and detected an error"]
  SOSCERR_1,
}
impl From<SOSCERR_A> for bool {
  #[inline(always)]
  fn from(variant: SOSCERR_A) -> Self {
    match variant {
      SOSCERR_A::SOSCERR_0 => false,
      SOSCERR_A::SOSCERR_1 => true,
    }
  }
}
#[doc = "Reader of field `SOSCERR`"]
pub type SOSCERR_R = crate::R<bool, SOSCERR_A>;
impl SOSCERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCERR_A {
    match self.bits {
      false => SOSCERR_A::SOSCERR_0,
      true => SOSCERR_A::SOSCERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOSCERR_0`"]
  #[inline(always)]
  pub fn is_soscerr_0(&self) -> bool {
    *self == SOSCERR_A::SOSCERR_0
  }
  #[doc = "Checks if the value of the field is `SOSCERR_1`"]
  #[inline(always)]
  pub fn is_soscerr_1(&self) -> bool {
    *self == SOSCERR_A::SOSCERR_1
  }
}
#[doc = "Write proxy for field `SOSCERR`"]
pub struct SOSCERR_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "System OSC Clock Monitor is disabled or has not detected an error"]
  #[inline(always)]
  pub fn soscerr_0(self) -> &'a mut W {
    self.variant(SOSCERR_A::SOSCERR_0)
  }
  #[doc = "System OSC Clock Monitor is enabled and detected an error"]
  #[inline(always)]
  pub fn soscerr_1(self) -> &'a mut W {
    self.variant(SOSCERR_A::SOSCERR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - System OSC Enable"]
  #[inline(always)]
  pub fn soscen(&self) -> SOSCEN_R {
    SOSCEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - System OSC Stop Enable"]
  #[inline(always)]
  pub fn soscsten(&self) -> SOSCSTEN_R {
    SOSCSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - System OSC Low Power Enable"]
  #[inline(always)]
  pub fn sosclpen(&self) -> SOSCLPEN_R {
    SOSCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 16 - System OSC Clock Monitor"]
  #[inline(always)]
  pub fn sosccm(&self) -> SOSCCM_R {
    SOSCCM_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
  #[inline(always)]
  pub fn sosccmre(&self) -> SOSCCMRE_R {
    SOSCCMRE_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - System OSC Valid"]
  #[inline(always)]
  pub fn soscvld(&self) -> SOSCVLD_R {
    SOSCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - System OSC Selected"]
  #[inline(always)]
  pub fn soscsel(&self) -> SOSCSEL_R {
    SOSCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - System OSC Clock Error"]
  #[inline(always)]
  pub fn soscerr(&self) -> SOSCERR_R {
    SOSCERR_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - System OSC Enable"]
  #[inline(always)]
  pub fn soscen(&mut self) -> SOSCEN_W {
    SOSCEN_W { w: self }
  }
  #[doc = "Bit 1 - System OSC Stop Enable"]
  #[inline(always)]
  pub fn soscsten(&mut self) -> SOSCSTEN_W {
    SOSCSTEN_W { w: self }
  }
  #[doc = "Bit 2 - System OSC Low Power Enable"]
  #[inline(always)]
  pub fn sosclpen(&mut self) -> SOSCLPEN_W {
    SOSCLPEN_W { w: self }
  }
  #[doc = "Bit 16 - System OSC Clock Monitor"]
  #[inline(always)]
  pub fn sosccm(&mut self) -> SOSCCM_W {
    SOSCCM_W { w: self }
  }
  #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
  #[inline(always)]
  pub fn sosccmre(&mut self) -> SOSCCMRE_W {
    SOSCCMRE_W { w: self }
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
  #[doc = "Bit 26 - System OSC Clock Error"]
  #[inline(always)]
  pub fn soscerr(&mut self) -> SOSCERR_W {
    SOSCERR_W { w: self }
  }
}
