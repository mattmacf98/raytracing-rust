use crate::{common::{random_double, random_int_range}, vec3::{dot, Point3, Vec3}};

pub struct Perlin {
    rand_vec: [Vec3; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256]
}

impl Perlin {

    pub fn new() -> Perlin {
        let mut rand_vec = [Vec3::random(); 256];
        for i in 0..256 {
            rand_vec[i] = Vec3::random_range(-1.0, 1.0);
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin {
            rand_vec,
            perm_x,
            perm_y,
            perm_z
        }
    }

    pub fn noise(&self, scale: f64, p: &Point3) -> f64 {
       let mut u = f64::abs(p.x() * scale) - f64::floor(f64::abs(p.x() * scale));
       let mut v = f64::abs(p.y() * scale) - f64::floor(f64::abs(p.y() * scale));
       let mut w = f64::abs(p.z() * scale) - f64::floor(f64::abs(p.z() * scale));

       u = u*u*(3.0-2.0*u);
       v = v*v*(3.0-2.0*v);
       w = w*w*(3.0-2.0*w);

       let i = f64::floor(f64::abs(p.x() * scale)) as usize;
       let j = f64::floor(f64::abs(p.y() * scale)) as usize;
       let k = f64::floor(f64::abs(p.z() * scale)) as usize;
       let mut c = [[[Vec3::random(); 2];2];2];

       for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[(self.perm_x[(i + di) & 255] ^ self.perm_y[(j + dj) & 255] ^ self.perm_z[(k + dk) & 255]) as usize]
                }
            }
       }

       Perlin::perlin_interp(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(1.0, &temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        f64::abs(accum)
    }

    fn perlin_generate_perm() -> [i32; 256] {
        let mut p = [0; 256];
        for i in 0..256 {
            p[i] = i as i32;
        }

        Perlin::permute(256, &mut p);

        p
    }

    fn permute(n: usize, p: &mut [i32; 256]) {
        for i in (0..n-1).rev() {
            let target: usize = random_int_range(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
       let uu = u*u*(3.0-2.0*u);
       let vv = v*v*(3.0-2.0*v);
       let ww = w*w*(3.0-2.0*w);
       let mut accum = 0.0;

       for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                let term_one = uu * (i as f64) + ((1 - (i as i32)) as f64) * (1.0 - uu);
                let term_two = vv * (j as f64) + ((1 - (j as i32)) as f64) * (1.0 - vv);
                let term_three = ww * (k as f64) + ((1 - (k as i32)) as f64) * (1.0 - ww);
                accum += term_one * term_two * term_three * dot(c[i][j][k], weight);
            }
        }
       }

        accum
    }
    
}