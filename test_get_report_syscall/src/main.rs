use syscalls::*;
use serde::*;
use anyhow::{Result, Ok};
use ssh_key;
use std::time::Instant;
use std::sync::Mutex;
use std::env;



const SAMPLE_DATA: &str = "
/// The supported TEE types:
/// - Tdx: TDX TEE.
/// - Sgx: SGX TEE.
/// - Sevsnp: SEV-SNP TEE.
/// - Sample: A dummy TEE that used to test/demo the KBC functionalities.
";

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref GLOBAL_STATISTICS_KEEPER: Mutex<SystemCallStatistic> = Mutex::new(SystemCallStatistic::default());
}


#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SnpAttestationReportSignature {
	pub r: Vec<u8>, // 72 bytes,
	pub s: Vec<u8>, //72 bytes,
	pub reserved: Vec<u8>,  // 368 bytes,
}

#[repr(C)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TcbVersion {
    pub boot_loader: u8,
    pub tee: u8,
    pub reserved: Vec<u8>,
    pub snp: u8,
    pub microcode: u8,
    pub raw: Vec<u8>,
}


#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AttestationReport {
	pub version: u32,		/* 0x000 */
	pub guest_svn: u32,	/* 0x004 */
	pub policy: u64,			/* 0x008 */
	pub family_id: Vec<u8>, /* 16 bytes, 0x010 */
	pub image_id: Vec<u8>, /*16 bytes, 0x020 */
	pub vmpl: u32,				/* 0x030 */
	pub signature_algo: u32,		/* 0x034 */
	pub platform_version: TcbVersion,  /* 0x038 */
	pub platform_info: u64,		/* 0x040 */
	pub flags: u32,			/* 0x048 */
	pub reserved0: u32,		/* 0x04C */
	pub report_data: Vec<u8>, /*64 bytes, 0x050 */
	pub measurement: Vec<u8>, 	/*48 bytes, 0x090 */
	pub host_data: Vec<u8>, /*32 bytes, 0x0C0 */
	pub id_key_digest: Vec<u8>, /*48 bytes, 0x0E0 */
	pub author_key_digest: Vec<u8>, /*48 bytes, 0x110 */
	pub report_id: Vec<u8>, /*32 bytes, 0x140 */
	pub report_id_ma: Vec<u8>, 	/*32 bytes, 0x160 */
	pub reported_tcb: TcbVersion,	/* 0x180 */
	pub reserved1: Vec<u8>, /*24 bytes, 0x188 */
	pub chip_id: Vec<u8>, /*64 bytes, 0x1A0 */
	pub reserved2: Vec<u8>, /*192 bytes, 0x1E0 */
	pub signature: SnpAttestationReportSignature  /* 0x2A0 */
}


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Requst {
    pub software_based_report_requered: bool,
    pub use_user_provided_signing_key: bool,
    pub signing_key_length: usize,
    //  AMD SNP REPORT has 1183 bytes, INTEL TDX report has 1024 bytes, so 4kb array should be enough to  hold the  Base64 encoded attestation tdx/snp report 
    pub signing_key: [u8; 4096],  
}

