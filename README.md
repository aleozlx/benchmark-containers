# benchmark-containers

```bash
# Maximize CPU utilization on all cores (user time)
kubectl create -f cpu.yml

# Maximize context switching (system time)
kubectl create -f cs.yml

# Allocate and free all the physical memory repeatedly
kubectl create -f mem.yml

# Allocate virtual memory till crash
kubectl create -f oom.yml

# Multiply number of processes exponentially
kubectl create -f forkbomb.yml
```