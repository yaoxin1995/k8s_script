
use syscalls::*;

const SAMPLE_DATA: &str = "
/// The supported TEE types:
/// - Tdx: TDX TEE.
/// - Sgx: SGX TEE.
/// - Sevsnp: SEV-SNP TEE.
/// - Sample: A dummy TEE that used to test/demo the KBC functionalities.
";

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

fn get_report() -> Result<usize, Errno>{
    let mut report_info = Report::default();
    
    print!("default report info {:?}", report_info);

    let user_data = SAMPLE_DATA.as_bytes().to_vec();

    {
        print!("get_report before, user_data_addr {:?}, user_data_len {:?}, report_info_adr {:?}", user_data.as_ptr() as *const _, user_data.len() as u64,  &mut report_info as *mut _);
    }
    
    let res = unsafe { syscall !(Sysno::syscall_num_get_report, user_data.as_ptr() as *const _, user_data.len() as u64, &mut report_info as *mut _) };

    if res.is_err() {

        print!("syscall_num_get_report got error {:?}", res);
        return res;
    }
    print!("get_report after {:?}", report_info);

    res    
}


fn main() {
    print!("main start");

    let res = get_report();

    print!("main get report {:?}", res);
}
