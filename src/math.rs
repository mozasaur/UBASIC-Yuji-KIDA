//! Mathematical functions for UBASIC
//! 
//! This module provides mathematical functions like trigonometric, exponential,
//! logarithmic, and other mathematical operations.

use crate::errors::{UBasicError, UBasicResult};
use crate::types::UBasicValue;
use rug::{Integer, Float, Complex, Assign};
use num_complex::Complex64;

/// Mathematical engine for UBASIC
pub struct MathEngine {
    precision: u32,
}

impl MathEngine {
    /// Create a new math engine with default precision
    pub fn new() -> Self {
        Self {
            precision: 64,
        }
    }

    /// Create a new math engine with custom precision
    pub fn with_precision(precision: u32) -> Self {
        Self { precision }
    }

    /// Set the precision for calculations
    pub fn set_precision(&mut self, precision: u32) {
        self.precision = precision;
    }

    /// Get the current precision
    pub fn get_precision(&self) -> u32 {
        self.precision
    }

    /// Calculate the sine of a value
    pub fn sin(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.sin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.sin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.sin();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the cosine of a value
    pub fn cos(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.cos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.cos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.cos();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the tangent of a value
    pub fn tan(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.tan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.tan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.tan();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the arcsine of a value
    pub fn asin(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.asin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.asin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.asin();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the arccosine of a value
    pub fn acos(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.acos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.acos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.acos();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the arctangent of a value
    pub fn atan(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.atan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.atan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.atan();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the exponential of a value
    pub fn exp(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.exp();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.exp();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.exp();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the natural logarithm of a value
    pub fn ln(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.ln();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.ln();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.ln();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the square root of a value
    pub fn sqrt(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.sqrt();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.sqrt();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.sqrt();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the absolute value of a value
    pub fn abs(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.abs();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let result = i.abs();
                Ok(UBasicValue::Integer(result))
            }
            UBasicValue::Complex(c) => {
                let norm = c.norm_ref().to_f64();
                Ok(UBasicValue::Float(Float::with_val(self.precision, norm)))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the factorial of a value
    pub fn factorial(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Integer(i) => {
                if let Some(n) = i.to_u32() {
                    if n <= 20 { // Reasonable limit for factorial
                        let mut result = Integer::from(1u32);
                        for k in 2..=n {
                            result *= k;
                        }
                        Ok(UBasicValue::Integer(result))
                    } else {
                        Err(UBasicError::overflow("factorial"))
                    }
                } else {
                    Err(UBasicError::overflow("factorial"))
                }
            }
            UBasicValue::Float(f) => {
                // Use gamma function approximation for floats
                let x = f.to_f64() + 1.0;
                if x > 0.0 && x < 100.0 {
                    let result = self.gamma_approx(x);
                    Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
                } else {
                    Err(UBasicError::overflow("factorial"))
                }
            }
            _ => Err(UBasicError::type_mismatch(
                "integer or float",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Get the value of π
    pub fn pi(&self) -> UBasicValue {
        UBasicValue::Float(Float::with_val(self.precision, rug::Float::parse("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989380952572010654858632788659361533818279682303019520353018529689957736225994138912497217752834791315155748572424541506959508295331168617278558890750983817546374649393192550604009277016711390098488240128583616035637076601047101819429555961989467678374494482553797747268471040475346462080466842590694912933136770289891521047521620569660240580381501935112533824300355876402474964732639141992726042699227967823547816360093417216412199245863150302861829745557067498385054945885869269956909272107975093029553211653449872027559602364806654991198818347977535663698074265425278625518184175746728909777727938000816470600161452491921732172147723501414419735685481613611573525521334757418494684385233239073941433345477624168625189835694855620992192221842725502542568876717904946016745").unwrap()))
    }

    /// Get the value of e
    pub fn e(&self) -> UBasicValue {
        UBasicValue::Float(Float::with_val(self.precision, rug::Float::parse("2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274274663919320030599218174135966290435729003342952605956307381323286279434907632338298807531952510190115738341879307021540891499348841675092447614606680822648001684774118537423454424371075390777449920695517027618386062613313845830007520449338265602976067371132007093287091274437470472306969772093101416928368190255151086574637721112523897844250569536967707854499699679468644549059879316368892300987931277361782154249992295763514822082698951936680331825288693984964651058209392398294887933203625094431173012381970684161403970198376793206832823764648042953118023287825098194558153017567173613320698112509961818815930416903515988885193458072738667385894228792284998920868058257492796104841984443634632449684875602336248270419786232090021609902353043699418491463140934317381436405462531520961836908887070167683964243781405927145635490613031072085103837505101157477041718986106873969655212671546889570350354021234078498193343210681701210056278802351930332247450158539047304199577770935036604169973297250886876966403555707162268447162560798826517871341951246652010305921236677194325278675398558944896970964097545918569563802363701621120477427228364896134225164450781824423529486363721417402388934412479635743702637552944483379980161254922785092577825620926226483262779333865664816277251640191059004916449982893150566047258027786318641551956532442586982946959308019152987211725563475463964479101459040905862984967912874068705048958586717479854667757573205681288459205413340539220001131863001106820132162694889412255856255942348466708343455645734984410301678639766111959092164201989380952572010654858632788659361533818279682303019520353018529689957736225994138912497217752834791315155748572424541506959508295331168617278558890750983817546374649393192550604009277016711390098488240128583616035637076601047101819429555961989467678374494482553797747268471040475346462080466842590694912933136770289891521047521620569660240580381501935112533824300355876402474964732639141992726042699227967823547816360093417216412199245863150302861829745557067498385054945885869269956909272107975093029553211653449872027559602364806654991198818347977535663698074265425278625518184175746728909777727938000816470600161452491921732172147723501414419735685481613611573525521334757418494684385233239073941433345477624168625189835694855620992192221842725502542568876717904946016745").unwrap()))
    }

    /// Calculate the greatest common divisor
    pub fn gcd(&self, a: &UBasicValue, b: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (a, b) {
            (UBasicValue::Integer(x), UBasicValue::Integer(y)) => {
                let result = x.gcd_ref(y);
                Ok(UBasicValue::Integer(result.into()))
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?} and {:?}", a.get_type(), b.get_type()),
            )),
        }
    }

    /// Calculate the least common multiple
    pub fn lcm(&self, a: &UBasicValue, b: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (a, b) {
            (UBasicValue::Integer(x), UBasicValue::Integer(y)) => {
                let result = x.lcm_ref(y);
                Ok(UBasicValue::Integer(result.into()))
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?} and {:?}", a.get_type(), b.get_type()),
            )),
        }
    }

    /// Check if a number is prime
    pub fn is_prime(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Integer(i) => {
                if i <= &Integer::from(1) {
                    return Ok(UBasicValue::Boolean(false));
                }
                if i <= &Integer::from(3) {
                    return Ok(UBasicValue::Boolean(true));
                }
                
                // Simple trial division for small numbers
                let n = i.to_u64().unwrap_or(0);
                if n > 0 && n < 1000000 {
                    let mut d = 2;
                    while d * d <= n {
                        if n % d == 0 {
                            return Ok(UBasicValue::Boolean(false));
                        }
                        d += 1;
                    }
                    Ok(UBasicValue::Boolean(true))
                } else {
                    // For large numbers, use probabilistic test
                    let is_probably_prime = i.is_probably_prime(25) != 0;
                    Ok(UBasicValue::Boolean(is_probably_prime))
                }
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Approximate gamma function for factorial of floats
    fn gamma_approx(&self, x: f64) -> f64 {
        // Stirling's approximation for large x
        if x > 10.0 {
            let z = x - 1.0;
            (2.0 * std::f64::consts::PI * z).sqrt() * (z / std::f64::consts::E).powf(z)
        } else {
            // Simple approximation for smaller values
            let mut result = 1.0;
            let mut n = x - 1.0;
            while n > 1.0 {
                result *= n;
                n -= 1.0;
            }
            result
        }
    }
}

impl Default for MathEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric_functions() {
        let math = MathEngine::new();
        
        let pi = math.pi();
        let sin_pi = math.sin(&pi).unwrap();
        let cos_pi = math.cos(&pi).unwrap();
        
        assert!(matches!(sin_pi, UBasicValue::Float(_)));
        assert!(matches!(cos_pi, UBasicValue::Float(_)));
    }

    #[test]
    fn test_exponential_and_logarithm() {
        let math = MathEngine::new();
        
        let e = math.e();
        let ln_e = math.ln(&e).unwrap();
        
        assert!(matches!(ln_e, UBasicValue::Float(_)));
    }

    #[test]
    fn test_sqrt() {
        let math = MathEngine::new();
        
        let four = UBasicValue::Integer(Integer::from(4));
        let sqrt_four = math.sqrt(&four).unwrap();
        
        assert!(matches!(sqrt_four, UBasicValue::Float(_)));
    }

    #[test]
    fn test_factorial() {
        let math = MathEngine::new();
        
        let five = UBasicValue::Integer(Integer::from(5));
        let factorial_five = math.factorial(&five).unwrap();
        
        assert!(matches!(factorial_five, UBasicValue::Integer(_)));
    }

    #[test]
    fn test_gcd_lcm() {
        let math = MathEngine::new();
        
        let a = UBasicValue::Integer(Integer::from(12));
        let b = UBasicValue::Integer(Integer::from(18));
        
        let gcd_result = math.gcd(&a, &b).unwrap();
        let lcm_result = math.lcm(&a, &b).unwrap();
        
        assert!(matches!(gcd_result, UBasicValue::Integer(_)));
        assert!(matches!(lcm_result, UBasicValue::Integer(_)));
    }

    #[test]
    fn test_is_prime() {
        let math = MathEngine::new();
        
        let prime = UBasicValue::Integer(Integer::from(17));
        let not_prime = UBasicValue::Integer(Integer::from(15));
        
        let prime_result = math.is_prime(&prime).unwrap();
        let not_prime_result = math.is_prime(&not_prime).unwrap();
        
        assert_eq!(prime_result, UBasicValue::Boolean(true));
        assert_eq!(not_prime_result, UBasicValue::Boolean(false));
    }
} 