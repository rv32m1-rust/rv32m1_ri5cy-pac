#[doc = "Reader of register RSTGT_R"]
pub type R = crate::R<u16, super::RSTGT_R>;
#[doc = "Reader of field `RSTGTN`"]
pub type RSTGTN_R = crate::R<u8, u8>;
#[doc = "Reader of field `RSTGMS`"]
pub type RSTGMS_R = crate::R<u8, u8>;
#[doc = "RSTGSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTGSM_A {
  #[doc = "0: Idle, waiting for the first data pattern write."]
  RSTGSM_0,
  #[doc = "1: Waiting for the second data pattern write."]
  RSTGSM_1,
  #[doc = "2: The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The \"01\" state persists for only one clock cycle. Software cannot observe this state."]
  RSTGSM_2,
  #[doc = "3: This state encoding is never used and therefore reserved."]
  RSTGSM_3,
}
impl From<RSTGSM_A> for u8 {
  #[inline(always)]
  fn from(variant: RSTGSM_A) -> Self {
    match variant {
      RSTGSM_A::RSTGSM_0 => 0,
      RSTGSM_A::RSTGSM_1 => 1,
      RSTGSM_A::RSTGSM_2 => 2,
      RSTGSM_A::RSTGSM_3 => 3,
    }
  }
}
#[doc = "Reader of field `RSTGSM`"]
pub type RSTGSM_R = crate::R<u8, RSTGSM_A>;
impl RSTGSM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSTGSM_A {
    match self.bits {
      0 => RSTGSM_A::RSTGSM_0,
      1 => RSTGSM_A::RSTGSM_1,
      2 => RSTGSM_A::RSTGSM_2,
      3 => RSTGSM_A::RSTGSM_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `RSTGSM_0`"]
  #[inline(always)]
  pub fn is_rstgsm_0(&self) -> bool {
    *self == RSTGSM_A::RSTGSM_0
  }
  #[doc = "Checks if the value of the field is `RSTGSM_1`"]
  #[inline(always)]
  pub fn is_rstgsm_1(&self) -> bool {
    *self == RSTGSM_A::RSTGSM_1
  }
  #[doc = "Checks if the value of the field is `RSTGSM_2`"]
  #[inline(always)]
  pub fn is_rstgsm_2(&self) -> bool {
    *self == RSTGSM_A::RSTGSM_2
  }
  #[doc = "Checks if the value of the field is `RSTGSM_3`"]
  #[inline(always)]
  pub fn is_rstgsm_3(&self) -> bool {
    *self == RSTGSM_A::RSTGSM_3
  }
}
#[doc = "Reader of field `ROZ`"]
pub type ROZ_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - RSTGTN"]
  #[inline(always)]
  pub fn rstgtn(&self) -> RSTGTN_R {
    RSTGTN_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:11 - RSTGMS"]
  #[inline(always)]
  pub fn rstgms(&self) -> RSTGMS_R {
    RSTGMS_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
  #[doc = "Bits 12:13 - RSTGSM"]
  #[inline(always)]
  pub fn rstgsm(&self) -> RSTGSM_R {
    RSTGSM_R::new(((self.bits >> 12) & 0x03) as u8)
  }
  #[doc = "Bits 14:15 - ROZ"]
  #[inline(always)]
  pub fn roz(&self) -> ROZ_R {
    ROZ_R::new(((self.bits >> 14) & 0x03) as u8)
  }
}
