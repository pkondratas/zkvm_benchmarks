# sudo docker run --rm -it --device nvidia.com/gpu=all nvidia/cuda:13.2.0-devel-ubuntu24.04   nvidia-smi
FROM nvidia/cuda:12.9.0-devel-ubuntu24.04

RUN apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config git protobuf-compiler libclang-dev time nano

RUN git clone --no-recurse-submodules https://github.com/pkondratas/zkvm_benchmarks.git && \
    cd zkvm_benchmarks && \
    git submodule update --init leanSig && \
    git checkout gpu-proving

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

RUN curl -L https://risczero.com/install | bash
ENV PATH="/root/.risc0/bin:$PATH"

RUN rzup install

RUN curl -L https://sp1up.succinct.xyz | bash
ENV PATH="/root/.sp1/bin:${PATH}"

RUN sp1up

WORKDIR /zkvm_benchmarks/

CMD ["/bin/bash"]