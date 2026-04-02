# Rust Ray Tracer

A CPU-based ray tracer built from scratch in Rust. This project is a mathematical implementation of light physics, based on the highly acclaimed book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley. 

It leverages the Rust type system for linear algebra and the `rayon` crate for high-performance, lock-free parallel rendering.

## Mathematical Foundation

The core of the engine relies on vector calculus and optical physics to simulate how light interacts with matter.

### 1. The Ray Equation
A ray is defined as a function of a parameter $t$:
$$\mathbf{P}(t) = \mathbf{A} + t\mathbf{b}$$
Where $\mathbf{A}$ is the ray origin, $\mathbf{b}$ is the ray direction, and $\mathbf{P}(t)$ is a 3D position along the line.

### 2. Sphere Intersection
To determine if a ray hits a sphere, we check if the ray's point $\mathbf{P}(t)$ satisfies the sphere equation:
$$(\mathbf{P} - \mathbf{C}) \cdot (\mathbf{P} - \mathbf{C}) = r^2$$
Substituting the ray equation into the sphere equation yields a quadratic equation in terms of $t$:
$$t^2 (\mathbf{b} \cdot \mathbf{b}) + 2t \mathbf{b} \cdot (\mathbf{A}-\mathbf{C}) + (\mathbf{A}-\mathbf{C}) \cdot (\mathbf{A}-\mathbf{C}) - r^2 = 0$$
By calculating the discriminant ($b^2 - 4ac$), we can determine if the ray hits the sphere twice (intersects), once (tangent), or never (misses).

### 3. Materials and Scattering
* **Lambertian (Diffuse):** Uses rejection sampling to find a random point $\mathbf{S}$ inside a unit sphere tangent to the hit point, simulating diffuse light scattering.
* **Metal (Reflective):** Calculates perfect reflection using the vector reflection formula:
    $$\mathbf{r}_{out} = \mathbf{v} - 2(\mathbf{v} \cdot \mathbf{n})\mathbf{n}$$
* **Dielectric (Glass):** Implements refraction using Snell's Law ($\eta \sin\theta = \eta' \sin\theta'$). It also uses Christophe Schlick's polynomial approximation to compute specular reflection depending on the viewing angle.

### 4. Camera & Depth of Field (Defocus Blur)
The camera implements a thin-lens approximation. Rays are generated from a random point on a unit disk (scaled by the aperture) and directed toward the focal plane, simulating the depth of field effect found in real physical cameras.

## Features
* Linear Algebra / Vector Math Engine
* Multithreaded Rendering (`rayon`)
* Antialiasing (MSAA)
* Gamma Correction (Gamma 2)
* Depth of Field
* Materials: Lambertian, Metal (with fuzziness), Dielectric

## Execution

The output format is a `.ppm` file. For maximum performance, run the compiler in release mode:

```bash
cargo run --release > result.ppm
