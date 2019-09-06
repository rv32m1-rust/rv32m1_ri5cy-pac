#[doc = "Reader of register AUTOCMD12_ERR_STATUS"]
pub type R = crate::R<u32, super::AUTOCMD12_ERR_STATUS>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12NE_A {
  #[doc = "0: Executed"]
  AC12NE_0,
  #[doc = "1: Not executed"]
  AC12NE_1,
}
impl From<AC12NE_A> for bool {
  #[inline(always)]
  fn from(variant: AC12NE_A) -> Self {
    match variant {
      AC12NE_A::AC12NE_0 => false,
      AC12NE_A::AC12NE_1 => true,
    }
  }
}
#[doc = "Reader of field `AC12NE`"]
pub type AC12NE_R = crate::R<bool, AC12NE_A>;
impl AC12NE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> AC12NE_A {
    match self.bits {
      false => AC12NE_A::AC12NE_0,
      true => AC12NE_A::AC12NE_1,
    }
  }
  #[doc = "Checks if the value of the field is `AC12NE_0`"]
  #[inline(always)]
  pub fn is_ac12ne_0(&self) -> bool {
    *self == AC12NE_A::AC12NE_0
  }
  #[doc = "Checks if the value of the field is `AC12NE_1`"]
  #[inline(always)]
  pub fn is_ac12ne_1(&self) -> bool {
    *self == AC12NE_A::AC12NE_1
  }
}
#[doc = "Auto CMD12 / 23 Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12TOE_A {
  #[doc = "0: No error"]
  AC12TOE_0,
  #[doc = "1: Time out"]
  AC12TOE_1,
}
impl From<AC12TOE_A> for bool {
  #[inline(always)]
  fn from(variant: AC12TOE_A) -> Self {
    match variant {
      AC12TOE_A::AC12TOE_0 => false,
      AC12TOE_A::AC12TOE_1 => true,
    }
  }
}
#[doc = "Reader of field `AC12TOE`"]
pub type AC12TOE_R = crate::R<bool, AC12TOE_A>;
impl AC12TOE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> AC12TOE_A {
    match self.bits {
      false => AC12TOE_A::AC12TOE_0,
      true => AC12TOE_A::AC12TOE_1,
    }
  }
  #[doc = "Checks if the value of the field is `AC12TOE_0`"]
  #[inline(always)]
  pub fn is_ac12toe_0(&self) -> bool {
    *self == AC12TOE_A::AC12TOE_0
  }
  #[doc = "Checks if the value of the field is `AC12TOE_1`"]
  #[inline(always)]
  pub fn is_ac12toe_1(&self) -> bool {
    *self == AC12TOE_A::AC12TOE_1
  }
}
#[doc = "Auto CMD12 / 23 End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EBE_A {
  #[doc = "0: No error"]
  AC12EBE_0,
  #[doc = "1: End Bit Error Generated"]
  AC12EBE_1,
}
impl From<AC12EBE_A> for bool {
  #[inline(always)]
  fn from(variant: AC12EBE_A) -> Self {
    match variant {
      AC12EBE_A::AC12EBE_0 => false,
      AC12EBE_A::AC12EBE_1 => true,
    }
  }
}
#[doc = "Reader of field `AC12EBE`"]
pub type AC12EBE_R = crate::R<bool, AC12EBE_A>;
impl AC12EBE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> AC12EBE_A {
    match self.bits {
      false => AC12EBE_A::AC12EBE_0,
      true => AC12EBE_A::AC12EBE_1,
    }
  }
  #[doc = "Checks if the value of the field is `AC12EBE_0`"]
  #[inline(always)]
  pub fn is_ac12ebe_0(&self) -> bool {
    *self == AC12EBE_A::AC12EBE_0
  }
  #[doc = "Checks if the value of the field is `AC12EBE_1`"]
  #[inline(always)]
  pub fn is_ac12ebe_1(&self) -> bool {
    *self == AC12EBE_A::AC12EBE_1
  }
}
#[doc = "Auto CMD12 / 23 CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12CE_A {
  #[doc = "0: No CRC error"]
  AC12CE_0,
  #[doc = "1: CRC Error Met in Auto CMD12/23 Response"]
  AC12CE_1,
}
impl From<AC12CE_A> for bool {
  #[inline(always)]
  fn from(variant: AC12CE_A) -> Self {
    match variant {
      AC12CE_A::AC12CE_0 => false,
      AC12CE_A::AC12CE_1 => true,
    }
  }
}
#[doc = "Reader of field `AC12CE`"]
pub type AC12CE_R = crate::R<bool, AC12CE_A>;
impl AC12CE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> AC12CE_A {
    match self.bits {
      false => AC12CE_A::AC12CE_0,
      true => AC12CE_A::AC12CE_1,
    }
  }
  #[doc = "Checks if the value of the field is `AC12CE_0`"]
  #[inline(always)]
  pub fn is_ac12ce_0(&self) -> bool {
    *self == AC12CE_A::AC12CE_0
  }
  #[doc = "Checks if the value of the field is `AC12CE_1`"]
  #[inline(always)]
  pub fn is_ac12ce_1(&self) -> bool {
    *self == AC12CE_A::AC12CE_1
  }
}
#[doc = "Auto CMD12 / 23 Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12IE_A {
  #[doc = "0: No error"]
  AC12IE_0,
  #[doc = "1: Error, the CMD index in response is not CMD12/23"]
  AC12IE_1,
}
impl From<AC12IE_A> for bool {
  #[inline(always)]
  fn from(variant: AC12IE_A) -> Self {
    match variant {
      AC12IE_A::AC12IE_0 => false,
      AC12IE_A::AC12IE_1 => true,
    }
  }
}
#[doc = "Reader of field `AC12IE`"]
pub type AC12IE_R = crate::R<bool, AC12IE_A>;
impl AC12IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> AC12IE_A {
    match self.bits {
      false => AC12IE_A::AC12IE_0,
      true => AC12IE_A::AC12IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `AC12IE_0`"]
  #[inline(always)]
  pub fn is_ac12ie_0(&self) -> bool {
    *self == AC12IE_A::AC12IE_0
  }
  #[doc = "Checks if the value of the field is `AC12IE_1`"]
  #[inline(always)]
  pub fn is_ac12ie_1(&self) -> bool {
    *self == AC12IE_A::AC12IE_1
  }
}
#[doc = "Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNIBAC12E_A {
  #[doc = "0: No error"]
  CNIBAC12E_0,
  #[doc = "1: Not Issued"]
  CNIBAC12E_1,
}
impl From<CNIBAC12E_A> for bool {
  #[inline(always)]
  fn from(variant: CNIBAC12E_A) -> Self {
    match variant {
      CNIBAC12E_A::CNIBAC12E_0 => false,
      CNIBAC12E_A::CNIBAC12E_1 => true,
    }
  }
}
#[doc = "Reader of field `CNIBAC12E`"]
pub type CNIBAC12E_R = crate::R<bool, CNIBAC12E_A>;
impl CNIBAC12E_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CNIBAC12E_A {
    match self.bits {
      false => CNIBAC12E_A::CNIBAC12E_0,
      true => CNIBAC12E_A::CNIBAC12E_1,
    }
  }
  #[doc = "Checks if the value of the field is `CNIBAC12E_0`"]
  #[inline(always)]
  pub fn is_cnibac12e_0(&self) -> bool {
    *self == CNIBAC12E_A::CNIBAC12E_0
  }
  #[doc = "Checks if the value of the field is `CNIBAC12E_1`"]
  #[inline(always)]
  pub fn is_cnibac12e_1(&self) -> bool {
    *self == CNIBAC12E_A::CNIBAC12E_1
  }
}
impl R {
  #[doc = "Bit 0 - Auto CMD12 Not Executed"]
  #[inline(always)]
  pub fn ac12ne(&self) -> AC12NE_R {
    AC12NE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Auto CMD12 / 23 Timeout Error"]
  #[inline(always)]
  pub fn ac12toe(&self) -> AC12TOE_R {
    AC12TOE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Auto CMD12 / 23 End Bit Error"]
  #[inline(always)]
  pub fn ac12ebe(&self) -> AC12EBE_R {
    AC12EBE_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Auto CMD12 / 23 CRC Error"]
  #[inline(always)]
  pub fn ac12ce(&self) -> AC12CE_R {
    AC12CE_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Auto CMD12 / 23 Index Error"]
  #[inline(always)]
  pub fn ac12ie(&self) -> AC12IE_R {
    AC12IE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
  #[inline(always)]
  pub fn cnibac12e(&self) -> CNIBAC12E_R {
    CNIBAC12E_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
