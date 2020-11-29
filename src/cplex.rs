pub struct Cplex<T> {
    real: T,
    im: T,
}

impl<T> Cplex<T> {
    #[inline(always)]
    pub fn new(real: T, im: T) -> Cplex<T> {
        Cplex { real, im }
    }

    // Sq Returns the Squared value
    #[inline(always)]
    pub fn Sq(&self) -> T
    where
        T: num::traits::Float,
    {
        (self.real * self.real) + (self.im * self.im)
    }

    // AddTo Adds another complex to this complex (similar to "+=" operator)
    #[inline(always)]
    pub fn AddTo(&mut self, b: &Cplex<T>)
    where
        T: num::traits::Float + num::traits::NumAssign,
    {
        self.real += b.real;
        self.im += b.im;
    }

    // MultBy Multiplies this complex by another complex (similar to "*=" operator)
    /*#[inline(always)]
    pub fn MultBy(&mut self, b: &mut Cplex<T>)
    where
        T: num::traits::Float,
    {
        let re = (self.real * b.real) - (self.im * b.im);
        let im = (self.real * b.im) + (self.im * b.real);
        b.real = re;
        b.im = im;
    }*/

    // Pow2 Multiplies this complex by its self
    #[inline(always)]
    pub fn Pow2(&mut self)
    where
        T: num::traits::Float,
    {
        let re = (self.real * self.real) - (self.im * self.im);
        let im = (self.real * self.im) + (self.im * self.real);
        self.real = re;
        self.im = im;
    }
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
