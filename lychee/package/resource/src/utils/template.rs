
use tera::{Value, Function, Result as TeraResult};
use std::collections::HashMap;
// 实现 CheckIsHref 函数
pub struct CheckIsHrefFunction;

impl Function for CheckIsHrefFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        // 获取参数
        let url = match args.get("url") {
            Some(val) => val.as_str().unwrap_or(""),
            None => return Err("Missing 'url' parameter".into()),
        };
        
        let target = match args.get("target") {
            Some(val) => val.as_str().unwrap_or(""),
            None => return Err("Missing 'target' parameter".into()),
        };
        
        let is_power = match args.get("is_power") {
            Some(val) => val.as_bool().unwrap_or(false),
            None => return Err("Missing 'is_power' parameter".into()),
        };
        
        // 实现逻辑
        if !is_power {
            Ok(Value::String("".to_string()))
        } else {
            let link = format!("<a href='{}'>{}</a>", url, target);
            Ok(Value::String(link))
        }
    }
}

// 实现 CheckPower 函数
pub struct CheckPowerFunction;

impl Function for CheckPowerFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        // 获取参数
        let controller = match args.get("controller") {
            Some(val) => val.as_str().ok_or("'controller' must be a string")?,
            None => return Err("Missing 'controller' parameter".into()),
        };
        
        let action = match args.get("action") {
            Some(val) => val.as_str().ok_or("'action' must be a string")?,
            None => return Err("Missing 'action' parameter".into()),
        };
        
        let id = match args.get("id") {
            Some(val) => val.as_i64().ok_or("'id' must be an integer")?,
            None => return Err("Missing 'id' parameter".into()),
        };
        
        // 实现逻辑
        if id == 1 { // admin 有超级权限
            Ok(Value::Bool(true))
        } else {
            // match models::get_editor_powers_by_id(id) {
            //     Ok(power) => {
            //         let result = power.get(controller)
            //             .map(|actions| actions.iter().any(|a| a == action))
            //             .unwrap_or(false);
                    
            //         Ok(Value::Bool(result))
            //     },
            //     Err(_) => Ok(Value::Bool(false)),
            // }
            Ok(Value::Bool(false))
        }
    }
}