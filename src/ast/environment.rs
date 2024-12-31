use crate::ast::eval::EvalRes;
use std::cell::RefCell;
use std::collections::HashMap;

pub trait Env {
    fn get(&self, key: &str) -> Result<EvalRes, String>;

    fn get_ref(&self, key: &str) -> Option<&EvalRes>;

    fn put(&self, key: String, val: EvalRes) -> Result<(), &'static str>;

    fn free(&self, keys: &str) -> Result<(), &'static str>;
}

pub struct EnvWrapper<E: Env = MapEnv> {
    env: E,
}

impl EnvWrapper {
    pub fn new() -> EnvWrapper {
        EnvWrapper {
            env: MapEnv::new()
        }
    }

    pub fn new_with<E: Env>(env: E) -> EnvWrapper<E> {
        EnvWrapper::<E> { env }
    }
}

impl Env for EnvWrapper {
    fn get(&self, key: &str) -> Result<EvalRes, String> {
        self.env.get(key)
    }

    fn get_ref(&self, key: &str) -> Option<&EvalRes> {
        self.env.get_ref(key)
    }

    fn put(&self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env.put(key, val)
    }

    fn free(&self, keys: &str) -> Result<(), &'static str> {
        self.env.free(keys)
    }
}


pub struct MapEnv {
    env_map: RefCell<HashMap<String, EvalRes>>,
}

impl MapEnv {
    pub fn new() -> MapEnv {
        MapEnv {
            env_map: RefCell::new(HashMap::new()),
        }
    }
}

impl Env for MapEnv {
    fn get(&self, key: &str) -> Result<EvalRes, String> {
        match self.env_map.borrow_mut().remove(key) {
            None => { Err(format!("val undefined:[{}]", key)) }
            Some(eval_res) => { Ok(eval_res) }
        }
    }

    fn get_ref(& self, key: & str) -> Option<&EvalRes> {
        // self.env_map.borrow().get(key)
        todo!()
    }

    fn put(&self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env_map.borrow_mut().insert(key, val);
        Ok(())
    }

    fn free(&self, key: &str) -> Result<(), &'static str> {
        self.env_map.borrow_mut().remove(key);
        Ok(())
    }
}

