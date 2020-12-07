// Sq Returns the Squared value
#[inline(always)]
pub fn sq<T>(a: &num::Complex<T>) -> T
where
    T: num::traits::Float,
{
    (a.re * a.re) + (a.im * a.im)
}

// Add_c Adds another complex to this complex (similar to "+=" operator)
#[inline(always)]
pub fn add_c<'a, T>(a: &'a mut num::Complex<T>, b: &'a num::Complex<T>) -> &'a num::Complex<T>
where
    T: num::traits::Float + num::traits::NumAssign,
{
    a.re += b.re;
    a.im += b.im;
    a
}

// Pow2 Multiplies this complex by its self
#[inline(always)]
pub fn pow2<T>(a: &mut num::Complex<T>) -> &mut num::Complex<T>
where
    T: num::traits::Float,
{
    let re = (a.re * a.re) - (a.im * a.im);
    let im = (a.re * a.im) + (a.im * a.re);
    a.re = re;
    a.im = im;
    a
}

// mandel is a combination of the Pow2 function and add_c function.
// It preforms z = z * z + c where z is a and c is b.
// This function can also be written as z.pow2().add_c(c)
#[inline(always)]
pub fn mandel<'a, T>(a: &'a mut num::Complex<T>, b: &'a num::Complex<T>) -> &'a num::Complex<T>
where
    T: num::traits::Float,
{
    let new_re = (a.re * a.re) - (a.im * a.im);
    let new_im = (a.re * a.im) + (a.im * a.re);
    a.re = new_re + b.re;
    a.im = new_im + b.im;

    a
}

//impl Mult<T> for Cplex<T> {}

/*
// Complex is a struct :)
type Complex struct {
    Real float64
    Im   float64
}

// NewComplex Creates a new Complex number
func NewComplex(_real, _im float64) *Complex {
    c := new(Complex)
    c.Real = _real
    c.Im = _im
    return c
}

// Abs Returns the absolute value
func Abs(z *Complex) float64 {
    sq := (z.Real * z.Real) + (z.Im * z.Im)
    result := math.Sqrt(sq)
    return result
}

// Sq Returns the Squared value
func Sq(z *Complex) float64 {
    result := (z.Real * z.Real) + (z.Im * z.Im)
    //result := math.Sqrt(sq)
    return result
}

// Conj Returns the conjugate
func Conj(z *Complex) *Complex {
    result := NewComplex(z.Real, -(z.Im))
    return result
}

// AddTo Adds another complex to this complex (similar to "+=" operator)
func (c *Complex) AddTo(b *Complex) {
    c.Real += b.Real
    c.Im += b.Im
}

// Add two complex numbers together to make a third complex
func Add(a, b *Complex) *Complex {
    c := NewComplex(a.Real, a.Im)
    c.AddTo(b)
    return c
}

// SubFrom Subtracts another complex from this complex (similar to "-=" operator)
func (c *Complex) SubFrom(b *Complex) {
    c.Real -= b.Real
    c.Im -= b.Im
}

// Sub Subtracts one complex from another to make a third complex
func Sub(a, b *Complex) *Complex {
    c := NewComplex(a.Real, a.Im)
    c.SubFrom(b)
    return c
}

// MultBy Multiplies this complex by another complex (similar to "*=" operator)
func (c *Complex) MultBy(b *Complex) {
    re := (c.Real * b.Real) - (c.Im * b.Im)
    im := (c.Real * b.Im) + (c.Im * b.Real)
    //c.Real -= re
    //c.Im -= im
    c.Real = re
    c.Im = im
}

// Mult Multiplies one complex by another to make a third complex
func Mult(a, b *Complex) *Complex {
    re := (a.Real * b.Real) - (a.Im * b.Im)
    im := (a.Real * b.Im) + (a.Im * b.Real)
    c := NewComplex(re, im)
    return c
}*/
