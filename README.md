# XMSS signature aggregation using general-purpose zkVMs

This is the repo of XMSS signature aggregation using [leanSig] library

## Repository organization

Repository is organized in the following structure:

```text
zkvm_benchmarks
├── .github/workflows                  <-- [Pipeline to build the image based on commits to cpu/* and gpu/* branchers]
├── common                             <-- [Utilities folder]
├── leanMultisig                       <-- [leanVM subrepository (not used for benchmarks)]
├── leanSig                            <-- [leanSig library subrepository]
├── results                            <-- [Compressed results]
├── terraform                          <-- [IaC to deploy CPU and GPU instances to Runpod]
└── zkvm
    ├── risc0
    │   ├── host-cpu                  <-- [RISC Zero CPU prover host environment]
    │   ├── host-gpu                  <-- [RISC Zero GPU prover host environment]
    │   └── methods
    │       └── guest                 <-- [RISC Zero guest program]
    │
    └── sp1
        ├── host-cpu                  <-- [SP1 Hypercube CPU prover host environment]
        ├── host-gpu                  <-- [SP1 Hypercube GPU prover host environment]
        └── program                   <-- [SP1 Hypercube guest program]
```

## Image building for CPU and GPU provers

To build the image for either CPU or GPU prover locally, run from root:

```bash
docker build -t tag -f Dockerfile.[cpu/gpu] .
```

Or create branch that starts with prefix 'cpu/' or 'gpu/' for either CPU or GPU host environments. To push image to the Docker Hub, DOCKERHUB_USERNAME and DOCKERHUB_TOKEN should be set in repository settings. 

## Terraform

First, execute in terraform/ directory:

```bash
terraform init
```

To execute the following terraform scripts, 'runpod_api_key' and optionally 'cpu_bench' are passed. By default, 'cpu_bench' is set to true.
To see resources to be deployed before actually doing so, run:

```bash
terraform plan -var="runpod_api_key=your_api_key" -var="cpu_bench=true"
```

To deploy, run:

```bash
terraform apply -var="runpod_api_key=your_api_key" -var="cpu_bench=true"
```

## General information

To run the benchmarks locally, use `bench_cpu.sh` and `bench_gpu.sh` scripts for CPU and GPU provers, respectively. Note, that sufficient resources are required to run the benchmarks locally. Furthermore, segment/shard may need to be adjusted.

To execute RISC Zero prover in def mode, pass `RISC0_DEV_MODE=1` as environment variable before executing the prover.

Individual prover crates can be executed locally by modifying Dockerfile's or installing missing dependencied as declared in them.