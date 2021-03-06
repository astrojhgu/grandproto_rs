#![allow(clippy::identity_op)]
//use bitfield::*;

pub type Daq = Daq_<[u32; 2]>;
pub type Trig = Trig_<[u32; 4]>;
pub type Gps = Gps_<[u32; 1]>;
pub type Adc = Adc_<[u32; 1]>;
pub type IntReg = IntReg_<[u32; 11]>;
pub type Data = Data_<[u32; 5]>;
pub type Slc = Slc_<[u32; 16]>;
pub type RdIntReg = RdIntReg_<[u32; 13]>;
pub type Ack = Ack_<[u32; 2]>;

pub trait Decode
where
    Self: Sized,
{
    fn decode(_: &[u32]) -> Option<Self>;
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Daq_([u32]);
    impl Debug;
    u32;
    pub u8, daq_on, set_daq_on: 0, 0;//1
    pub u8, ant_on, set_ant_on: 3, 1;//3
    pub u8, rd_wr_plus, set_rd_wr_plus:4,4;//1
    pub u8, en_osc, set_en_osc:5,5;//1
    pub u8, cntrl_adc, set_cntrl_adc:7,6; //2
    pub u16,offst, set_offst:19,8; //12
    pub u8, enable_pd, set_enable_pd:22,20; //3
    pub u8, dis_lna, set_dis_lna:25,23;//3
    pub u8, le, set_le:31, 26; //6
    pub u8, att1, set_att1:32+6, 32+0; //7
    pub u8, att2, set_att2:32+13,32+7;//7
}

impl Decode for Daq {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 2];
        if data.len() < 2 {
            None
        } else {
            result.copy_from_slice(&data[..2]);
            Some(Daq_(result))
        }
    }
}

bitfield! {

    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Trig_([u32]);
    impl Debug;
    u32;
    pub u8, st, set_st:0,0;
    pub u8, trg_en, set_trg_en:6, 1;

    pub u16,cntrl_dac, set_cntrl_dac:23,8;

    pub u16,th1m, set_th1m:32+11, 32+0;
    pub u16,th1p, set_th1p:32+23, 32+12;


    pub u16,th2m, set_th2m:64+11, 64+0;
    pub u16,th2p, set_th2p:64+23, 64+12;


    pub u16,th3m, set_th3m:96+11, 96+0;
    pub u16,th3p, set_th3p:96+23, 96+12;
}

impl Decode for Trig {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 4];
        match data[0] & 0x80_00_00 {
            1 => {
                if data.is_empty() {
                    None
                } else {
                    result[0] = data[0];
                    Some(result)
                }
            }
            _ => {
                if data.len() < 4 {
                    None
                } else {
                    result.copy_from_slice(&data[..4]);
                    Some(result)
                }
            }
        }.map(Trig_)
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Gps_([u32]);
    impl Debug;
    u32;
    pub u8, rwb, set_rwb:0,0;//1
    pub u8, addr, set_addr:7,1;//7
    pub u8, wrd, set_wrd:13,8;//6
}

impl Decode for Gps {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 1];
        if data.is_empty() {
            None
        } else {
            result.copy_from_slice(&data[..1]);
            Some(Gps_(result))
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Adc_([u32]);
    impl Debug;
    u16;
    pub u16, reg_func, set_reg_func: 10,0;//16
    pub u8,  addr, set_addr: 15,11;
}

impl Decode for Adc {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 1];
        if data.is_empty() {
            None
        } else {
            result.copy_from_slice(&data[..1]);
            Some(Adc_(result))
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct IntReg_([u32]);
    impl Debug;
    u32;
    pub u8, write,set_write:0,0;

    pub u32,board_mac, set_board_mac: 32+31,32+0;
    pub u32,board_ip, set_board_ip:64+31, 64+0;

    pub u64,srv_mac1, set_srv_mac1: 96+47,96+0;
    pub u32,srv_ip1,set_srv_ip1:160+31, 160+0;

    pub u64,srv_mac2, set_srv_mac2: 192+47,192+0;
    pub u32,srv_ip2,set_srv_ip2:256+31, 256+0;

    pub u16, port1, set_port1:288+15,288+0;
    pub u16, port2, set_port2:320+15,320+0;
}

