#[doc = "Register `FDR` reader"]
pub type R = crate::R<FDR_SPEC>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FDR_SPEC>;
#[doc = "Field `STEP` reader - Step Value"]
pub type STEP_R = crate::FieldReader<u16>;
#[doc = "Field `STEP` writer - Step Value"]
pub type STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SM` reader - Suspend Mode"]
pub type SM_R = crate::BitReader;
#[doc = "Field `SM` writer - Suspend Mode"]
pub type SM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC` reader - Suspend Control"]
pub type SC_R = crate::FieldReader;
#[doc = "Field `SC` writer - Suspend Control"]
pub type SC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DM` reader - Divider Mode"]
pub type DM_R = crate::FieldReader;
#[doc = "Field `DM` writer - Divider Mode"]
pub type DM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESULT` reader - Result Value"]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `SUSACK` reader - Suspend Mode Acknowledge"]
pub type SUSACK_R = crate::BitReader;
#[doc = "Field `SUSREQ` reader - Suspend Mode Request"]
pub type SUSREQ_R = crate::BitReader;
#[doc = "Field `ENHW` reader - Enable Hardware Clock Control"]
pub type ENHW_R = crate::BitReader;
#[doc = "Field `ENHW` writer - Enable Hardware Clock Control"]
pub type ENHW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCLK` reader - Disable Clock"]
pub type DISCLK_R = crate::BitReader;
#[doc = "Field `DISCLK` writer - Disable Clock"]
pub type DISCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Suspend Mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Suspend Control"]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Suspend Mode Acknowledge"]
    #[inline(always)]
    pub fn susack(&self) -> SUSACK_R {
        SUSACK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Suspend Mode Request"]
    #[inline(always)]
    pub fn susreq(&self) -> SUSREQ_R {
        SUSREQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Hardware Clock Control"]
    #[inline(always)]
    pub fn enhw(&self) -> ENHW_R {
        ENHW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable Clock"]
    #[inline(always)]
    pub fn disclk(&self) -> DISCLK_R {
        DISCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<FDR_SPEC> {
        STEP_W::new(self, 0)
    }
    #[doc = "Bit 11 - Suspend Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<FDR_SPEC> {
        SM_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Suspend Control"]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> SC_W<FDR_SPEC> {
        SC_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<FDR_SPEC> {
        DM_W::new(self, 14)
    }
    #[doc = "Bit 30 - Enable Hardware Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn enhw(&mut self) -> ENHW_W<FDR_SPEC> {
        ENHW_W::new(self, 30)
    }
    #[doc = "Bit 31 - Disable Clock"]
    #[inline(always)]
    #[must_use]
    pub fn disclk(&mut self) -> DISCLK_W<FDR_SPEC> {
        DISCLK_W::new(self, 31)
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
#[doc = "CAN Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
