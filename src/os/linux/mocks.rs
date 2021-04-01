#![cfg(test)]

/// Content of /proc/cpuinfo
pub(crate) static CPUINFO: &str = "processor   : 2
vendor_id   : AuthenticAMD
cpu family  : 23
model       : 113
model name  : AMD Ryzen 5 3600 6-Core Processor
stepping    : 0
microcode   : 0x8701013
cpu MHz     : 2053.971
cache size  : 512 KB
physical id : 0
siblings    : 12
core id     : 2
cpu cores   : 6
apicid      : 4
initial apicid  : 4
fpu     : yes
fpu_exception   : yes
cpuid level : 16
wp      : yes
flags       : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs        : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips    : 7189.98
TLB size    : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]";

/// Content of /proc/net/dev
pub(crate) static NET_DEV: &str = "Inter-|   Receive                                                |  Transmit
face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
   lo: 17776656  127989    0    0    0     0          0         0 17776656  127989    0    0    0     0       0          0
enp8s0: 482459368  349468    0    0    0     0          0      4785 16133415  198549    0    0    0     0       0          0
br-211476fe73de:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0
docker0:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0";

/// Content of /proc/{pid}/stat
pub(crate) static PROCESS_STAT: &str = "69035 (alacritty) S 1 69035 69035 0 -1 4194304 32394 0 1 0 3977 293 0 0 20 0 26 0 967628 2158927872 45316 18446744073709551615 94056859889664 94056864021361 140722125732880 0 0 0 0 4100 66624 0 0 0 17 6 0 0 0 0 0 94056865348576 94056865641928 94056873410560 140722125737093 140722125737103 140722125737103 140722125737957 0";
/// Content of /proc/{pid}/stat with whitespace in process name
pub(crate) static PROCESS_STAT_WHITESPACE_NAME: &str = "1483 (tmux: server) S 1 1483 1483 0 -1 4194368 1521 252 0 0 440 132 0 0 20 0 1 0 8224 12197888 1380 18446744073709551615 93969366433792 93969366876629 140722694246592 0 0 0 0 528386 134433281 0 0 0 17 6 0 0 0 0 0 93969367038768 93969367086920 93969395699712 140722694253341 140722694253346 140722694253346 140722694254570 0";

/// Content of /sys/block/{dev}/stat
pub(crate) static SYS_BLOCK_DEV_STAT: &str = "     327       72     8832      957       31        1      206      775        0     1620     2427        0        0        0        0       47      693";

pub(crate) static MOUNTS: &str = "proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
sys /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
dev /dev devtmpfs rw,nosuid,relatime,size=8144744k,nr_inodes=2036186,mode=755,inode64 0 0
run /run tmpfs rw,nosuid,nodev,relatime,mode=755,inode64 0 0
efivarfs /sys/firmware/efi/efivars efivarfs rw,nosuid,nodev,noexec,relatime 0 0
/dev/mapper/vgroot-root / ext4 rw,relatime 0 0
securityfs /sys/kernel/security securityfs rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /dev/shm tmpfs rw,nosuid,nodev,inode64 0 0
devpts /dev/pts devpts rw,nosuid,noexec,relatime,gid=5,mode=620,ptmxmode=000 0 0
tmpfs /sys/fs/cgroup tmpfs ro,nosuid,nodev,noexec,size=4096k,nr_inodes=1024,mode=755,inode64 0 0
cgroup2 /sys/fs/cgroup/unified cgroup2 rw,nosuid,nodev,noexec,relatime,nsdelegate 0 0
cgroup /sys/fs/cgroup/systemd cgroup rw,nosuid,nodev,noexec,relatime,xattr,name=systemd 0 0
pstore /sys/fs/pstore pstore rw,nosuid,nodev,noexec,relatime 0 0
none /sys/fs/bpf bpf rw,nosuid,nodev,noexec,relatime,mode=700 0 0
cgroup /sys/fs/cgroup/rdma cgroup rw,nosuid,nodev,noexec,relatime,rdma 0 0
cgroup /sys/fs/cgroup/cpu,cpuacct cgroup rw,nosuid,nodev,noexec,relatime,cpu,cpuacct 0 0
cgroup /sys/fs/cgroup/cpuset cgroup rw,nosuid,nodev,noexec,relatime,cpuset 0 0
cgroup /sys/fs/cgroup/pids cgroup rw,nosuid,nodev,noexec,relatime,pids 0 0
cgroup /sys/fs/cgroup/freezer cgroup rw,nosuid,nodev,noexec,relatime,freezer 0 0
cgroup /sys/fs/cgroup/blkio cgroup rw,nosuid,nodev,noexec,relatime,blkio 0 0
cgroup /sys/fs/cgroup/net_cls,net_prio cgroup rw,nosuid,nodev,noexec,relatime,net_cls,net_prio 0 0
cgroup /sys/fs/cgroup/memory cgroup rw,nosuid,nodev,noexec,relatime,memory 0 0
cgroup /sys/fs/cgroup/hugetlb cgroup rw,nosuid,nodev,noexec,relatime,hugetlb 0 0
cgroup /sys/fs/cgroup/devices cgroup rw,nosuid,nodev,noexec,relatime,devices 0 0
cgroup /sys/fs/cgroup/perf_event cgroup rw,nosuid,nodev,noexec,relatime,perf_event 0 0
systemd-1 /proc/sys/fs/binfmt_misc autofs rw,relatime,fd=30,pgrp=1,timeout=0,minproto=5,maxproto=5,direct,pipe_ino=12809 0 0
hugetlbfs /dev/hugepages hugetlbfs rw,relatime,pagesize=2M 0 0
mqueue /dev/mqueue mqueue rw,nosuid,nodev,noexec,relatime 0 0
debugfs /sys/kernel/debug debugfs rw,nosuid,nodev,noexec,relatime 0 0
tracefs /sys/kernel/tracing tracefs rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /tmp tmpfs rw,nosuid,nodev,nr_inodes=409600,inode64 0 0
configfs /sys/kernel/config configfs rw,nosuid,nodev,noexec,relatime 0 0
fusectl /sys/fs/fuse/connections fusectl rw,nosuid,nodev,noexec,relatime 0 0
/dev/mapper/vgroot-home /home ext4 rw,relatime 0 0
/dev/mapper/vgroot-var /var ext4 rw,relatime 0 0
/dev/nvme0n1p1 /boot vfat rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=iso8859-1,shortname=mixed,utf8,errors=remount-ro 0 0
/dev/mapper/vgstor-rand /mnt/rand ext4 rw,relatime 0 0
/dev/mapper/vgstor-docs /mnt/docs ext4 rw,relatime 0 0
/dev/mapper/vgstor-media /mnt/media ext4 rw,relatime 0 0
tmpfs /run/user/1000 tmpfs rw,nosuid,nodev,relatime,size=1631388k,nr_inodes=407847,mode=700,uid=1000,gid=1000,inode64 0 0
gvfsd-fuse /run/user/1000/gvfs fuse.gvfsd-fuse rw,nosuid,nodev,relatime,user_id=1000,group_id=1000 0 0";