impl Default for Requst {
    fn default() -> Self {
        Self { 
            software_based_report_requered: false,
            use_user_provided_signing_key: false,
            signing_key_length: 0,
            signing_key: [0; 4096] 
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Report {
    //  AMD SNP: 2, TDX: 3, See https_attestation_provisioning_cli::Tee
    pub tee_type: u64,
    pub report_length: u64,
    //  AMD SNP REPORT has 1183 bytes, INTEL TDX report has 1024 bytes, so 4kb array should be enough to  hold the  Base64 encoded attestation tdx/snp report 
    pub report: [u8; 4096],  
}

impl Default for Report {
    fn default() -> Self {
        let default_type = Tee::Sample;
        Self { tee_type: default_type as u64,
            report_length: 0,
            report: [0; 4096] 
        }
    }
}

/// The supported TEE types:
/// - Tdx: TDX TEE.
/// - Sgx: SGX TEE.
/// - Sevsnp: SEV-SNP TEE.
/// - Sample: A dummy TEE that used to test/demo the KBC functionalities.
#[derive(Debug, Clone, Copy)]
pub enum Tee {
    Sev,
    Sgx,
    Snp,
    Tdx,

    // This value is only used for testing an attestation server, and should not
    // be used in an actual attestation scenario.
    Sample,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[repr(C)]
pub struct SoftwareBasedAttestationReport {
    signature: Vec<u8>,
    software_measurement: Vec<u8>,
    user_data_hash: Vec<u8>,
}



#[derive(Debug, Default)]
pub struct SystemCallStatistic {
    pub test_get_hardware_report_in_ns: Vec<f64>,
    pub test_get_software_report_signed_by_kbs_in_ns: Vec<f64>,
    pub test_get_software_report_signed_by_user_provided_key_in_ns: Vec<f64>
}



fn test_get_hardware_report(loop_times: i32) -> Result<()> {

    println!("test_get_hardware_report start, loop time is {}", loop_times);

    // Test hardware report
    let mut hardware_report = Report::default();
    let hardware_report_req = Requst {
        software_based_report_requered: false,
        ..Default::default()
    };
    let user_data = SAMPLE_DATA.as_bytes().to_vec();

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();

    // println!("test_get_hardware_report, user_data_addr {:?}, user_data_len {:?}, hardware_report_req {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  hardware_report_req);
    loop {

        let start = Instant::now();
        let res = unsafe { syscall !(Sysno::syscall_num_get_report, user_data.as_ptr() as *const _, user_data.len() as u64, & hardware_report_req as *const _,  &mut hardware_report as *mut _) };
        if res.is_err() {
            println!("test_get_hardware_report got error {:?} loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_get_hardware_report got error {:?}, loop time {:?}", res, i)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_get_hardware_report_in_ns.push(period);

        let raw_report = &hardware_report.report[..hardware_report.report_length as usize];
    
        let _: AttestationReport = serde_json::from_slice(raw_report)
            .map_err(|e| anyhow::Error::msg(format!("test_get_hardware_report, serde_json::from_slice(raw_report) failed {:?}, looped time {:?}", e, i)))?;
        // println!("test_get_hardware_report report {:?}", report);

        i = i + 1;
        if i > loop_times {
            break;
        }
        
    }



    Ok(())

}


fn test_get_software_report_signed_by_kbs(loop_times: i32) -> Result<()> {

    println!("test_get_software_report_signed_by_kbs start loop time is {}", loop_times);

    let mut soft_report = Report::default();
    let software_report_req = Requst {
        software_based_report_requered: true,
        ..Default::default()
    };
    let user_data = SAMPLE_DATA.as_bytes().to_vec();


    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();
    loop {

        let start = Instant::now();

        let res = unsafe { syscall !(Sysno::syscall_num_get_report, user_data.as_ptr() as *const _, user_data.len() as u64, & software_report_req as *const _,  &mut soft_report as *mut _) };
        if res.is_err() {
            println!("test_get_software_report_signed_by_kbs got error {:?}, loop time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_get_software_report_signed_by_kbs got error {:?}, looped time {:?}", res, i)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_get_software_report_signed_by_kbs_in_ns.push(period);


        let raw_report = &soft_report.report[..soft_report.report_length as usize];
        let _: SoftwareBasedAttestationReport = serde_json::from_slice(raw_report)
            .map_err(|e| anyhow::Error::msg(format!("test_get_software_report_signed_by_kbs, serde_json::from_slice(raw_report) failed {:?}, looped time {:?}", e, i)))?;
    
    
        // println!("test_get_software_report_signed_by_kbs report {:?}", report);
        i = i + 1;
        if i > loop_times {
            break;
        }
    }


    Ok(())

}


fn prepare_signing_key() -> Result<Vec<u8>> {

    println!("prepare_signing_key start");

    let comment = "the private key generated by kbs".to_string();
    let curve =  ssh_key::EcdsaCurve::NistP256;
    let mut rng = ssh_key::rand_core::OsRng;
    let key_pair =  ssh_key::private::EcdsaKeypair::random(&mut rng, curve).unwrap();

    let key_data = ssh_key::private::KeypairData::Ecdsa(key_pair);
    let private_key = ssh_key::PrivateKey::new(key_data, comment).unwrap();
    let private_key_to_byte = private_key.to_bytes().unwrap();

    let private_key_pem = private_key.to_openssh(ssh_key::LineEnding::LF).unwrap();
    println!("prepare_signing_key generated private_key {}", &*private_key_pem);

    Ok((&*private_key_to_byte).clone())
}


fn test_get_software_report_signed_by_user_provided_key(loop_times: i32) -> Result<()> {

    println!("test_get_software_report_signed_by_user_provided_key start, loop time is {}", loop_times);

    let signing_key = prepare_signing_key()
        .map_err(|e| anyhow::Error::msg(format!("test_get_software_report_signed_by_user_provided_key, prepare_signing_key() failed {:?}", e)))?;

    assert!(signing_key.len() < 4096);

    let mut soft_report = Report::default();
    let mut software_report_req = Requst::default();
    software_report_req.software_based_report_requered = true;
    software_report_req.signing_key_length = signing_key.len();
    software_report_req.use_user_provided_signing_key = true;
    software_report_req.signing_key[..signing_key.len()].clone_from_slice(&signing_key);

    let user_data = SAMPLE_DATA.as_bytes().to_vec();

    let mut i = 0;
    let mut statistic_keeper = GLOBAL_STATISTICS_KEEPER.lock().unwrap();
    loop {

        let start = Instant::now();

        let res = unsafe { syscall !(Sysno::syscall_num_get_report, user_data.as_ptr() as *const _, user_data.len() as u64, & software_report_req as *const _,  &mut soft_report as *mut _) };
        if res.is_err() {
            println!("test_get_software_report_signed_by_user_provided_key got error {:?}, looped time {:?}", res, i);
            return Err(anyhow::Error::msg(format!("test_get_software_report_signed_by_user_provided_key got error {:?}", res)));
        }
        let end = Instant::now();
        let period = (end - start).as_nanos() as f64;

        statistic_keeper.test_get_software_report_signed_by_user_provided_key_in_ns.push(period);


        let raw_report = &soft_report.report[..soft_report.report_length as usize];
        let _: SoftwareBasedAttestationReport = serde_json::from_slice(raw_report)
            .map_err(|e| anyhow::Error::msg(format!("test_get_software_report_signed_by_user_provided_key, serde_json::from_slice(raw_report) failed {:?} looped time {:?}", e, i)))?;
    
    
        // println!("test_get_software_report_signed_by_kbs report {:?}", report);
        i = i + 1;
        if i > loop_times {
            break;
        }
    }

    Ok(())

}



fn get_report(loop_time: i32) -> Result<()>{

    let res = test_get_hardware_report(loop_time);
    if res.is_err() {
        println!("get_report got error {:?}", res);
        return Err(anyhow::Error::msg(format!("get_report got error {:?}", res)));
    }



    let res = test_get_software_report_signed_by_kbs(loop_time);
    if res.is_err() {
        println!("get_report got error {:?}", res);
        return Err(anyhow::Error::msg(format!("get_report got error {:?}", res)));
    }


    let res = test_get_software_report_signed_by_user_provided_key(loop_time);
    if res.is_err() {
        println!("get_report got error {:?}", res);
        return Err(anyhow::Error::msg(format!("get_report got error {:?}", res)));
    }



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



    let test_get_hardware_report_in_ns = get_statistic(&statistic_keeper.test_get_hardware_report_in_ns).unwrap();

    let test_get_software_report_signed_by_kbs_in_ns = get_statistic(&statistic_keeper.test_get_software_report_signed_by_kbs_in_ns).unwrap();

    let test_get_software_report_signed_by_user_provided_key_in_ns = get_statistic(&statistic_keeper.test_get_software_report_signed_by_user_provided_key_in_ns).unwrap();



    println!("test_get_hardware_report_in_ns {:?}", test_get_hardware_report_in_ns);

    println!("test_get_software_report_signed_by_kbs_in_ns {:?}", test_get_software_report_signed_by_kbs_in_ns);

    println!("test_get_software_report_signed_by_user_provided_key_in_ns {:?}", test_get_software_report_signed_by_user_provided_key_in_ns);
  
    Ok(())
}


fn main() {
    println!("main start");


    let args: Vec<String> = env::args().collect();

    let loop_time = args[1].parse::<i32>().unwrap();



    let res = get_report(loop_time);
    if res.is_err() {
        println!("main got error {:?}", res);
    }


    println!("3 test passed");


    let res = unsafe { syscall !(Sysno::pause) };
    if res.is_err() {
        println!("main pause got error {:?}", res);
    }
}
