resource "runpod_network_volume" "storage" {
  name           = "storage"
  size           = 10

  data_center_id = "EU-RO-1"
}

resource "runpod_pod" "gpu_instance" {
  count             = var.cpu_bench ? 0 : 1

  name              = "gpu-instance"
  image_name        = "pkondratas/zkvm-gpu-bench"
  compute_type      = "GPU"
  gpu_type_ids      = ["NVIDIA GeForce RTX 5090"]
  data_center_ids   = ["EU-RO-1"]

  gpu_count            = 1
  cloud_type           = "COMMUNITY"
  support_public_ip    = true
  network_volume_id    = runpod_network_volume.storage.id

  container_disk_in_gb = 50

  ports = ["8888/http", "22/tcp"]
}

resource "runpod_pod" "cpu_instance" {
  count             = var.cpu_bench ? 1 : 0

  name              = "cpu-instance"
  image_name        = "pkondratas/zkvm-cpu-bench"
  compute_type      = "CPU"
  data_center_ids   = ["EU-RO-1"]

  cloud_type           = "COMMUNITY"
  support_public_ip    = true
  network_volume_id    = runpod_network_volume.storage.id

  cpu_flavor_ids       = ["cpu5c"]

  vcpu_count           = 32
  container_disk_in_gb = 50

  ports = ["8888/http", "22/tcp"]
}