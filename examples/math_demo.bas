REM Mathematical Functions Demo for UBASIC Rust
REM Demonstrates various mathematical operations and functions

PRINT "UBASIC Rust Mathematical Functions Demo"
PRINT "======================================"

REM Basic arithmetic
LET a = 15
LET b = 7
PRINT "a =", a, "b =", b
PRINT "a + b =", a + b
PRINT "a - b =", a - b
PRINT "a * b =", a * b
PRINT "a / b =", a / b
PRINT "a MOD b =", a MOD b

PRINT

REM Trigonometric functions
LET angle = 0.785398  REM π/4 radians (45 degrees)
PRINT "Angle =", angle, "radians (45 degrees)"
PRINT "SIN(angle) =", SIN(angle)
PRINT "COS(angle) =", COS(angle)
PRINT "TAN(angle) =", TAN(angle)

PRINT

REM Exponential and logarithmic functions
LET x = 2.5
PRINT "x =", x
PRINT "EXP(x) =", EXP(x)
PRINT "LN(x) =", LN(x)
PRINT "SQRT(x) =", SQRT(x)

PRINT

REM Constants
PRINT "Mathematical Constants:"
PRINT "π (pi) =", PI
PRINT "e =", E

PRINT

REM Factorial
LET n = 5
PRINT "Factorial of", n, "=", FACTORIAL(n)

PRINT

REM Complex numbers (if supported)
PRINT "Complex Number Operations:"
LET z1 = (3 + 4i)
LET z2 = (1 + 2i)
PRINT "z1 =", z1
PRINT "z2 =", z2
PRINT "z1 + z2 =", z1 + z2
PRINT "z1 * z2 =", z1 * z2

PRINT
PRINT "Demo completed!" 