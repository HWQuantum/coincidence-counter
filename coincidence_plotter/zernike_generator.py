import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from sympy import Symbol, cos, sin, sqrt, atan2
from scipy.integrate import dblquad
import sympy

r = Symbol('r')
theta = Symbol('t')

# generates a list for the radial zernike polynomials with [(coeff, order)]
# so the radial part is given by sum(coeff*(r**order))

def radial_coeff(m, n, l):
    if (n-m)%2 == 0:
        return ((np.power(-1, l) * np.math.factorial(n-l))/
                (np.math.factorial(l)*
                    np.math.factorial(0.5*(n+m)-l)*
                    np.math.factorial(0.5*(n-m)-l)))
    else:
        return 0

def radial(m, n):
    return [(radial_coeff(m, n, l), n-2*l) for l in range(0, int(round((n-m)/2))+1)]

def z_val(m, n):
    abs_m = np.abs(m)
    rs = [(i, j) for (i, j) in radial(abs_m, n) if i != 0]
    if m==0:
        norm = np.sqrt(n+1)
    else:
        norm = np.sqrt(2*(n+1))
    if m < 0:
        def poly(r, theta):
            # r = np.sqrt(x**2+y**2)
            # theta = np.arctan2(y, x)
            return norm*np.sum([i*np.power(r, j) for (i, j) in rs], axis=0)*np.sin(abs_m*theta)
        return poly
    else:
        def poly(r, theta):
            # r = np.sqrt(x**2+y**2)
            # theta = np.arctan2(y, x)
            return norm*np.sum([i*np.power(r, j) for (i, j) in rs], axis=0)*np.cos(abs_m*theta)
        return poly

def zernike(m, n):
    abs_m = np.abs(m)
    rs = [(i, j) for (i, j) in radial(abs_m, n) if i != 0]
    if m==0:
        norm = np.sqrt(n+1)
    else:
        norm = np.sqrt(2*(n+1))
    if m < 0:
        def poly(r, theta):
            return norm*np.sum([i*np.power(r, j) for (i, j) in rs], axis=0)*np.sin(abs_m*theta)
        return poly
    else:
        def poly(r, theta):
            return norm*np.sum([i*np.power(r, j) for (i, j) in rs], axis=0)*np.cos(abs_m*theta)
        return poly

def zernike_cartesian(m, n):
    abs_m = np.abs(m)
    rs = [(i, j) for (i, j) in radial(abs_m, n) if i != 0]
    if m==0:
        norm = np.sqrt(n+1)
    else:
        norm = np.sqrt(2*(n+1))
    if m < 0:
        def poly(x, y):
            r = np.sqrt(x**2+y**2)
            theta = np.arctan2(y, x)
            return norm*np.sum([i*np.power(r, j) for (i, j) in rs], axis=0)*np.sin(abs_m*theta)
        return poly
    else:
        def poly(x, y):
            r = np.sqrt(x**2+y**2)
            theta = np.arctan2(y, x)
            return norm*np.sum([i*np.power(r, j) for (i, j) in rs], axis=0)*np.cos(abs_m*theta)
        return poly

def plot_z_surface(m, n):
    fig = plt.figure()
    ax = fig.gca(projection='3d')
    x = np.linspace(-1, 1, 40)
    y = np.linspace(-1, 1, 40)
    X, Y = np.meshgrid(x, y)
    filter = np.where(np.sqrt(X**2+Y**2)<=1)
    X = X[filter]
    Y = Y[filter]
    r = np.sqrt(X**2+Y**2)
    theta = np.arctan2(Y, X)
    a = z_val(m, n)
    z = a(r, theta)
    ax.plot_trisurf(X, Y, z)
    plt.show()

def z_symbol(m, n):
    abs_m = np.abs(m)
    rs = [(i, j) for (i, j) in radial(abs_m, n) if i != 0]
    if m==0:
        norm = sqrt(n+1)
    else:
        norm = sqrt(2*(n+1))
    if m < 0:
        return norm*sum([i*(r**j) for (i, j) in rs])*sin(abs_m*theta)
    else:
        return norm*sum([i*(r**j) for (i, j) in rs])*cos(abs_m*theta)

def integrate_symbol():
    x = Symbol('x')
    y = Symbol('y')
    a = z_symbol(0, 0)
    b = z_symbol(0, 0)
    print(a)
    expression = (r).subs(r, sqrt(x**2+y**2)).subs(theta, atan2(y, x))
    print(sympy.integrate(expression, (x, -sqrt(sympy.pi)/2, sqrt(sympy.pi)/2), (y, -sqrt(sympy.pi)/2, sqrt(sympy.pi)/2)))

if __name__ == '__main__':
    l = sqrt(np.pi)/2
    print(dblquad(lambda x, y: zernike_cartesian(0, 10)(x, y)*zernike_cartesian(5, 5)(x, y), -l, l, -l, l))
    print(dblquad(lambda x, y: zernike(0, 10)(x, y)*zernike(5, 5)(x, y)*x, 0, np.pi*2, 0, 1))
