resource "runpod_pod" "gpu_instance" {
  count             = var.cpu_bench ? 0 : 1

  name              = "gpu-instance"
  image_name        = "pkondratas/zkvm-gpu-bench"
  gpu_type_ids      = ["NVIDIA RTX A4500"]  #["NVIDIA RTX PRO 4500 Blackwell"]
  compute_type      = "GPU"

  gpu_count            = 1
  cloud_type           = "COMMUNITY"
  support_public_ip    = true

  volume_in_gb         = 0
  container_disk_in_gb = 50

  ports = ["8888/http", "22/tcp"]
}

resource "runpod_pod" "cpu_instance" {
  count             = var.cpu_bench ? 1 : 0

  name              = "cpu-instance"
  image_name        = "pkondratas/zkvm-cpu-bench"
  compute_type      = "CPU"

  cloud_type           = "COMMUNITY"
  support_public_ip    = true

  cpu_flavor_ids       = ["cpu3g"] # ["cpu5c"]

  #vcpu_count           = 32
  volume_in_gb         = 50
  container_disk_in_gb = 5

  ports = ["8888/http", "22/tcp"]
}