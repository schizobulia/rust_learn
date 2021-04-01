/// 比如我们在java中一个线程start之后并不会阻塞主线程
/// new Thread1().start();
/// new Thread2().start();
/// 两个线程会同时执行(不考虑当前系统可用线程数量)
///
/// 但在rust中我们如果希望多个任务同时进行的时候 目前好像都是使用第三方的包 例：futures
/// 以下简单的示例可能是一个方式
use std::thread;
use std::time::Duration;

//此处发现每一输出的顺序都不同
pub fn example() {
    let mut task = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            println!("{}", i);
        });
        task.push(handle);
    }

    loop {
        let t = task.pop().unwrap();
        t.join().unwrap();
        if task.len() == 0 {
            println!("end");
            return;
        }
    }
}