docker run -p 8443:8443 -v /root/green-element-chain-rs/rust-sgx-sdk:/root/sgx -v /root/sgx/start_aesm_service.sh:/root/start_aesm_service.sh -ti --device /dev/isgx wenbin/rust-0.9.6
