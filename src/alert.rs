use log::error;
use notify_rust::Notification;

pub fn info(body: String) {
    match Notification::new()
        .appname("ARP Alert")
        .summary("Info")
        .body(&body)
        .show()
    {
        Ok(_) => (),
        Err(e) => error!("{e}"),
    }
}

pub fn alert(body: String) {
    match Notification::new()
        .appname("ARP Alert")
        .summary("Alert")
        .body(&body)
        .show()
    {
        Ok(_) => (),
        Err(e) => error!("{e}"),
    }
}
