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



fn test_read_syscall(loop_times: i32, fd: i32) -> anyhow::Result<()> {

    println!("test_read_syscall start, loop time is {}", loop_times);

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {

        let len = 1000;
        let mut buf: Vec<u8> = vec![0; len];

        let start = Instant::now();
        let res = unsafe { syscall!(Sysno::read, fd, buf.as_mut_ptr() as *const _, len) };
        if res.is_err() {
            println!("test_read_syscall got error {:?} loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_read_syscall got error {:?}, loop time {:?}", res, i)));
        }

        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_read_syscall_in_ns.push(period);

        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(())
}


fn test_write_syscall(loop_times: i32, fd: i32) -> anyhow::Result<()> {

    println!("test_write_syscall start, loop time is {}", loop_times);

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {

        let len = 1000;
        let mut buf: Vec<u8> = vec![0; len];

        let start = Instant::now();
        let res = unsafe { syscall!(Sysno::write, fd, buf.as_mut_ptr() as *const _, len) };
        if res.is_err() {
            println!("test_write_syscall got error {:?} loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_write_syscall got error {:?}, loop time {:?}", res, i)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_write_syscall_in_ns.push(period);

        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(())
}



fn test_ppid_syscall(loop_times: i32, fd: i32) -> anyhow::Result<()> {

    println!("test_ppid_syscall start, loop time is {}", loop_times);

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {

        let len = 1000;
        let mut buf: Vec<u8> = vec![0; len];

        let start = Instant::now();
        let res = unsafe { syscall!(Sysno::getppid) };
        if res.is_err() {
            println!("test_write_syscall got error {:?} loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_write_syscall got error {:?}, loop time {:?}", res, i)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_getppid_syscall_in_ns.push(period);

        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(())
}



fn test_getpid_syscall(loop_times: i32, fd: i32) -> anyhow::Result<()> {

    println!("test_getpid_syscall start, loop time is {}", loop_times);

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {

        let len = 1000;
        let mut buf: Vec<u8> = vec![0; len];

        let start = Instant::now();
        let res = unsafe { syscall!(Sysno::getpid) };
        if res.is_err() {
            println!("test_getpid_syscall got error {:?} loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_getpid_syscall got error {:?}, loop time {:?}", res, i)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_getpid_syscall_in_ns.push(period);

        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(())
}



#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct LibcStat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    pub pad: [i64; 3],
}


fn test_fstat_syscall(loop_times: i32, fd: i32) -> anyhow::Result<()> {

    println!("test_getpid_syscall start, loop time is {}", loop_times);

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {

        let mut s: LibcStat = LibcStat::default();

        let start = Instant::now();
        let res = unsafe { syscall!(Sysno::fstat, fd, &mut s as *mut _) };
        if res.is_err() {
            println!("test_fstat_syscall got error {:?} loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_fstat_syscall got error {:?}, loop time {:?}", res, i)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_fstat_syscall_in_ns.push(period);

        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(())
}

fn do_test(loop_time: i32) -> anyhow::Result<()>{


    let path = Path::new("quark.log");
    let display = path.display();
    
    let mut file = match File::options()
                                        .read(true)
                                            .write(true)
                                                .open(path){
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let fd = file.as_raw_fd();

    let res = test_read_syscall(loop_time, fd);
    if res.is_err() {
        println!("test_read_syscall got error {:?}", res);
        return Err(anyhow::Error::msg(format!("test_read_syscall got error {:?}", res)));
    }

    let res = test_write_syscall(loop_time, fd);
    if res.is_err() {
        println!("test_write_syscall got error {:?}", res);
        return Err(anyhow::Error::msg(format!("test_write_syscall got error {:?}", res)));
    }


    let res = test_ppid_syscall(loop_time, fd);
    if res.is_err() {
        println!("test_ppid_syscall got error {:?}", res);
        return Err(anyhow::Error::msg(format!("test_ppid_syscall got error {:?}", res)));
    }


    let res = test_getpid_syscall(loop_time, fd);
    if res.is_err() {
        println!("test_getpid_syscall got error {:?}", res);
        return Err(anyhow::Error::msg(format!("test_getpid_syscall got error {:?}", res)));
    }


    let res = test_fstat_syscall(loop_time, fd);
    if res.is_err() {
        println!("test_fstat_syscall got error {:?}", res);
        return Err(anyhow::Error::msg(format!("test_fstat_syscall got error {:?}", res)));
    }

    // let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();
    // println!("{:?}", *statistic_keeper);

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



    let test_fstat_syscall_in_ns = get_statistic(&statistic_keeper.test_fstat_syscall_in_ns).unwrap();

    let test_getpid_syscall_in_ns = get_statistic(&statistic_keeper.test_getpid_syscall_in_ns).unwrap();

    let test_getppid_syscall_in_ns = get_statistic(&statistic_keeper.test_getppid_syscall_in_ns).unwrap();

    let test_read_syscall_in_ns = get_statistic(&statistic_keeper.test_read_syscall_in_ns).unwrap();

    let test_write_syscall_in_ns = get_statistic(&statistic_keeper.test_write_syscall_in_ns).unwrap();



    println!("test_fstat_syscall_in_ns {:?}", test_fstat_syscall_in_ns);

    println!("test_getpid_syscall_in_ns {:?}", test_getpid_syscall_in_ns);

    println!("test_getppid_syscall_in_ns {:?}", test_getppid_syscall_in_ns);

    println!("test_read_syscall_in_ns {:?}", test_read_syscall_in_ns);

    println!("test_write_syscall_in_ns {:?}", test_write_syscall_in_ns);
  
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
