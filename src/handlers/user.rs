use crate::{
    model::{PartialUser, Settable, User},
    redis_helper::{redis_add, redis_get_slice, redis_delete}
};
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{Error as AWError, HttpRequest, HttpResponse, web};
use futures::future::{join_all, join};
use redis_async::{resp::RespValue, resp_array};

/// gets a list of users nickname from a list of ids...
/// it will be [[id, nickname]]
pub async fn from_ids(
    redis: web::Data<Addr<RedisActor>>,
    user_ids: web::Json<Vec<String>>,
) -> Result<HttpResponse, AWError> {
    let user_ids = user_ids.into_inner();

    let commands = user_ids.iter().map(|id| {
        let domain = format!("user:{}", id);
        redis.send(Command(resp_array!["GET", &domain]))
    });

    let res: Vec<Result<RespValue, AWError>> = join_all(commands.into_iter())
        .await
        .into_iter()
        .map(|item| {
            item.map_err(AWError::from)
                .and_then(|res| res.map_err(AWError::from))
        })
        .collect();

    let mut user_nicknames: Vec<(String, String)> = Vec::new();

    if !res.iter().all(|res| match res {
        Ok(RespValue::BulkString(x)) => {
            let user:User = serde_json::from_slice(x).expect("I should be able to deserialize");
            user_nicknames.push((user.id(), user.nickname));
            true
        }
        _ => false,
    }) {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().json(user_nicknames))
    }
}

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    user_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, AWError> {
    let user_id = user_id.into_inner();

    let domain = format!("user:{}", user_id);

    let get = redis.send(Command(resp_array!["GET", &domain])).await?;

    match get {
        Ok(RespValue::BulkString(x)) => Ok(HttpResponse::Ok().body(x)),
        _ => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn force_add(
    redis: web::Data<Addr<RedisActor>>,
    user: web::Json<PartialUser>,
) -> Result<HttpResponse, AWError> {
    let temp_code_domain = "temp_code:temp";
    let mut user: User = user.into_inner().into();
    user.is_verified = true;
    let access_token = "token";
    let access_token_domain = format!("access_token:{}", &access_token);

    let set_user = redis_add(user.to_owned(), &redis); 

    let set_at = redis.send(Command(resp_array!["SET", &access_token_domain, &user.id()]));
    let set_tc = redis.send(Command(resp_array!["SET", temp_code_domain, &user.id()]));

    let (user_add, _) = join(set_user, join(set_tc, set_at)).await;

    match user_add {
        true => Ok(HttpResponse::Ok().json((&user, access_token))),
        false => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn sign_up(
    redis: web::Data<Addr<RedisActor>>,
    p_user: web::Json<PartialUser>,
) -> Result<HttpResponse, AWError> {
    let p_user = p_user.into_inner();

    let temp_code = "temp";
    let email = &p_user.email.to_owned();
    let temp_code_domain = format!("temp_code:{}",&temp_code);

    let user: User = p_user.into();

    // we need to delete the previous temp code if this email was already
    // registered, this endpoint effectively works as 'change nickname'
    let get_current_user = redis.send(Command(resp_array!["GET", &user.domain()])).await?;

    if let Ok(RespValue::BulkString(x)) = get_current_user {
        let prev_user:User = serde_json::from_slice(&x).unwrap();
        let prev_partial_user = PartialUser{ 
            nickname: prev_user.nickname,
            email:email.to_string()
        };
        let prev_temp_code = "temp";
        let prev_temp_code_domain = format!("temp_code:{}", &prev_temp_code);
        let _del_prev = redis.send(Command(resp_array!["DEL", &prev_temp_code_domain])).await?;
    }
     
    let temp = redis.send(Command(resp_array!["SET", &temp_code_domain, &user.id()]));
    let expire = redis.send(Command(resp_array!["EXPIRE", &temp_code_domain, "1800"]));

    let user_add = redis_add(user, &redis);

    let redis = join(user_add, join(temp, expire));

    let (add, _tokens) = redis.await;

    match add {
        true => Ok(HttpResponse::Ok().body("email was sent with temp code")),
        false => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn verify_temp_code(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>, // user_id and temp
) -> Result<HttpResponse, AWError> {
    let (user_id, temp_code) = path.into_inner();

    let domain = format!("user:{}", user_id);
    let code_domain = format!("temp_code:{}", &temp_code);

    let get = redis.send(Command(resp_array!["GET", &domain]));
    let temp = redis.send(Command(resp_array!["GET",&code_domain]));

    let (user, uid_from_temp) = join(get, temp).await;

    let uid: String = match uid_from_temp? {
        Ok(RespValue::BulkString(x)) => String::from_utf8(x).unwrap(),
        _ => return Ok(HttpResponse::Unauthorized().finish())
    };

    let mut user: User = match user? {
        Ok(RespValue::BulkString(x)) => {
            serde_json::from_slice(&x).expect("user should be deserializable")
        }
        _ => return Ok(HttpResponse::InternalServerError().finish()),
    };

    if user.id() == uid {
        let access_token = "token";
        let token_domain = format!("access_token:{}",&access_token);
        let set_token = redis.send(Command(resp_array!["SET", &token_domain, &user.id()]));
        user.is_verified = true;

        let set = redis.send(Command(resp_array!["SET", &domain, user.json()]));

        let (set, _set_token) = join(set, set_token).await;

        match set? {
            Ok(RespValue::SimpleString(x)) if x == "OK" => Ok(HttpResponse::Ok().body(access_token)),
            _=> Ok(HttpResponse::InternalServerError().body("unable to set access token"))
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("invalid user_id, temp_code pair"))
    }
}

pub async fn verify_auth_code(
    redis: web::Data<Addr<RedisActor>>,
    user_id: web::Path<String>,
    req: HttpRequest,
    )-> Result<HttpResponse, AWError> {

    let user_id = user_id.into_inner();
    
    let is_valid = true;
    
    match is_valid {
        true => Ok(HttpResponse::Ok().body("yes")),
        false => Ok(HttpResponse::Unauthorized().finish())
    }
}
