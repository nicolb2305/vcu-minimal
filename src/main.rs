use std::collections::HashMap;

use futures_util::StreamExt;
use tokio::sync::mpsc::{self, Sender};
use tokio_tungstenite::tungstenite::Message;
use vcu_minimal::{
    api::{
        api_types::{ChampSelectActorType, LolChampSelectChampSelectAction},
        ws_types::{SubscriptionType, WebSocketEvent, WebSocketResponse},
    },
    client::ApiClient,
    ws::WebSocketClient,
};

fn spawn_client_listener(mut client: WebSocketClient, tx: Sender<WebSocketResponse>) {
    let async_block = async move {
        while let Some(Ok(Message::Text(v))) = client.socket.next().await {
            if let Ok(val) = serde_json::from_str::<WebSocketEvent>(&v) {
                dbg!(&val);
                tx.send(val.data).await.unwrap()
            }
        }
    };
    tokio::spawn(async_block);
}

fn spawn_stdin_listener(tx: Sender<String>) {
    let async_block = async move {
        loop {
            let stdin = std::io::stdin();
            let mut line_buf = String::new();
            if stdin.read_line(&mut line_buf).is_ok() {
                let line = line_buf.trim_end().to_string();
                line_buf.clear();
                tx.send(line).await.unwrap();
            }
        }
    };
    tokio::spawn(async_block);
}

fn process_client_message(msg: WebSocketResponse) -> Vec<ChampSelectActorType> {
    match msg {
        WebSocketResponse::ChampSelect(val) => val.data.actions.into_iter().flatten().collect(),
        _ => unimplemented!(),
    }
}

async fn process_stdin_message(
    client: &ApiClient,
    mut msg: String,
    actions: &[ChampSelectActorType],
) {
    msg = msg
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let my_selection = client
        .get_lol_champ_select_v1_session_my_selection()
        .await
        .unwrap();
    let champions = client
        .get_lol_champ_select_v1_all_grid_champions()
        .await
        .unwrap()
        .into_iter()
        .map(|x| {
            (
                x.name
                    .to_lowercase()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect(),
                x.id,
            )
        })
        .collect::<HashMap<String, i32>>();

    if let Some(player) = actions.iter().find(|x| x.cell_id() == my_selection.cell_id) {
        let action = match msg.as_str() {
            "" => LolChampSelectChampSelectAction {
                id: player.id(),
                actor_cell_id: player.cell_id(),
                champion_id: player.champion_id(),
                type_: "pick".to_owned(),
                completed: true,
                is_ally_action: true,
            },
            k if champions.contains_key(k) => LolChampSelectChampSelectAction {
                id: player.id(),
                actor_cell_id: player.cell_id(),
                champion_id: *champions.get(&msg).unwrap(),
                type_: "pick".to_owned(),
                completed: false,
                is_ally_action: true,
            },
            _ => return,
        };

        client
            .patch_lol_champ_select_v1_session_actions_by_id(player.id(), action)
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let client = ApiClient::new().unwrap();

    let (tx_ws, mut rx_ws) = mpsc::channel(5);
    let (tx_stdin, mut rx_stdin) = mpsc::channel(5);

    let mut ws_client = WebSocketClient::new().await;
    ws_client
        .subscribe(SubscriptionType::LolChampSelectV1Session)
        .await
        .unwrap();

    spawn_client_listener(ws_client, tx_ws);
    spawn_stdin_listener(tx_stdin);

    let mut actions: Vec<ChampSelectActorType> = vec![];
    loop {
        tokio::select! {
            Some(msg) = rx_ws.recv() => {
                actions = process_client_message(msg);
                dbg!(&actions);
            },
            Some(champion_name) = rx_stdin.recv() => {
                process_stdin_message(&client, champion_name, &actions).await
            }
        }
    }
}
