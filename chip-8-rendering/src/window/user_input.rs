#[derive(Clone, Debug)]
pub enum KeyStatus {
    Pressed,
    Released,
}

pub struct Keys {
    pub(crate) one: (KeyStatus, KeyStatus),
    pub(crate) two: (KeyStatus, KeyStatus),
    pub(crate) three: (KeyStatus, KeyStatus),
    pub(crate) four: (KeyStatus, KeyStatus),
    pub(crate) a: (KeyStatus, KeyStatus),
    pub(crate) c: (KeyStatus, KeyStatus),
    pub(crate) d: (KeyStatus, KeyStatus),
    pub(crate) e: (KeyStatus, KeyStatus),
    pub(crate) f: (KeyStatus, KeyStatus),
    pub(crate) n: (KeyStatus, KeyStatus),
    pub(crate) o: (KeyStatus, KeyStatus),
    pub(crate) p: (KeyStatus, KeyStatus),
    pub(crate) q: (KeyStatus, KeyStatus),
    pub(crate) r: (KeyStatus, KeyStatus),
    pub(crate) s: (KeyStatus, KeyStatus),
    pub(crate) v: (KeyStatus, KeyStatus),
    pub(crate) w: (KeyStatus, KeyStatus),
    pub(crate) x: (KeyStatus, KeyStatus),
    pub(crate) z: (KeyStatus, KeyStatus),
    pub(crate) space: (KeyStatus, KeyStatus),
}

impl Keys {
    pub fn new() -> Self {
        Self {
            one: (KeyStatus::Released, KeyStatus::Released),
            two: (KeyStatus::Released, KeyStatus::Released),
            three: (KeyStatus::Released, KeyStatus::Released),
            four: (KeyStatus::Released, KeyStatus::Released),
            a: (KeyStatus::Released, KeyStatus::Released),
            c: (KeyStatus::Released, KeyStatus::Released),
            d: (KeyStatus::Released, KeyStatus::Released),
            e: (KeyStatus::Released, KeyStatus::Released),
            f: (KeyStatus::Released, KeyStatus::Released),
            n: (KeyStatus::Released, KeyStatus::Released),
            o: (KeyStatus::Released, KeyStatus::Released),
            p: (KeyStatus::Released, KeyStatus::Released),
            q: (KeyStatus::Released, KeyStatus::Released),
            r: (KeyStatus::Released, KeyStatus::Released),
            s: (KeyStatus::Released, KeyStatus::Released),
            v: (KeyStatus::Released, KeyStatus::Released),
            w: (KeyStatus::Released, KeyStatus::Released),
            x: (KeyStatus::Released, KeyStatus::Released),
            z: (KeyStatus::Released, KeyStatus::Released),
            space: (KeyStatus::Released, KeyStatus::Released),
        }
    }

    pub fn update_last_key_states(&mut self) {
        self.one.1 = self.one.0.clone();
        self.two.1 = self.two.0.clone();
        self.three.1 = self.three.0.clone();
        self.four.1 = self.four.0.clone();
        self.a.1 = self.a.0.clone();
        self.c.1 = self.c.0.clone();
        self.d.1 = self.d.0.clone();
        self.e.1 = self.e.0.clone();
        self.f.1 = self.f.0.clone();
        self.n.1 = self.n.0.clone();
        self.o.1 = self.o.0.clone();
        self.p.1 = self.p.0.clone();
        self.q.1 = self.q.0.clone();
        self.r.1 = self.r.0.clone();
        self.s.1 = self.s.0.clone();
        self.v.1 = self.v.0.clone();
        self.w.1 = self.w.0.clone();
        self.x.1 = self.x.0.clone();
        self.z.1 = self.z.0.clone();
        self.space.1 = self.space.0.clone();
    }

    pub fn get_key_status(&self, key: &str) -> Option<(KeyStatus, KeyStatus)> {
        match key {
            "1" => Some(self.one.clone()),
            "2" => Some(self.two.clone()),
            "3" => Some(self.three.clone()),
            "4" => Some(self.four.clone()),
            "a" => Some(self.a.clone()),
            "c" => Some(self.c.clone()),
            "d" => Some(self.d.clone()),
            "e" => Some(self.e.clone()),
            "f" => Some(self.f.clone()),
            "n" => Some(self.n.clone()),
            "o" => Some(self.o.clone()),
            "p" => Some(self.p.clone()),
            "q" => Some(self.q.clone()),
            "r" => Some(self.r.clone()),
            "s" => Some(self.s.clone()),
            "v" => Some(self.v.clone()),
            "w" => Some(self.w.clone()),
            "x" => Some(self.x.clone()),
            "z" => Some(self.z.clone()),
            " " => Some(self.space.clone()),
            _ => None,
        }
    }

    pub fn set_key_state(&mut self, key: &str, state: KeyStatus) {
        match key {
            "1" => self.one.0 = state,
            "2" => self.two.0 = state,
            "3" => self.three.0 = state,
            "4" => self.four.0 = state,
            "a" => self.a.0 = state,
            "c" => self.c.0 = state,
            "d" => self.d.0 = state,
            "e" => self.e.0 = state,
            "f" => self.f.0 = state,
            "n" => self.n.0 = state,
            "o" => self.o.0 = state,
            "p" => self.p.0 = state,
            "q" => self.q.0 = state,
            "r" => self.r.0 = state,
            "s" => self.s.0 = state,
            "v" => self.v.0 = state,
            "w" => self.w.0 = state,
            "x" => self.x.0 = state,
            "z" => self.z.0 = state,
            " " => self.space.0 = state,
            _ => (),
        }
    }
}