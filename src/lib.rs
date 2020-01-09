use cqc::builder::Builder;
use cqc::Request;
use cqc::Response;
use cqc::builder::RemoteId;
use cqc::hdr::CmdOpt;
use cqc::hdr::MeasOut;
use cqc::XtraHdr;
use cqc::hdr::CommHdr;
use bincode::Config;
use std::net::TcpStream;

pub struct Cqc {
    builder: Builder,
    stream: TcpStream,
    coder: Config
}

impl Cqc {
    pub fn new(app_id: u16, host: &str, port: u16) -> Cqc {
        let builder = Builder::new(app_id);
        let stream = TcpStream::connect((host, port)).unwrap();
        let mut coder = bincode::config();
        coder.big_endian();
        Cqc { builder, stream, coder }
    }

    fn send_request(&self, request: &Request) {
        self.coder.serialize_into(&self.stream, request).expect("Could not send request");
    }

    fn recv_response(&self) -> Response {
        self.coder.deserialize_from(&self.stream).expect("Could not receive response")
    }

    pub fn create_qubit(&self, notify: bool) -> u16 {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }

        let request = self.builder.cmd_new(1337, options);
        self.send_request(&request);

        // expect NewOk + Extra Qubit Header
        let response = self.recv_response();
        assert!(response.cqc_hdr.msg_type.is_new_ok(), "Expected a NewOk response.");
        assert!(response.notify.is_qubit_hdr(), "Expected an Extra Qubit Header");
        
        if notify {
            self.wait_for_done();
        }

        response.notify.get_qubit_hdr().qubit_id
    }

    pub fn create_epr(&self, receiver: CommHdr, notify: bool) -> u16 {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }

        let request = self.builder.cmd_epr(1337, options, XtraHdr::Comm(receiver));
        self.send_request(&request);

        // expect EprOk + Entanglement Information Header
        let response = self.recv_response();
        assert!(response.cqc_hdr.msg_type.is_epr_ok(), "Expected an EprOk response.");
        assert!(response.notify.is_epr_hdr(), "Expected an Entanglement Information Header");
        
        if notify {
            self.wait_for_done();
        }

        response.notify.get_epr_hdr().qubit_hdr.qubit_id
    }

    pub fn recv_epr(&self, notify: bool) -> u16 {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }
        
        let request = self.builder.cmd_epr_recv(1337, options);
        self.send_request(&request);

        // expect EprOk + Entanglement Information Header
        let response = self.recv_response();
        assert!(response.cqc_hdr.msg_type.is_epr_ok(), "Expected an EprOk response.");
        assert!(response.notify.is_epr_hdr(), "Expected an Entanglement Information Header");
        
        if notify {
            self.wait_for_done();
        }

        response.notify.get_epr_hdr().qubit_hdr.qubit_id
    }

    pub fn measure_qubit(&self, qubit_id: u16, notify: bool) -> MeasOut {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }
        
        let request = self.builder.cmd_measure(qubit_id, options);
        self.send_request(&request);

        // expect MeasOut + Measurement Outcome Header
        let response = self.recv_response();
        assert!(response.cqc_hdr.msg_type.is_measout(), "Expected a MeasOut response.");
        assert!(response.notify.is_meas_out_hdr(), "Expected a Measurement Outcome Header");
        
        if notify {
            self.wait_for_done();
        }

        response.notify.get_meas_out_hdr().meas_out
    }

    pub fn send_qubit(&self, qubit_id: u16, remote_id: RemoteId, notify: bool) {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }
        
        let request = self.builder.cmd_send(qubit_id, options, remote_id);
        self.send_request(&request);

        if notify {
            self.wait_for_done();
        }
    }

    pub fn recv_qubit(&self, notify: bool) -> u16 {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }
        
        let request = self.builder.cmd_recv(1337, options);
        self.send_request(&request);

        // expect Recv + Extra Qubit Header
        let response = self.recv_response();
        assert!(response.cqc_hdr.msg_type.is_recv(), "Expected a Recv response.");
        assert!(response.notify.is_qubit_hdr(), "Expected an Extra Qubit Header");
        
        if notify {
            self.wait_for_done();
        }

        response.notify.get_qubit_hdr().qubit_id
    }

    pub fn apply_h(&self, qubit_id: u16, notify: bool) {
        let mut options = CmdOpt::empty();
        if notify {
            options = *options.set_notify();
        }
        
        let request = self.builder.cmd_h(qubit_id, options);
        self.send_request(&request);

        if notify {
            self.wait_for_done();
        }
    }

    fn wait_for_done(&self) {
        let response = self.recv_response();
        assert!(response.cqc_hdr.msg_type.is_done(), "Expected a Done response.");
    }
}