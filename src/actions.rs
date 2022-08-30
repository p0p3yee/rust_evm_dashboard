use crate::models::{NewAccount, Account, Endpoint, NewEndpoint, NewSetting, Setting};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::apierror::ApiError;

fn parse_diesel_error (err: DieselError) -> ApiError {
    match err {
        DieselError::InvalidCString(_) => ApiError::InvalidData,
        DieselError::DatabaseError(kind, info) => match kind {
            diesel::result::DatabaseErrorKind::UniqueViolation => ApiError::AlreadyExists,
            kind => {
                println!("Unknown database error kind: {:?} : {:?}", kind, info.message());
                ApiError::DatabaseInternalError
            },
        },
        DieselError::NotFound => ApiError::TargetNotFound,
        DieselError::QueryBuilderError(_) => ApiError::NoUpdateRequired,
        e => ApiError::Error(e.to_string()),
    }
}

pub async fn inititialize_setting(conn: &SqliteConnection, id: i32) -> Result<i32, ApiError> {
    use crate::schema::settings;

    let new_setting = NewSetting {
        selected_endpoint_id: id
    };

    let result = diesel::insert_into(settings::table)
        .values(&new_setting)
        .execute(conn);
    
    match result {
        Ok(_) => Ok(id),
        Err(e) => Err(parse_diesel_error(e)),
    }
}

// Return account address if success
pub async fn create_account<'a>(conn: &SqliteConnection, addr: &'a str, name: &'a str, pkey: &'a str) -> Result<String, ApiError> {
    use crate::schema::accounts;

    let new_acc = NewAccount {
        address: addr,
        name: name,
        private_key: pkey,
    };

    let result = diesel::insert_into(accounts::table)
        .values(&new_acc)
        .execute(conn);
    
    match result {
        Ok(_) => Ok(addr.to_string()),
        Err(e) => Err(parse_diesel_error(e)),
    }
}

// Return endpoint name if success
pub async fn create_endpoint<'a>(conn: &SqliteConnection, ename: &'a str, eurl: &'a str, esymbol: &'a str, eexplorer_url: &'a str, echain_id: &'a str) -> Result<String, ApiError> {
    use crate::schema::endpoints::dsl::*;

    let new_endpoint = NewEndpoint {
        name: ename,
        url: eurl,
        symbol: esymbol,
        explorer_url: eexplorer_url,
        chain_id: echain_id
    };

    let exists_id = endpoints
        .filter(url.eq(eurl))
        .first::<Endpoint>(conn);

    if exists_id.is_ok() {
        return Err(ApiError::Error("URL already exists".to_string()))
    }

    let result = diesel::insert_into(endpoints)
        .values(&new_endpoint)
        .execute(conn);

    match result {
        Ok(_) => Ok(ename.to_string()),
        Err(e) => Err(parse_diesel_error(e)),
    }
}

pub async fn update_current_setting(conn: &SqliteConnection, from_id: i32, to_id: i32) -> Result<i32, ApiError> {
    use crate::schema::settings::dsl::*;

    let result = diesel::update(settings.find(from_id))
        .set(selected_endpoint_id.eq(to_id))
        .execute(conn);

    match result {
        Ok(num) => {
            if num == 0 {
                return Err(ApiError::TargetNotFound)
            }
            Ok(to_id)
        },
        Err(e) => Err(parse_diesel_error(e))
    }
}

// Return new account name if success
pub async fn update_account_name(conn: &SqliteConnection, target_addr: &str, new_name: &str) -> Result<String, ApiError> {
    use crate::schema::accounts::dsl::*;

    let result = diesel::update(accounts.find(target_addr))
        .set(name.eq(new_name))
        .execute(conn);

    match result {
        Ok(num) => {
            if num == 0 {
                return Err(ApiError::TargetNotFound)
            }
            Ok(new_name.to_string())
        },
        Err(e) => Err(parse_diesel_error(e))
    }
}

// Return the new endpoint if success
pub async fn update_endpoint_data(conn: &SqliteConnection, target_id: i32, new_name: &str, new_url: &str, new_symbol: &str, new_chain_id: &str, new_explorer_url: Option<String>) -> Result<Endpoint, ApiError> {
    use crate::schema::endpoints::dsl::*;

    let exists_id = endpoints
        .filter(url.eq(new_url))
        .first::<Endpoint>(conn);

    if exists_id.is_ok() {
        return Err(ApiError::Error("URL already exists".to_string()))
    }

    let result = diesel::update(endpoints.find(target_id))
        .set((
            name.eq(new_name.to_string()),
            url.eq(new_url.to_string()),
            symbol.eq(new_symbol.to_string()),
        ))
        .execute(conn);
    
        match result {
            Ok(num) => {
                if num == 0 {
                    return Err(ApiError::TargetNotFound)
                }
                Ok(Endpoint {
                    id: target_id,
                    name: new_name.to_string(),
                    url: new_url.to_string(),
                    symbol: new_symbol.to_string(),
                    explorer_url: new_explorer_url,
                    chain_id: new_chain_id.to_string()
                })
            },
            Err(e) => Err(parse_diesel_error(e))
        }
}

pub async fn get_current_setting(conn: &SqliteConnection) -> Result<Vec<Setting>, ApiError> {
    use crate::schema::settings::dsl::settings;
    match settings.load::<Setting>(conn) {
        Ok(s) => Ok(s),
        Err(e) => Err(parse_diesel_error(e)),
    }
}

pub async fn get_all_accounts(conn: &SqliteConnection) -> Result<Vec<Account>, ApiError> {
    use crate::schema::accounts::dsl::accounts;
    match accounts.load::<Account>(conn) {
        Ok(acc) => Ok(acc),
        Err(e) => Err(parse_diesel_error(e)),
    }
}

pub async fn get_all_endpoints(conn: &SqliteConnection) -> Result<Vec<Endpoint>, ApiError> {
    use crate::schema::endpoints::dsl::endpoints;
    match endpoints.load::<Endpoint>(conn) {
        Ok(eps) => Ok(eps),
        Err(e) => Err(parse_diesel_error(e)),
    }
}