//! # Quadrature Encoder Interface
use crate::hal::{self, Direction};
use crate::stm32::RCC;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::{TIM1, TIM5};

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::{TIM2, TIM3, TIM4};

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::TIM8;

pub trait Pins<TIM> {}
use crate::timer::PinC1;
use crate::timer::PinC2;

impl<TIM, PC1, PC2> Pins<TIM> for (PC1, PC2)
where
    PC1: PinC1<TIM>,
    PC2: PinC2<TIM>,
{
}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident, $apbenr:ident, $apbrstr:ident, $bits:ident),)+) => {
        $(
            impl<PINS> Qei<$TIM, PINS> {
                /// Configures a TIM peripheral as a quadrature encoder interface input
                pub fn $tim(tim: $TIM, pins: PINS) -> Self
                where
                    PINS: Pins<$TIM>
                {
                    let rcc = unsafe { &(*RCC::ptr()) };
                    // enable and reset peripheral to a clean slate state
                    rcc.$apbenr.modify(|_, w| w.$timXen().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().clear_bit());

                    // Configure TxC1 and TxC2 as captures
                    tim.ccmr1_output()
                        .write(|w| unsafe { w.cc1s().bits(0b01).cc2s().bits(0b01) });

                    // enable and configure to capture on rising edge
                    tim.ccer.write(|w| {
                        w.cc1e()
                            .set_bit()
                            .cc1p()
                            .clear_bit()
                            .cc2e()
                            .set_bit()
                            .cc2p()
                            .clear_bit()
                    });

                    // configure as quadrature encoder
                    // some chip variants declare `.bits()` as unsafe, some don't
                    #[allow(unused_unsafe)]
                    tim.smcr.write(|w| unsafe { w.sms().bits(3) });

                    tim.arr.write(|w| unsafe { w.bits(core::u32::MAX) });
                    tim.cr1.write(|w| w.cen().set_bit());

                    Qei { tim, pins }
                }

                /// Releases the TIM peripheral and QEI pins
                pub fn release(self) -> ($TIM, PINS) {
                    (self.tim, self.pins)
                }
            }

            impl<PINS> hal::Qei for Qei<$TIM, PINS> {
                type Count = $bits;

                fn count(&self) -> $bits {
                    self.tim.cnt.read().bits() as $bits
                }

                fn direction(&self) -> Direction {
                    if self.tim.cr1.read().dir().bit_is_clear() {
                        hal::Direction::Upcounting
                    } else {
                        hal::Direction::Downcounting
                    }
                }
            }

        )+
    }
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
hal! {
    TIM1: (tim1, tim1en, tim1rst, apb2enr, apb2rstr, u16),
    TIM5: (tim5, tim5en, tim5rst, apb1enr, apb1rstr, u32),
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
hal! {
    TIM2: (tim2, tim2en, tim2rst, apb1enr, apb1rstr, u32),
    TIM3: (tim3, tim3en, tim3rst, apb1enr, apb1rstr, u16),
    TIM4: (tim4, tim4en, tim4rst, apb1enr, apb1rstr, u16),
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
hal! {
    TIM8: (tim8, tim8en, tim8rst, apb2enr, apb2rstr, u16),
}
