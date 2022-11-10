use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::{AppContext, GetCandlesHttpAction};

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None);

    result.register_get_action(Arc::new(GetCandlesHttpAction::new(app.clone())));

    result
}
