use enterprise_architect_freetrial as ea;

fn main() {
    ea::delete_reg();
    match ea::delete_dat_file() {
        Err(err) => {
            let msg = format!("删除文件失败。{:?}", err);
            ea::message_box("错误", &msg.as_str());
            return;
        }
        _ => {}
    }
    ea::message_box("提示", "处理成功");
}
