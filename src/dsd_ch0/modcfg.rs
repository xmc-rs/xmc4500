#[doc = "Register `MODCFG` reader"]
pub type R = crate::R<MODCFG_SPEC>;
#[doc = "Register `MODCFG` writer"]
pub type W = crate::W<MODCFG_SPEC>;
#[doc = "Field `DIVM` reader - Divider Factor for Modulator Clock"]
pub type DIVM_R = crate::FieldReader<DIVM_A>;
#[doc = "Divider Factor for Modulator Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: fMOD = fCLK / 2"]
    VALUE1 = 0,
    #[doc = "1: fMOD = fCLK / 4"]
    VALUE2 = 1,
    #[doc = "2: fMOD = fCLK / 6"]
    VALUE3 = 2,
    #[doc = "15: fMOD = fCLK / 32"]
    VALUE4 = 15,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVM_A {
    type Ux = u8;
}
impl DIVM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIVM_A> {
        match self.bits {
            0 => Some(DIVM_A::VALUE1),
            1 => Some(DIVM_A::VALUE2),
            2 => Some(DIVM_A::VALUE3),
            15 => Some(DIVM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "fMOD = fCLK / 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVM_A::VALUE1
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVM_A::VALUE2
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVM_A::VALUE3
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVM_A::VALUE4
    }
}
#[doc = "Field `DIVM` writer - Divider Factor for Modulator Clock"]
pub type DIVM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DIVM_A>;
impl<'a, REG, const O: u8> DIVM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fMOD = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::VALUE1)
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::VALUE2)
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::VALUE3)
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::VALUE4)
    }
}
#[doc = "Write Control for Divider Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DWC_AW {
    #[doc = "0: No write access to divider factor"]
    VALUE1 = 0,
    #[doc = "1: Bitfield DIVM can be written"]
    VALUE2 = 1,
}
impl From<DWC_AW> for bool {
    #[inline(always)]
    fn from(variant: DWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWC` writer - Write Control for Divider Factor"]
pub type DWC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DWC_AW>;
impl<'a, REG, const O: u8> DWC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to divider factor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DWC_AW::VALUE1)
    }
    #[doc = "Bitfield DIVM can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DWC_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    #[must_use]
    pub fn divm(&mut self) -> DIVM_W<MODCFG_SPEC, 16> {
        DIVM_W::new(self)
    }
    #[doc = "Bit 23 - Write Control for Divider Factor"]
    #[inline(always)]
    #[must_use]
    pub fn dwc(&mut self) -> DWC_W<MODCFG_SPEC, 23> {
        DWC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Modulator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODCFG_SPEC;
impl crate::RegisterSpec for MODCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modcfg::R`](R) reader structure"]
impl crate::Readable for MODCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modcfg::W`](W) writer structure"]
impl crate::Writable for MODCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODCFG to value 0"]
impl crate::Resettable for MODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
