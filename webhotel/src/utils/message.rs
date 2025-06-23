use serde_json::json;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JampSessage{
    pub title : String, //标题
    pub staus : bool, //true表示成功，false表示失败
    pub wait : i32, //等待时间
    pub message: String, //提示信息
    pub url : String, //跳转地址
    pub platform_token: String, //平台token
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunnyMsgStruct { //此处要使用Big Camel而不要使用Snake Camel
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

