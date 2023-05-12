use syscalls::*;
use std::time::Instant;
use std::sync::Mutex;
use std::env;
use std::os::unix::io::AsRawFd; 
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::result::Result::Ok;

use std::fs;





#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref GLOBAL_STATISTICS_KEEPER: Mutex<SystemCallStatistic> = Mutex::new(SystemCallStatistic::default());
}








#[derive(Debug, Default)]
pub struct SystemCallStatistic {
    pub test_read_syscall_in_ns: Vec<f64>,
    pub test_write_syscall_in_ns: Vec<f64>,
    pub test_getppid_syscall_in_ns: Vec<f64>,
    pub test_getpid_syscall_in_ns: Vec<f64>,
    pub test_fstat_syscall_in_ns: Vec<f64>
}



fn test_read_syscall(loop_times: i32) -> anyhow::Result<Vec<u8>> {

    println!("test_read_syscall start, loop time is {}", loop_times);

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    let len = 50;
    let  buf: Vec<u8> = vec![0; len];
    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {
        let mut buf: Vec<u8> = vec![0; len];
        let start = Instant::now();
        let path = Path::new("/secret/root-ca.pem");
        let display = path.display();
        
        {
            let file = match File::options()
            .read(true)
                .open(path){
                    // The `description` method of `io::Error` returns a string that
                    // describes the error
                    Err(why) => panic!("couldn't open {}: {}", display, why),
                    Ok(file) => file,
            };
            let fd = file.as_raw_fd();
            let res = unsafe { syscall!(Sysno::read, fd, buf.as_mut_ptr() as *const _, len) };
            if res.is_err() {
                println!("test_read_syscall got error {:?} loop time {:?}", res, i);
                return Err(anyhow::Error::msg(format!("test_read_syscall got error {:?}, loop time {:?}", res, i)));
            }
        }

        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_read_syscall_in_ns.push(period);

        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(buf)
}


fn do_test(loop_time: i32) -> anyhow::Result<()>{

    let res = test_read_syscall(loop_time);
    if res.is_err() {
        println!("test_read_syscall got error {:?}", res);
        return Err(anyhow::Error::msg(format!("test_read_syscall got error {:?}", res)));
    }

    let buf = res.unwrap();

    println!("read content {:?}", String::from_utf8_lossy(&buf));
    calcaulate_statistic_result().unwrap();
    Ok(())
}



use statrs::distribution::Uniform;
use statrs::statistics::Median;
use statrs::statistics::Statistics;

#[derive(Debug, Default)]
struct OurStatistic {
    mean: f64,
    min: f64,
    max: f64,
    std_deviation: f64,
    median : f64
}



fn get_statistic(data: &[f64]) -> anyhow::Result<OurStatistic> {



    // println!("data {:?}", data);
    let n = Uniform::new(data.min(), data.max()).unwrap();

    let s = OurStatistic {
        std_deviation: data.std_dev(),
        mean: data.mean(),
        min: data.min(),
        max: data.max(),
        median: n.median(), 
    };

    Ok(s)
}


fn calcaulate_statistic_result() -> anyhow::Result<()> {

    let statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();


    let test_read_syscall_in_ns = get_statistic(&statistic_keeper.test_read_syscall_in_ns).unwrap();


    println!("test_read_syscall_in_ns {:?}", test_read_syscall_in_ns);



    println!("data set {:?}", statistic_keeper.test_read_syscall_in_ns);

    Ok(())
}


fn main() {
    println!("main start");


    let args: Vec<String> = env::args().collect();

    let loop_time = args[1].parse::<i32>().unwrap();



    let res = do_test(loop_time);
    if res.is_err() {
        println!("main got error {:?}", res);
    }


    println!("test passed");


    let res = unsafe { syscall !(Sysno::pause) };
    if res.is_err() {
        println!("main pause got error {:?}", res);
    }
}
