use serde::{Serialize, Deserialize};
use redis::{ToRedisArgs, RedisWrite};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Galaxy {
    // #[serde(rename = "_id")]
    // id: mongodb::bson::oid::ObjectId,
    pub name: String,
    pub constellation: String,
}


impl ToRedisArgs for &Galaxy {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize Galaxy as string"))
    }
}

