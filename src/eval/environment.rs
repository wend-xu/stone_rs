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

    // fn set_outer(&mut self, outer: &'static mut EnvWrapper) -> Result<(), &'static str>;
}

pub struct EnvWrapper<'wrapper> {
    env: Box<dyn Env+'wrapper>,
}

impl<'wrapper> EnvWrapper<'wrapper> {
    pub fn new() -> EnvWrapper<'wrapper> {
        EnvWrapper {
            env: Box::new(MapEnv::new())
        }
    }

    pub fn new_nest() -> EnvWrapper<'wrapper> {
        EnvWrapper::new_with(Box::new(MapEnv::new()))
    }

    pub fn new_with(env: Box<dyn Env+'wrapper>) -> EnvWrapper<'wrapper> {
        EnvWrapper { env }
    }
}

impl<'wrapper> Env for EnvWrapper<'wrapper>{
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

pub struct MapNestedEnv<'outer,'env> {
    env_map: HashMap<String, EvalRes>,
    outer: Option<&'outer mut EnvWrapper<'env>>,
}

impl<'outer,'env> MapNestedEnv<'outer,'env> {
    pub fn new() -> MapNestedEnv<'outer,'env> {
        MapNestedEnv::<'outer,'env> {
            env_map: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_with(outer: &'outer mut EnvWrapper<'env>) -> MapNestedEnv<'outer,'env> {
        MapNestedEnv::<'outer,'env> {
            env_map: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn wrapper(self) -> EnvWrapper<'outer> {
        EnvWrapper::new_with(Box::new(self))
    }

    pub fn set_outer(&mut self, outer: &'outer mut EnvWrapper<'env>) -> &mut MapNestedEnv<'outer,'env> {
        self.outer = Some(outer);
        self
    }
}




impl<'outer,'env> Env for MapNestedEnv<'outer,'env> {
    fn get(&mut self, key: &str) -> Result<EvalRes, String> {
        let mut value_op = self.env_map.remove(key);
        if value_op.is_none() && self.outer.is_some() {
            self.outer.as_mut().unwrap().get(key)
        } else { Ok(value_op.unwrap()) }
    }

    fn get_ref(&self, key: &str) -> Result<&EvalRes, String> {
        let value_ref_op = self.env_map.get(key);
        if value_ref_op.is_none() && self.outer.is_some() {
            self.outer.as_ref().unwrap().get_ref(key)
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
        if self.env_map.contains_key(key) { Some(Box::new(self)) } else if self.outer.is_some() { self.outer.as_mut().unwrap().where_env(key) } else { None }
    }

    fn put_new(&mut self, key: String, val: EvalRes) -> Result<(), &'static str> {
        self.env_map.insert(key, val);
        Ok(())
    }

}
