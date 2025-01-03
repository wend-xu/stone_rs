use crate::eval::eval::EvalRes;
use std::collections::HashMap;

pub trait Env {
    fn get(&mut self, key: &str) -> Result<EvalRes, String>;

    fn get_ref(&self, key: &str) ->  Result<&EvalRes, String>;

    fn put(&mut self, key: String, val: EvalRes) -> Result<(), &'static str>;

    fn free(&mut self, keys: &str) -> Result<(), &'static str>;
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
    fn get(&mut self, key: &str) -> Result<EvalRes, String> {
        self.env.get(key)
    }

    fn get_ref(&self, key: &str) -> Result<&EvalRes, String> {
        self.env.get_ref(key)
    }

    fn put(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env.put(key, val)
    }

    fn free(&mut self, keys: &str) -> Result<(), &'static str> {
        self.env.free(keys)
    }
}


pub struct MapEnv {
    env_map: HashMap<String, EvalRes>,
}

impl MapEnv {
    pub fn new() -> MapEnv {
        MapEnv {
            env_map: HashMap::new(),
        }
    }
}

impl Env for MapEnv {
    fn get(&mut self, key: &str) -> Result<EvalRes, String> {
        match self.env_map.remove(key) {
            None => { Err(format!("val undefined:[{}]", key)) }
            Some(eval_res) => { Ok(eval_res) }
        }
    }

    fn get_ref(&self, key: &str) -> Result<&EvalRes, String> {
        match self.env_map.get(key){
            None => { Err(format!("val undefined:[{}]", key)) }
            Some(eval_res) => { Ok(eval_res) }
        }
    }

    fn put(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env_map.insert(key, val);
        Ok(())
    }

    fn free(&mut self, key: &str) -> Result<(), &'static str> {
        self.env_map.remove(key);
        Ok(())
    }
}