impl Decode for IntReg {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 11];
        if data.is_empty() {
            None
        } else {
            match data[0] & 1 {
                1 => {
                    if data.len() < 11 {
                        None
                    } else {
                        result.copy_from_slice(&data[..11]);
                        Some(result)
                    }
                }
                _ => Some(result),
            }
        }.map(IntReg_)
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Data_([u32]);
    impl Debug;
    u32;
    pub u32,ip, set_ip: 31, 0;
    pub u32,ts2, set_ts2:32+31, 32+0;
    pub u8, ts1trigger, set_ts1trigger:64+7,64+0;
    pub u8, ts1pps, set_ts1pps:64+15, 64+8;
    pub u16, sss, set_sss: 64+31,64+16;
    pub u32,event_count, set_event_count:96+31,96+0;
    pub u8, trig_pattern, set_trig_pattern:128+5, 128+0;
}

impl Decode for Data {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 5];
        if data.len() < 5 {
            None
        } else {
            result.copy_from_slice(&data[..5]);
            Some(Data_(result))
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Slc_([u32]);
    impl Debug;
    u32;
    pub ip, set_ip:31,0;
    pub u16,vpower1, set_vpower1:32+11,32+0;
    pub u16,vpower2, set_vpower2:32+23,32+12;
    pub u16,vpower3, set_vpower3:64+11,64+0;
    pub u16,vpower4, set_vpower4:64+23,64+12;
    pub u16,vpower5, set_vpower5:96+11,96+0;
    pub u16,vpower6, set_vpower6:96+23,96+12;

    pub u16,th1m, set_th1m:128+11, 128+0;
    pub u16,th1p, set_th1p:128+23, 128+12;
    pub u16,th2m, set_th2m:160+11, 160+0;
    pub u16,th2p, set_th2p:160+23, 160+12;
    pub u16,th3m, set_th3m:192+11, 192+0;
    pub u16,th3p, set_th3p:192+23, 192+12;

    pub u16,temp, set_temp:224+12, 224+0;

    pub total_trig_rate, set_total_rate:256+31, 256+0;
    pub ch1p_trig_rate, set_ch1p_trig_rate:288+31, 288+0;
    pub ch2p_trig_rate, set_ch2p_trig_rate:320+31, 320+0;
    pub ch3p_trig_rate, set_ch3p_trig_rate:352+31, 352+0;

    pub ch1m_trig_rate, set_ch1m_trig_rate:384+31, 384+0;
    pub ch2m_trig_rate, set_ch2m_trig_rate:416+31, 416+0;
    pub ch3m_trig_rate, set_ch3m_trig_rate:448+31, 448+0;

    pub max_coarse, set_max_coarse:480+31, 480+0;
}

impl Decode for Slc {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 16];
        if data.len() < 16 {
            None
        } else {
            result.copy_from_slice(&data[..16]);
            Some(Slc_(result))
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct RdIntReg_([u32]);
    impl Debug;
    u32;
    pub u32,ip, set_ip:31, 0;
    pub u32,board_mac, set_board_mac:32+31, 32+0;
    pub u32,board_ip, set_board_ip:64+31, 64+0;

    pub u64,srv_mac1, set_srv_mac1: 96+47,96+0;
    pub u32,srv_ip1,set_srv_ip1:160+31, 160+0;

    pub u64,srv_mac2, set_srv_mac2: 192+47,192+0;
    pub u32,srv_ip2,set_srv_ip2:256+31, 256+0;

    pub u16, port1, set_port1:288+15,288+0;
    pub u16, port2, set_port2:320+15,320+0;

    pub u64, serial, set_serial:352+63, 352+0;
}

impl Decode for RdIntReg {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 13];
        if data.len() < 13 {
            None
        } else {
            result.copy_from_slice(&data[..13]);
            Some(RdIntReg_(result))
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Ack_([u32]);
    impl Debug;
    u32;
    pub u32,ip, set_ip:31, 0;
    pub u16, msg_ack, set_msg_ack:32+15, 32+0;
}

impl Decode for Ack {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 2];
        if data.len() < 2 {
            None
        } else {
            result.copy_from_slice(&data[..2]);
            Some(Ack_(result))
        }
    }
}
