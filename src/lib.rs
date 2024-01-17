use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use prr_msgs::msg::*;
use zenoh_manage_utils::logger;
use async_std::{self, io::ReadExt};

pub async fn wasd_controller(
    node_name:&str, 
    pub_topic:&str,
    pub_power_rate:f32
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let mut stdin = async_std::io::stdin();
    let mut input = [0_u8];

    let msg = format!("Start pub:{}", publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop {
        let _ = stdin.read_exact(&mut input).await.unwrap();

        match input[0] {
            b'w'=>{
                let cmd = CmdVel{
                    x:0.0,
                    y:1.0*pub_power_rate,
                    rotation_power:0.0
                };

                logger::log_info(node_name, "Send message".to_string());

                publisher.put(serialize_cmdvel(&cmd)).res().await.unwrap();
            },
            b'a'=>{
                let cmd = CmdVel{
                    x:1.0*pub_power_rate,
                    y:0.0,
                    rotation_power:0.0
                };

                logger::log_info(node_name, "Send message".to_string());

                publisher.put(serialize_cmdvel(&cmd)).res().await.unwrap();
            },
            b's'=>{
                let cmd = CmdVel{
                    x:0.0,
                    y:-1.0*pub_power_rate,
                    rotation_power:0.0
                };

                logger::log_info(node_name, "Send message".to_string());

                publisher.put(serialize_cmdvel(&cmd)).res().await.unwrap();
            }
            b'd'=>{
                let cmd = CmdVel{
                    x:1.0*pub_power_rate,
                    y:0.0,
                    rotation_power:0.0
                };

                logger::log_info(node_name, "Send message".to_string());

                publisher.put(serialize_cmdvel(&cmd)).res().await.unwrap();
            }
            0 => (),
            _ => (),
        }
    }
}