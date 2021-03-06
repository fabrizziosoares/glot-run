use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::user;
use crate::glot_run::datastore;



#[derive(Debug, serde::Deserialize)]
struct RequestBody {
    token: ascii::AsciiString,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {
    api::check_access_token(&config.api.admin_access_token, request)?;

    let req_body: RequestBody = api::read_json_body(request)?;
    let user = user::new(&req_body.token);

    let data_root = config.server.data_root.lock().unwrap();
    datastore::add_entry(&data_root.users_path(), &user.id.to_string(), &user)
        .map_err(handle_datastore_error)?;

    api::prepare_json_response(&user)
}

fn handle_datastore_error(err: datastore::AddError) -> api::ErrorResponse {

    api::ErrorResponse{
        status_code: 500,
        body: api::ErrorBody{
            error: "datastore".to_string(),
            message: err.to_string(),
        }
    }
}
