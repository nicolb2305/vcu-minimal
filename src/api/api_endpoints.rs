use reqwest::Method;

use crate::{api::api_types::*, client::ApiClient, client::ApiResult};

const NO_BODY: Option<&str> = None;

impl ApiClient {
    pub async fn get_lol_active_boosts_v1_active_boosts(
        &self,
    ) -> ApiResult<LolActiveBoostsActiveBoosts> {
        let url = "/lol-active-boosts/v1/active-boosts";
        self.request_deserialize(Method::GET, url, NO_BODY).await
    }

    pub async fn get_lol_chat_v1_friends(&self) -> ApiResult<Vec<LolChatFriendResource>> {
        let url = "/lol-chat/v1/friends";
        self.request_deserialize(Method::GET, url, NO_BODY).await
    }

    pub async fn patch_lol_champ_select_v1_session_actions_by_id(
        &self,
        id: i64,
        body: LolChampSelectChampSelectAction,
    ) -> ApiResult<()> {
        let url = format!("/lol-champ-select/v1/session/actions/{id}");
        self.request(Method::PATCH, &url, Some(body)).await
    }

    pub async fn get_lol_champ_select_v1_session_my_selection(
        &self,
    ) -> ApiResult<LolChampSelectChampSelectPlayerSelection> {
        let url = "/lol-champ-select/v1/session/my-selection";
        self.request_deserialize(Method::GET, url, NO_BODY).await
    }

    pub async fn get_lol_champ_select_v1_all_grid_champions(
        &self,
    ) -> ApiResult<Vec<LolChampSelectChampGridChampion>> {
        let url = "/lol-champ-select/v1/all-grid-champions";
        self.request_deserialize(Method::GET, url, NO_BODY).await
    }

    pub async fn post_riotclient_kill_ux(&self) -> ApiResult<()> {
        let url = "/riotclient/kill-ux";
        self.request(Method::POST, url, NO_BODY).await
    }

    pub async fn post_riotclient_launch_ux(&self) -> ApiResult<()> {
        let url = "/riotclient/launch-ux";
        self.request(Method::POST, url, NO_BODY).await
    }
}
