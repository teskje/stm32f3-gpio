# ! [ allow ( non_camel_case_types ) ]
# ! [ allow ( non_snake_case ) ]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "General-purpose I/Os, port A"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os, port A"]
pub mod gpioa {
    #[doc = r"Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - GPIO port mode register"]
        pub moder: MODER,
        #[doc = "0x04 - GPIO port output type register"]
        pub otyper: OTYPER,
        #[doc = "0x08 - GPIO port output speed register"]
        pub ospeedr: OSPEEDR,
        #[doc = "0x0c - GPIO port pull-up/pull-down register"]
        pub pupdr: PUPDR,
        #[doc = "0x10 - GPIO port input data register"]
        pub idr: IDR,
        #[doc = "0x14 - GPIO port output data register"]
        pub udr: UDR,
        #[doc = "0x18 - GPIO port bit set/reset register"]
        pub bsrr: BSRR,
        #[doc = "0x1c - GPIO port configuration lock register"]
        pub lckr: LCKR,
        #[doc = "0x20 - GPIO alternate function low register"]
        pub afrl: AFRL,
        #[doc = "0x24 - GPIO alternate function high register"]
        pub afrh: AFRH,
        #[doc = "0x28 - GPIO port bit reset register"]
        pub brr: BRR,
    }
    #[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](moder) module"]
    pub type MODER = crate::Reg<u32, _MODER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _MODER;
    #[doc = "`read()` method returns [moder::R](moder::R) reader structure"]
    impl crate::Readable for MODER {}
    #[doc = "`write(|w| ..)` method takes [moder::W](moder::W) writer structure"]
    impl crate::Writable for MODER {}
    #[doc = "GPIO port mode register"]
    pub mod moder {
        #[doc = "Reader of register MODER"]
        pub type R = crate::R<u32, super::MODER>;
        #[doc = "Writer for register MODER"]
        pub type W = crate::W<u32, super::MODER>;
        #[doc = "Register MODER `reset()`'s with value 0xa800_0000"]
        impl crate::ResetValue for super::MODER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0xa800_0000
            }
        }
        #[doc = "Reader of field `MODER0`"]
        pub type MODER0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER0`"]
        pub struct MODER0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `MODER1`"]
        pub type MODER1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER1`"]
        pub struct MODER1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `MODER2`"]
        pub type MODER2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER2`"]
        pub struct MODER2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `MODER3`"]
        pub type MODER3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER3`"]
        pub struct MODER3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `MODER4`"]
        pub type MODER4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER4`"]
        pub struct MODER4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `MODER5`"]
        pub type MODER5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER5`"]
        pub struct MODER5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `MODER6`"]
        pub type MODER6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER6`"]
        pub struct MODER6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `MODER7`"]
        pub type MODER7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER7`"]
        pub struct MODER7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `MODER8`"]
        pub type MODER8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER8`"]
        pub struct MODER8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `MODER9`"]
        pub type MODER9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER9`"]
        pub struct MODER9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `MODER10`"]
        pub type MODER10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER10`"]
        pub struct MODER10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `MODER11`"]
        pub type MODER11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER11`"]
        pub struct MODER11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `MODER12`"]
        pub type MODER12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER12`"]
        pub struct MODER12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `MODER13`"]
        pub type MODER13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER13`"]
        pub struct MODER13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `MODER14`"]
        pub type MODER14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER14`"]
        pub struct MODER14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `MODER15`"]
        pub type MODER15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER15`"]
        pub struct MODER15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&self) -> MODER0_R {
                MODER0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&self) -> MODER1_R {
                MODER1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&self) -> MODER2_R {
                MODER2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&self) -> MODER3_R {
                MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&self) -> MODER4_R {
                MODER4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&self) -> MODER5_R {
                MODER5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&self) -> MODER6_R {
                MODER6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&self) -> MODER7_R {
                MODER7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&self) -> MODER8_R {
                MODER8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&self) -> MODER9_R {
                MODER9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&self) -> MODER10_R {
                MODER10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&self) -> MODER11_R {
                MODER11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&self) -> MODER12_R {
                MODER12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&self) -> MODER13_R {
                MODER13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&self) -> MODER14_R {
                MODER14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&self) -> MODER15_R {
                MODER15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&mut self) -> MODER0_W {
                MODER0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&mut self) -> MODER1_W {
                MODER1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&mut self) -> MODER2_W {
                MODER2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&mut self) -> MODER3_W {
                MODER3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&mut self) -> MODER4_W {
                MODER4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&mut self) -> MODER5_W {
                MODER5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&mut self) -> MODER6_W {
                MODER6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&mut self) -> MODER7_W {
                MODER7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&mut self) -> MODER8_W {
                MODER8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&mut self) -> MODER9_W {
                MODER9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&mut self) -> MODER10_W {
                MODER10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&mut self) -> MODER11_W {
                MODER11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&mut self) -> MODER12_W {
                MODER12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&mut self) -> MODER13_W {
                MODER13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&mut self) -> MODER14_W {
                MODER14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&mut self) -> MODER15_W {
                MODER15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](otyper) module"]
    pub type OTYPER = crate::Reg<u32, _OTYPER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OTYPER;
    #[doc = "`read()` method returns [otyper::R](otyper::R) reader structure"]
    impl crate::Readable for OTYPER {}
    #[doc = "`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure"]
    impl crate::Writable for OTYPER {}
    #[doc = "GPIO port output type register"]
    pub mod otyper {
        #[doc = "Reader of register OTYPER"]
        pub type R = crate::R<u32, super::OTYPER>;
        #[doc = "Writer for register OTYPER"]
        pub type W = crate::W<u32, super::OTYPER>;
        #[doc = "Register OTYPER `reset()`'s with value 0"]
        impl crate::ResetValue for super::OTYPER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OT0`"]
        pub type OT0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT0`"]
        pub struct OT0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT0_W<'a> {
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
        #[doc = "Reader of field `OT1`"]
        pub type OT1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT1`"]
        pub struct OT1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT1_W<'a> {
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
        #[doc = "Reader of field `OT2`"]
        pub type OT2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT2`"]
        pub struct OT2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT2_W<'a> {
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
        #[doc = "Reader of field `OT3`"]
        pub type OT3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT3`"]
        pub struct OT3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT3_W<'a> {
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
        #[doc = "Reader of field `OT4`"]
        pub type OT4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT4`"]
        pub struct OT4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OT5`"]
        pub type OT5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT5`"]
        pub struct OT5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `OT6`"]
        pub type OT6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT6`"]
        pub struct OT6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OT7`"]
        pub type OT7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT7`"]
        pub struct OT7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `OT8`"]
        pub type OT8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT8`"]
        pub struct OT8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OT9`"]
        pub type OT9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT9`"]
        pub struct OT9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `OT10`"]
        pub type OT10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT10`"]
        pub struct OT10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OT11`"]
        pub type OT11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT11`"]
        pub struct OT11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `OT12`"]
        pub type OT12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT12`"]
        pub struct OT12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OT13`"]
        pub type OT13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT13`"]
        pub struct OT13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `OT14`"]
        pub type OT14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT14`"]
        pub struct OT14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OT15`"]
        pub type OT15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT15`"]
        pub struct OT15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&self) -> OT0_R {
                OT0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&self) -> OT1_R {
                OT1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&self) -> OT2_R {
                OT2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&self) -> OT3_R {
                OT3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&self) -> OT4_R {
                OT4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&self) -> OT5_R {
                OT5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&self) -> OT6_R {
                OT6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&self) -> OT7_R {
                OT7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&self) -> OT8_R {
                OT8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&self) -> OT9_R {
                OT9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&self) -> OT10_R {
                OT10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&self) -> OT11_R {
                OT11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&self) -> OT12_R {
                OT12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&self) -> OT13_R {
                OT13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&self) -> OT14_R {
                OT14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&self) -> OT15_R {
                OT15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&mut self) -> OT0_W {
                OT0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&mut self) -> OT1_W {
                OT1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&mut self) -> OT2_W {
                OT2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&mut self) -> OT3_W {
                OT3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&mut self) -> OT4_W {
                OT4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&mut self) -> OT5_W {
                OT5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&mut self) -> OT6_W {
                OT6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&mut self) -> OT7_W {
                OT7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&mut self) -> OT8_W {
                OT8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&mut self) -> OT9_W {
                OT9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&mut self) -> OT10_W {
                OT10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&mut self) -> OT11_W {
                OT11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&mut self) -> OT12_W {
                OT12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&mut self) -> OT13_W {
                OT13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&mut self) -> OT14_W {
                OT14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&mut self) -> OT15_W {
                OT15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](ospeedr) module"]
    pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OSPEEDR;
    #[doc = "`read()` method returns [ospeedr::R](ospeedr::R) reader structure"]
    impl crate::Readable for OSPEEDR {}
    #[doc = "`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure"]
    impl crate::Writable for OSPEEDR {}
    #[doc = "GPIO port output speed register"]
    pub mod ospeedr {
        #[doc = "Reader of register OSPEEDR"]
        pub type R = crate::R<u32, super::OSPEEDR>;
        #[doc = "Writer for register OSPEEDR"]
        pub type W = crate::W<u32, super::OSPEEDR>;
        #[doc = "Register OSPEEDR `reset()`'s with value 0x0c00_0000"]
        impl crate::ResetValue for super::OSPEEDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0x0c00_0000
            }
        }
        #[doc = "Reader of field `OSPEEDR0`"]
        pub type OSPEEDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR0`"]
        pub struct OSPEEDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR1`"]
        pub type OSPEEDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR1`"]
        pub struct OSPEEDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR2`"]
        pub type OSPEEDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR2`"]
        pub struct OSPEEDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR3`"]
        pub type OSPEEDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR3`"]
        pub struct OSPEEDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR4`"]
        pub type OSPEEDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR4`"]
        pub struct OSPEEDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR5`"]
        pub type OSPEEDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR5`"]
        pub struct OSPEEDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR6`"]
        pub type OSPEEDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR6`"]
        pub struct OSPEEDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR7`"]
        pub type OSPEEDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR7`"]
        pub struct OSPEEDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR8`"]
        pub type OSPEEDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR8`"]
        pub struct OSPEEDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR9`"]
        pub type OSPEEDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR9`"]
        pub struct OSPEEDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR10`"]
        pub type OSPEEDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR10`"]
        pub struct OSPEEDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR11`"]
        pub type OSPEEDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR11`"]
        pub struct OSPEEDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR12`"]
        pub type OSPEEDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR12`"]
        pub struct OSPEEDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR13`"]
        pub type OSPEEDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR13`"]
        pub struct OSPEEDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR14`"]
        pub type OSPEEDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR14`"]
        pub struct OSPEEDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR15`"]
        pub type OSPEEDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR15`"]
        pub struct OSPEEDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&self) -> OSPEEDR0_R {
                OSPEEDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&self) -> OSPEEDR1_R {
                OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&self) -> OSPEEDR2_R {
                OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&self) -> OSPEEDR3_R {
                OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&self) -> OSPEEDR4_R {
                OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&self) -> OSPEEDR5_R {
                OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&self) -> OSPEEDR6_R {
                OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&self) -> OSPEEDR7_R {
                OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&self) -> OSPEEDR8_R {
                OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&self) -> OSPEEDR9_R {
                OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&self) -> OSPEEDR10_R {
                OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&self) -> OSPEEDR11_R {
                OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&self) -> OSPEEDR12_R {
                OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&self) -> OSPEEDR13_R {
                OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&self) -> OSPEEDR14_R {
                OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&self) -> OSPEEDR15_R {
                OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
                OSPEEDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
                OSPEEDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
                OSPEEDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
                OSPEEDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
                OSPEEDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
                OSPEEDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
                OSPEEDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
                OSPEEDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
                OSPEEDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
                OSPEEDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
                OSPEEDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
                OSPEEDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
                OSPEEDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
                OSPEEDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
                OSPEEDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
                OSPEEDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](pupdr) module"]
    pub type PUPDR = crate::Reg<u32, _PUPDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _PUPDR;
    #[doc = "`read()` method returns [pupdr::R](pupdr::R) reader structure"]
    impl crate::Readable for PUPDR {}
    #[doc = "`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure"]
    impl crate::Writable for PUPDR {}
    #[doc = "GPIO port pull-up/pull-down register"]
    pub mod pupdr {
        #[doc = "Reader of register PUPDR"]
        pub type R = crate::R<u32, super::PUPDR>;
        #[doc = "Writer for register PUPDR"]
        pub type W = crate::W<u32, super::PUPDR>;
        #[doc = "Register PUPDR `reset()`'s with value 0x6400_0000"]
        impl crate::ResetValue for super::PUPDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0x6400_0000
            }
        }
        #[doc = "Reader of field `PUPDR0`"]
        pub type PUPDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR0`"]
        pub struct PUPDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR1`"]
        pub type PUPDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR1`"]
        pub struct PUPDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR2`"]
        pub type PUPDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR2`"]
        pub struct PUPDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR3`"]
        pub type PUPDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR3`"]
        pub struct PUPDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR4`"]
        pub type PUPDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR4`"]
        pub struct PUPDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR5`"]
        pub type PUPDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR5`"]
        pub struct PUPDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR6`"]
        pub type PUPDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR6`"]
        pub struct PUPDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR7`"]
        pub type PUPDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR7`"]
        pub struct PUPDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR8`"]
        pub type PUPDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR8`"]
        pub struct PUPDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR9`"]
        pub type PUPDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR9`"]
        pub struct PUPDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR10`"]
        pub type PUPDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR10`"]
        pub struct PUPDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR11`"]
        pub type PUPDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR11`"]
        pub struct PUPDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR12`"]
        pub type PUPDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR12`"]
        pub struct PUPDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR13`"]
        pub type PUPDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR13`"]
        pub struct PUPDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR14`"]
        pub type PUPDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR14`"]
        pub struct PUPDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR15`"]
        pub type PUPDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR15`"]
        pub struct PUPDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&self) -> PUPDR0_R {
                PUPDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&self) -> PUPDR1_R {
                PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&self) -> PUPDR2_R {
                PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&self) -> PUPDR3_R {
                PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&self) -> PUPDR4_R {
                PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&self) -> PUPDR5_R {
                PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&self) -> PUPDR6_R {
                PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&self) -> PUPDR7_R {
                PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&self) -> PUPDR8_R {
                PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&self) -> PUPDR9_R {
                PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&self) -> PUPDR10_R {
                PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&self) -> PUPDR11_R {
                PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&self) -> PUPDR12_R {
                PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&self) -> PUPDR13_R {
                PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&self) -> PUPDR14_R {
                PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&self) -> PUPDR15_R {
                PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&mut self) -> PUPDR0_W {
                PUPDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&mut self) -> PUPDR1_W {
                PUPDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&mut self) -> PUPDR2_W {
                PUPDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&mut self) -> PUPDR3_W {
                PUPDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&mut self) -> PUPDR4_W {
                PUPDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&mut self) -> PUPDR5_W {
                PUPDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&mut self) -> PUPDR6_W {
                PUPDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&mut self) -> PUPDR7_W {
                PUPDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&mut self) -> PUPDR8_W {
                PUPDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&mut self) -> PUPDR9_W {
                PUPDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&mut self) -> PUPDR10_W {
                PUPDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&mut self) -> PUPDR11_W {
                PUPDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&mut self) -> PUPDR12_W {
                PUPDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&mut self) -> PUPDR13_W {
                PUPDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&mut self) -> PUPDR14_W {
                PUPDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&mut self) -> PUPDR15_W {
                PUPDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
    pub type IDR = crate::Reg<u32, _IDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _IDR;
    #[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
    impl crate::Readable for IDR {}
    #[doc = "GPIO port input data register"]
    pub mod idr {
        #[doc = "Reader of register IDR"]
        pub type R = crate::R<u32, super::IDR>;
        #[doc = "Reader of field `IDR0`"]
        pub type IDR0_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR1`"]
        pub type IDR1_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR2`"]
        pub type IDR2_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR3`"]
        pub type IDR3_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR4`"]
        pub type IDR4_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR5`"]
        pub type IDR5_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR6`"]
        pub type IDR6_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR7`"]
        pub type IDR7_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR8`"]
        pub type IDR8_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR9`"]
        pub type IDR9_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR10`"]
        pub type IDR10_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR11`"]
        pub type IDR11_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR12`"]
        pub type IDR12_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR13`"]
        pub type IDR13_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR14`"]
        pub type IDR14_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR15`"]
        pub type IDR15_R = crate::R<bool, bool>;
        impl R {
            #[doc = "Bit 0 - Pin 0 input data bit"]
            #[inline(always)]
            pub fn idr0(&self) -> IDR0_R {
                IDR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 input data bit"]
            #[inline(always)]
            pub fn idr1(&self) -> IDR1_R {
                IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 input data bit"]
            #[inline(always)]
            pub fn idr2(&self) -> IDR2_R {
                IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 input data bit"]
            #[inline(always)]
            pub fn idr3(&self) -> IDR3_R {
                IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 input data bit"]
            #[inline(always)]
            pub fn idr4(&self) -> IDR4_R {
                IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 input data bit"]
            #[inline(always)]
            pub fn idr5(&self) -> IDR5_R {
                IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 input data bit"]
            #[inline(always)]
            pub fn idr6(&self) -> IDR6_R {
                IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 input data bit"]
            #[inline(always)]
            pub fn idr7(&self) -> IDR7_R {
                IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 input data bit"]
            #[inline(always)]
            pub fn idr8(&self) -> IDR8_R {
                IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 input data bit"]
            #[inline(always)]
            pub fn idr9(&self) -> IDR9_R {
                IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 input data bit"]
            #[inline(always)]
            pub fn idr10(&self) -> IDR10_R {
                IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 input data bit"]
            #[inline(always)]
            pub fn idr11(&self) -> IDR11_R {
                IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 input data bit"]
            #[inline(always)]
            pub fn idr12(&self) -> IDR12_R {
                IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 input data bit"]
            #[inline(always)]
            pub fn idr13(&self) -> IDR13_R {
                IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 input data bit"]
            #[inline(always)]
            pub fn idr14(&self) -> IDR14_R {
                IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 input data bit"]
            #[inline(always)]
            pub fn idr15(&self) -> IDR15_R {
                IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
    }
    #[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udr](udr) module"]
    pub type UDR = crate::Reg<u32, _UDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _UDR;
    #[doc = "`read()` method returns [udr::R](udr::R) reader structure"]
    impl crate::Readable for UDR {}
    #[doc = "`write(|w| ..)` method takes [udr::W](udr::W) writer structure"]
    impl crate::Writable for UDR {}
    #[doc = "GPIO port output data register"]
    pub mod udr {
        #[doc = "Reader of register UDR"]
        pub type R = crate::R<u32, super::UDR>;
        #[doc = "Writer for register UDR"]
        pub type W = crate::W<u32, super::UDR>;
        #[doc = "Register UDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::UDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `ODR0`"]
        pub type ODR0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR0`"]
        pub struct ODR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR0_W<'a> {
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
        #[doc = "Reader of field `ODR1`"]
        pub type ODR1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR1`"]
        pub struct ODR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR1_W<'a> {
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
        #[doc = "Reader of field `ODR2`"]
        pub type ODR2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR2`"]
        pub struct ODR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR2_W<'a> {
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
        #[doc = "Reader of field `ODR3`"]
        pub type ODR3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR3`"]
        pub struct ODR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR3_W<'a> {
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
        #[doc = "Reader of field `ODR4`"]
        pub type ODR4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR4`"]
        pub struct ODR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `ODR5`"]
        pub type ODR5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR5`"]
        pub struct ODR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `ODR6`"]
        pub type ODR6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR6`"]
        pub struct ODR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `ODR7`"]
        pub type ODR7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR7`"]
        pub struct ODR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `ODR8`"]
        pub type ODR8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR8`"]
        pub struct ODR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `ODR9`"]
        pub type ODR9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR9`"]
        pub struct ODR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `ODR10`"]
        pub type ODR10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR10`"]
        pub struct ODR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `ODR11`"]
        pub type ODR11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR11`"]
        pub struct ODR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `ODR12`"]
        pub type ODR12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR12`"]
        pub struct ODR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `ODR13`"]
        pub type ODR13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR13`"]
        pub struct ODR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `ODR14`"]
        pub type ODR14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR14`"]
        pub struct ODR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `ODR15`"]
        pub type ODR15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR15`"]
        pub struct ODR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&self) -> ODR0_R {
                ODR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&self) -> ODR1_R {
                ODR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&self) -> ODR2_R {
                ODR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&self) -> ODR3_R {
                ODR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&self) -> ODR4_R {
                ODR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&self) -> ODR5_R {
                ODR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&self) -> ODR6_R {
                ODR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&self) -> ODR7_R {
                ODR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&self) -> ODR8_R {
                ODR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&self) -> ODR9_R {
                ODR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&self) -> ODR10_R {
                ODR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&self) -> ODR11_R {
                ODR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&self) -> ODR12_R {
                ODR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&self) -> ODR13_R {
                ODR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&self) -> ODR14_R {
                ODR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&self) -> ODR15_R {
                ODR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&mut self) -> ODR0_W {
                ODR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&mut self) -> ODR1_W {
                ODR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&mut self) -> ODR2_W {
                ODR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&mut self) -> ODR3_W {
                ODR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&mut self) -> ODR4_W {
                ODR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&mut self) -> ODR5_W {
                ODR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&mut self) -> ODR6_W {
                ODR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&mut self) -> ODR7_W {
                ODR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&mut self) -> ODR8_W {
                ODR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&mut self) -> ODR9_W {
                ODR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&mut self) -> ODR10_W {
                ODR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&mut self) -> ODR11_W {
                ODR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&mut self) -> ODR12_W {
                ODR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&mut self) -> ODR13_W {
                ODR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&mut self) -> ODR14_W {
                ODR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&mut self) -> ODR15_W {
                ODR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
    pub type BSRR = crate::Reg<u32, _BSRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BSRR;
    #[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
    impl crate::Writable for BSRR {}
    #[doc = "GPIO port bit set/reset register"]
    pub mod bsrr {
        #[doc = "Writer for register BSRR"]
        pub type W = crate::W<u32, super::BSRR>;
        #[doc = "Register BSRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BSRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BS0`"]
        pub struct BS0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS0_W<'a> {
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
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BS1`"]
        pub struct BS1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS1_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BS2`"]
        pub struct BS2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS2_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS3`"]
        pub struct BS3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS3_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS4`"]
        pub struct BS4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS5`"]
        pub struct BS5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS6`"]
        pub struct BS6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS7`"]
        pub struct BS7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
        #[doc = "Write proxy for field `BS8`"]
        pub struct BS8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
        #[doc = "Write proxy for field `BS9`"]
        pub struct BS9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS10`"]
        pub struct BS10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
        #[doc = "Write proxy for field `BS11`"]
        pub struct BS11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS12`"]
        pub struct BS12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS13`"]
        pub struct BS13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
        #[doc = "Write proxy for field `BS14`"]
        pub struct BS14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
        #[doc = "Write proxy for field `BS15`"]
        pub struct BS15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
        impl W {
            #[doc = "Bit 0 - Pin 0 set bit"]
            #[inline(always)]
            pub fn bs0(&mut self) -> BS0_W {
                BS0_W { w: self }
            }
            #[doc = "Bit 16 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 set bit"]
            #[inline(always)]
            pub fn bs1(&mut self) -> BS1_W {
                BS1_W { w: self }
            }
            #[doc = "Bit 17 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 set bit"]
            #[inline(always)]
            pub fn bs2(&mut self) -> BS2_W {
                BS2_W { w: self }
            }
            #[doc = "Bit 18 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 set bit"]
            #[inline(always)]
            pub fn bs3(&mut self) -> BS3_W {
                BS3_W { w: self }
            }
            #[doc = "Bit 19 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 set bit"]
            #[inline(always)]
            pub fn bs4(&mut self) -> BS4_W {
                BS4_W { w: self }
            }
            #[doc = "Bit 20 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 set bit"]
            #[inline(always)]
            pub fn bs5(&mut self) -> BS5_W {
                BS5_W { w: self }
            }
            #[doc = "Bit 21 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 set bit"]
            #[inline(always)]
            pub fn bs6(&mut self) -> BS6_W {
                BS6_W { w: self }
            }
            #[doc = "Bit 22 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 set bit"]
            #[inline(always)]
            pub fn bs7(&mut self) -> BS7_W {
                BS7_W { w: self }
            }
            #[doc = "Bit 23 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 set bit"]
            #[inline(always)]
            pub fn bs8(&mut self) -> BS8_W {
                BS8_W { w: self }
            }
            #[doc = "Bit 24 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 set bit"]
            #[inline(always)]
            pub fn bs9(&mut self) -> BS9_W {
                BS9_W { w: self }
            }
            #[doc = "Bit 25 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 set bit"]
            #[inline(always)]
            pub fn bs10(&mut self) -> BS10_W {
                BS10_W { w: self }
            }
            #[doc = "Bit 26 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 set bit"]
            #[inline(always)]
            pub fn bs11(&mut self) -> BS11_W {
                BS11_W { w: self }
            }
            #[doc = "Bit 27 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 set bit"]
            #[inline(always)]
            pub fn bs12(&mut self) -> BS12_W {
                BS12_W { w: self }
            }
            #[doc = "Bit 28 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 set bit"]
            #[inline(always)]
            pub fn bs13(&mut self) -> BS13_W {
                BS13_W { w: self }
            }
            #[doc = "Bit 29 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 set bit"]
            #[inline(always)]
            pub fn bs14(&mut self) -> BS14_W {
                BS14_W { w: self }
            }
            #[doc = "Bit 30 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 set bit"]
            #[inline(always)]
            pub fn bs15(&mut self) -> BS15_W {
                BS15_W { w: self }
            }
            #[doc = "Bit 31 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
    pub type LCKR = crate::Reg<u32, _LCKR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _LCKR;
    #[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
    impl crate::Readable for LCKR {}
    #[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
    impl crate::Writable for LCKR {}
    #[doc = "GPIO port configuration lock register"]
    pub mod lckr {
        #[doc = "Reader of register LCKR"]
        pub type R = crate::R<u32, super::LCKR>;
        #[doc = "Writer for register LCKR"]
        pub type W = crate::W<u32, super::LCKR>;
        #[doc = "Register LCKR `reset()`'s with value 0"]
        impl crate::ResetValue for super::LCKR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `LCK0`"]
        pub type LCK0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK0`"]
        pub struct LCK0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK0_W<'a> {
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
        #[doc = "Reader of field `LCK1`"]
        pub type LCK1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK1`"]
        pub struct LCK1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK1_W<'a> {
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
        #[doc = "Reader of field `LCK2`"]
        pub type LCK2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK2`"]
        pub struct LCK2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK2_W<'a> {
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
        #[doc = "Reader of field `LCK3`"]
        pub type LCK3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK3`"]
        pub struct LCK3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK3_W<'a> {
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
        #[doc = "Reader of field `LCK4`"]
        pub type LCK4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK4`"]
        pub struct LCK4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `LCK5`"]
        pub type LCK5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK5`"]
        pub struct LCK5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `LCK6`"]
        pub type LCK6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK6`"]
        pub struct LCK6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `LCK7`"]
        pub type LCK7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK7`"]
        pub struct LCK7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `LCK8`"]
        pub type LCK8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK8`"]
        pub struct LCK8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `LCK9`"]
        pub type LCK9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK9`"]
        pub struct LCK9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `LCK10`"]
        pub type LCK10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK10`"]
        pub struct LCK10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `LCK11`"]
        pub type LCK11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK11`"]
        pub struct LCK11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `LCK12`"]
        pub type LCK12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK12`"]
        pub struct LCK12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `LCK13`"]
        pub type LCK13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK13`"]
        pub struct LCK13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `LCK14`"]
        pub type LCK14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK14`"]
        pub struct LCK14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `LCK15`"]
        pub type LCK15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK15`"]
        pub struct LCK15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Reader of field `LCKK`"]
        pub type LCKK_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCKK`"]
        pub struct LCKK_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCKK_W<'a> {
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
        impl R {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&self) -> LCK0_R {
                LCK0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&self) -> LCK1_R {
                LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&self) -> LCK2_R {
                LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&self) -> LCK3_R {
                LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&self) -> LCK4_R {
                LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&self) -> LCK5_R {
                LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&self) -> LCK6_R {
                LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&self) -> LCK7_R {
                LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&self) -> LCK8_R {
                LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&self) -> LCK9_R {
                LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&self) -> LCK10_R {
                LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&self) -> LCK11_R {
                LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&self) -> LCK12_R {
                LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&self) -> LCK13_R {
                LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&self) -> LCK14_R {
                LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&self) -> LCK15_R {
                LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&self) -> LCKK_R {
                LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&mut self) -> LCK0_W {
                LCK0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&mut self) -> LCK1_W {
                LCK1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&mut self) -> LCK2_W {
                LCK2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&mut self) -> LCK3_W {
                LCK3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&mut self) -> LCK4_W {
                LCK4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&mut self) -> LCK5_W {
                LCK5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&mut self) -> LCK6_W {
                LCK6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&mut self) -> LCK7_W {
                LCK7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&mut self) -> LCK8_W {
                LCK8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&mut self) -> LCK9_W {
                LCK9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&mut self) -> LCK10_W {
                LCK10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&mut self) -> LCK11_W {
                LCK11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&mut self) -> LCK12_W {
                LCK12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&mut self) -> LCK13_W {
                LCK13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&mut self) -> LCK14_W {
                LCK14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&mut self) -> LCK15_W {
                LCK15_W { w: self }
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&mut self) -> LCKK_W {
                LCKK_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](afrl) module"]
    pub type AFRL = crate::Reg<u32, _AFRL>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRL;
    #[doc = "`read()` method returns [afrl::R](afrl::R) reader structure"]
    impl crate::Readable for AFRL {}
    #[doc = "`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure"]
    impl crate::Writable for AFRL {}
    #[doc = "GPIO alternate function low register"]
    pub mod afrl {
        #[doc = "Reader of register AFRL"]
        pub type R = crate::R<u32, super::AFRL>;
        #[doc = "Writer for register AFRL"]
        pub type W = crate::W<u32, super::AFRL>;
        #[doc = "Register AFRL `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRL {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR0`"]
        pub type AFR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR0`"]
        pub struct AFR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
                self.w
            }
        }
        #[doc = "Reader of field `AFR1`"]
        pub type AFR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR1`"]
        pub struct AFR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `AFR2`"]
        pub type AFR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR2`"]
        pub struct AFR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `AFR3`"]
        pub type AFR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR3`"]
        pub struct AFR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `AFR4`"]
        pub type AFR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR4`"]
        pub struct AFR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `AFR5`"]
        pub type AFR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR5`"]
        pub struct AFR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `AFR6`"]
        pub type AFR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR6`"]
        pub struct AFR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `AFR7`"]
        pub type AFR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR7`"]
        pub struct AFR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&self) -> AFR0_R {
                AFR0_R::new((self.bits & 0x0f) as u8)
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&self) -> AFR1_R {
                AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&self) -> AFR2_R {
                AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&self) -> AFR3_R {
                AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&self) -> AFR4_R {
                AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&self) -> AFR5_R {
                AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&self) -> AFR6_R {
                AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&self) -> AFR7_R {
                AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&mut self) -> AFR0_W {
                AFR0_W { w: self }
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&mut self) -> AFR1_W {
                AFR1_W { w: self }
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&mut self) -> AFR2_W {
                AFR2_W { w: self }
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&mut self) -> AFR3_W {
                AFR3_W { w: self }
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&mut self) -> AFR4_W {
                AFR4_W { w: self }
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&mut self) -> AFR5_W {
                AFR5_W { w: self }
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&mut self) -> AFR6_W {
                AFR6_W { w: self }
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&mut self) -> AFR7_W {
                AFR7_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](afrh) module"]
    pub type AFRH = crate::Reg<u32, _AFRH>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRH;
    #[doc = "`read()` method returns [afrh::R](afrh::R) reader structure"]
    impl crate::Readable for AFRH {}
    #[doc = "`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure"]
    impl crate::Writable for AFRH {}
    #[doc = "GPIO alternate function high register"]
    pub mod afrh {
        #[doc = "Reader of register AFRH"]
        pub type R = crate::R<u32, super::AFRH>;
        #[doc = "Writer for register AFRH"]
        pub type W = crate::W<u32, super::AFRH>;
        #[doc = "Register AFRH `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRH {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR8`"]
        pub type AFR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR8`"]
        pub struct AFR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 32)) | (((value as u32) & 0x0f) << 32);
                self.w
            }
        }
        #[doc = "Reader of field `AFR9`"]
        pub type AFR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR9`"]
        pub struct AFR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 36)) | (((value as u32) & 0x0f) << 36);
                self.w
            }
        }
        #[doc = "Reader of field `AFR10`"]
        pub type AFR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR10`"]
        pub struct AFR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 40)) | (((value as u32) & 0x0f) << 40);
                self.w
            }
        }
        #[doc = "Reader of field `AFR11`"]
        pub type AFR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR11`"]
        pub struct AFR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 44)) | (((value as u32) & 0x0f) << 44);
                self.w
            }
        }
        #[doc = "Reader of field `AFR12`"]
        pub type AFR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR12`"]
        pub struct AFR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 48)) | (((value as u32) & 0x0f) << 48);
                self.w
            }
        }
        #[doc = "Reader of field `AFR13`"]
        pub type AFR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR13`"]
        pub struct AFR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 52)) | (((value as u32) & 0x0f) << 52);
                self.w
            }
        }
        #[doc = "Reader of field `AFR14`"]
        pub type AFR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR14`"]
        pub struct AFR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 56)) | (((value as u32) & 0x0f) << 56);
                self.w
            }
        }
        #[doc = "Reader of field `AFR15`"]
        pub type AFR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR15`"]
        pub struct AFR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 60)) | (((value as u32) & 0x0f) << 60);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&self) -> AFR8_R {
                AFR8_R::new(((self.bits >> 32) & 0x0f) as u8)
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&self) -> AFR9_R {
                AFR9_R::new(((self.bits >> 36) & 0x0f) as u8)
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&self) -> AFR10_R {
                AFR10_R::new(((self.bits >> 40) & 0x0f) as u8)
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&self) -> AFR11_R {
                AFR11_R::new(((self.bits >> 44) & 0x0f) as u8)
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&self) -> AFR12_R {
                AFR12_R::new(((self.bits >> 48) & 0x0f) as u8)
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&self) -> AFR13_R {
                AFR13_R::new(((self.bits >> 52) & 0x0f) as u8)
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&self) -> AFR14_R {
                AFR14_R::new(((self.bits >> 56) & 0x0f) as u8)
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&self) -> AFR15_R {
                AFR15_R::new(((self.bits >> 60) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&mut self) -> AFR8_W {
                AFR8_W { w: self }
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&mut self) -> AFR9_W {
                AFR9_W { w: self }
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&mut self) -> AFR10_W {
                AFR10_W { w: self }
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&mut self) -> AFR11_W {
                AFR11_W { w: self }
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&mut self) -> AFR12_W {
                AFR12_W { w: self }
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&mut self) -> AFR13_W {
                AFR13_W { w: self }
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&mut self) -> AFR14_W {
                AFR14_W { w: self }
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&mut self) -> AFR15_W {
                AFR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
    pub type BRR = crate::Reg<u32, _BRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BRR;
    #[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
    impl crate::Writable for BRR {}
    #[doc = "GPIO port bit reset register"]
    pub mod brr {
        #[doc = "Writer for register BRR"]
        pub type W = crate::W<u32, super::BRR>;
        #[doc = "Register BRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
}
#[doc = "General-purpose I/Os, port B"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4800_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os, port B"]
pub mod gpiob {
    #[doc = r"Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - GPIO port mode register"]
        pub moder: MODER,
        #[doc = "0x04 - GPIO port output type register"]
        pub otyper: OTYPER,
        #[doc = "0x08 - GPIO port output speed register"]
        pub ospeedr: OSPEEDR,
        #[doc = "0x0c - GPIO port pull-up/pull-down register"]
        pub pupdr: PUPDR,
        #[doc = "0x10 - GPIO port input data register"]
        pub idr: IDR,
        #[doc = "0x14 - GPIO port output data register"]
        pub udr: UDR,
        #[doc = "0x18 - GPIO port bit set/reset register"]
        pub bsrr: BSRR,
        #[doc = "0x1c - GPIO port configuration lock register"]
        pub lckr: LCKR,
        #[doc = "0x20 - GPIO alternate function low register"]
        pub afrl: AFRL,
        #[doc = "0x24 - GPIO alternate function high register"]
        pub afrh: AFRH,
        #[doc = "0x28 - GPIO port bit reset register"]
        pub brr: BRR,
    }
    #[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](moder) module"]
    pub type MODER = crate::Reg<u32, _MODER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _MODER;
    #[doc = "`read()` method returns [moder::R](moder::R) reader structure"]
    impl crate::Readable for MODER {}
    #[doc = "`write(|w| ..)` method takes [moder::W](moder::W) writer structure"]
    impl crate::Writable for MODER {}
    #[doc = "GPIO port mode register"]
    pub mod moder {
        #[doc = "Reader of register MODER"]
        pub type R = crate::R<u32, super::MODER>;
        #[doc = "Writer for register MODER"]
        pub type W = crate::W<u32, super::MODER>;
        #[doc = "Register MODER `reset()`'s with value 0x0280"]
        impl crate::ResetValue for super::MODER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0x0280
            }
        }
        #[doc = "Reader of field `MODER0`"]
        pub type MODER0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER0`"]
        pub struct MODER0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `MODER1`"]
        pub type MODER1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER1`"]
        pub struct MODER1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `MODER2`"]
        pub type MODER2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER2`"]
        pub struct MODER2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `MODER3`"]
        pub type MODER3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER3`"]
        pub struct MODER3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `MODER4`"]
        pub type MODER4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER4`"]
        pub struct MODER4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `MODER5`"]
        pub type MODER5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER5`"]
        pub struct MODER5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `MODER6`"]
        pub type MODER6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER6`"]
        pub struct MODER6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `MODER7`"]
        pub type MODER7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER7`"]
        pub struct MODER7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `MODER8`"]
        pub type MODER8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER8`"]
        pub struct MODER8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `MODER9`"]
        pub type MODER9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER9`"]
        pub struct MODER9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `MODER10`"]
        pub type MODER10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER10`"]
        pub struct MODER10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `MODER11`"]
        pub type MODER11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER11`"]
        pub struct MODER11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `MODER12`"]
        pub type MODER12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER12`"]
        pub struct MODER12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `MODER13`"]
        pub type MODER13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER13`"]
        pub struct MODER13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `MODER14`"]
        pub type MODER14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER14`"]
        pub struct MODER14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `MODER15`"]
        pub type MODER15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER15`"]
        pub struct MODER15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&self) -> MODER0_R {
                MODER0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&self) -> MODER1_R {
                MODER1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&self) -> MODER2_R {
                MODER2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&self) -> MODER3_R {
                MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&self) -> MODER4_R {
                MODER4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&self) -> MODER5_R {
                MODER5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&self) -> MODER6_R {
                MODER6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&self) -> MODER7_R {
                MODER7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&self) -> MODER8_R {
                MODER8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&self) -> MODER9_R {
                MODER9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&self) -> MODER10_R {
                MODER10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&self) -> MODER11_R {
                MODER11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&self) -> MODER12_R {
                MODER12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&self) -> MODER13_R {
                MODER13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&self) -> MODER14_R {
                MODER14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&self) -> MODER15_R {
                MODER15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&mut self) -> MODER0_W {
                MODER0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&mut self) -> MODER1_W {
                MODER1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&mut self) -> MODER2_W {
                MODER2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&mut self) -> MODER3_W {
                MODER3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&mut self) -> MODER4_W {
                MODER4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&mut self) -> MODER5_W {
                MODER5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&mut self) -> MODER6_W {
                MODER6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&mut self) -> MODER7_W {
                MODER7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&mut self) -> MODER8_W {
                MODER8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&mut self) -> MODER9_W {
                MODER9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&mut self) -> MODER10_W {
                MODER10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&mut self) -> MODER11_W {
                MODER11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&mut self) -> MODER12_W {
                MODER12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&mut self) -> MODER13_W {
                MODER13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&mut self) -> MODER14_W {
                MODER14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&mut self) -> MODER15_W {
                MODER15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](otyper) module"]
    pub type OTYPER = crate::Reg<u32, _OTYPER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OTYPER;
    #[doc = "`read()` method returns [otyper::R](otyper::R) reader structure"]
    impl crate::Readable for OTYPER {}
    #[doc = "`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure"]
    impl crate::Writable for OTYPER {}
    #[doc = "GPIO port output type register"]
    pub mod otyper {
        #[doc = "Reader of register OTYPER"]
        pub type R = crate::R<u32, super::OTYPER>;
        #[doc = "Writer for register OTYPER"]
        pub type W = crate::W<u32, super::OTYPER>;
        #[doc = "Register OTYPER `reset()`'s with value 0"]
        impl crate::ResetValue for super::OTYPER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OT0`"]
        pub type OT0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT0`"]
        pub struct OT0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT0_W<'a> {
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
        #[doc = "Reader of field `OT1`"]
        pub type OT1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT1`"]
        pub struct OT1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT1_W<'a> {
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
        #[doc = "Reader of field `OT2`"]
        pub type OT2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT2`"]
        pub struct OT2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT2_W<'a> {
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
        #[doc = "Reader of field `OT3`"]
        pub type OT3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT3`"]
        pub struct OT3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT3_W<'a> {
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
        #[doc = "Reader of field `OT4`"]
        pub type OT4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT4`"]
        pub struct OT4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OT5`"]
        pub type OT5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT5`"]
        pub struct OT5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `OT6`"]
        pub type OT6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT6`"]
        pub struct OT6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OT7`"]
        pub type OT7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT7`"]
        pub struct OT7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `OT8`"]
        pub type OT8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT8`"]
        pub struct OT8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OT9`"]
        pub type OT9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT9`"]
        pub struct OT9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `OT10`"]
        pub type OT10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT10`"]
        pub struct OT10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OT11`"]
        pub type OT11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT11`"]
        pub struct OT11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `OT12`"]
        pub type OT12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT12`"]
        pub struct OT12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OT13`"]
        pub type OT13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT13`"]
        pub struct OT13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `OT14`"]
        pub type OT14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT14`"]
        pub struct OT14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OT15`"]
        pub type OT15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT15`"]
        pub struct OT15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&self) -> OT0_R {
                OT0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&self) -> OT1_R {
                OT1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&self) -> OT2_R {
                OT2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&self) -> OT3_R {
                OT3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&self) -> OT4_R {
                OT4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&self) -> OT5_R {
                OT5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&self) -> OT6_R {
                OT6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&self) -> OT7_R {
                OT7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&self) -> OT8_R {
                OT8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&self) -> OT9_R {
                OT9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&self) -> OT10_R {
                OT10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&self) -> OT11_R {
                OT11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&self) -> OT12_R {
                OT12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&self) -> OT13_R {
                OT13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&self) -> OT14_R {
                OT14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&self) -> OT15_R {
                OT15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&mut self) -> OT0_W {
                OT0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&mut self) -> OT1_W {
                OT1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&mut self) -> OT2_W {
                OT2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&mut self) -> OT3_W {
                OT3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&mut self) -> OT4_W {
                OT4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&mut self) -> OT5_W {
                OT5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&mut self) -> OT6_W {
                OT6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&mut self) -> OT7_W {
                OT7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&mut self) -> OT8_W {
                OT8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&mut self) -> OT9_W {
                OT9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&mut self) -> OT10_W {
                OT10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&mut self) -> OT11_W {
                OT11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&mut self) -> OT12_W {
                OT12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&mut self) -> OT13_W {
                OT13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&mut self) -> OT14_W {
                OT14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&mut self) -> OT15_W {
                OT15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](ospeedr) module"]
    pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OSPEEDR;
    #[doc = "`read()` method returns [ospeedr::R](ospeedr::R) reader structure"]
    impl crate::Readable for OSPEEDR {}
    #[doc = "`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure"]
    impl crate::Writable for OSPEEDR {}
    #[doc = "GPIO port output speed register"]
    pub mod ospeedr {
        #[doc = "Reader of register OSPEEDR"]
        pub type R = crate::R<u32, super::OSPEEDR>;
        #[doc = "Writer for register OSPEEDR"]
        pub type W = crate::W<u32, super::OSPEEDR>;
        #[doc = "Register OSPEEDR `reset()`'s with value 0xc0"]
        impl crate::ResetValue for super::OSPEEDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0xc0
            }
        }
        #[doc = "Reader of field `OSPEEDR0`"]
        pub type OSPEEDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR0`"]
        pub struct OSPEEDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR1`"]
        pub type OSPEEDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR1`"]
        pub struct OSPEEDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR2`"]
        pub type OSPEEDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR2`"]
        pub struct OSPEEDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR3`"]
        pub type OSPEEDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR3`"]
        pub struct OSPEEDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR4`"]
        pub type OSPEEDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR4`"]
        pub struct OSPEEDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR5`"]
        pub type OSPEEDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR5`"]
        pub struct OSPEEDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR6`"]
        pub type OSPEEDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR6`"]
        pub struct OSPEEDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR7`"]
        pub type OSPEEDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR7`"]
        pub struct OSPEEDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR8`"]
        pub type OSPEEDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR8`"]
        pub struct OSPEEDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR9`"]
        pub type OSPEEDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR9`"]
        pub struct OSPEEDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR10`"]
        pub type OSPEEDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR10`"]
        pub struct OSPEEDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR11`"]
        pub type OSPEEDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR11`"]
        pub struct OSPEEDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR12`"]
        pub type OSPEEDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR12`"]
        pub struct OSPEEDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR13`"]
        pub type OSPEEDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR13`"]
        pub struct OSPEEDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR14`"]
        pub type OSPEEDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR14`"]
        pub struct OSPEEDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR15`"]
        pub type OSPEEDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR15`"]
        pub struct OSPEEDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&self) -> OSPEEDR0_R {
                OSPEEDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&self) -> OSPEEDR1_R {
                OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&self) -> OSPEEDR2_R {
                OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&self) -> OSPEEDR3_R {
                OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&self) -> OSPEEDR4_R {
                OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&self) -> OSPEEDR5_R {
                OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&self) -> OSPEEDR6_R {
                OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&self) -> OSPEEDR7_R {
                OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&self) -> OSPEEDR8_R {
                OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&self) -> OSPEEDR9_R {
                OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&self) -> OSPEEDR10_R {
                OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&self) -> OSPEEDR11_R {
                OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&self) -> OSPEEDR12_R {
                OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&self) -> OSPEEDR13_R {
                OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&self) -> OSPEEDR14_R {
                OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&self) -> OSPEEDR15_R {
                OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
                OSPEEDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
                OSPEEDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
                OSPEEDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
                OSPEEDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
                OSPEEDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
                OSPEEDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
                OSPEEDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
                OSPEEDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
                OSPEEDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
                OSPEEDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
                OSPEEDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
                OSPEEDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
                OSPEEDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
                OSPEEDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
                OSPEEDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
                OSPEEDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](pupdr) module"]
    pub type PUPDR = crate::Reg<u32, _PUPDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _PUPDR;
    #[doc = "`read()` method returns [pupdr::R](pupdr::R) reader structure"]
    impl crate::Readable for PUPDR {}
    #[doc = "`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure"]
    impl crate::Writable for PUPDR {}
    #[doc = "GPIO port pull-up/pull-down register"]
    pub mod pupdr {
        #[doc = "Reader of register PUPDR"]
        pub type R = crate::R<u32, super::PUPDR>;
        #[doc = "Writer for register PUPDR"]
        pub type W = crate::W<u32, super::PUPDR>;
        #[doc = "Register PUPDR `reset()`'s with value 0x0100"]
        impl crate::ResetValue for super::PUPDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0x0100
            }
        }
        #[doc = "Reader of field `PUPDR0`"]
        pub type PUPDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR0`"]
        pub struct PUPDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR1`"]
        pub type PUPDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR1`"]
        pub struct PUPDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR2`"]
        pub type PUPDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR2`"]
        pub struct PUPDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR3`"]
        pub type PUPDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR3`"]
        pub struct PUPDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR4`"]
        pub type PUPDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR4`"]
        pub struct PUPDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR5`"]
        pub type PUPDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR5`"]
        pub struct PUPDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR6`"]
        pub type PUPDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR6`"]
        pub struct PUPDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR7`"]
        pub type PUPDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR7`"]
        pub struct PUPDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR8`"]
        pub type PUPDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR8`"]
        pub struct PUPDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR9`"]
        pub type PUPDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR9`"]
        pub struct PUPDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR10`"]
        pub type PUPDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR10`"]
        pub struct PUPDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR11`"]
        pub type PUPDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR11`"]
        pub struct PUPDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR12`"]
        pub type PUPDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR12`"]
        pub struct PUPDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR13`"]
        pub type PUPDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR13`"]
        pub struct PUPDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR14`"]
        pub type PUPDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR14`"]
        pub struct PUPDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR15`"]
        pub type PUPDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR15`"]
        pub struct PUPDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&self) -> PUPDR0_R {
                PUPDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&self) -> PUPDR1_R {
                PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&self) -> PUPDR2_R {
                PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&self) -> PUPDR3_R {
                PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&self) -> PUPDR4_R {
                PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&self) -> PUPDR5_R {
                PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&self) -> PUPDR6_R {
                PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&self) -> PUPDR7_R {
                PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&self) -> PUPDR8_R {
                PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&self) -> PUPDR9_R {
                PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&self) -> PUPDR10_R {
                PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&self) -> PUPDR11_R {
                PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&self) -> PUPDR12_R {
                PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&self) -> PUPDR13_R {
                PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&self) -> PUPDR14_R {
                PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&self) -> PUPDR15_R {
                PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&mut self) -> PUPDR0_W {
                PUPDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&mut self) -> PUPDR1_W {
                PUPDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&mut self) -> PUPDR2_W {
                PUPDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&mut self) -> PUPDR3_W {
                PUPDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&mut self) -> PUPDR4_W {
                PUPDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&mut self) -> PUPDR5_W {
                PUPDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&mut self) -> PUPDR6_W {
                PUPDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&mut self) -> PUPDR7_W {
                PUPDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&mut self) -> PUPDR8_W {
                PUPDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&mut self) -> PUPDR9_W {
                PUPDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&mut self) -> PUPDR10_W {
                PUPDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&mut self) -> PUPDR11_W {
                PUPDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&mut self) -> PUPDR12_W {
                PUPDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&mut self) -> PUPDR13_W {
                PUPDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&mut self) -> PUPDR14_W {
                PUPDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&mut self) -> PUPDR15_W {
                PUPDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
    pub type IDR = crate::Reg<u32, _IDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _IDR;
    #[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
    impl crate::Readable for IDR {}
    #[doc = "GPIO port input data register"]
    pub mod idr {
        #[doc = "Reader of register IDR"]
        pub type R = crate::R<u32, super::IDR>;
        #[doc = "Reader of field `IDR0`"]
        pub type IDR0_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR1`"]
        pub type IDR1_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR2`"]
        pub type IDR2_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR3`"]
        pub type IDR3_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR4`"]
        pub type IDR4_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR5`"]
        pub type IDR5_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR6`"]
        pub type IDR6_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR7`"]
        pub type IDR7_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR8`"]
        pub type IDR8_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR9`"]
        pub type IDR9_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR10`"]
        pub type IDR10_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR11`"]
        pub type IDR11_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR12`"]
        pub type IDR12_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR13`"]
        pub type IDR13_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR14`"]
        pub type IDR14_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR15`"]
        pub type IDR15_R = crate::R<bool, bool>;
        impl R {
            #[doc = "Bit 0 - Pin 0 input data bit"]
            #[inline(always)]
            pub fn idr0(&self) -> IDR0_R {
                IDR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 input data bit"]
            #[inline(always)]
            pub fn idr1(&self) -> IDR1_R {
                IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 input data bit"]
            #[inline(always)]
            pub fn idr2(&self) -> IDR2_R {
                IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 input data bit"]
            #[inline(always)]
            pub fn idr3(&self) -> IDR3_R {
                IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 input data bit"]
            #[inline(always)]
            pub fn idr4(&self) -> IDR4_R {
                IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 input data bit"]
            #[inline(always)]
            pub fn idr5(&self) -> IDR5_R {
                IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 input data bit"]
            #[inline(always)]
            pub fn idr6(&self) -> IDR6_R {
                IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 input data bit"]
            #[inline(always)]
            pub fn idr7(&self) -> IDR7_R {
                IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 input data bit"]
            #[inline(always)]
            pub fn idr8(&self) -> IDR8_R {
                IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 input data bit"]
            #[inline(always)]
            pub fn idr9(&self) -> IDR9_R {
                IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 input data bit"]
            #[inline(always)]
            pub fn idr10(&self) -> IDR10_R {
                IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 input data bit"]
            #[inline(always)]
            pub fn idr11(&self) -> IDR11_R {
                IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 input data bit"]
            #[inline(always)]
            pub fn idr12(&self) -> IDR12_R {
                IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 input data bit"]
            #[inline(always)]
            pub fn idr13(&self) -> IDR13_R {
                IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 input data bit"]
            #[inline(always)]
            pub fn idr14(&self) -> IDR14_R {
                IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 input data bit"]
            #[inline(always)]
            pub fn idr15(&self) -> IDR15_R {
                IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
    }
    #[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udr](udr) module"]
    pub type UDR = crate::Reg<u32, _UDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _UDR;
    #[doc = "`read()` method returns [udr::R](udr::R) reader structure"]
    impl crate::Readable for UDR {}
    #[doc = "`write(|w| ..)` method takes [udr::W](udr::W) writer structure"]
    impl crate::Writable for UDR {}
    #[doc = "GPIO port output data register"]
    pub mod udr {
        #[doc = "Reader of register UDR"]
        pub type R = crate::R<u32, super::UDR>;
        #[doc = "Writer for register UDR"]
        pub type W = crate::W<u32, super::UDR>;
        #[doc = "Register UDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::UDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `ODR0`"]
        pub type ODR0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR0`"]
        pub struct ODR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR0_W<'a> {
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
        #[doc = "Reader of field `ODR1`"]
        pub type ODR1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR1`"]
        pub struct ODR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR1_W<'a> {
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
        #[doc = "Reader of field `ODR2`"]
        pub type ODR2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR2`"]
        pub struct ODR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR2_W<'a> {
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
        #[doc = "Reader of field `ODR3`"]
        pub type ODR3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR3`"]
        pub struct ODR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR3_W<'a> {
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
        #[doc = "Reader of field `ODR4`"]
        pub type ODR4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR4`"]
        pub struct ODR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `ODR5`"]
        pub type ODR5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR5`"]
        pub struct ODR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `ODR6`"]
        pub type ODR6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR6`"]
        pub struct ODR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `ODR7`"]
        pub type ODR7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR7`"]
        pub struct ODR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `ODR8`"]
        pub type ODR8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR8`"]
        pub struct ODR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `ODR9`"]
        pub type ODR9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR9`"]
        pub struct ODR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `ODR10`"]
        pub type ODR10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR10`"]
        pub struct ODR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `ODR11`"]
        pub type ODR11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR11`"]
        pub struct ODR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `ODR12`"]
        pub type ODR12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR12`"]
        pub struct ODR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `ODR13`"]
        pub type ODR13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR13`"]
        pub struct ODR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `ODR14`"]
        pub type ODR14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR14`"]
        pub struct ODR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `ODR15`"]
        pub type ODR15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR15`"]
        pub struct ODR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&self) -> ODR0_R {
                ODR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&self) -> ODR1_R {
                ODR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&self) -> ODR2_R {
                ODR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&self) -> ODR3_R {
                ODR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&self) -> ODR4_R {
                ODR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&self) -> ODR5_R {
                ODR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&self) -> ODR6_R {
                ODR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&self) -> ODR7_R {
                ODR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&self) -> ODR8_R {
                ODR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&self) -> ODR9_R {
                ODR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&self) -> ODR10_R {
                ODR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&self) -> ODR11_R {
                ODR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&self) -> ODR12_R {
                ODR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&self) -> ODR13_R {
                ODR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&self) -> ODR14_R {
                ODR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&self) -> ODR15_R {
                ODR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&mut self) -> ODR0_W {
                ODR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&mut self) -> ODR1_W {
                ODR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&mut self) -> ODR2_W {
                ODR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&mut self) -> ODR3_W {
                ODR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&mut self) -> ODR4_W {
                ODR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&mut self) -> ODR5_W {
                ODR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&mut self) -> ODR6_W {
                ODR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&mut self) -> ODR7_W {
                ODR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&mut self) -> ODR8_W {
                ODR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&mut self) -> ODR9_W {
                ODR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&mut self) -> ODR10_W {
                ODR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&mut self) -> ODR11_W {
                ODR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&mut self) -> ODR12_W {
                ODR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&mut self) -> ODR13_W {
                ODR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&mut self) -> ODR14_W {
                ODR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&mut self) -> ODR15_W {
                ODR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
    pub type BSRR = crate::Reg<u32, _BSRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BSRR;
    #[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
    impl crate::Writable for BSRR {}
    #[doc = "GPIO port bit set/reset register"]
    pub mod bsrr {
        #[doc = "Writer for register BSRR"]
        pub type W = crate::W<u32, super::BSRR>;
        #[doc = "Register BSRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BSRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BS0`"]
        pub struct BS0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS0_W<'a> {
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
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BS1`"]
        pub struct BS1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS1_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BS2`"]
        pub struct BS2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS2_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS3`"]
        pub struct BS3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS3_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS4`"]
        pub struct BS4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS5`"]
        pub struct BS5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS6`"]
        pub struct BS6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS7`"]
        pub struct BS7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
        #[doc = "Write proxy for field `BS8`"]
        pub struct BS8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
        #[doc = "Write proxy for field `BS9`"]
        pub struct BS9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS10`"]
        pub struct BS10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
        #[doc = "Write proxy for field `BS11`"]
        pub struct BS11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS12`"]
        pub struct BS12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS13`"]
        pub struct BS13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
        #[doc = "Write proxy for field `BS14`"]
        pub struct BS14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
        #[doc = "Write proxy for field `BS15`"]
        pub struct BS15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
        impl W {
            #[doc = "Bit 0 - Pin 0 set bit"]
            #[inline(always)]
            pub fn bs0(&mut self) -> BS0_W {
                BS0_W { w: self }
            }
            #[doc = "Bit 16 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 set bit"]
            #[inline(always)]
            pub fn bs1(&mut self) -> BS1_W {
                BS1_W { w: self }
            }
            #[doc = "Bit 17 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 set bit"]
            #[inline(always)]
            pub fn bs2(&mut self) -> BS2_W {
                BS2_W { w: self }
            }
            #[doc = "Bit 18 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 set bit"]
            #[inline(always)]
            pub fn bs3(&mut self) -> BS3_W {
                BS3_W { w: self }
            }
            #[doc = "Bit 19 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 set bit"]
            #[inline(always)]
            pub fn bs4(&mut self) -> BS4_W {
                BS4_W { w: self }
            }
            #[doc = "Bit 20 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 set bit"]
            #[inline(always)]
            pub fn bs5(&mut self) -> BS5_W {
                BS5_W { w: self }
            }
            #[doc = "Bit 21 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 set bit"]
            #[inline(always)]
            pub fn bs6(&mut self) -> BS6_W {
                BS6_W { w: self }
            }
            #[doc = "Bit 22 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 set bit"]
            #[inline(always)]
            pub fn bs7(&mut self) -> BS7_W {
                BS7_W { w: self }
            }
            #[doc = "Bit 23 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 set bit"]
            #[inline(always)]
            pub fn bs8(&mut self) -> BS8_W {
                BS8_W { w: self }
            }
            #[doc = "Bit 24 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 set bit"]
            #[inline(always)]
            pub fn bs9(&mut self) -> BS9_W {
                BS9_W { w: self }
            }
            #[doc = "Bit 25 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 set bit"]
            #[inline(always)]
            pub fn bs10(&mut self) -> BS10_W {
                BS10_W { w: self }
            }
            #[doc = "Bit 26 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 set bit"]
            #[inline(always)]
            pub fn bs11(&mut self) -> BS11_W {
                BS11_W { w: self }
            }
            #[doc = "Bit 27 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 set bit"]
            #[inline(always)]
            pub fn bs12(&mut self) -> BS12_W {
                BS12_W { w: self }
            }
            #[doc = "Bit 28 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 set bit"]
            #[inline(always)]
            pub fn bs13(&mut self) -> BS13_W {
                BS13_W { w: self }
            }
            #[doc = "Bit 29 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 set bit"]
            #[inline(always)]
            pub fn bs14(&mut self) -> BS14_W {
                BS14_W { w: self }
            }
            #[doc = "Bit 30 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 set bit"]
            #[inline(always)]
            pub fn bs15(&mut self) -> BS15_W {
                BS15_W { w: self }
            }
            #[doc = "Bit 31 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
    pub type LCKR = crate::Reg<u32, _LCKR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _LCKR;
    #[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
    impl crate::Readable for LCKR {}
    #[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
    impl crate::Writable for LCKR {}
    #[doc = "GPIO port configuration lock register"]
    pub mod lckr {
        #[doc = "Reader of register LCKR"]
        pub type R = crate::R<u32, super::LCKR>;
        #[doc = "Writer for register LCKR"]
        pub type W = crate::W<u32, super::LCKR>;
        #[doc = "Register LCKR `reset()`'s with value 0"]
        impl crate::ResetValue for super::LCKR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `LCK0`"]
        pub type LCK0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK0`"]
        pub struct LCK0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK0_W<'a> {
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
        #[doc = "Reader of field `LCK1`"]
        pub type LCK1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK1`"]
        pub struct LCK1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK1_W<'a> {
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
        #[doc = "Reader of field `LCK2`"]
        pub type LCK2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK2`"]
        pub struct LCK2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK2_W<'a> {
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
        #[doc = "Reader of field `LCK3`"]
        pub type LCK3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK3`"]
        pub struct LCK3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK3_W<'a> {
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
        #[doc = "Reader of field `LCK4`"]
        pub type LCK4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK4`"]
        pub struct LCK4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `LCK5`"]
        pub type LCK5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK5`"]
        pub struct LCK5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `LCK6`"]
        pub type LCK6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK6`"]
        pub struct LCK6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `LCK7`"]
        pub type LCK7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK7`"]
        pub struct LCK7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `LCK8`"]
        pub type LCK8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK8`"]
        pub struct LCK8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `LCK9`"]
        pub type LCK9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK9`"]
        pub struct LCK9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `LCK10`"]
        pub type LCK10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK10`"]
        pub struct LCK10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `LCK11`"]
        pub type LCK11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK11`"]
        pub struct LCK11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `LCK12`"]
        pub type LCK12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK12`"]
        pub struct LCK12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `LCK13`"]
        pub type LCK13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK13`"]
        pub struct LCK13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `LCK14`"]
        pub type LCK14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK14`"]
        pub struct LCK14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `LCK15`"]
        pub type LCK15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK15`"]
        pub struct LCK15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Reader of field `LCKK`"]
        pub type LCKK_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCKK`"]
        pub struct LCKK_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCKK_W<'a> {
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
        impl R {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&self) -> LCK0_R {
                LCK0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&self) -> LCK1_R {
                LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&self) -> LCK2_R {
                LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&self) -> LCK3_R {
                LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&self) -> LCK4_R {
                LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&self) -> LCK5_R {
                LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&self) -> LCK6_R {
                LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&self) -> LCK7_R {
                LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&self) -> LCK8_R {
                LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&self) -> LCK9_R {
                LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&self) -> LCK10_R {
                LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&self) -> LCK11_R {
                LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&self) -> LCK12_R {
                LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&self) -> LCK13_R {
                LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&self) -> LCK14_R {
                LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&self) -> LCK15_R {
                LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&self) -> LCKK_R {
                LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&mut self) -> LCK0_W {
                LCK0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&mut self) -> LCK1_W {
                LCK1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&mut self) -> LCK2_W {
                LCK2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&mut self) -> LCK3_W {
                LCK3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&mut self) -> LCK4_W {
                LCK4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&mut self) -> LCK5_W {
                LCK5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&mut self) -> LCK6_W {
                LCK6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&mut self) -> LCK7_W {
                LCK7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&mut self) -> LCK8_W {
                LCK8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&mut self) -> LCK9_W {
                LCK9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&mut self) -> LCK10_W {
                LCK10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&mut self) -> LCK11_W {
                LCK11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&mut self) -> LCK12_W {
                LCK12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&mut self) -> LCK13_W {
                LCK13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&mut self) -> LCK14_W {
                LCK14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&mut self) -> LCK15_W {
                LCK15_W { w: self }
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&mut self) -> LCKK_W {
                LCKK_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](afrl) module"]
    pub type AFRL = crate::Reg<u32, _AFRL>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRL;
    #[doc = "`read()` method returns [afrl::R](afrl::R) reader structure"]
    impl crate::Readable for AFRL {}
    #[doc = "`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure"]
    impl crate::Writable for AFRL {}
    #[doc = "GPIO alternate function low register"]
    pub mod afrl {
        #[doc = "Reader of register AFRL"]
        pub type R = crate::R<u32, super::AFRL>;
        #[doc = "Writer for register AFRL"]
        pub type W = crate::W<u32, super::AFRL>;
        #[doc = "Register AFRL `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRL {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR0`"]
        pub type AFR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR0`"]
        pub struct AFR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
                self.w
            }
        }
        #[doc = "Reader of field `AFR1`"]
        pub type AFR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR1`"]
        pub struct AFR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `AFR2`"]
        pub type AFR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR2`"]
        pub struct AFR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `AFR3`"]
        pub type AFR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR3`"]
        pub struct AFR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `AFR4`"]
        pub type AFR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR4`"]
        pub struct AFR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `AFR5`"]
        pub type AFR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR5`"]
        pub struct AFR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `AFR6`"]
        pub type AFR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR6`"]
        pub struct AFR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `AFR7`"]
        pub type AFR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR7`"]
        pub struct AFR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&self) -> AFR0_R {
                AFR0_R::new((self.bits & 0x0f) as u8)
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&self) -> AFR1_R {
                AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&self) -> AFR2_R {
                AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&self) -> AFR3_R {
                AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&self) -> AFR4_R {
                AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&self) -> AFR5_R {
                AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&self) -> AFR6_R {
                AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&self) -> AFR7_R {
                AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&mut self) -> AFR0_W {
                AFR0_W { w: self }
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&mut self) -> AFR1_W {
                AFR1_W { w: self }
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&mut self) -> AFR2_W {
                AFR2_W { w: self }
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&mut self) -> AFR3_W {
                AFR3_W { w: self }
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&mut self) -> AFR4_W {
                AFR4_W { w: self }
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&mut self) -> AFR5_W {
                AFR5_W { w: self }
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&mut self) -> AFR6_W {
                AFR6_W { w: self }
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&mut self) -> AFR7_W {
                AFR7_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](afrh) module"]
    pub type AFRH = crate::Reg<u32, _AFRH>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRH;
    #[doc = "`read()` method returns [afrh::R](afrh::R) reader structure"]
    impl crate::Readable for AFRH {}
    #[doc = "`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure"]
    impl crate::Writable for AFRH {}
    #[doc = "GPIO alternate function high register"]
    pub mod afrh {
        #[doc = "Reader of register AFRH"]
        pub type R = crate::R<u32, super::AFRH>;
        #[doc = "Writer for register AFRH"]
        pub type W = crate::W<u32, super::AFRH>;
        #[doc = "Register AFRH `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRH {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR8`"]
        pub type AFR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR8`"]
        pub struct AFR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 32)) | (((value as u32) & 0x0f) << 32);
                self.w
            }
        }
        #[doc = "Reader of field `AFR9`"]
        pub type AFR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR9`"]
        pub struct AFR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 36)) | (((value as u32) & 0x0f) << 36);
                self.w
            }
        }
        #[doc = "Reader of field `AFR10`"]
        pub type AFR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR10`"]
        pub struct AFR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 40)) | (((value as u32) & 0x0f) << 40);
                self.w
            }
        }
        #[doc = "Reader of field `AFR11`"]
        pub type AFR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR11`"]
        pub struct AFR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 44)) | (((value as u32) & 0x0f) << 44);
                self.w
            }
        }
        #[doc = "Reader of field `AFR12`"]
        pub type AFR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR12`"]
        pub struct AFR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 48)) | (((value as u32) & 0x0f) << 48);
                self.w
            }
        }
        #[doc = "Reader of field `AFR13`"]
        pub type AFR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR13`"]
        pub struct AFR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 52)) | (((value as u32) & 0x0f) << 52);
                self.w
            }
        }
        #[doc = "Reader of field `AFR14`"]
        pub type AFR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR14`"]
        pub struct AFR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 56)) | (((value as u32) & 0x0f) << 56);
                self.w
            }
        }
        #[doc = "Reader of field `AFR15`"]
        pub type AFR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR15`"]
        pub struct AFR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 60)) | (((value as u32) & 0x0f) << 60);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&self) -> AFR8_R {
                AFR8_R::new(((self.bits >> 32) & 0x0f) as u8)
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&self) -> AFR9_R {
                AFR9_R::new(((self.bits >> 36) & 0x0f) as u8)
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&self) -> AFR10_R {
                AFR10_R::new(((self.bits >> 40) & 0x0f) as u8)
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&self) -> AFR11_R {
                AFR11_R::new(((self.bits >> 44) & 0x0f) as u8)
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&self) -> AFR12_R {
                AFR12_R::new(((self.bits >> 48) & 0x0f) as u8)
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&self) -> AFR13_R {
                AFR13_R::new(((self.bits >> 52) & 0x0f) as u8)
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&self) -> AFR14_R {
                AFR14_R::new(((self.bits >> 56) & 0x0f) as u8)
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&self) -> AFR15_R {
                AFR15_R::new(((self.bits >> 60) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&mut self) -> AFR8_W {
                AFR8_W { w: self }
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&mut self) -> AFR9_W {
                AFR9_W { w: self }
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&mut self) -> AFR10_W {
                AFR10_W { w: self }
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&mut self) -> AFR11_W {
                AFR11_W { w: self }
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&mut self) -> AFR12_W {
                AFR12_W { w: self }
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&mut self) -> AFR13_W {
                AFR13_W { w: self }
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&mut self) -> AFR14_W {
                AFR14_W { w: self }
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&mut self) -> AFR15_W {
                AFR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
    pub type BRR = crate::Reg<u32, _BRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BRR;
    #[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
    impl crate::Writable for BRR {}
    #[doc = "GPIO port bit reset register"]
    pub mod brr {
        #[doc = "Writer for register BRR"]
        pub type W = crate::W<u32, super::BRR>;
        #[doc = "Register BRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
}
#[doc = "General-purpose I/Os, port C"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os, port C"]
pub mod gpioc {
    #[doc = r"Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - GPIO port mode register"]
        pub moder: MODER,
        #[doc = "0x04 - GPIO port output type register"]
        pub otyper: OTYPER,
        #[doc = "0x08 - GPIO port output speed register"]
        pub ospeedr: OSPEEDR,
        #[doc = "0x0c - GPIO port pull-up/pull-down register"]
        pub pupdr: PUPDR,
        #[doc = "0x10 - GPIO port input data register"]
        pub idr: IDR,
        #[doc = "0x14 - GPIO port output data register"]
        pub udr: UDR,
        #[doc = "0x18 - GPIO port bit set/reset register"]
        pub bsrr: BSRR,
        #[doc = "0x1c - GPIO port configuration lock register"]
        pub lckr: LCKR,
        #[doc = "0x20 - GPIO alternate function low register"]
        pub afrl: AFRL,
        #[doc = "0x24 - GPIO alternate function high register"]
        pub afrh: AFRH,
        #[doc = "0x28 - GPIO port bit reset register"]
        pub brr: BRR,
    }
    #[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](moder) module"]
    pub type MODER = crate::Reg<u32, _MODER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _MODER;
    #[doc = "`read()` method returns [moder::R](moder::R) reader structure"]
    impl crate::Readable for MODER {}
    #[doc = "`write(|w| ..)` method takes [moder::W](moder::W) writer structure"]
    impl crate::Writable for MODER {}
    #[doc = "GPIO port mode register"]
    pub mod moder {
        #[doc = "Reader of register MODER"]
        pub type R = crate::R<u32, super::MODER>;
        #[doc = "Writer for register MODER"]
        pub type W = crate::W<u32, super::MODER>;
        #[doc = "Register MODER `reset()`'s with value 0"]
        impl crate::ResetValue for super::MODER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `MODER0`"]
        pub type MODER0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER0`"]
        pub struct MODER0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `MODER1`"]
        pub type MODER1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER1`"]
        pub struct MODER1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `MODER2`"]
        pub type MODER2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER2`"]
        pub struct MODER2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `MODER3`"]
        pub type MODER3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER3`"]
        pub struct MODER3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `MODER4`"]
        pub type MODER4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER4`"]
        pub struct MODER4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `MODER5`"]
        pub type MODER5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER5`"]
        pub struct MODER5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `MODER6`"]
        pub type MODER6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER6`"]
        pub struct MODER6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `MODER7`"]
        pub type MODER7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER7`"]
        pub struct MODER7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `MODER8`"]
        pub type MODER8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER8`"]
        pub struct MODER8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `MODER9`"]
        pub type MODER9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER9`"]
        pub struct MODER9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `MODER10`"]
        pub type MODER10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER10`"]
        pub struct MODER10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `MODER11`"]
        pub type MODER11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER11`"]
        pub struct MODER11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `MODER12`"]
        pub type MODER12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER12`"]
        pub struct MODER12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `MODER13`"]
        pub type MODER13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER13`"]
        pub struct MODER13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `MODER14`"]
        pub type MODER14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER14`"]
        pub struct MODER14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `MODER15`"]
        pub type MODER15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER15`"]
        pub struct MODER15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&self) -> MODER0_R {
                MODER0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&self) -> MODER1_R {
                MODER1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&self) -> MODER2_R {
                MODER2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&self) -> MODER3_R {
                MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&self) -> MODER4_R {
                MODER4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&self) -> MODER5_R {
                MODER5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&self) -> MODER6_R {
                MODER6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&self) -> MODER7_R {
                MODER7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&self) -> MODER8_R {
                MODER8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&self) -> MODER9_R {
                MODER9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&self) -> MODER10_R {
                MODER10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&self) -> MODER11_R {
                MODER11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&self) -> MODER12_R {
                MODER12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&self) -> MODER13_R {
                MODER13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&self) -> MODER14_R {
                MODER14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&self) -> MODER15_R {
                MODER15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&mut self) -> MODER0_W {
                MODER0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&mut self) -> MODER1_W {
                MODER1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&mut self) -> MODER2_W {
                MODER2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&mut self) -> MODER3_W {
                MODER3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&mut self) -> MODER4_W {
                MODER4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&mut self) -> MODER5_W {
                MODER5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&mut self) -> MODER6_W {
                MODER6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&mut self) -> MODER7_W {
                MODER7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&mut self) -> MODER8_W {
                MODER8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&mut self) -> MODER9_W {
                MODER9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&mut self) -> MODER10_W {
                MODER10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&mut self) -> MODER11_W {
                MODER11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&mut self) -> MODER12_W {
                MODER12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&mut self) -> MODER13_W {
                MODER13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&mut self) -> MODER14_W {
                MODER14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&mut self) -> MODER15_W {
                MODER15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](otyper) module"]
    pub type OTYPER = crate::Reg<u32, _OTYPER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OTYPER;
    #[doc = "`read()` method returns [otyper::R](otyper::R) reader structure"]
    impl crate::Readable for OTYPER {}
    #[doc = "`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure"]
    impl crate::Writable for OTYPER {}
    #[doc = "GPIO port output type register"]
    pub mod otyper {
        #[doc = "Reader of register OTYPER"]
        pub type R = crate::R<u32, super::OTYPER>;
        #[doc = "Writer for register OTYPER"]
        pub type W = crate::W<u32, super::OTYPER>;
        #[doc = "Register OTYPER `reset()`'s with value 0"]
        impl crate::ResetValue for super::OTYPER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OT0`"]
        pub type OT0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT0`"]
        pub struct OT0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT0_W<'a> {
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
        #[doc = "Reader of field `OT1`"]
        pub type OT1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT1`"]
        pub struct OT1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT1_W<'a> {
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
        #[doc = "Reader of field `OT2`"]
        pub type OT2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT2`"]
        pub struct OT2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT2_W<'a> {
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
        #[doc = "Reader of field `OT3`"]
        pub type OT3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT3`"]
        pub struct OT3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT3_W<'a> {
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
        #[doc = "Reader of field `OT4`"]
        pub type OT4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT4`"]
        pub struct OT4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OT5`"]
        pub type OT5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT5`"]
        pub struct OT5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `OT6`"]
        pub type OT6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT6`"]
        pub struct OT6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OT7`"]
        pub type OT7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT7`"]
        pub struct OT7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `OT8`"]
        pub type OT8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT8`"]
        pub struct OT8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OT9`"]
        pub type OT9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT9`"]
        pub struct OT9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `OT10`"]
        pub type OT10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT10`"]
        pub struct OT10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OT11`"]
        pub type OT11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT11`"]
        pub struct OT11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `OT12`"]
        pub type OT12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT12`"]
        pub struct OT12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OT13`"]
        pub type OT13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT13`"]
        pub struct OT13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `OT14`"]
        pub type OT14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT14`"]
        pub struct OT14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OT15`"]
        pub type OT15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT15`"]
        pub struct OT15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&self) -> OT0_R {
                OT0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&self) -> OT1_R {
                OT1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&self) -> OT2_R {
                OT2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&self) -> OT3_R {
                OT3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&self) -> OT4_R {
                OT4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&self) -> OT5_R {
                OT5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&self) -> OT6_R {
                OT6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&self) -> OT7_R {
                OT7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&self) -> OT8_R {
                OT8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&self) -> OT9_R {
                OT9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&self) -> OT10_R {
                OT10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&self) -> OT11_R {
                OT11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&self) -> OT12_R {
                OT12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&self) -> OT13_R {
                OT13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&self) -> OT14_R {
                OT14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&self) -> OT15_R {
                OT15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&mut self) -> OT0_W {
                OT0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&mut self) -> OT1_W {
                OT1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&mut self) -> OT2_W {
                OT2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&mut self) -> OT3_W {
                OT3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&mut self) -> OT4_W {
                OT4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&mut self) -> OT5_W {
                OT5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&mut self) -> OT6_W {
                OT6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&mut self) -> OT7_W {
                OT7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&mut self) -> OT8_W {
                OT8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&mut self) -> OT9_W {
                OT9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&mut self) -> OT10_W {
                OT10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&mut self) -> OT11_W {
                OT11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&mut self) -> OT12_W {
                OT12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&mut self) -> OT13_W {
                OT13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&mut self) -> OT14_W {
                OT14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&mut self) -> OT15_W {
                OT15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](ospeedr) module"]
    pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OSPEEDR;
    #[doc = "`read()` method returns [ospeedr::R](ospeedr::R) reader structure"]
    impl crate::Readable for OSPEEDR {}
    #[doc = "`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure"]
    impl crate::Writable for OSPEEDR {}
    #[doc = "GPIO port output speed register"]
    pub mod ospeedr {
        #[doc = "Reader of register OSPEEDR"]
        pub type R = crate::R<u32, super::OSPEEDR>;
        #[doc = "Writer for register OSPEEDR"]
        pub type W = crate::W<u32, super::OSPEEDR>;
        #[doc = "Register OSPEEDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::OSPEEDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OSPEEDR0`"]
        pub type OSPEEDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR0`"]
        pub struct OSPEEDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR1`"]
        pub type OSPEEDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR1`"]
        pub struct OSPEEDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR2`"]
        pub type OSPEEDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR2`"]
        pub struct OSPEEDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR3`"]
        pub type OSPEEDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR3`"]
        pub struct OSPEEDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR4`"]
        pub type OSPEEDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR4`"]
        pub struct OSPEEDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR5`"]
        pub type OSPEEDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR5`"]
        pub struct OSPEEDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR6`"]
        pub type OSPEEDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR6`"]
        pub struct OSPEEDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR7`"]
        pub type OSPEEDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR7`"]
        pub struct OSPEEDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR8`"]
        pub type OSPEEDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR8`"]
        pub struct OSPEEDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR9`"]
        pub type OSPEEDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR9`"]
        pub struct OSPEEDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR10`"]
        pub type OSPEEDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR10`"]
        pub struct OSPEEDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR11`"]
        pub type OSPEEDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR11`"]
        pub struct OSPEEDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR12`"]
        pub type OSPEEDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR12`"]
        pub struct OSPEEDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR13`"]
        pub type OSPEEDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR13`"]
        pub struct OSPEEDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR14`"]
        pub type OSPEEDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR14`"]
        pub struct OSPEEDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR15`"]
        pub type OSPEEDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR15`"]
        pub struct OSPEEDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&self) -> OSPEEDR0_R {
                OSPEEDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&self) -> OSPEEDR1_R {
                OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&self) -> OSPEEDR2_R {
                OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&self) -> OSPEEDR3_R {
                OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&self) -> OSPEEDR4_R {
                OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&self) -> OSPEEDR5_R {
                OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&self) -> OSPEEDR6_R {
                OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&self) -> OSPEEDR7_R {
                OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&self) -> OSPEEDR8_R {
                OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&self) -> OSPEEDR9_R {
                OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&self) -> OSPEEDR10_R {
                OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&self) -> OSPEEDR11_R {
                OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&self) -> OSPEEDR12_R {
                OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&self) -> OSPEEDR13_R {
                OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&self) -> OSPEEDR14_R {
                OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&self) -> OSPEEDR15_R {
                OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
                OSPEEDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
                OSPEEDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
                OSPEEDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
                OSPEEDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
                OSPEEDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
                OSPEEDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
                OSPEEDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
                OSPEEDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
                OSPEEDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
                OSPEEDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
                OSPEEDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
                OSPEEDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
                OSPEEDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
                OSPEEDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
                OSPEEDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
                OSPEEDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](pupdr) module"]
    pub type PUPDR = crate::Reg<u32, _PUPDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _PUPDR;
    #[doc = "`read()` method returns [pupdr::R](pupdr::R) reader structure"]
    impl crate::Readable for PUPDR {}
    #[doc = "`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure"]
    impl crate::Writable for PUPDR {}
    #[doc = "GPIO port pull-up/pull-down register"]
    pub mod pupdr {
        #[doc = "Reader of register PUPDR"]
        pub type R = crate::R<u32, super::PUPDR>;
        #[doc = "Writer for register PUPDR"]
        pub type W = crate::W<u32, super::PUPDR>;
        #[doc = "Register PUPDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::PUPDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `PUPDR0`"]
        pub type PUPDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR0`"]
        pub struct PUPDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR1`"]
        pub type PUPDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR1`"]
        pub struct PUPDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR2`"]
        pub type PUPDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR2`"]
        pub struct PUPDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR3`"]
        pub type PUPDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR3`"]
        pub struct PUPDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR4`"]
        pub type PUPDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR4`"]
        pub struct PUPDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR5`"]
        pub type PUPDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR5`"]
        pub struct PUPDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR6`"]
        pub type PUPDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR6`"]
        pub struct PUPDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR7`"]
        pub type PUPDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR7`"]
        pub struct PUPDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR8`"]
        pub type PUPDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR8`"]
        pub struct PUPDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR9`"]
        pub type PUPDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR9`"]
        pub struct PUPDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR10`"]
        pub type PUPDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR10`"]
        pub struct PUPDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR11`"]
        pub type PUPDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR11`"]
        pub struct PUPDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR12`"]
        pub type PUPDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR12`"]
        pub struct PUPDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR13`"]
        pub type PUPDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR13`"]
        pub struct PUPDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR14`"]
        pub type PUPDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR14`"]
        pub struct PUPDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR15`"]
        pub type PUPDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR15`"]
        pub struct PUPDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&self) -> PUPDR0_R {
                PUPDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&self) -> PUPDR1_R {
                PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&self) -> PUPDR2_R {
                PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&self) -> PUPDR3_R {
                PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&self) -> PUPDR4_R {
                PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&self) -> PUPDR5_R {
                PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&self) -> PUPDR6_R {
                PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&self) -> PUPDR7_R {
                PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&self) -> PUPDR8_R {
                PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&self) -> PUPDR9_R {
                PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&self) -> PUPDR10_R {
                PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&self) -> PUPDR11_R {
                PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&self) -> PUPDR12_R {
                PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&self) -> PUPDR13_R {
                PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&self) -> PUPDR14_R {
                PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&self) -> PUPDR15_R {
                PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&mut self) -> PUPDR0_W {
                PUPDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&mut self) -> PUPDR1_W {
                PUPDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&mut self) -> PUPDR2_W {
                PUPDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&mut self) -> PUPDR3_W {
                PUPDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&mut self) -> PUPDR4_W {
                PUPDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&mut self) -> PUPDR5_W {
                PUPDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&mut self) -> PUPDR6_W {
                PUPDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&mut self) -> PUPDR7_W {
                PUPDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&mut self) -> PUPDR8_W {
                PUPDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&mut self) -> PUPDR9_W {
                PUPDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&mut self) -> PUPDR10_W {
                PUPDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&mut self) -> PUPDR11_W {
                PUPDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&mut self) -> PUPDR12_W {
                PUPDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&mut self) -> PUPDR13_W {
                PUPDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&mut self) -> PUPDR14_W {
                PUPDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&mut self) -> PUPDR15_W {
                PUPDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
    pub type IDR = crate::Reg<u32, _IDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _IDR;
    #[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
    impl crate::Readable for IDR {}
    #[doc = "GPIO port input data register"]
    pub mod idr {
        #[doc = "Reader of register IDR"]
        pub type R = crate::R<u32, super::IDR>;
        #[doc = "Reader of field `IDR0`"]
        pub type IDR0_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR1`"]
        pub type IDR1_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR2`"]
        pub type IDR2_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR3`"]
        pub type IDR3_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR4`"]
        pub type IDR4_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR5`"]
        pub type IDR5_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR6`"]
        pub type IDR6_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR7`"]
        pub type IDR7_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR8`"]
        pub type IDR8_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR9`"]
        pub type IDR9_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR10`"]
        pub type IDR10_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR11`"]
        pub type IDR11_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR12`"]
        pub type IDR12_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR13`"]
        pub type IDR13_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR14`"]
        pub type IDR14_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR15`"]
        pub type IDR15_R = crate::R<bool, bool>;
        impl R {
            #[doc = "Bit 0 - Pin 0 input data bit"]
            #[inline(always)]
            pub fn idr0(&self) -> IDR0_R {
                IDR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 input data bit"]
            #[inline(always)]
            pub fn idr1(&self) -> IDR1_R {
                IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 input data bit"]
            #[inline(always)]
            pub fn idr2(&self) -> IDR2_R {
                IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 input data bit"]
            #[inline(always)]
            pub fn idr3(&self) -> IDR3_R {
                IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 input data bit"]
            #[inline(always)]
            pub fn idr4(&self) -> IDR4_R {
                IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 input data bit"]
            #[inline(always)]
            pub fn idr5(&self) -> IDR5_R {
                IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 input data bit"]
            #[inline(always)]
            pub fn idr6(&self) -> IDR6_R {
                IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 input data bit"]
            #[inline(always)]
            pub fn idr7(&self) -> IDR7_R {
                IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 input data bit"]
            #[inline(always)]
            pub fn idr8(&self) -> IDR8_R {
                IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 input data bit"]
            #[inline(always)]
            pub fn idr9(&self) -> IDR9_R {
                IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 input data bit"]
            #[inline(always)]
            pub fn idr10(&self) -> IDR10_R {
                IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 input data bit"]
            #[inline(always)]
            pub fn idr11(&self) -> IDR11_R {
                IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 input data bit"]
            #[inline(always)]
            pub fn idr12(&self) -> IDR12_R {
                IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 input data bit"]
            #[inline(always)]
            pub fn idr13(&self) -> IDR13_R {
                IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 input data bit"]
            #[inline(always)]
            pub fn idr14(&self) -> IDR14_R {
                IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 input data bit"]
            #[inline(always)]
            pub fn idr15(&self) -> IDR15_R {
                IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
    }
    #[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udr](udr) module"]
    pub type UDR = crate::Reg<u32, _UDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _UDR;
    #[doc = "`read()` method returns [udr::R](udr::R) reader structure"]
    impl crate::Readable for UDR {}
    #[doc = "`write(|w| ..)` method takes [udr::W](udr::W) writer structure"]
    impl crate::Writable for UDR {}
    #[doc = "GPIO port output data register"]
    pub mod udr {
        #[doc = "Reader of register UDR"]
        pub type R = crate::R<u32, super::UDR>;
        #[doc = "Writer for register UDR"]
        pub type W = crate::W<u32, super::UDR>;
        #[doc = "Register UDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::UDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `ODR0`"]
        pub type ODR0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR0`"]
        pub struct ODR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR0_W<'a> {
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
        #[doc = "Reader of field `ODR1`"]
        pub type ODR1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR1`"]
        pub struct ODR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR1_W<'a> {
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
        #[doc = "Reader of field `ODR2`"]
        pub type ODR2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR2`"]
        pub struct ODR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR2_W<'a> {
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
        #[doc = "Reader of field `ODR3`"]
        pub type ODR3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR3`"]
        pub struct ODR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR3_W<'a> {
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
        #[doc = "Reader of field `ODR4`"]
        pub type ODR4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR4`"]
        pub struct ODR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `ODR5`"]
        pub type ODR5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR5`"]
        pub struct ODR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `ODR6`"]
        pub type ODR6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR6`"]
        pub struct ODR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `ODR7`"]
        pub type ODR7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR7`"]
        pub struct ODR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `ODR8`"]
        pub type ODR8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR8`"]
        pub struct ODR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `ODR9`"]
        pub type ODR9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR9`"]
        pub struct ODR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `ODR10`"]
        pub type ODR10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR10`"]
        pub struct ODR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `ODR11`"]
        pub type ODR11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR11`"]
        pub struct ODR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `ODR12`"]
        pub type ODR12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR12`"]
        pub struct ODR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `ODR13`"]
        pub type ODR13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR13`"]
        pub struct ODR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `ODR14`"]
        pub type ODR14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR14`"]
        pub struct ODR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `ODR15`"]
        pub type ODR15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR15`"]
        pub struct ODR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&self) -> ODR0_R {
                ODR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&self) -> ODR1_R {
                ODR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&self) -> ODR2_R {
                ODR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&self) -> ODR3_R {
                ODR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&self) -> ODR4_R {
                ODR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&self) -> ODR5_R {
                ODR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&self) -> ODR6_R {
                ODR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&self) -> ODR7_R {
                ODR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&self) -> ODR8_R {
                ODR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&self) -> ODR9_R {
                ODR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&self) -> ODR10_R {
                ODR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&self) -> ODR11_R {
                ODR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&self) -> ODR12_R {
                ODR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&self) -> ODR13_R {
                ODR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&self) -> ODR14_R {
                ODR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&self) -> ODR15_R {
                ODR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&mut self) -> ODR0_W {
                ODR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&mut self) -> ODR1_W {
                ODR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&mut self) -> ODR2_W {
                ODR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&mut self) -> ODR3_W {
                ODR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&mut self) -> ODR4_W {
                ODR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&mut self) -> ODR5_W {
                ODR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&mut self) -> ODR6_W {
                ODR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&mut self) -> ODR7_W {
                ODR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&mut self) -> ODR8_W {
                ODR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&mut self) -> ODR9_W {
                ODR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&mut self) -> ODR10_W {
                ODR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&mut self) -> ODR11_W {
                ODR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&mut self) -> ODR12_W {
                ODR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&mut self) -> ODR13_W {
                ODR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&mut self) -> ODR14_W {
                ODR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&mut self) -> ODR15_W {
                ODR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
    pub type BSRR = crate::Reg<u32, _BSRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BSRR;
    #[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
    impl crate::Writable for BSRR {}
    #[doc = "GPIO port bit set/reset register"]
    pub mod bsrr {
        #[doc = "Writer for register BSRR"]
        pub type W = crate::W<u32, super::BSRR>;
        #[doc = "Register BSRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BSRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BS0`"]
        pub struct BS0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS0_W<'a> {
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
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BS1`"]
        pub struct BS1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS1_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BS2`"]
        pub struct BS2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS2_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS3`"]
        pub struct BS3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS3_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS4`"]
        pub struct BS4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS5`"]
        pub struct BS5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS6`"]
        pub struct BS6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS7`"]
        pub struct BS7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
        #[doc = "Write proxy for field `BS8`"]
        pub struct BS8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
        #[doc = "Write proxy for field `BS9`"]
        pub struct BS9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS10`"]
        pub struct BS10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
        #[doc = "Write proxy for field `BS11`"]
        pub struct BS11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS12`"]
        pub struct BS12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS13`"]
        pub struct BS13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
        #[doc = "Write proxy for field `BS14`"]
        pub struct BS14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
        #[doc = "Write proxy for field `BS15`"]
        pub struct BS15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
        impl W {
            #[doc = "Bit 0 - Pin 0 set bit"]
            #[inline(always)]
            pub fn bs0(&mut self) -> BS0_W {
                BS0_W { w: self }
            }
            #[doc = "Bit 16 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 set bit"]
            #[inline(always)]
            pub fn bs1(&mut self) -> BS1_W {
                BS1_W { w: self }
            }
            #[doc = "Bit 17 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 set bit"]
            #[inline(always)]
            pub fn bs2(&mut self) -> BS2_W {
                BS2_W { w: self }
            }
            #[doc = "Bit 18 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 set bit"]
            #[inline(always)]
            pub fn bs3(&mut self) -> BS3_W {
                BS3_W { w: self }
            }
            #[doc = "Bit 19 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 set bit"]
            #[inline(always)]
            pub fn bs4(&mut self) -> BS4_W {
                BS4_W { w: self }
            }
            #[doc = "Bit 20 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 set bit"]
            #[inline(always)]
            pub fn bs5(&mut self) -> BS5_W {
                BS5_W { w: self }
            }
            #[doc = "Bit 21 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 set bit"]
            #[inline(always)]
            pub fn bs6(&mut self) -> BS6_W {
                BS6_W { w: self }
            }
            #[doc = "Bit 22 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 set bit"]
            #[inline(always)]
            pub fn bs7(&mut self) -> BS7_W {
                BS7_W { w: self }
            }
            #[doc = "Bit 23 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 set bit"]
            #[inline(always)]
            pub fn bs8(&mut self) -> BS8_W {
                BS8_W { w: self }
            }
            #[doc = "Bit 24 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 set bit"]
            #[inline(always)]
            pub fn bs9(&mut self) -> BS9_W {
                BS9_W { w: self }
            }
            #[doc = "Bit 25 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 set bit"]
            #[inline(always)]
            pub fn bs10(&mut self) -> BS10_W {
                BS10_W { w: self }
            }
            #[doc = "Bit 26 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 set bit"]
            #[inline(always)]
            pub fn bs11(&mut self) -> BS11_W {
                BS11_W { w: self }
            }
            #[doc = "Bit 27 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 set bit"]
            #[inline(always)]
            pub fn bs12(&mut self) -> BS12_W {
                BS12_W { w: self }
            }
            #[doc = "Bit 28 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 set bit"]
            #[inline(always)]
            pub fn bs13(&mut self) -> BS13_W {
                BS13_W { w: self }
            }
            #[doc = "Bit 29 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 set bit"]
            #[inline(always)]
            pub fn bs14(&mut self) -> BS14_W {
                BS14_W { w: self }
            }
            #[doc = "Bit 30 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 set bit"]
            #[inline(always)]
            pub fn bs15(&mut self) -> BS15_W {
                BS15_W { w: self }
            }
            #[doc = "Bit 31 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
    pub type LCKR = crate::Reg<u32, _LCKR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _LCKR;
    #[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
    impl crate::Readable for LCKR {}
    #[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
    impl crate::Writable for LCKR {}
    #[doc = "GPIO port configuration lock register"]
    pub mod lckr {
        #[doc = "Reader of register LCKR"]
        pub type R = crate::R<u32, super::LCKR>;
        #[doc = "Writer for register LCKR"]
        pub type W = crate::W<u32, super::LCKR>;
        #[doc = "Register LCKR `reset()`'s with value 0"]
        impl crate::ResetValue for super::LCKR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `LCK0`"]
        pub type LCK0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK0`"]
        pub struct LCK0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK0_W<'a> {
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
        #[doc = "Reader of field `LCK1`"]
        pub type LCK1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK1`"]
        pub struct LCK1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK1_W<'a> {
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
        #[doc = "Reader of field `LCK2`"]
        pub type LCK2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK2`"]
        pub struct LCK2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK2_W<'a> {
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
        #[doc = "Reader of field `LCK3`"]
        pub type LCK3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK3`"]
        pub struct LCK3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK3_W<'a> {
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
        #[doc = "Reader of field `LCK4`"]
        pub type LCK4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK4`"]
        pub struct LCK4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `LCK5`"]
        pub type LCK5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK5`"]
        pub struct LCK5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `LCK6`"]
        pub type LCK6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK6`"]
        pub struct LCK6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `LCK7`"]
        pub type LCK7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK7`"]
        pub struct LCK7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `LCK8`"]
        pub type LCK8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK8`"]
        pub struct LCK8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `LCK9`"]
        pub type LCK9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK9`"]
        pub struct LCK9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `LCK10`"]
        pub type LCK10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK10`"]
        pub struct LCK10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `LCK11`"]
        pub type LCK11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK11`"]
        pub struct LCK11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `LCK12`"]
        pub type LCK12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK12`"]
        pub struct LCK12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `LCK13`"]
        pub type LCK13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK13`"]
        pub struct LCK13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `LCK14`"]
        pub type LCK14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK14`"]
        pub struct LCK14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `LCK15`"]
        pub type LCK15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK15`"]
        pub struct LCK15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Reader of field `LCKK`"]
        pub type LCKK_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCKK`"]
        pub struct LCKK_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCKK_W<'a> {
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
        impl R {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&self) -> LCK0_R {
                LCK0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&self) -> LCK1_R {
                LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&self) -> LCK2_R {
                LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&self) -> LCK3_R {
                LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&self) -> LCK4_R {
                LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&self) -> LCK5_R {
                LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&self) -> LCK6_R {
                LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&self) -> LCK7_R {
                LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&self) -> LCK8_R {
                LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&self) -> LCK9_R {
                LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&self) -> LCK10_R {
                LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&self) -> LCK11_R {
                LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&self) -> LCK12_R {
                LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&self) -> LCK13_R {
                LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&self) -> LCK14_R {
                LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&self) -> LCK15_R {
                LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&self) -> LCKK_R {
                LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&mut self) -> LCK0_W {
                LCK0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&mut self) -> LCK1_W {
                LCK1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&mut self) -> LCK2_W {
                LCK2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&mut self) -> LCK3_W {
                LCK3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&mut self) -> LCK4_W {
                LCK4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&mut self) -> LCK5_W {
                LCK5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&mut self) -> LCK6_W {
                LCK6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&mut self) -> LCK7_W {
                LCK7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&mut self) -> LCK8_W {
                LCK8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&mut self) -> LCK9_W {
                LCK9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&mut self) -> LCK10_W {
                LCK10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&mut self) -> LCK11_W {
                LCK11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&mut self) -> LCK12_W {
                LCK12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&mut self) -> LCK13_W {
                LCK13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&mut self) -> LCK14_W {
                LCK14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&mut self) -> LCK15_W {
                LCK15_W { w: self }
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&mut self) -> LCKK_W {
                LCKK_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](afrl) module"]
    pub type AFRL = crate::Reg<u32, _AFRL>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRL;
    #[doc = "`read()` method returns [afrl::R](afrl::R) reader structure"]
    impl crate::Readable for AFRL {}
    #[doc = "`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure"]
    impl crate::Writable for AFRL {}
    #[doc = "GPIO alternate function low register"]
    pub mod afrl {
        #[doc = "Reader of register AFRL"]
        pub type R = crate::R<u32, super::AFRL>;
        #[doc = "Writer for register AFRL"]
        pub type W = crate::W<u32, super::AFRL>;
        #[doc = "Register AFRL `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRL {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR0`"]
        pub type AFR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR0`"]
        pub struct AFR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
                self.w
            }
        }
        #[doc = "Reader of field `AFR1`"]
        pub type AFR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR1`"]
        pub struct AFR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `AFR2`"]
        pub type AFR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR2`"]
        pub struct AFR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `AFR3`"]
        pub type AFR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR3`"]
        pub struct AFR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `AFR4`"]
        pub type AFR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR4`"]
        pub struct AFR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `AFR5`"]
        pub type AFR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR5`"]
        pub struct AFR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `AFR6`"]
        pub type AFR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR6`"]
        pub struct AFR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `AFR7`"]
        pub type AFR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR7`"]
        pub struct AFR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&self) -> AFR0_R {
                AFR0_R::new((self.bits & 0x0f) as u8)
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&self) -> AFR1_R {
                AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&self) -> AFR2_R {
                AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&self) -> AFR3_R {
                AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&self) -> AFR4_R {
                AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&self) -> AFR5_R {
                AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&self) -> AFR6_R {
                AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&self) -> AFR7_R {
                AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&mut self) -> AFR0_W {
                AFR0_W { w: self }
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&mut self) -> AFR1_W {
                AFR1_W { w: self }
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&mut self) -> AFR2_W {
                AFR2_W { w: self }
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&mut self) -> AFR3_W {
                AFR3_W { w: self }
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&mut self) -> AFR4_W {
                AFR4_W { w: self }
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&mut self) -> AFR5_W {
                AFR5_W { w: self }
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&mut self) -> AFR6_W {
                AFR6_W { w: self }
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&mut self) -> AFR7_W {
                AFR7_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](afrh) module"]
    pub type AFRH = crate::Reg<u32, _AFRH>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRH;
    #[doc = "`read()` method returns [afrh::R](afrh::R) reader structure"]
    impl crate::Readable for AFRH {}
    #[doc = "`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure"]
    impl crate::Writable for AFRH {}
    #[doc = "GPIO alternate function high register"]
    pub mod afrh {
        #[doc = "Reader of register AFRH"]
        pub type R = crate::R<u32, super::AFRH>;
        #[doc = "Writer for register AFRH"]
        pub type W = crate::W<u32, super::AFRH>;
        #[doc = "Register AFRH `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRH {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR8`"]
        pub type AFR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR8`"]
        pub struct AFR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 32)) | (((value as u32) & 0x0f) << 32);
                self.w
            }
        }
        #[doc = "Reader of field `AFR9`"]
        pub type AFR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR9`"]
        pub struct AFR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 36)) | (((value as u32) & 0x0f) << 36);
                self.w
            }
        }
        #[doc = "Reader of field `AFR10`"]
        pub type AFR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR10`"]
        pub struct AFR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 40)) | (((value as u32) & 0x0f) << 40);
                self.w
            }
        }
        #[doc = "Reader of field `AFR11`"]
        pub type AFR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR11`"]
        pub struct AFR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 44)) | (((value as u32) & 0x0f) << 44);
                self.w
            }
        }
        #[doc = "Reader of field `AFR12`"]
        pub type AFR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR12`"]
        pub struct AFR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 48)) | (((value as u32) & 0x0f) << 48);
                self.w
            }
        }
        #[doc = "Reader of field `AFR13`"]
        pub type AFR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR13`"]
        pub struct AFR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 52)) | (((value as u32) & 0x0f) << 52);
                self.w
            }
        }
        #[doc = "Reader of field `AFR14`"]
        pub type AFR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR14`"]
        pub struct AFR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 56)) | (((value as u32) & 0x0f) << 56);
                self.w
            }
        }
        #[doc = "Reader of field `AFR15`"]
        pub type AFR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR15`"]
        pub struct AFR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 60)) | (((value as u32) & 0x0f) << 60);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&self) -> AFR8_R {
                AFR8_R::new(((self.bits >> 32) & 0x0f) as u8)
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&self) -> AFR9_R {
                AFR9_R::new(((self.bits >> 36) & 0x0f) as u8)
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&self) -> AFR10_R {
                AFR10_R::new(((self.bits >> 40) & 0x0f) as u8)
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&self) -> AFR11_R {
                AFR11_R::new(((self.bits >> 44) & 0x0f) as u8)
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&self) -> AFR12_R {
                AFR12_R::new(((self.bits >> 48) & 0x0f) as u8)
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&self) -> AFR13_R {
                AFR13_R::new(((self.bits >> 52) & 0x0f) as u8)
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&self) -> AFR14_R {
                AFR14_R::new(((self.bits >> 56) & 0x0f) as u8)
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&self) -> AFR15_R {
                AFR15_R::new(((self.bits >> 60) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&mut self) -> AFR8_W {
                AFR8_W { w: self }
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&mut self) -> AFR9_W {
                AFR9_W { w: self }
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&mut self) -> AFR10_W {
                AFR10_W { w: self }
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&mut self) -> AFR11_W {
                AFR11_W { w: self }
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&mut self) -> AFR12_W {
                AFR12_W { w: self }
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&mut self) -> AFR13_W {
                AFR13_W { w: self }
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&mut self) -> AFR14_W {
                AFR14_W { w: self }
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&mut self) -> AFR15_W {
                AFR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
    pub type BRR = crate::Reg<u32, _BRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BRR;
    #[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
    impl crate::Writable for BRR {}
    #[doc = "GPIO port bit reset register"]
    pub mod brr {
        #[doc = "Writer for register BRR"]
        pub type W = crate::W<u32, super::BRR>;
        #[doc = "Register BRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
}
#[doc = "General-purpose I/Os, port D"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4800_0c00 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os, port D"]
pub mod gpiod {
    #[doc = r"Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - GPIO port mode register"]
        pub moder: MODER,
        #[doc = "0x04 - GPIO port output type register"]
        pub otyper: OTYPER,
        #[doc = "0x08 - GPIO port output speed register"]
        pub ospeedr: OSPEEDR,
        #[doc = "0x0c - GPIO port pull-up/pull-down register"]
        pub pupdr: PUPDR,
        #[doc = "0x10 - GPIO port input data register"]
        pub idr: IDR,
        #[doc = "0x14 - GPIO port output data register"]
        pub udr: UDR,
        #[doc = "0x18 - GPIO port bit set/reset register"]
        pub bsrr: BSRR,
        #[doc = "0x1c - GPIO port configuration lock register"]
        pub lckr: LCKR,
        #[doc = "0x20 - GPIO alternate function low register"]
        pub afrl: AFRL,
        #[doc = "0x24 - GPIO alternate function high register"]
        pub afrh: AFRH,
        #[doc = "0x28 - GPIO port bit reset register"]
        pub brr: BRR,
    }
    #[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](moder) module"]
    pub type MODER = crate::Reg<u32, _MODER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _MODER;
    #[doc = "`read()` method returns [moder::R](moder::R) reader structure"]
    impl crate::Readable for MODER {}
    #[doc = "`write(|w| ..)` method takes [moder::W](moder::W) writer structure"]
    impl crate::Writable for MODER {}
    #[doc = "GPIO port mode register"]
    pub mod moder {
        #[doc = "Reader of register MODER"]
        pub type R = crate::R<u32, super::MODER>;
        #[doc = "Writer for register MODER"]
        pub type W = crate::W<u32, super::MODER>;
        #[doc = "Register MODER `reset()`'s with value 0"]
        impl crate::ResetValue for super::MODER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `MODER0`"]
        pub type MODER0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER0`"]
        pub struct MODER0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `MODER1`"]
        pub type MODER1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER1`"]
        pub struct MODER1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `MODER2`"]
        pub type MODER2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER2`"]
        pub struct MODER2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `MODER3`"]
        pub type MODER3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER3`"]
        pub struct MODER3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `MODER4`"]
        pub type MODER4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER4`"]
        pub struct MODER4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `MODER5`"]
        pub type MODER5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER5`"]
        pub struct MODER5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `MODER6`"]
        pub type MODER6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER6`"]
        pub struct MODER6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `MODER7`"]
        pub type MODER7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER7`"]
        pub struct MODER7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `MODER8`"]
        pub type MODER8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER8`"]
        pub struct MODER8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `MODER9`"]
        pub type MODER9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER9`"]
        pub struct MODER9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `MODER10`"]
        pub type MODER10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER10`"]
        pub struct MODER10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `MODER11`"]
        pub type MODER11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER11`"]
        pub struct MODER11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `MODER12`"]
        pub type MODER12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER12`"]
        pub struct MODER12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `MODER13`"]
        pub type MODER13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER13`"]
        pub struct MODER13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `MODER14`"]
        pub type MODER14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER14`"]
        pub struct MODER14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `MODER15`"]
        pub type MODER15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER15`"]
        pub struct MODER15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&self) -> MODER0_R {
                MODER0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&self) -> MODER1_R {
                MODER1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&self) -> MODER2_R {
                MODER2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&self) -> MODER3_R {
                MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&self) -> MODER4_R {
                MODER4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&self) -> MODER5_R {
                MODER5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&self) -> MODER6_R {
                MODER6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&self) -> MODER7_R {
                MODER7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&self) -> MODER8_R {
                MODER8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&self) -> MODER9_R {
                MODER9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&self) -> MODER10_R {
                MODER10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&self) -> MODER11_R {
                MODER11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&self) -> MODER12_R {
                MODER12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&self) -> MODER13_R {
                MODER13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&self) -> MODER14_R {
                MODER14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&self) -> MODER15_R {
                MODER15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&mut self) -> MODER0_W {
                MODER0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&mut self) -> MODER1_W {
                MODER1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&mut self) -> MODER2_W {
                MODER2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&mut self) -> MODER3_W {
                MODER3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&mut self) -> MODER4_W {
                MODER4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&mut self) -> MODER5_W {
                MODER5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&mut self) -> MODER6_W {
                MODER6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&mut self) -> MODER7_W {
                MODER7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&mut self) -> MODER8_W {
                MODER8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&mut self) -> MODER9_W {
                MODER9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&mut self) -> MODER10_W {
                MODER10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&mut self) -> MODER11_W {
                MODER11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&mut self) -> MODER12_W {
                MODER12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&mut self) -> MODER13_W {
                MODER13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&mut self) -> MODER14_W {
                MODER14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&mut self) -> MODER15_W {
                MODER15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](otyper) module"]
    pub type OTYPER = crate::Reg<u32, _OTYPER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OTYPER;
    #[doc = "`read()` method returns [otyper::R](otyper::R) reader structure"]
    impl crate::Readable for OTYPER {}
    #[doc = "`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure"]
    impl crate::Writable for OTYPER {}
    #[doc = "GPIO port output type register"]
    pub mod otyper {
        #[doc = "Reader of register OTYPER"]
        pub type R = crate::R<u32, super::OTYPER>;
        #[doc = "Writer for register OTYPER"]
        pub type W = crate::W<u32, super::OTYPER>;
        #[doc = "Register OTYPER `reset()`'s with value 0"]
        impl crate::ResetValue for super::OTYPER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OT0`"]
        pub type OT0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT0`"]
        pub struct OT0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT0_W<'a> {
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
        #[doc = "Reader of field `OT1`"]
        pub type OT1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT1`"]
        pub struct OT1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT1_W<'a> {
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
        #[doc = "Reader of field `OT2`"]
        pub type OT2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT2`"]
        pub struct OT2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT2_W<'a> {
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
        #[doc = "Reader of field `OT3`"]
        pub type OT3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT3`"]
        pub struct OT3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT3_W<'a> {
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
        #[doc = "Reader of field `OT4`"]
        pub type OT4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT4`"]
        pub struct OT4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OT5`"]
        pub type OT5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT5`"]
        pub struct OT5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `OT6`"]
        pub type OT6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT6`"]
        pub struct OT6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OT7`"]
        pub type OT7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT7`"]
        pub struct OT7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `OT8`"]
        pub type OT8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT8`"]
        pub struct OT8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OT9`"]
        pub type OT9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT9`"]
        pub struct OT9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `OT10`"]
        pub type OT10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT10`"]
        pub struct OT10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OT11`"]
        pub type OT11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT11`"]
        pub struct OT11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `OT12`"]
        pub type OT12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT12`"]
        pub struct OT12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OT13`"]
        pub type OT13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT13`"]
        pub struct OT13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `OT14`"]
        pub type OT14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT14`"]
        pub struct OT14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OT15`"]
        pub type OT15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT15`"]
        pub struct OT15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&self) -> OT0_R {
                OT0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&self) -> OT1_R {
                OT1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&self) -> OT2_R {
                OT2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&self) -> OT3_R {
                OT3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&self) -> OT4_R {
                OT4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&self) -> OT5_R {
                OT5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&self) -> OT6_R {
                OT6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&self) -> OT7_R {
                OT7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&self) -> OT8_R {
                OT8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&self) -> OT9_R {
                OT9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&self) -> OT10_R {
                OT10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&self) -> OT11_R {
                OT11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&self) -> OT12_R {
                OT12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&self) -> OT13_R {
                OT13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&self) -> OT14_R {
                OT14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&self) -> OT15_R {
                OT15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&mut self) -> OT0_W {
                OT0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&mut self) -> OT1_W {
                OT1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&mut self) -> OT2_W {
                OT2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&mut self) -> OT3_W {
                OT3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&mut self) -> OT4_W {
                OT4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&mut self) -> OT5_W {
                OT5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&mut self) -> OT6_W {
                OT6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&mut self) -> OT7_W {
                OT7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&mut self) -> OT8_W {
                OT8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&mut self) -> OT9_W {
                OT9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&mut self) -> OT10_W {
                OT10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&mut self) -> OT11_W {
                OT11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&mut self) -> OT12_W {
                OT12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&mut self) -> OT13_W {
                OT13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&mut self) -> OT14_W {
                OT14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&mut self) -> OT15_W {
                OT15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](ospeedr) module"]
    pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OSPEEDR;
    #[doc = "`read()` method returns [ospeedr::R](ospeedr::R) reader structure"]
    impl crate::Readable for OSPEEDR {}
    #[doc = "`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure"]
    impl crate::Writable for OSPEEDR {}
    #[doc = "GPIO port output speed register"]
    pub mod ospeedr {
        #[doc = "Reader of register OSPEEDR"]
        pub type R = crate::R<u32, super::OSPEEDR>;
        #[doc = "Writer for register OSPEEDR"]
        pub type W = crate::W<u32, super::OSPEEDR>;
        #[doc = "Register OSPEEDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::OSPEEDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OSPEEDR0`"]
        pub type OSPEEDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR0`"]
        pub struct OSPEEDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR1`"]
        pub type OSPEEDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR1`"]
        pub struct OSPEEDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR2`"]
        pub type OSPEEDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR2`"]
        pub struct OSPEEDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR3`"]
        pub type OSPEEDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR3`"]
        pub struct OSPEEDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR4`"]
        pub type OSPEEDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR4`"]
        pub struct OSPEEDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR5`"]
        pub type OSPEEDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR5`"]
        pub struct OSPEEDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR6`"]
        pub type OSPEEDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR6`"]
        pub struct OSPEEDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR7`"]
        pub type OSPEEDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR7`"]
        pub struct OSPEEDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR8`"]
        pub type OSPEEDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR8`"]
        pub struct OSPEEDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR9`"]
        pub type OSPEEDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR9`"]
        pub struct OSPEEDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR10`"]
        pub type OSPEEDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR10`"]
        pub struct OSPEEDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR11`"]
        pub type OSPEEDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR11`"]
        pub struct OSPEEDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR12`"]
        pub type OSPEEDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR12`"]
        pub struct OSPEEDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR13`"]
        pub type OSPEEDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR13`"]
        pub struct OSPEEDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR14`"]
        pub type OSPEEDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR14`"]
        pub struct OSPEEDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR15`"]
        pub type OSPEEDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR15`"]
        pub struct OSPEEDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&self) -> OSPEEDR0_R {
                OSPEEDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&self) -> OSPEEDR1_R {
                OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&self) -> OSPEEDR2_R {
                OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&self) -> OSPEEDR3_R {
                OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&self) -> OSPEEDR4_R {
                OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&self) -> OSPEEDR5_R {
                OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&self) -> OSPEEDR6_R {
                OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&self) -> OSPEEDR7_R {
                OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&self) -> OSPEEDR8_R {
                OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&self) -> OSPEEDR9_R {
                OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&self) -> OSPEEDR10_R {
                OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&self) -> OSPEEDR11_R {
                OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&self) -> OSPEEDR12_R {
                OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&self) -> OSPEEDR13_R {
                OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&self) -> OSPEEDR14_R {
                OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&self) -> OSPEEDR15_R {
                OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
                OSPEEDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
                OSPEEDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
                OSPEEDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
                OSPEEDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
                OSPEEDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
                OSPEEDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
                OSPEEDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
                OSPEEDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
                OSPEEDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
                OSPEEDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
                OSPEEDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
                OSPEEDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
                OSPEEDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
                OSPEEDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
                OSPEEDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
                OSPEEDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](pupdr) module"]
    pub type PUPDR = crate::Reg<u32, _PUPDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _PUPDR;
    #[doc = "`read()` method returns [pupdr::R](pupdr::R) reader structure"]
    impl crate::Readable for PUPDR {}
    #[doc = "`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure"]
    impl crate::Writable for PUPDR {}
    #[doc = "GPIO port pull-up/pull-down register"]
    pub mod pupdr {
        #[doc = "Reader of register PUPDR"]
        pub type R = crate::R<u32, super::PUPDR>;
        #[doc = "Writer for register PUPDR"]
        pub type W = crate::W<u32, super::PUPDR>;
        #[doc = "Register PUPDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::PUPDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `PUPDR0`"]
        pub type PUPDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR0`"]
        pub struct PUPDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR1`"]
        pub type PUPDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR1`"]
        pub struct PUPDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR2`"]
        pub type PUPDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR2`"]
        pub struct PUPDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR3`"]
        pub type PUPDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR3`"]
        pub struct PUPDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR4`"]
        pub type PUPDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR4`"]
        pub struct PUPDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR5`"]
        pub type PUPDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR5`"]
        pub struct PUPDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR6`"]
        pub type PUPDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR6`"]
        pub struct PUPDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR7`"]
        pub type PUPDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR7`"]
        pub struct PUPDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR8`"]
        pub type PUPDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR8`"]
        pub struct PUPDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR9`"]
        pub type PUPDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR9`"]
        pub struct PUPDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR10`"]
        pub type PUPDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR10`"]
        pub struct PUPDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR11`"]
        pub type PUPDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR11`"]
        pub struct PUPDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR12`"]
        pub type PUPDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR12`"]
        pub struct PUPDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR13`"]
        pub type PUPDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR13`"]
        pub struct PUPDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR14`"]
        pub type PUPDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR14`"]
        pub struct PUPDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR15`"]
        pub type PUPDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR15`"]
        pub struct PUPDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&self) -> PUPDR0_R {
                PUPDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&self) -> PUPDR1_R {
                PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&self) -> PUPDR2_R {
                PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&self) -> PUPDR3_R {
                PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&self) -> PUPDR4_R {
                PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&self) -> PUPDR5_R {
                PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&self) -> PUPDR6_R {
                PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&self) -> PUPDR7_R {
                PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&self) -> PUPDR8_R {
                PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&self) -> PUPDR9_R {
                PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&self) -> PUPDR10_R {
                PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&self) -> PUPDR11_R {
                PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&self) -> PUPDR12_R {
                PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&self) -> PUPDR13_R {
                PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&self) -> PUPDR14_R {
                PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&self) -> PUPDR15_R {
                PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&mut self) -> PUPDR0_W {
                PUPDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&mut self) -> PUPDR1_W {
                PUPDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&mut self) -> PUPDR2_W {
                PUPDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&mut self) -> PUPDR3_W {
                PUPDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&mut self) -> PUPDR4_W {
                PUPDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&mut self) -> PUPDR5_W {
                PUPDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&mut self) -> PUPDR6_W {
                PUPDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&mut self) -> PUPDR7_W {
                PUPDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&mut self) -> PUPDR8_W {
                PUPDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&mut self) -> PUPDR9_W {
                PUPDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&mut self) -> PUPDR10_W {
                PUPDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&mut self) -> PUPDR11_W {
                PUPDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&mut self) -> PUPDR12_W {
                PUPDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&mut self) -> PUPDR13_W {
                PUPDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&mut self) -> PUPDR14_W {
                PUPDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&mut self) -> PUPDR15_W {
                PUPDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
    pub type IDR = crate::Reg<u32, _IDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _IDR;
    #[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
    impl crate::Readable for IDR {}
    #[doc = "GPIO port input data register"]
    pub mod idr {
        #[doc = "Reader of register IDR"]
        pub type R = crate::R<u32, super::IDR>;
        #[doc = "Reader of field `IDR0`"]
        pub type IDR0_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR1`"]
        pub type IDR1_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR2`"]
        pub type IDR2_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR3`"]
        pub type IDR3_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR4`"]
        pub type IDR4_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR5`"]
        pub type IDR5_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR6`"]
        pub type IDR6_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR7`"]
        pub type IDR7_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR8`"]
        pub type IDR8_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR9`"]
        pub type IDR9_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR10`"]
        pub type IDR10_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR11`"]
        pub type IDR11_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR12`"]
        pub type IDR12_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR13`"]
        pub type IDR13_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR14`"]
        pub type IDR14_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR15`"]
        pub type IDR15_R = crate::R<bool, bool>;
        impl R {
            #[doc = "Bit 0 - Pin 0 input data bit"]
            #[inline(always)]
            pub fn idr0(&self) -> IDR0_R {
                IDR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 input data bit"]
            #[inline(always)]
            pub fn idr1(&self) -> IDR1_R {
                IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 input data bit"]
            #[inline(always)]
            pub fn idr2(&self) -> IDR2_R {
                IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 input data bit"]
            #[inline(always)]
            pub fn idr3(&self) -> IDR3_R {
                IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 input data bit"]
            #[inline(always)]
            pub fn idr4(&self) -> IDR4_R {
                IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 input data bit"]
            #[inline(always)]
            pub fn idr5(&self) -> IDR5_R {
                IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 input data bit"]
            #[inline(always)]
            pub fn idr6(&self) -> IDR6_R {
                IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 input data bit"]
            #[inline(always)]
            pub fn idr7(&self) -> IDR7_R {
                IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 input data bit"]
            #[inline(always)]
            pub fn idr8(&self) -> IDR8_R {
                IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 input data bit"]
            #[inline(always)]
            pub fn idr9(&self) -> IDR9_R {
                IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 input data bit"]
            #[inline(always)]
            pub fn idr10(&self) -> IDR10_R {
                IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 input data bit"]
            #[inline(always)]
            pub fn idr11(&self) -> IDR11_R {
                IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 input data bit"]
            #[inline(always)]
            pub fn idr12(&self) -> IDR12_R {
                IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 input data bit"]
            #[inline(always)]
            pub fn idr13(&self) -> IDR13_R {
                IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 input data bit"]
            #[inline(always)]
            pub fn idr14(&self) -> IDR14_R {
                IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 input data bit"]
            #[inline(always)]
            pub fn idr15(&self) -> IDR15_R {
                IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
    }
    #[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udr](udr) module"]
    pub type UDR = crate::Reg<u32, _UDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _UDR;
    #[doc = "`read()` method returns [udr::R](udr::R) reader structure"]
    impl crate::Readable for UDR {}
    #[doc = "`write(|w| ..)` method takes [udr::W](udr::W) writer structure"]
    impl crate::Writable for UDR {}
    #[doc = "GPIO port output data register"]
    pub mod udr {
        #[doc = "Reader of register UDR"]
        pub type R = crate::R<u32, super::UDR>;
        #[doc = "Writer for register UDR"]
        pub type W = crate::W<u32, super::UDR>;
        #[doc = "Register UDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::UDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `ODR0`"]
        pub type ODR0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR0`"]
        pub struct ODR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR0_W<'a> {
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
        #[doc = "Reader of field `ODR1`"]
        pub type ODR1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR1`"]
        pub struct ODR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR1_W<'a> {
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
        #[doc = "Reader of field `ODR2`"]
        pub type ODR2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR2`"]
        pub struct ODR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR2_W<'a> {
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
        #[doc = "Reader of field `ODR3`"]
        pub type ODR3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR3`"]
        pub struct ODR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR3_W<'a> {
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
        #[doc = "Reader of field `ODR4`"]
        pub type ODR4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR4`"]
        pub struct ODR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `ODR5`"]
        pub type ODR5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR5`"]
        pub struct ODR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `ODR6`"]
        pub type ODR6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR6`"]
        pub struct ODR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `ODR7`"]
        pub type ODR7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR7`"]
        pub struct ODR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `ODR8`"]
        pub type ODR8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR8`"]
        pub struct ODR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `ODR9`"]
        pub type ODR9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR9`"]
        pub struct ODR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `ODR10`"]
        pub type ODR10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR10`"]
        pub struct ODR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `ODR11`"]
        pub type ODR11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR11`"]
        pub struct ODR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `ODR12`"]
        pub type ODR12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR12`"]
        pub struct ODR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `ODR13`"]
        pub type ODR13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR13`"]
        pub struct ODR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `ODR14`"]
        pub type ODR14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR14`"]
        pub struct ODR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `ODR15`"]
        pub type ODR15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR15`"]
        pub struct ODR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&self) -> ODR0_R {
                ODR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&self) -> ODR1_R {
                ODR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&self) -> ODR2_R {
                ODR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&self) -> ODR3_R {
                ODR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&self) -> ODR4_R {
                ODR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&self) -> ODR5_R {
                ODR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&self) -> ODR6_R {
                ODR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&self) -> ODR7_R {
                ODR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&self) -> ODR8_R {
                ODR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&self) -> ODR9_R {
                ODR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&self) -> ODR10_R {
                ODR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&self) -> ODR11_R {
                ODR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&self) -> ODR12_R {
                ODR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&self) -> ODR13_R {
                ODR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&self) -> ODR14_R {
                ODR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&self) -> ODR15_R {
                ODR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&mut self) -> ODR0_W {
                ODR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&mut self) -> ODR1_W {
                ODR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&mut self) -> ODR2_W {
                ODR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&mut self) -> ODR3_W {
                ODR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&mut self) -> ODR4_W {
                ODR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&mut self) -> ODR5_W {
                ODR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&mut self) -> ODR6_W {
                ODR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&mut self) -> ODR7_W {
                ODR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&mut self) -> ODR8_W {
                ODR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&mut self) -> ODR9_W {
                ODR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&mut self) -> ODR10_W {
                ODR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&mut self) -> ODR11_W {
                ODR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&mut self) -> ODR12_W {
                ODR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&mut self) -> ODR13_W {
                ODR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&mut self) -> ODR14_W {
                ODR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&mut self) -> ODR15_W {
                ODR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
    pub type BSRR = crate::Reg<u32, _BSRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BSRR;
    #[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
    impl crate::Writable for BSRR {}
    #[doc = "GPIO port bit set/reset register"]
    pub mod bsrr {
        #[doc = "Writer for register BSRR"]
        pub type W = crate::W<u32, super::BSRR>;
        #[doc = "Register BSRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BSRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BS0`"]
        pub struct BS0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS0_W<'a> {
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
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BS1`"]
        pub struct BS1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS1_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BS2`"]
        pub struct BS2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS2_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS3`"]
        pub struct BS3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS3_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS4`"]
        pub struct BS4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS5`"]
        pub struct BS5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS6`"]
        pub struct BS6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS7`"]
        pub struct BS7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
        #[doc = "Write proxy for field `BS8`"]
        pub struct BS8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
        #[doc = "Write proxy for field `BS9`"]
        pub struct BS9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS10`"]
        pub struct BS10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
        #[doc = "Write proxy for field `BS11`"]
        pub struct BS11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS12`"]
        pub struct BS12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS13`"]
        pub struct BS13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
        #[doc = "Write proxy for field `BS14`"]
        pub struct BS14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
        #[doc = "Write proxy for field `BS15`"]
        pub struct BS15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
        impl W {
            #[doc = "Bit 0 - Pin 0 set bit"]
            #[inline(always)]
            pub fn bs0(&mut self) -> BS0_W {
                BS0_W { w: self }
            }
            #[doc = "Bit 16 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 set bit"]
            #[inline(always)]
            pub fn bs1(&mut self) -> BS1_W {
                BS1_W { w: self }
            }
            #[doc = "Bit 17 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 set bit"]
            #[inline(always)]
            pub fn bs2(&mut self) -> BS2_W {
                BS2_W { w: self }
            }
            #[doc = "Bit 18 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 set bit"]
            #[inline(always)]
            pub fn bs3(&mut self) -> BS3_W {
                BS3_W { w: self }
            }
            #[doc = "Bit 19 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 set bit"]
            #[inline(always)]
            pub fn bs4(&mut self) -> BS4_W {
                BS4_W { w: self }
            }
            #[doc = "Bit 20 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 set bit"]
            #[inline(always)]
            pub fn bs5(&mut self) -> BS5_W {
                BS5_W { w: self }
            }
            #[doc = "Bit 21 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 set bit"]
            #[inline(always)]
            pub fn bs6(&mut self) -> BS6_W {
                BS6_W { w: self }
            }
            #[doc = "Bit 22 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 set bit"]
            #[inline(always)]
            pub fn bs7(&mut self) -> BS7_W {
                BS7_W { w: self }
            }
            #[doc = "Bit 23 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 set bit"]
            #[inline(always)]
            pub fn bs8(&mut self) -> BS8_W {
                BS8_W { w: self }
            }
            #[doc = "Bit 24 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 set bit"]
            #[inline(always)]
            pub fn bs9(&mut self) -> BS9_W {
                BS9_W { w: self }
            }
            #[doc = "Bit 25 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 set bit"]
            #[inline(always)]
            pub fn bs10(&mut self) -> BS10_W {
                BS10_W { w: self }
            }
            #[doc = "Bit 26 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 set bit"]
            #[inline(always)]
            pub fn bs11(&mut self) -> BS11_W {
                BS11_W { w: self }
            }
            #[doc = "Bit 27 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 set bit"]
            #[inline(always)]
            pub fn bs12(&mut self) -> BS12_W {
                BS12_W { w: self }
            }
            #[doc = "Bit 28 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 set bit"]
            #[inline(always)]
            pub fn bs13(&mut self) -> BS13_W {
                BS13_W { w: self }
            }
            #[doc = "Bit 29 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 set bit"]
            #[inline(always)]
            pub fn bs14(&mut self) -> BS14_W {
                BS14_W { w: self }
            }
            #[doc = "Bit 30 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 set bit"]
            #[inline(always)]
            pub fn bs15(&mut self) -> BS15_W {
                BS15_W { w: self }
            }
            #[doc = "Bit 31 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
    pub type LCKR = crate::Reg<u32, _LCKR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _LCKR;
    #[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
    impl crate::Readable for LCKR {}
    #[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
    impl crate::Writable for LCKR {}
    #[doc = "GPIO port configuration lock register"]
    pub mod lckr {
        #[doc = "Reader of register LCKR"]
        pub type R = crate::R<u32, super::LCKR>;
        #[doc = "Writer for register LCKR"]
        pub type W = crate::W<u32, super::LCKR>;
        #[doc = "Register LCKR `reset()`'s with value 0"]
        impl crate::ResetValue for super::LCKR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `LCK0`"]
        pub type LCK0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK0`"]
        pub struct LCK0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK0_W<'a> {
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
        #[doc = "Reader of field `LCK1`"]
        pub type LCK1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK1`"]
        pub struct LCK1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK1_W<'a> {
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
        #[doc = "Reader of field `LCK2`"]
        pub type LCK2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK2`"]
        pub struct LCK2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK2_W<'a> {
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
        #[doc = "Reader of field `LCK3`"]
        pub type LCK3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK3`"]
        pub struct LCK3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK3_W<'a> {
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
        #[doc = "Reader of field `LCK4`"]
        pub type LCK4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK4`"]
        pub struct LCK4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `LCK5`"]
        pub type LCK5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK5`"]
        pub struct LCK5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `LCK6`"]
        pub type LCK6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK6`"]
        pub struct LCK6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `LCK7`"]
        pub type LCK7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK7`"]
        pub struct LCK7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `LCK8`"]
        pub type LCK8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK8`"]
        pub struct LCK8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `LCK9`"]
        pub type LCK9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK9`"]
        pub struct LCK9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `LCK10`"]
        pub type LCK10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK10`"]
        pub struct LCK10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `LCK11`"]
        pub type LCK11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK11`"]
        pub struct LCK11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `LCK12`"]
        pub type LCK12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK12`"]
        pub struct LCK12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `LCK13`"]
        pub type LCK13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK13`"]
        pub struct LCK13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `LCK14`"]
        pub type LCK14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK14`"]
        pub struct LCK14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `LCK15`"]
        pub type LCK15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK15`"]
        pub struct LCK15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Reader of field `LCKK`"]
        pub type LCKK_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCKK`"]
        pub struct LCKK_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCKK_W<'a> {
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
        impl R {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&self) -> LCK0_R {
                LCK0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&self) -> LCK1_R {
                LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&self) -> LCK2_R {
                LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&self) -> LCK3_R {
                LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&self) -> LCK4_R {
                LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&self) -> LCK5_R {
                LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&self) -> LCK6_R {
                LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&self) -> LCK7_R {
                LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&self) -> LCK8_R {
                LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&self) -> LCK9_R {
                LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&self) -> LCK10_R {
                LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&self) -> LCK11_R {
                LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&self) -> LCK12_R {
                LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&self) -> LCK13_R {
                LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&self) -> LCK14_R {
                LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&self) -> LCK15_R {
                LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&self) -> LCKK_R {
                LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&mut self) -> LCK0_W {
                LCK0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&mut self) -> LCK1_W {
                LCK1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&mut self) -> LCK2_W {
                LCK2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&mut self) -> LCK3_W {
                LCK3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&mut self) -> LCK4_W {
                LCK4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&mut self) -> LCK5_W {
                LCK5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&mut self) -> LCK6_W {
                LCK6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&mut self) -> LCK7_W {
                LCK7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&mut self) -> LCK8_W {
                LCK8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&mut self) -> LCK9_W {
                LCK9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&mut self) -> LCK10_W {
                LCK10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&mut self) -> LCK11_W {
                LCK11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&mut self) -> LCK12_W {
                LCK12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&mut self) -> LCK13_W {
                LCK13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&mut self) -> LCK14_W {
                LCK14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&mut self) -> LCK15_W {
                LCK15_W { w: self }
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&mut self) -> LCKK_W {
                LCKK_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](afrl) module"]
    pub type AFRL = crate::Reg<u32, _AFRL>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRL;
    #[doc = "`read()` method returns [afrl::R](afrl::R) reader structure"]
    impl crate::Readable for AFRL {}
    #[doc = "`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure"]
    impl crate::Writable for AFRL {}
    #[doc = "GPIO alternate function low register"]
    pub mod afrl {
        #[doc = "Reader of register AFRL"]
        pub type R = crate::R<u32, super::AFRL>;
        #[doc = "Writer for register AFRL"]
        pub type W = crate::W<u32, super::AFRL>;
        #[doc = "Register AFRL `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRL {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR0`"]
        pub type AFR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR0`"]
        pub struct AFR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
                self.w
            }
        }
        #[doc = "Reader of field `AFR1`"]
        pub type AFR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR1`"]
        pub struct AFR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `AFR2`"]
        pub type AFR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR2`"]
        pub struct AFR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `AFR3`"]
        pub type AFR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR3`"]
        pub struct AFR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `AFR4`"]
        pub type AFR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR4`"]
        pub struct AFR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `AFR5`"]
        pub type AFR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR5`"]
        pub struct AFR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `AFR6`"]
        pub type AFR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR6`"]
        pub struct AFR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `AFR7`"]
        pub type AFR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR7`"]
        pub struct AFR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&self) -> AFR0_R {
                AFR0_R::new((self.bits & 0x0f) as u8)
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&self) -> AFR1_R {
                AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&self) -> AFR2_R {
                AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&self) -> AFR3_R {
                AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&self) -> AFR4_R {
                AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&self) -> AFR5_R {
                AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&self) -> AFR6_R {
                AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&self) -> AFR7_R {
                AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&mut self) -> AFR0_W {
                AFR0_W { w: self }
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&mut self) -> AFR1_W {
                AFR1_W { w: self }
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&mut self) -> AFR2_W {
                AFR2_W { w: self }
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&mut self) -> AFR3_W {
                AFR3_W { w: self }
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&mut self) -> AFR4_W {
                AFR4_W { w: self }
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&mut self) -> AFR5_W {
                AFR5_W { w: self }
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&mut self) -> AFR6_W {
                AFR6_W { w: self }
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&mut self) -> AFR7_W {
                AFR7_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](afrh) module"]
    pub type AFRH = crate::Reg<u32, _AFRH>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRH;
    #[doc = "`read()` method returns [afrh::R](afrh::R) reader structure"]
    impl crate::Readable for AFRH {}
    #[doc = "`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure"]
    impl crate::Writable for AFRH {}
    #[doc = "GPIO alternate function high register"]
    pub mod afrh {
        #[doc = "Reader of register AFRH"]
        pub type R = crate::R<u32, super::AFRH>;
        #[doc = "Writer for register AFRH"]
        pub type W = crate::W<u32, super::AFRH>;
        #[doc = "Register AFRH `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRH {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR8`"]
        pub type AFR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR8`"]
        pub struct AFR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 32)) | (((value as u32) & 0x0f) << 32);
                self.w
            }
        }
        #[doc = "Reader of field `AFR9`"]
        pub type AFR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR9`"]
        pub struct AFR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 36)) | (((value as u32) & 0x0f) << 36);
                self.w
            }
        }
        #[doc = "Reader of field `AFR10`"]
        pub type AFR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR10`"]
        pub struct AFR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 40)) | (((value as u32) & 0x0f) << 40);
                self.w
            }
        }
        #[doc = "Reader of field `AFR11`"]
        pub type AFR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR11`"]
        pub struct AFR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 44)) | (((value as u32) & 0x0f) << 44);
                self.w
            }
        }
        #[doc = "Reader of field `AFR12`"]
        pub type AFR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR12`"]
        pub struct AFR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 48)) | (((value as u32) & 0x0f) << 48);
                self.w
            }
        }
        #[doc = "Reader of field `AFR13`"]
        pub type AFR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR13`"]
        pub struct AFR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 52)) | (((value as u32) & 0x0f) << 52);
                self.w
            }
        }
        #[doc = "Reader of field `AFR14`"]
        pub type AFR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR14`"]
        pub struct AFR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 56)) | (((value as u32) & 0x0f) << 56);
                self.w
            }
        }
        #[doc = "Reader of field `AFR15`"]
        pub type AFR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR15`"]
        pub struct AFR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 60)) | (((value as u32) & 0x0f) << 60);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&self) -> AFR8_R {
                AFR8_R::new(((self.bits >> 32) & 0x0f) as u8)
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&self) -> AFR9_R {
                AFR9_R::new(((self.bits >> 36) & 0x0f) as u8)
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&self) -> AFR10_R {
                AFR10_R::new(((self.bits >> 40) & 0x0f) as u8)
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&self) -> AFR11_R {
                AFR11_R::new(((self.bits >> 44) & 0x0f) as u8)
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&self) -> AFR12_R {
                AFR12_R::new(((self.bits >> 48) & 0x0f) as u8)
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&self) -> AFR13_R {
                AFR13_R::new(((self.bits >> 52) & 0x0f) as u8)
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&self) -> AFR14_R {
                AFR14_R::new(((self.bits >> 56) & 0x0f) as u8)
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&self) -> AFR15_R {
                AFR15_R::new(((self.bits >> 60) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&mut self) -> AFR8_W {
                AFR8_W { w: self }
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&mut self) -> AFR9_W {
                AFR9_W { w: self }
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&mut self) -> AFR10_W {
                AFR10_W { w: self }
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&mut self) -> AFR11_W {
                AFR11_W { w: self }
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&mut self) -> AFR12_W {
                AFR12_W { w: self }
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&mut self) -> AFR13_W {
                AFR13_W { w: self }
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&mut self) -> AFR14_W {
                AFR14_W { w: self }
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&mut self) -> AFR15_W {
                AFR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
    pub type BRR = crate::Reg<u32, _BRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BRR;
    #[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
    impl crate::Writable for BRR {}
    #[doc = "GPIO port bit reset register"]
    pub mod brr {
        #[doc = "Writer for register BRR"]
        pub type W = crate::W<u32, super::BRR>;
        #[doc = "Register BRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
}
#[doc = "General-purpose I/Os, port F"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiof::RegisterBlock {
        0x4800_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os, port F"]
pub mod gpiof {
    #[doc = r"Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - GPIO port mode register"]
        pub moder: MODER,
        #[doc = "0x04 - GPIO port output type register"]
        pub otyper: OTYPER,
        #[doc = "0x08 - GPIO port output speed register"]
        pub ospeedr: OSPEEDR,
        #[doc = "0x0c - GPIO port pull-up/pull-down register"]
        pub pupdr: PUPDR,
        #[doc = "0x10 - GPIO port input data register"]
        pub idr: IDR,
        #[doc = "0x14 - GPIO port output data register"]
        pub udr: UDR,
        #[doc = "0x18 - GPIO port bit set/reset register"]
        pub bsrr: BSRR,
        #[doc = "0x1c - GPIO port configuration lock register"]
        pub lckr: LCKR,
        #[doc = "0x20 - GPIO alternate function low register"]
        pub afrl: AFRL,
        #[doc = "0x24 - GPIO alternate function high register"]
        pub afrh: AFRH,
        #[doc = "0x28 - GPIO port bit reset register"]
        pub brr: BRR,
    }
    #[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](moder) module"]
    pub type MODER = crate::Reg<u32, _MODER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _MODER;
    #[doc = "`read()` method returns [moder::R](moder::R) reader structure"]
    impl crate::Readable for MODER {}
    #[doc = "`write(|w| ..)` method takes [moder::W](moder::W) writer structure"]
    impl crate::Writable for MODER {}
    #[doc = "GPIO port mode register"]
    pub mod moder {
        #[doc = "Reader of register MODER"]
        pub type R = crate::R<u32, super::MODER>;
        #[doc = "Writer for register MODER"]
        pub type W = crate::W<u32, super::MODER>;
        #[doc = "Register MODER `reset()`'s with value 0"]
        impl crate::ResetValue for super::MODER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `MODER0`"]
        pub type MODER0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER0`"]
        pub struct MODER0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `MODER1`"]
        pub type MODER1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER1`"]
        pub struct MODER1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `MODER2`"]
        pub type MODER2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER2`"]
        pub struct MODER2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `MODER3`"]
        pub type MODER3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER3`"]
        pub struct MODER3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `MODER4`"]
        pub type MODER4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER4`"]
        pub struct MODER4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `MODER5`"]
        pub type MODER5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER5`"]
        pub struct MODER5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `MODER6`"]
        pub type MODER6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER6`"]
        pub struct MODER6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `MODER7`"]
        pub type MODER7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER7`"]
        pub struct MODER7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `MODER8`"]
        pub type MODER8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER8`"]
        pub struct MODER8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `MODER9`"]
        pub type MODER9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER9`"]
        pub struct MODER9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `MODER10`"]
        pub type MODER10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER10`"]
        pub struct MODER10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `MODER11`"]
        pub type MODER11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER11`"]
        pub struct MODER11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `MODER12`"]
        pub type MODER12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER12`"]
        pub struct MODER12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `MODER13`"]
        pub type MODER13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER13`"]
        pub struct MODER13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `MODER14`"]
        pub type MODER14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER14`"]
        pub struct MODER14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `MODER15`"]
        pub type MODER15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `MODER15`"]
        pub struct MODER15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> MODER15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&self) -> MODER0_R {
                MODER0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&self) -> MODER1_R {
                MODER1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&self) -> MODER2_R {
                MODER2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&self) -> MODER3_R {
                MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&self) -> MODER4_R {
                MODER4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&self) -> MODER5_R {
                MODER5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&self) -> MODER6_R {
                MODER6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&self) -> MODER7_R {
                MODER7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&self) -> MODER8_R {
                MODER8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&self) -> MODER9_R {
                MODER9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&self) -> MODER10_R {
                MODER10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&self) -> MODER11_R {
                MODER11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&self) -> MODER12_R {
                MODER12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&self) -> MODER13_R {
                MODER13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&self) -> MODER14_R {
                MODER14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&self) -> MODER15_R {
                MODER15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder0(&mut self) -> MODER0_W {
                MODER0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder1(&mut self) -> MODER1_W {
                MODER1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder2(&mut self) -> MODER2_W {
                MODER2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder3(&mut self) -> MODER3_W {
                MODER3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder4(&mut self) -> MODER4_W {
                MODER4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder5(&mut self) -> MODER5_W {
                MODER5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder6(&mut self) -> MODER6_W {
                MODER6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder7(&mut self) -> MODER7_W {
                MODER7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder8(&mut self) -> MODER8_W {
                MODER8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder9(&mut self) -> MODER9_W {
                MODER9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder10(&mut self) -> MODER10_W {
                MODER10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder11(&mut self) -> MODER11_W {
                MODER11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder12(&mut self) -> MODER12_W {
                MODER12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder13(&mut self) -> MODER13_W {
                MODER13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder14(&mut self) -> MODER14_W {
                MODER14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O mode configuration bits"]
            #[inline(always)]
            pub fn moder15(&mut self) -> MODER15_W {
                MODER15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](otyper) module"]
    pub type OTYPER = crate::Reg<u32, _OTYPER>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OTYPER;
    #[doc = "`read()` method returns [otyper::R](otyper::R) reader structure"]
    impl crate::Readable for OTYPER {}
    #[doc = "`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure"]
    impl crate::Writable for OTYPER {}
    #[doc = "GPIO port output type register"]
    pub mod otyper {
        #[doc = "Reader of register OTYPER"]
        pub type R = crate::R<u32, super::OTYPER>;
        #[doc = "Writer for register OTYPER"]
        pub type W = crate::W<u32, super::OTYPER>;
        #[doc = "Register OTYPER `reset()`'s with value 0"]
        impl crate::ResetValue for super::OTYPER {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OT0`"]
        pub type OT0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT0`"]
        pub struct OT0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT0_W<'a> {
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
        #[doc = "Reader of field `OT1`"]
        pub type OT1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT1`"]
        pub struct OT1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT1_W<'a> {
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
        #[doc = "Reader of field `OT2`"]
        pub type OT2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT2`"]
        pub struct OT2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT2_W<'a> {
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
        #[doc = "Reader of field `OT3`"]
        pub type OT3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT3`"]
        pub struct OT3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT3_W<'a> {
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
        #[doc = "Reader of field `OT4`"]
        pub type OT4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT4`"]
        pub struct OT4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OT5`"]
        pub type OT5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT5`"]
        pub struct OT5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `OT6`"]
        pub type OT6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT6`"]
        pub struct OT6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OT7`"]
        pub type OT7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT7`"]
        pub struct OT7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `OT8`"]
        pub type OT8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT8`"]
        pub struct OT8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OT9`"]
        pub type OT9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT9`"]
        pub struct OT9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `OT10`"]
        pub type OT10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT10`"]
        pub struct OT10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OT11`"]
        pub type OT11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT11`"]
        pub struct OT11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `OT12`"]
        pub type OT12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT12`"]
        pub struct OT12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OT13`"]
        pub type OT13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT13`"]
        pub struct OT13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `OT14`"]
        pub type OT14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT14`"]
        pub struct OT14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OT15`"]
        pub type OT15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `OT15`"]
        pub struct OT15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OT15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&self) -> OT0_R {
                OT0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&self) -> OT1_R {
                OT1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&self) -> OT2_R {
                OT2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&self) -> OT3_R {
                OT3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&self) -> OT4_R {
                OT4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&self) -> OT5_R {
                OT5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&self) -> OT6_R {
                OT6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&self) -> OT7_R {
                OT7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&self) -> OT8_R {
                OT8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&self) -> OT9_R {
                OT9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&self) -> OT10_R {
                OT10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&self) -> OT11_R {
                OT11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&self) -> OT12_R {
                OT12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&self) -> OT13_R {
                OT13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&self) -> OT14_R {
                OT14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&self) -> OT15_R {
                OT15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot0(&mut self) -> OT0_W {
                OT0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot1(&mut self) -> OT1_W {
                OT1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot2(&mut self) -> OT2_W {
                OT2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot3(&mut self) -> OT3_W {
                OT3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot4(&mut self) -> OT4_W {
                OT4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot5(&mut self) -> OT5_W {
                OT5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot6(&mut self) -> OT6_W {
                OT6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot7(&mut self) -> OT7_W {
                OT7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot8(&mut self) -> OT8_W {
                OT8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot9(&mut self) -> OT9_W {
                OT9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot10(&mut self) -> OT10_W {
                OT10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot11(&mut self) -> OT11_W {
                OT11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot12(&mut self) -> OT12_W {
                OT12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot13(&mut self) -> OT13_W {
                OT13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot14(&mut self) -> OT14_W {
                OT14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 I/O output type configuration bit"]
            #[inline(always)]
            pub fn ot15(&mut self) -> OT15_W {
                OT15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](ospeedr) module"]
    pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _OSPEEDR;
    #[doc = "`read()` method returns [ospeedr::R](ospeedr::R) reader structure"]
    impl crate::Readable for OSPEEDR {}
    #[doc = "`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure"]
    impl crate::Writable for OSPEEDR {}
    #[doc = "GPIO port output speed register"]
    pub mod ospeedr {
        #[doc = "Reader of register OSPEEDR"]
        pub type R = crate::R<u32, super::OSPEEDR>;
        #[doc = "Writer for register OSPEEDR"]
        pub type W = crate::W<u32, super::OSPEEDR>;
        #[doc = "Register OSPEEDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::OSPEEDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `OSPEEDR0`"]
        pub type OSPEEDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR0`"]
        pub struct OSPEEDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR1`"]
        pub type OSPEEDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR1`"]
        pub struct OSPEEDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR2`"]
        pub type OSPEEDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR2`"]
        pub struct OSPEEDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR3`"]
        pub type OSPEEDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR3`"]
        pub struct OSPEEDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR4`"]
        pub type OSPEEDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR4`"]
        pub struct OSPEEDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR5`"]
        pub type OSPEEDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR5`"]
        pub struct OSPEEDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR6`"]
        pub type OSPEEDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR6`"]
        pub struct OSPEEDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR7`"]
        pub type OSPEEDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR7`"]
        pub struct OSPEEDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR8`"]
        pub type OSPEEDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR8`"]
        pub struct OSPEEDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR9`"]
        pub type OSPEEDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR9`"]
        pub struct OSPEEDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR10`"]
        pub type OSPEEDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR10`"]
        pub struct OSPEEDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR11`"]
        pub type OSPEEDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR11`"]
        pub struct OSPEEDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR12`"]
        pub type OSPEEDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR12`"]
        pub struct OSPEEDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR13`"]
        pub type OSPEEDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR13`"]
        pub struct OSPEEDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR14`"]
        pub type OSPEEDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR14`"]
        pub struct OSPEEDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `OSPEEDR15`"]
        pub type OSPEEDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `OSPEEDR15`"]
        pub struct OSPEEDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> OSPEEDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&self) -> OSPEEDR0_R {
                OSPEEDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&self) -> OSPEEDR1_R {
                OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&self) -> OSPEEDR2_R {
                OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&self) -> OSPEEDR3_R {
                OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&self) -> OSPEEDR4_R {
                OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&self) -> OSPEEDR5_R {
                OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&self) -> OSPEEDR6_R {
                OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&self) -> OSPEEDR7_R {
                OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&self) -> OSPEEDR8_R {
                OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&self) -> OSPEEDR9_R {
                OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&self) -> OSPEEDR10_R {
                OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&self) -> OSPEEDR11_R {
                OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&self) -> OSPEEDR12_R {
                OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&self) -> OSPEEDR13_R {
                OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&self) -> OSPEEDR14_R {
                OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&self) -> OSPEEDR15_R {
                OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
                OSPEEDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
                OSPEEDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
                OSPEEDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
                OSPEEDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
                OSPEEDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
                OSPEEDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
                OSPEEDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
                OSPEEDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
                OSPEEDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
                OSPEEDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
                OSPEEDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
                OSPEEDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
                OSPEEDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
                OSPEEDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
                OSPEEDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O output speed configuration bits"]
            #[inline(always)]
            pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
                OSPEEDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](pupdr) module"]
    pub type PUPDR = crate::Reg<u32, _PUPDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _PUPDR;
    #[doc = "`read()` method returns [pupdr::R](pupdr::R) reader structure"]
    impl crate::Readable for PUPDR {}
    #[doc = "`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure"]
    impl crate::Writable for PUPDR {}
    #[doc = "GPIO port pull-up/pull-down register"]
    pub mod pupdr {
        #[doc = "Reader of register PUPDR"]
        pub type R = crate::R<u32, super::PUPDR>;
        #[doc = "Writer for register PUPDR"]
        pub type W = crate::W<u32, super::PUPDR>;
        #[doc = "Register PUPDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::PUPDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `PUPDR0`"]
        pub type PUPDR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR0`"]
        pub struct PUPDR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR1`"]
        pub type PUPDR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR1`"]
        pub struct PUPDR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR2`"]
        pub type PUPDR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR2`"]
        pub struct PUPDR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR3`"]
        pub type PUPDR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR3`"]
        pub struct PUPDR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR4`"]
        pub type PUPDR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR4`"]
        pub struct PUPDR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR5`"]
        pub type PUPDR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR5`"]
        pub struct PUPDR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR6`"]
        pub type PUPDR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR6`"]
        pub struct PUPDR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR7`"]
        pub type PUPDR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR7`"]
        pub struct PUPDR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR8`"]
        pub type PUPDR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR8`"]
        pub struct PUPDR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR9`"]
        pub type PUPDR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR9`"]
        pub struct PUPDR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR10`"]
        pub type PUPDR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR10`"]
        pub struct PUPDR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR11`"]
        pub type PUPDR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR11`"]
        pub struct PUPDR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR12`"]
        pub type PUPDR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR12`"]
        pub struct PUPDR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR13`"]
        pub type PUPDR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR13`"]
        pub struct PUPDR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR14`"]
        pub type PUPDR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR14`"]
        pub struct PUPDR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
                self.w
            }
        }
        #[doc = "Reader of field `PUPDR15`"]
        pub type PUPDR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `PUPDR15`"]
        pub struct PUPDR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> PUPDR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&self) -> PUPDR0_R {
                PUPDR0_R::new((self.bits & 0x03) as u8)
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&self) -> PUPDR1_R {
                PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&self) -> PUPDR2_R {
                PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&self) -> PUPDR3_R {
                PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&self) -> PUPDR4_R {
                PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&self) -> PUPDR5_R {
                PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&self) -> PUPDR6_R {
                PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&self) -> PUPDR7_R {
                PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&self) -> PUPDR8_R {
                PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&self) -> PUPDR9_R {
                PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&self) -> PUPDR10_R {
                PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&self) -> PUPDR11_R {
                PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&self) -> PUPDR12_R {
                PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&self) -> PUPDR13_R {
                PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&self) -> PUPDR14_R {
                PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&self) -> PUPDR15_R {
                PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:1 - Pin 0 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr0(&mut self) -> PUPDR0_W {
                PUPDR0_W { w: self }
            }
            #[doc = "Bits 2:3 - Pin 1 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr1(&mut self) -> PUPDR1_W {
                PUPDR1_W { w: self }
            }
            #[doc = "Bits 4:5 - Pin 2 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr2(&mut self) -> PUPDR2_W {
                PUPDR2_W { w: self }
            }
            #[doc = "Bits 6:7 - Pin 3 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr3(&mut self) -> PUPDR3_W {
                PUPDR3_W { w: self }
            }
            #[doc = "Bits 8:9 - Pin 4 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr4(&mut self) -> PUPDR4_W {
                PUPDR4_W { w: self }
            }
            #[doc = "Bits 10:11 - Pin 5 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr5(&mut self) -> PUPDR5_W {
                PUPDR5_W { w: self }
            }
            #[doc = "Bits 12:13 - Pin 6 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr6(&mut self) -> PUPDR6_W {
                PUPDR6_W { w: self }
            }
            #[doc = "Bits 14:15 - Pin 7 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr7(&mut self) -> PUPDR7_W {
                PUPDR7_W { w: self }
            }
            #[doc = "Bits 16:17 - Pin 8 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr8(&mut self) -> PUPDR8_W {
                PUPDR8_W { w: self }
            }
            #[doc = "Bits 18:19 - Pin 9 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr9(&mut self) -> PUPDR9_W {
                PUPDR9_W { w: self }
            }
            #[doc = "Bits 20:21 - Pin 10 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr10(&mut self) -> PUPDR10_W {
                PUPDR10_W { w: self }
            }
            #[doc = "Bits 22:23 - Pin 11 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr11(&mut self) -> PUPDR11_W {
                PUPDR11_W { w: self }
            }
            #[doc = "Bits 24:25 - Pin 12 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr12(&mut self) -> PUPDR12_W {
                PUPDR12_W { w: self }
            }
            #[doc = "Bits 26:27 - Pin 13 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr13(&mut self) -> PUPDR13_W {
                PUPDR13_W { w: self }
            }
            #[doc = "Bits 28:29 - Pin 14 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr14(&mut self) -> PUPDR14_W {
                PUPDR14_W { w: self }
            }
            #[doc = "Bits 30:31 - Pin 15 I/O pull-up or pull-down configuration bits"]
            #[inline(always)]
            pub fn pupdr15(&mut self) -> PUPDR15_W {
                PUPDR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
    pub type IDR = crate::Reg<u32, _IDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _IDR;
    #[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
    impl crate::Readable for IDR {}
    #[doc = "GPIO port input data register"]
    pub mod idr {
        #[doc = "Reader of register IDR"]
        pub type R = crate::R<u32, super::IDR>;
        #[doc = "Reader of field `IDR0`"]
        pub type IDR0_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR1`"]
        pub type IDR1_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR2`"]
        pub type IDR2_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR3`"]
        pub type IDR3_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR4`"]
        pub type IDR4_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR5`"]
        pub type IDR5_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR6`"]
        pub type IDR6_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR7`"]
        pub type IDR7_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR8`"]
        pub type IDR8_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR9`"]
        pub type IDR9_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR10`"]
        pub type IDR10_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR11`"]
        pub type IDR11_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR12`"]
        pub type IDR12_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR13`"]
        pub type IDR13_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR14`"]
        pub type IDR14_R = crate::R<bool, bool>;
        #[doc = "Reader of field `IDR15`"]
        pub type IDR15_R = crate::R<bool, bool>;
        impl R {
            #[doc = "Bit 0 - Pin 0 input data bit"]
            #[inline(always)]
            pub fn idr0(&self) -> IDR0_R {
                IDR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 input data bit"]
            #[inline(always)]
            pub fn idr1(&self) -> IDR1_R {
                IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 input data bit"]
            #[inline(always)]
            pub fn idr2(&self) -> IDR2_R {
                IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 input data bit"]
            #[inline(always)]
            pub fn idr3(&self) -> IDR3_R {
                IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 input data bit"]
            #[inline(always)]
            pub fn idr4(&self) -> IDR4_R {
                IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 input data bit"]
            #[inline(always)]
            pub fn idr5(&self) -> IDR5_R {
                IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 input data bit"]
            #[inline(always)]
            pub fn idr6(&self) -> IDR6_R {
                IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 input data bit"]
            #[inline(always)]
            pub fn idr7(&self) -> IDR7_R {
                IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 input data bit"]
            #[inline(always)]
            pub fn idr8(&self) -> IDR8_R {
                IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 input data bit"]
            #[inline(always)]
            pub fn idr9(&self) -> IDR9_R {
                IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 input data bit"]
            #[inline(always)]
            pub fn idr10(&self) -> IDR10_R {
                IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 input data bit"]
            #[inline(always)]
            pub fn idr11(&self) -> IDR11_R {
                IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 input data bit"]
            #[inline(always)]
            pub fn idr12(&self) -> IDR12_R {
                IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 input data bit"]
            #[inline(always)]
            pub fn idr13(&self) -> IDR13_R {
                IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 input data bit"]
            #[inline(always)]
            pub fn idr14(&self) -> IDR14_R {
                IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 input data bit"]
            #[inline(always)]
            pub fn idr15(&self) -> IDR15_R {
                IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
    }
    #[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udr](udr) module"]
    pub type UDR = crate::Reg<u32, _UDR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _UDR;
    #[doc = "`read()` method returns [udr::R](udr::R) reader structure"]
    impl crate::Readable for UDR {}
    #[doc = "`write(|w| ..)` method takes [udr::W](udr::W) writer structure"]
    impl crate::Writable for UDR {}
    #[doc = "GPIO port output data register"]
    pub mod udr {
        #[doc = "Reader of register UDR"]
        pub type R = crate::R<u32, super::UDR>;
        #[doc = "Writer for register UDR"]
        pub type W = crate::W<u32, super::UDR>;
        #[doc = "Register UDR `reset()`'s with value 0"]
        impl crate::ResetValue for super::UDR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `ODR0`"]
        pub type ODR0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR0`"]
        pub struct ODR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR0_W<'a> {
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
        #[doc = "Reader of field `ODR1`"]
        pub type ODR1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR1`"]
        pub struct ODR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR1_W<'a> {
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
        #[doc = "Reader of field `ODR2`"]
        pub type ODR2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR2`"]
        pub struct ODR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR2_W<'a> {
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
        #[doc = "Reader of field `ODR3`"]
        pub type ODR3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR3`"]
        pub struct ODR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR3_W<'a> {
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
        #[doc = "Reader of field `ODR4`"]
        pub type ODR4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR4`"]
        pub struct ODR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `ODR5`"]
        pub type ODR5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR5`"]
        pub struct ODR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `ODR6`"]
        pub type ODR6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR6`"]
        pub struct ODR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `ODR7`"]
        pub type ODR7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR7`"]
        pub struct ODR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `ODR8`"]
        pub type ODR8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR8`"]
        pub struct ODR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `ODR9`"]
        pub type ODR9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR9`"]
        pub struct ODR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `ODR10`"]
        pub type ODR10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR10`"]
        pub struct ODR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `ODR11`"]
        pub type ODR11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR11`"]
        pub struct ODR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `ODR12`"]
        pub type ODR12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR12`"]
        pub struct ODR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `ODR13`"]
        pub type ODR13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR13`"]
        pub struct ODR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `ODR14`"]
        pub type ODR14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR14`"]
        pub struct ODR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `ODR15`"]
        pub type ODR15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `ODR15`"]
        pub struct ODR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> ODR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl R {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&self) -> ODR0_R {
                ODR0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&self) -> ODR1_R {
                ODR1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&self) -> ODR2_R {
                ODR2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&self) -> ODR3_R {
                ODR3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&self) -> ODR4_R {
                ODR4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&self) -> ODR5_R {
                ODR5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&self) -> ODR6_R {
                ODR6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&self) -> ODR7_R {
                ODR7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&self) -> ODR8_R {
                ODR8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&self) -> ODR9_R {
                ODR9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&self) -> ODR10_R {
                ODR10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&self) -> ODR11_R {
                ODR11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&self) -> ODR12_R {
                ODR12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&self) -> ODR13_R {
                ODR13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&self) -> ODR14_R {
                ODR14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&self) -> ODR15_R {
                ODR15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 output data bit"]
            #[inline(always)]
            pub fn odr0(&mut self) -> ODR0_W {
                ODR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 output data bit"]
            #[inline(always)]
            pub fn odr1(&mut self) -> ODR1_W {
                ODR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 output data bit"]
            #[inline(always)]
            pub fn odr2(&mut self) -> ODR2_W {
                ODR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 output data bit"]
            #[inline(always)]
            pub fn odr3(&mut self) -> ODR3_W {
                ODR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 output data bit"]
            #[inline(always)]
            pub fn odr4(&mut self) -> ODR4_W {
                ODR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 output data bit"]
            #[inline(always)]
            pub fn odr5(&mut self) -> ODR5_W {
                ODR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 output data bit"]
            #[inline(always)]
            pub fn odr6(&mut self) -> ODR6_W {
                ODR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 output data bit"]
            #[inline(always)]
            pub fn odr7(&mut self) -> ODR7_W {
                ODR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 output data bit"]
            #[inline(always)]
            pub fn odr8(&mut self) -> ODR8_W {
                ODR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 output data bit"]
            #[inline(always)]
            pub fn odr9(&mut self) -> ODR9_W {
                ODR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 output data bit"]
            #[inline(always)]
            pub fn odr10(&mut self) -> ODR10_W {
                ODR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 output data bit"]
            #[inline(always)]
            pub fn odr11(&mut self) -> ODR11_W {
                ODR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 output data bit"]
            #[inline(always)]
            pub fn odr12(&mut self) -> ODR12_W {
                ODR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 output data bit"]
            #[inline(always)]
            pub fn odr13(&mut self) -> ODR13_W {
                ODR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 output data bit"]
            #[inline(always)]
            pub fn odr14(&mut self) -> ODR14_W {
                ODR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 output data bit"]
            #[inline(always)]
            pub fn odr15(&mut self) -> ODR15_W {
                ODR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
    pub type BSRR = crate::Reg<u32, _BSRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BSRR;
    #[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
    impl crate::Writable for BSRR {}
    #[doc = "GPIO port bit set/reset register"]
    pub mod bsrr {
        #[doc = "Writer for register BSRR"]
        pub type W = crate::W<u32, super::BSRR>;
        #[doc = "Register BSRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BSRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BS0`"]
        pub struct BS0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS0_W<'a> {
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
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BS1`"]
        pub struct BS1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS1_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BS2`"]
        pub struct BS2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS2_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS3`"]
        pub struct BS3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS3_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS4`"]
        pub struct BS4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS5`"]
        pub struct BS5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS6`"]
        pub struct BS6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS7`"]
        pub struct BS7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
        #[doc = "Write proxy for field `BS8`"]
        pub struct BS8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
        #[doc = "Write proxy for field `BS9`"]
        pub struct BS9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS10`"]
        pub struct BS10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
        #[doc = "Write proxy for field `BS11`"]
        pub struct BS11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS12`"]
        pub struct BS12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
                self.w
            }
        }
        #[doc = "Write proxy for field `BS13`"]
        pub struct BS13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
        #[doc = "Write proxy for field `BS14`"]
        pub struct BS14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
        #[doc = "Write proxy for field `BS15`"]
        pub struct BS15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BS15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
        impl W {
            #[doc = "Bit 0 - Pin 0 set bit"]
            #[inline(always)]
            pub fn bs0(&mut self) -> BS0_W {
                BS0_W { w: self }
            }
            #[doc = "Bit 16 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 set bit"]
            #[inline(always)]
            pub fn bs1(&mut self) -> BS1_W {
                BS1_W { w: self }
            }
            #[doc = "Bit 17 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 set bit"]
            #[inline(always)]
            pub fn bs2(&mut self) -> BS2_W {
                BS2_W { w: self }
            }
            #[doc = "Bit 18 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 set bit"]
            #[inline(always)]
            pub fn bs3(&mut self) -> BS3_W {
                BS3_W { w: self }
            }
            #[doc = "Bit 19 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 set bit"]
            #[inline(always)]
            pub fn bs4(&mut self) -> BS4_W {
                BS4_W { w: self }
            }
            #[doc = "Bit 20 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 set bit"]
            #[inline(always)]
            pub fn bs5(&mut self) -> BS5_W {
                BS5_W { w: self }
            }
            #[doc = "Bit 21 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 set bit"]
            #[inline(always)]
            pub fn bs6(&mut self) -> BS6_W {
                BS6_W { w: self }
            }
            #[doc = "Bit 22 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 set bit"]
            #[inline(always)]
            pub fn bs7(&mut self) -> BS7_W {
                BS7_W { w: self }
            }
            #[doc = "Bit 23 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 set bit"]
            #[inline(always)]
            pub fn bs8(&mut self) -> BS8_W {
                BS8_W { w: self }
            }
            #[doc = "Bit 24 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 set bit"]
            #[inline(always)]
            pub fn bs9(&mut self) -> BS9_W {
                BS9_W { w: self }
            }
            #[doc = "Bit 25 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 set bit"]
            #[inline(always)]
            pub fn bs10(&mut self) -> BS10_W {
                BS10_W { w: self }
            }
            #[doc = "Bit 26 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 set bit"]
            #[inline(always)]
            pub fn bs11(&mut self) -> BS11_W {
                BS11_W { w: self }
            }
            #[doc = "Bit 27 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 set bit"]
            #[inline(always)]
            pub fn bs12(&mut self) -> BS12_W {
                BS12_W { w: self }
            }
            #[doc = "Bit 28 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 set bit"]
            #[inline(always)]
            pub fn bs13(&mut self) -> BS13_W {
                BS13_W { w: self }
            }
            #[doc = "Bit 29 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 set bit"]
            #[inline(always)]
            pub fn bs14(&mut self) -> BS14_W {
                BS14_W { w: self }
            }
            #[doc = "Bit 30 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 set bit"]
            #[inline(always)]
            pub fn bs15(&mut self) -> BS15_W {
                BS15_W { w: self }
            }
            #[doc = "Bit 31 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
    pub type LCKR = crate::Reg<u32, _LCKR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _LCKR;
    #[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
    impl crate::Readable for LCKR {}
    #[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
    impl crate::Writable for LCKR {}
    #[doc = "GPIO port configuration lock register"]
    pub mod lckr {
        #[doc = "Reader of register LCKR"]
        pub type R = crate::R<u32, super::LCKR>;
        #[doc = "Writer for register LCKR"]
        pub type W = crate::W<u32, super::LCKR>;
        #[doc = "Register LCKR `reset()`'s with value 0"]
        impl crate::ResetValue for super::LCKR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `LCK0`"]
        pub type LCK0_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK0`"]
        pub struct LCK0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK0_W<'a> {
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
        #[doc = "Reader of field `LCK1`"]
        pub type LCK1_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK1`"]
        pub struct LCK1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK1_W<'a> {
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
        #[doc = "Reader of field `LCK2`"]
        pub type LCK2_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK2`"]
        pub struct LCK2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK2_W<'a> {
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
        #[doc = "Reader of field `LCK3`"]
        pub type LCK3_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK3`"]
        pub struct LCK3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK3_W<'a> {
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
        #[doc = "Reader of field `LCK4`"]
        pub type LCK4_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK4`"]
        pub struct LCK4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `LCK5`"]
        pub type LCK5_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK5`"]
        pub struct LCK5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Reader of field `LCK6`"]
        pub type LCK6_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK6`"]
        pub struct LCK6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Reader of field `LCK7`"]
        pub type LCK7_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK7`"]
        pub struct LCK7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Reader of field `LCK8`"]
        pub type LCK8_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK8`"]
        pub struct LCK8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `LCK9`"]
        pub type LCK9_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK9`"]
        pub struct LCK9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Reader of field `LCK10`"]
        pub type LCK10_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK10`"]
        pub struct LCK10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Reader of field `LCK11`"]
        pub type LCK11_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK11`"]
        pub struct LCK11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Reader of field `LCK12`"]
        pub type LCK12_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK12`"]
        pub struct LCK12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `LCK13`"]
        pub type LCK13_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK13`"]
        pub struct LCK13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Reader of field `LCK14`"]
        pub type LCK14_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK14`"]
        pub struct LCK14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Reader of field `LCK15`"]
        pub type LCK15_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCK15`"]
        pub struct LCK15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCK15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        #[doc = "Reader of field `LCKK`"]
        pub type LCKK_R = crate::R<bool, bool>;
        #[doc = "Write proxy for field `LCKK`"]
        pub struct LCKK_W<'a> {
            w: &'a mut W,
        }
        impl<'a> LCKK_W<'a> {
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
        impl R {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&self) -> LCK0_R {
                LCK0_R::new((self.bits & 0x01) != 0)
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&self) -> LCK1_R {
                LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&self) -> LCK2_R {
                LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&self) -> LCK3_R {
                LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&self) -> LCK4_R {
                LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&self) -> LCK5_R {
                LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&self) -> LCK6_R {
                LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&self) -> LCK7_R {
                LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&self) -> LCK8_R {
                LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&self) -> LCK9_R {
                LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&self) -> LCK10_R {
                LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&self) -> LCK11_R {
                LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&self) -> LCK12_R {
                LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&self) -> LCK13_R {
                LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&self) -> LCK14_R {
                LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&self) -> LCK15_R {
                LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&self) -> LCKK_R {
                LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 lock bit"]
            #[inline(always)]
            pub fn lck0(&mut self) -> LCK0_W {
                LCK0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 lock bit"]
            #[inline(always)]
            pub fn lck1(&mut self) -> LCK1_W {
                LCK1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 lock bit"]
            #[inline(always)]
            pub fn lck2(&mut self) -> LCK2_W {
                LCK2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 lock bit"]
            #[inline(always)]
            pub fn lck3(&mut self) -> LCK3_W {
                LCK3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 lock bit"]
            #[inline(always)]
            pub fn lck4(&mut self) -> LCK4_W {
                LCK4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 lock bit"]
            #[inline(always)]
            pub fn lck5(&mut self) -> LCK5_W {
                LCK5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 lock bit"]
            #[inline(always)]
            pub fn lck6(&mut self) -> LCK6_W {
                LCK6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 lock bit"]
            #[inline(always)]
            pub fn lck7(&mut self) -> LCK7_W {
                LCK7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 lock bit"]
            #[inline(always)]
            pub fn lck8(&mut self) -> LCK8_W {
                LCK8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 lock bit"]
            #[inline(always)]
            pub fn lck9(&mut self) -> LCK9_W {
                LCK9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 lock bit"]
            #[inline(always)]
            pub fn lck10(&mut self) -> LCK10_W {
                LCK10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 lock bit"]
            #[inline(always)]
            pub fn lck11(&mut self) -> LCK11_W {
                LCK11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 lock bit"]
            #[inline(always)]
            pub fn lck12(&mut self) -> LCK12_W {
                LCK12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 lock bit"]
            #[inline(always)]
            pub fn lck13(&mut self) -> LCK13_W {
                LCK13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 lock bit"]
            #[inline(always)]
            pub fn lck14(&mut self) -> LCK14_W {
                LCK14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 lock bit"]
            #[inline(always)]
            pub fn lck15(&mut self) -> LCK15_W {
                LCK15_W { w: self }
            }
            #[doc = "Bit 16 - Lock key"]
            #[inline(always)]
            pub fn lckk(&mut self) -> LCKK_W {
                LCKK_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](afrl) module"]
    pub type AFRL = crate::Reg<u32, _AFRL>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRL;
    #[doc = "`read()` method returns [afrl::R](afrl::R) reader structure"]
    impl crate::Readable for AFRL {}
    #[doc = "`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure"]
    impl crate::Writable for AFRL {}
    #[doc = "GPIO alternate function low register"]
    pub mod afrl {
        #[doc = "Reader of register AFRL"]
        pub type R = crate::R<u32, super::AFRL>;
        #[doc = "Writer for register AFRL"]
        pub type W = crate::W<u32, super::AFRL>;
        #[doc = "Register AFRL `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRL {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR0`"]
        pub type AFR0_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR0`"]
        pub struct AFR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR0_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
                self.w
            }
        }
        #[doc = "Reader of field `AFR1`"]
        pub type AFR1_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR1`"]
        pub struct AFR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR1_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
                self.w
            }
        }
        #[doc = "Reader of field `AFR2`"]
        pub type AFR2_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR2`"]
        pub struct AFR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR2_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
                self.w
            }
        }
        #[doc = "Reader of field `AFR3`"]
        pub type AFR3_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR3`"]
        pub struct AFR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR3_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
                self.w
            }
        }
        #[doc = "Reader of field `AFR4`"]
        pub type AFR4_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR4`"]
        pub struct AFR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR4_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
                self.w
            }
        }
        #[doc = "Reader of field `AFR5`"]
        pub type AFR5_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR5`"]
        pub struct AFR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR5_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
                self.w
            }
        }
        #[doc = "Reader of field `AFR6`"]
        pub type AFR6_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR6`"]
        pub struct AFR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR6_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
                self.w
            }
        }
        #[doc = "Reader of field `AFR7`"]
        pub type AFR7_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR7`"]
        pub struct AFR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR7_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&self) -> AFR0_R {
                AFR0_R::new((self.bits & 0x0f) as u8)
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&self) -> AFR1_R {
                AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&self) -> AFR2_R {
                AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&self) -> AFR3_R {
                AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&self) -> AFR4_R {
                AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&self) -> AFR5_R {
                AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&self) -> AFR6_R {
                AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&self) -> AFR7_R {
                AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 0:3 - Pin 0 alternate function selection bits"]
            #[inline(always)]
            pub fn afr0(&mut self) -> AFR0_W {
                AFR0_W { w: self }
            }
            #[doc = "Bits 4:7 - Pin 1 alternate function selection bits"]
            #[inline(always)]
            pub fn afr1(&mut self) -> AFR1_W {
                AFR1_W { w: self }
            }
            #[doc = "Bits 8:11 - Pin 2 alternate function selection bits"]
            #[inline(always)]
            pub fn afr2(&mut self) -> AFR2_W {
                AFR2_W { w: self }
            }
            #[doc = "Bits 12:15 - Pin 3 alternate function selection bits"]
            #[inline(always)]
            pub fn afr3(&mut self) -> AFR3_W {
                AFR3_W { w: self }
            }
            #[doc = "Bits 16:19 - Pin 4 alternate function selection bits"]
            #[inline(always)]
            pub fn afr4(&mut self) -> AFR4_W {
                AFR4_W { w: self }
            }
            #[doc = "Bits 20:23 - Pin 5 alternate function selection bits"]
            #[inline(always)]
            pub fn afr5(&mut self) -> AFR5_W {
                AFR5_W { w: self }
            }
            #[doc = "Bits 24:27 - Pin 6 alternate function selection bits"]
            #[inline(always)]
            pub fn afr6(&mut self) -> AFR6_W {
                AFR6_W { w: self }
            }
            #[doc = "Bits 28:31 - Pin 7 alternate function selection bits"]
            #[inline(always)]
            pub fn afr7(&mut self) -> AFR7_W {
                AFR7_W { w: self }
            }
        }
    }
    #[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](afrh) module"]
    pub type AFRH = crate::Reg<u32, _AFRH>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _AFRH;
    #[doc = "`read()` method returns [afrh::R](afrh::R) reader structure"]
    impl crate::Readable for AFRH {}
    #[doc = "`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure"]
    impl crate::Writable for AFRH {}
    #[doc = "GPIO alternate function high register"]
    pub mod afrh {
        #[doc = "Reader of register AFRH"]
        pub type R = crate::R<u32, super::AFRH>;
        #[doc = "Writer for register AFRH"]
        pub type W = crate::W<u32, super::AFRH>;
        #[doc = "Register AFRH `reset()`'s with value 0"]
        impl crate::ResetValue for super::AFRH {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Reader of field `AFR8`"]
        pub type AFR8_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR8`"]
        pub struct AFR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR8_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 32)) | (((value as u32) & 0x0f) << 32);
                self.w
            }
        }
        #[doc = "Reader of field `AFR9`"]
        pub type AFR9_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR9`"]
        pub struct AFR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR9_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 36)) | (((value as u32) & 0x0f) << 36);
                self.w
            }
        }
        #[doc = "Reader of field `AFR10`"]
        pub type AFR10_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR10`"]
        pub struct AFR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR10_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 40)) | (((value as u32) & 0x0f) << 40);
                self.w
            }
        }
        #[doc = "Reader of field `AFR11`"]
        pub type AFR11_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR11`"]
        pub struct AFR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR11_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 44)) | (((value as u32) & 0x0f) << 44);
                self.w
            }
        }
        #[doc = "Reader of field `AFR12`"]
        pub type AFR12_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR12`"]
        pub struct AFR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR12_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 48)) | (((value as u32) & 0x0f) << 48);
                self.w
            }
        }
        #[doc = "Reader of field `AFR13`"]
        pub type AFR13_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR13`"]
        pub struct AFR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR13_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 52)) | (((value as u32) & 0x0f) << 52);
                self.w
            }
        }
        #[doc = "Reader of field `AFR14`"]
        pub type AFR14_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR14`"]
        pub struct AFR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR14_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 56)) | (((value as u32) & 0x0f) << 56);
                self.w
            }
        }
        #[doc = "Reader of field `AFR15`"]
        pub type AFR15_R = crate::R<u8, u8>;
        #[doc = "Write proxy for field `AFR15`"]
        pub struct AFR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> AFR15_W<'a> {
            #[doc = r"Writes raw bits to the field"]
            #[inline(always)]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                self.w.bits = (self.w.bits & !(0x0f << 60)) | (((value as u32) & 0x0f) << 60);
                self.w
            }
        }
        impl R {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&self) -> AFR8_R {
                AFR8_R::new(((self.bits >> 32) & 0x0f) as u8)
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&self) -> AFR9_R {
                AFR9_R::new(((self.bits >> 36) & 0x0f) as u8)
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&self) -> AFR10_R {
                AFR10_R::new(((self.bits >> 40) & 0x0f) as u8)
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&self) -> AFR11_R {
                AFR11_R::new(((self.bits >> 44) & 0x0f) as u8)
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&self) -> AFR12_R {
                AFR12_R::new(((self.bits >> 48) & 0x0f) as u8)
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&self) -> AFR13_R {
                AFR13_R::new(((self.bits >> 52) & 0x0f) as u8)
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&self) -> AFR14_R {
                AFR14_R::new(((self.bits >> 56) & 0x0f) as u8)
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&self) -> AFR15_R {
                AFR15_R::new(((self.bits >> 60) & 0x0f) as u8)
            }
        }
        impl W {
            #[doc = "Bits 32:35 - Pin 8 alternate function selection bits"]
            #[inline(always)]
            pub fn afr8(&mut self) -> AFR8_W {
                AFR8_W { w: self }
            }
            #[doc = "Bits 36:39 - Pin 9 alternate function selection bits"]
            #[inline(always)]
            pub fn afr9(&mut self) -> AFR9_W {
                AFR9_W { w: self }
            }
            #[doc = "Bits 40:43 - Pin 10 alternate function selection bits"]
            #[inline(always)]
            pub fn afr10(&mut self) -> AFR10_W {
                AFR10_W { w: self }
            }
            #[doc = "Bits 44:47 - Pin 11 alternate function selection bits"]
            #[inline(always)]
            pub fn afr11(&mut self) -> AFR11_W {
                AFR11_W { w: self }
            }
            #[doc = "Bits 48:51 - Pin 12 alternate function selection bits"]
            #[inline(always)]
            pub fn afr12(&mut self) -> AFR12_W {
                AFR12_W { w: self }
            }
            #[doc = "Bits 52:55 - Pin 13 alternate function selection bits"]
            #[inline(always)]
            pub fn afr13(&mut self) -> AFR13_W {
                AFR13_W { w: self }
            }
            #[doc = "Bits 56:59 - Pin 14 alternate function selection bits"]
            #[inline(always)]
            pub fn afr14(&mut self) -> AFR14_W {
                AFR14_W { w: self }
            }
            #[doc = "Bits 60:63 - Pin 15 alternate function selection bits"]
            #[inline(always)]
            pub fn afr15(&mut self) -> AFR15_W {
                AFR15_W { w: self }
            }
        }
    }
    #[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
    pub type BRR = crate::Reg<u32, _BRR>;
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub struct _BRR;
    #[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
    impl crate::Writable for BRR {}
    #[doc = "GPIO port bit reset register"]
    pub mod brr {
        #[doc = "Writer for register BRR"]
        pub type W = crate::W<u32, super::BRR>;
        #[doc = "Register BRR `reset()`'s with value 0"]
        impl crate::ResetValue for super::BRR {
            type Type = u32;
            #[inline(always)]
            fn reset_value() -> Self::Type {
                0
            }
        }
        #[doc = "Write proxy for field `BR0`"]
        pub struct BR0_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR0_W<'a> {
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
        #[doc = "Write proxy for field `BR1`"]
        pub struct BR1_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR1_W<'a> {
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
        #[doc = "Write proxy for field `BR2`"]
        pub struct BR2_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR2_W<'a> {
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
        #[doc = "Write proxy for field `BR3`"]
        pub struct BR3_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR3_W<'a> {
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
        #[doc = "Write proxy for field `BR4`"]
        pub struct BR4_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR4_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR5`"]
        pub struct BR5_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR5_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR6`"]
        pub struct BR6_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR6_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR7`"]
        pub struct BR7_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR7_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR8`"]
        pub struct BR8_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR8_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR9`"]
        pub struct BR9_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR9_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR10`"]
        pub struct BR10_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR10_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR11`"]
        pub struct BR11_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR11_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR12`"]
        pub struct BR12_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR12_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR13`"]
        pub struct BR13_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR13_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR14`"]
        pub struct BR14_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR14_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
                self.w
            }
        }
        #[doc = "Write proxy for field `BR15`"]
        pub struct BR15_W<'a> {
            w: &'a mut W,
        }
        impl<'a> BR15_W<'a> {
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
                self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
                self.w
            }
        }
        impl W {
            #[doc = "Bit 0 - Pin 0 reset bit"]
            #[inline(always)]
            pub fn br0(&mut self) -> BR0_W {
                BR0_W { w: self }
            }
            #[doc = "Bit 1 - Pin 1 reset bit"]
            #[inline(always)]
            pub fn br1(&mut self) -> BR1_W {
                BR1_W { w: self }
            }
            #[doc = "Bit 2 - Pin 2 reset bit"]
            #[inline(always)]
            pub fn br2(&mut self) -> BR2_W {
                BR2_W { w: self }
            }
            #[doc = "Bit 3 - Pin 3 reset bit"]
            #[inline(always)]
            pub fn br3(&mut self) -> BR3_W {
                BR3_W { w: self }
            }
            #[doc = "Bit 4 - Pin 4 reset bit"]
            #[inline(always)]
            pub fn br4(&mut self) -> BR4_W {
                BR4_W { w: self }
            }
            #[doc = "Bit 5 - Pin 5 reset bit"]
            #[inline(always)]
            pub fn br5(&mut self) -> BR5_W {
                BR5_W { w: self }
            }
            #[doc = "Bit 6 - Pin 6 reset bit"]
            #[inline(always)]
            pub fn br6(&mut self) -> BR6_W {
                BR6_W { w: self }
            }
            #[doc = "Bit 7 - Pin 7 reset bit"]
            #[inline(always)]
            pub fn br7(&mut self) -> BR7_W {
                BR7_W { w: self }
            }
            #[doc = "Bit 8 - Pin 8 reset bit"]
            #[inline(always)]
            pub fn br8(&mut self) -> BR8_W {
                BR8_W { w: self }
            }
            #[doc = "Bit 9 - Pin 9 reset bit"]
            #[inline(always)]
            pub fn br9(&mut self) -> BR9_W {
                BR9_W { w: self }
            }
            #[doc = "Bit 10 - Pin 10 reset bit"]
            #[inline(always)]
            pub fn br10(&mut self) -> BR10_W {
                BR10_W { w: self }
            }
            #[doc = "Bit 11 - Pin 11 reset bit"]
            #[inline(always)]
            pub fn br11(&mut self) -> BR11_W {
                BR11_W { w: self }
            }
            #[doc = "Bit 12 - Pin 12 reset bit"]
            #[inline(always)]
            pub fn br12(&mut self) -> BR12_W {
                BR12_W { w: self }
            }
            #[doc = "Bit 13 - Pin 13 reset bit"]
            #[inline(always)]
            pub fn br13(&mut self) -> BR13_W {
                BR13_W { w: self }
            }
            #[doc = "Bit 14 - Pin 14 reset bit"]
            #[inline(always)]
            pub fn br14(&mut self) -> BR14_W {
                BR14_W { w: self }
            }
            #[doc = "Bit 15 - Pin 15 reset bit"]
            #[inline(always)]
            pub fn br15(&mut self) -> BR15_W {
                BR15_W { w: self }
            }
        }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
        }
    }
}
