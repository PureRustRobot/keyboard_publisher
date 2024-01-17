use async_std;
use zenoh::Error;

use keyboard_publisher::wasd_controller;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(wasd_controller("wasd_cmd_publisher", "cmd_vel",  1.0));

    task.await?;

    Ok(())
}