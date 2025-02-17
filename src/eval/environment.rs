use crate::eval::eval::EvalRes;
use std::collections::HashMap;
use std::ops::Deref;

pub trait Env {
    fn get(&mut self, key: &str) -> Result<EvalRes, String>;

    fn get_ref(&self, key: &str) -> Result<&EvalRes, String>;

    fn put(&mut self, key: String, val: EvalRes) -> Result<(), &'static str>;

    fn free(&mut self, keys: &str) -> Result<(), &'static str>;

    fn where_env(&mut self, key: &str) -> Option<Box<&mut dyn Env>>;

    fn put_new(&mut self, key: String, val: EvalRes) -> Result<(), &'static str>;
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

    fn where_env(&mut self, key: &str) -> Option<Box<&mut dyn Env>> {
        self.env.where_env(key)
    }


    fn put_new(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env.put_new(key, val)
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
        match self.env_map.get(key) {
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

    fn where_env(&mut self, key: &str) -> Option<Box<&mut dyn Env>> {
        if self.env_map.contains_key(key) { Some(Box::new(self)) } else { None }
    }

    fn put_new(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env_map.insert(key, val);
        Ok(())
    }
}

pub struct MapNestedEnv {
    env_map: HashMap<String, EvalRes>,
    outer: MapEnv,
}

impl MapNestedEnv {
    fn new(outer: MapEnv) -> MapNestedEnv {
        MapNestedEnv {
            env_map: HashMap::new(),
            outer,
        }
    }

    fn set_outer(&mut self, outer: MapEnv) {
        self.outer = outer;
    }
}

impl Env for MapNestedEnv {
    fn get(&mut self, key: &str) -> Result<EvalRes, String> {
        let mut value_op = self.env_map.remove(key);
        if value_op.is_none() {
            self.outer.get(key)
        } else { Ok(value_op.unwrap()) }
    }

    fn get_ref(&self, key: &str) -> Result<&EvalRes, String> {
        let value_ref_op = self.env_map.get(key);
        if value_ref_op.is_none() {
            self.outer.get_ref(key)
        } else { Ok(value_ref_op.unwrap()) }
    }

    fn put(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        let env_op = self.where_env(&key);
        match env_op {
            None => {
                self.env_map.insert(key, val);
                Ok(())
            }
            Some(mut env_had) => {
                env_had.put(key, val)
            }
        }
    }

    // 此处不允许释放外部作用域的变量，是否需要呢？
    fn free(&mut self, key: &str) -> Result<(), &'static str> {
        self.env_map.remove(key);
        Ok(())
    }

    fn where_env(&mut self, key: &str) -> Option<Box<&mut dyn Env>> {
        if self.env_map.contains_key(key) { Some(Box::new(self)) } else { self.outer.where_env(key) }
    }

    fn put_new(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env_map.insert(key, val);
        Ok(())
    }
}

