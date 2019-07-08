use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn almost_equal(&self, other: &Vector) -> bool {
        self.almost_equal_with_epsilon(other, 0.0000001)
    }

    pub fn almost_equal_with_epsilon(&self, other: &Vector, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon
            && (self.z - other.z).abs() < epsilon
    }

    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unitx() -> Vector {
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unity() -> Vector {
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn unitz() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalized(&self) -> Vector {
        let len = self.len();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        self + -1.0 * other
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        other * self
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}

impl Ray {
    fn forwarded(&self, distance: f32) -> Ray {
        Ray {
            pos: self.pos + self.dir * distance,
            dir: self.dir,
        }
    }

    pub fn almost_equal(&self, other: &Ray) -> bool {
        self.pos.almost_equal(&other.pos) && self.dir.almost_equal(&other.dir)
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect_ray(&self, ray: &Ray) -> Intersection {
        // Math based on information found on
        // http://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
        //
        let pos_to_center = self.center - ray.pos;
        // No support for intersections with rays coming from inside the sphere at the moment.
        if pos_to_center.len() <= self.radius {
            return Intersection::None;
        }
        // tcenter is how far along the ray dir we need to go in order for the line orthogonal to
        // the ray to cross the sphere's center. Let's call that point on the ray C.
        let tcenter = pos_to_center.dot(&ray.dir);
        // The sphere is in the opposite direction.
        if tcenter < 0.0 {
            return Intersection::None;
        }
        // We now have a right triangle with [ray.pos C] being one of its leg and [ray.pos
        // sphere.center] being its hypotenuse. The distance between C and self.center is what we
        // need to find out and its the remaining leg of the triangle – let's use the Pythagorean
        // theorem. We'll call the [C self.center] distance d.
        let d = (pos_to_center.len().powf(2.0) - tcenter.powf(2.0)).sqrt();
        // If we miss the sphere totally the distance d will be greater than the radius, let's bail
        // in that case.
        if d > self.radius {
            return Intersection::None;
        }
        // Now we have two right triangles with self.radius being its hypotenuse and d forming one
        // of its legs. The remaining leg is a distance tdelta that we'll use to move forward and
        // backward along the ray starting with point C in order to get two points at which we
        // intersect the sphere. Again – just Pythagorean theorem at work here.
        let tdelta = (self.radius.powf(2.0) - d.powf(2.0)).sqrt();
        // We can now calculate two points at which we cross the sphere, but we only need the
        // closer one so let's do just that.
        let intersection_point = ray.forwarded(tcenter - tdelta).pos;
        Intersection::Hit(intersection_point)
    }
}

#[derive(Debug)]
pub enum Intersection {
    None,
    Hit(Vector),
}

impl Intersection {
    pub fn almost_equal(&self, other: &Intersection) -> bool {
        match self {
            Intersection::None => match other {
                Intersection::None => true,
                _ => false,
            },
            Intersection::Hit(v1) => match other {
                Intersection::Hit(v2) => v1.almost_equal(&v2),
                _ => false,
            },
        }
    }
}

pub fn almost_equal(a: f32, b: f32) -> bool {
    almost_equal_with_epsilon(a, b, 0.0000001)
}

pub fn almost_equal_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

pub struct Camera {
    pub position: Vector,
    // The forward and up vectors have to be normalized
    pub forward: Vector,
    pub up: Vector,
    pub aspect_ratio: f32,
    pub fovx: Radians,
}

impl Camera {
    pub fn screen_ray(&self, x: f32, y: f32) -> Ray {
        // We assume that a screen lies 1 unit in front of the camera. The center (x: 0.5, y: 0.5) of the screen
        // lies directly on the forward axis.
        assert!(0.0 <= x && x <= 1.0);
        assert!(0.0 <= y && y <= 1.0);
        let right = self.forward.cross(&self.up);
        // top left corner is x -1.0, y 1.0
        let xunit = posunit_to_unit(x);
        let yunit = -posunit_to_unit(y);
        // The distance between a point on the screen and the center of the screen forms a right
        // triangle with the distance between the camera and the center of the screen and the
        // distance between the camera and the point. Since we know the maximum angle we can go in
        // either direction (fovx/2 for x, fovy/2 for y) we first calculate the size of the screen
        // 1 unit in front of the camera using tangent:
        let screen_width = 2.0 * (self.fovx.0 / 2.0).tan();
        let screen_height = screen_width / self.aspect_ratio;
        // What's left now is to calculate the point at the screen we're looking at and a ray
        // pointing to it:
        let point_at_screen = self.position
            + self.forward
            + right * xunit * screen_width / 2.0
            + self.up * yunit * screen_height / 2.0;
        let ray = Ray {
            pos: self.position,
            dir: (point_at_screen - self.position).normalized(),
        };
        ray
    }
}

pub fn posunit_to_unit(value: f32) -> f32 {
    // Convert value in range [0.0, 1.0] to value in range [-1.0, 1.0]
    value * 2.0 - 1.0
}

pub struct Radians(pub f32);
