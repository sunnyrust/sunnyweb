use serde_json::json;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpMessage{
    pub title : String, //标题
    pub status : bool, //true表示成功，false表示失败
    pub wait : i32, //等待时间
    pub message: String, //提示信息
    pub url : String, //跳转地址
    pub platform_token: String, //平台token
}
impl Default for JumpMessage {
    fn default() -> Self {
        JumpMessage {
            title: "Message".to_string(),
            status: true,
            wait: 3,
            message: "OK".to_string(),
            url: "".to_string(),
            platform_token: "".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunnyMsgStruct { 
    pub status:bool, 
    pub msg:String,
    pub data:serde_json::Value  //json结构
 }

 pub fn sunny_msg(status:bool, msg:String, data:serde_json::Value) -> SunnyMsgStruct {
    SunnyMsgStruct {
        status,
        msg,
        data
    }
}

 pub fn sunny_msg_json(status:bool, msg:String, data:serde_json::Value) -> serde_json::Value {
    json!({
        "status": status,
        "msg": msg,
        "data": data
    })
 }

 pub fn sunny_msg_str(status:bool, msg:String, data:String) -> serde_json::Value {
    json!({
        "status": status,
        "msg": msg,
        "data": data
    })
 }

