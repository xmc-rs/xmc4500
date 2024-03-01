#[doc = "Register `ECRD` reader"]
pub type R = crate::R<EcrdSpec>;
#[doc = "Field `CAPV` reader - Timer Capture Value"]
pub type CapvR = crate::FieldReader<u16>;
#[doc = "Field `FPCV` reader - Prescaler Capture value"]
pub type FpcvR = crate::FieldReader;
#[doc = "Slice pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sptr {
    #[doc = "0: CC40"]
    Value1 = 0,
    #[doc = "1: CC41"]
    Value2 = 1,
    #[doc = "2: CC42"]
    Value3 = 2,
    #[doc = "3: CC43"]
    Value4 = 3,
}
impl From<Sptr> for u8 {
    #[inline(always)]
    fn from(variant: Sptr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sptr {
    type Ux = u8;
}
#[doc = "Field `SPTR` reader - Slice pointer"]
pub type SptrR = crate::FieldReader<Sptr>;
impl SptrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sptr {
        match self.bits {
            0 => Sptr::Value1,
            1 => Sptr::Value2,
            2 => Sptr::Value3,
            3 => Sptr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CC40"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sptr::Value1
    }
    #[doc = "CC41"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sptr::Value2
    }
    #[doc = "CC42"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sptr::Value3
    }
    #[doc = "CC43"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sptr::Value4
    }
}
#[doc = "Capture register pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vptr {
    #[doc = "0: Capture register 0"]
    Value1 = 0,
    #[doc = "1: Capture register 1"]
    Value2 = 1,
    #[doc = "2: Capture register 2"]
    Value3 = 2,
    #[doc = "3: Capture register 3"]
    Value4 = 3,
}
impl From<Vptr> for u8 {
    #[inline(always)]
    fn from(variant: Vptr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vptr {
    type Ux = u8;
}
#[doc = "Field `VPTR` reader - Capture register pointer"]
pub type VptrR = crate::FieldReader<Vptr>;
impl VptrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vptr {
        match self.bits {
            0 => Vptr::Value1,
            1 => Vptr::Value2,
            2 => Vptr::Value3,
            3 => Vptr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture register 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vptr::Value1
    }
    #[doc = "Capture register 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vptr::Value2
    }
    #[doc = "Capture register 2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Vptr::Value3
    }
    #[doc = "Capture register 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Vptr::Value4
    }
}
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffl {
    #[doc = "0: No new value was captured into this register"]
    Value1 = 0,
    #[doc = "1: A new value has been captured into this register"]
    Value2 = 1,
}
impl From<Ffl> for bool {
    #[inline(always)]
    fn from(variant: Ffl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFL` reader - Full Flag"]
pub type FflR = crate::BitReader<Ffl>;
impl FflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffl {
        match self.bits {
            false => Ffl::Value1,
            true => Ffl::Value2,
        }
    }
    #[doc = "No new value was captured into this register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ffl::Value1
    }
    #[doc = "A new value has been captured into this register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ffl::Value2
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Capture Value"]
    #[inline(always)]
    pub fn capv(&self) -> CapvR {
        CapvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Prescaler Capture value"]
    #[inline(always)]
    pub fn fpcv(&self) -> FpcvR {
        FpcvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Slice pointer"]
    #[inline(always)]
    pub fn sptr(&self) -> SptrR {
        SptrR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Capture register pointer"]
    #[inline(always)]
    pub fn vptr(&self) -> VptrR {
        VptrR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Full Flag"]
    #[inline(always)]
    pub fn ffl(&self) -> FflR {
        FflR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Extended Capture Mode Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecrd::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrdSpec;
impl crate::RegisterSpec for EcrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecrd::R`](R) reader structure"]
impl crate::Readable for EcrdSpec {}
#[doc = "`reset()` method sets ECRD to value 0"]
impl crate::Resettable for EcrdSpec {
    const RESET_VALUE: u32 = 0;
}
