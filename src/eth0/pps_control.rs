#[doc = "Register `PPS_CONTROL` reader"]
pub type R = crate::R<PpsControlSpec>;
#[doc = "Register `PPS_CONTROL` writer"]
pub type W = crate::W<PpsControlSpec>;
#[doc = "Field `PPSCTRL_PPSCMD` reader - PPSCTRL0 or PPSCMD0"]
pub type PpsctrlPpscmdR = crate::FieldReader;
#[doc = "Field `PPSCTRL_PPSCMD` writer - PPSCTRL0 or PPSCMD0"]
pub type PpsctrlPpscmdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable"]
pub type Ppsen0R = crate::BitReader;
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS0 Output"]
pub type Trgtmodsel0R = crate::FieldReader;
#[doc = "Field `PPSCMD1` reader - Flexible PPS1 Output Control"]
pub type Ppscmd1R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL1` reader - Target Time Register Mode for PPS1 Output"]
pub type Trgtmodsel1R = crate::FieldReader;
#[doc = "Field `PPSCMD2` reader - Flexible PPS2 Output Control"]
pub type Ppscmd2R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL2` reader - Target Time Register Mode for PPS2 Output"]
pub type Trgtmodsel2R = crate::FieldReader;
#[doc = "Field `PPSCMD3` reader - Flexible PPS3 Output Control"]
pub type Ppscmd3R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL3` reader - Target Time Register Mode for PPS3 Output"]
pub type Trgtmodsel3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL0 or PPSCMD0"]
    #[inline(always)]
    pub fn ppsctrl_ppscmd(&self) -> PpsctrlPpscmdR {
        PpsctrlPpscmdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&self) -> Ppsen0R {
        Ppsen0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> Trgtmodsel0R {
        Trgtmodsel0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control"]
    #[inline(always)]
    pub fn ppscmd1(&self) -> Ppscmd1R {
        Ppscmd1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output"]
    #[inline(always)]
    pub fn trgtmodsel1(&self) -> Trgtmodsel1R {
        Trgtmodsel1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control"]
    #[inline(always)]
    pub fn ppscmd2(&self) -> Ppscmd2R {
        Ppscmd2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output"]
    #[inline(always)]
    pub fn trgtmodsel2(&self) -> Trgtmodsel2R {
        Trgtmodsel2R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control"]
    #[inline(always)]
    pub fn ppscmd3(&self) -> Ppscmd3R {
        Ppscmd3R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output"]
    #[inline(always)]
    pub fn trgtmodsel3(&self) -> Trgtmodsel3R {
        Trgtmodsel3R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL0 or PPSCMD0"]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl_ppscmd(&mut self) -> PpsctrlPpscmdW<PpsControlSpec> {
        PpsctrlPpscmdW::new(self, 0)
    }
}
#[doc = "PPS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpsControlSpec;
impl crate::RegisterSpec for PpsControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps_control::R`](R) reader structure"]
impl crate::Readable for PpsControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pps_control::W`](W) writer structure"]
impl crate::Writable for PpsControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPS_CONTROL to value 0"]
impl crate::Resettable for PpsControlSpec {
    const RESET_VALUE: u32 = 0;
}
