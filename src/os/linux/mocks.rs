#![cfg(test)]

/// Content of /proc/meminfo
pub(crate) static MEMINFO: &str = "MemTotal:       16320968 kB
MemFree:        12528752 kB
MemAvailable:   14641684 kB
Buffers:          127548 kB
Cached:          2158520 kB
SwapCached:            0 kB
Active:          1805296 kB
Inactive:        1319376 kB
Active(anon):     802480 kB
Inactive(anon):    43880 kB
Active(file):    1002816 kB
Inactive(file):  1275496 kB
Unevictable:          32 kB
Mlocked:              32 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Dirty:               632 kB
Writeback:             0 kB
AnonPages:        838632 kB
Mapped:           636564 kB
Shmem:             44848 kB
KReclaimable:     170588 kB
Slab:             317492 kB
SReclaimable:     170588 kB
SUnreclaim:       146904 kB
KernelStack:       13600 kB
PageTables:        11460 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     8160484 kB
Committed_AS:    4216960 kB
VmallocTotal:   34359738367 kB
VmallocUsed:       32520 kB
VmallocChunk:          0 kB
Percpu:            23808 kB
HardwareCorrupted:     0 kB
AnonHugePages:         0 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      527152 kB
DirectMap2M:     6746112 kB
DirectMap1G:    10485760 kB";

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
flags       : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca";

/// Content of /proc/uptime
pub(crate) static UPTIME: &str = "5771.26 68230.61";

/// Output of route command
pub(crate) static ROUTE: &str = "Kernel IP routing table
Destination     Gateway         Genmask         Flags Metric Ref    Use Iface
default         _gateway        0.0.0.0         UG    202    0        0 enp8s0
172.17.0.0      0.0.0.0         255.255.0.0     U     0      0        0 docker0
172.18.0.0      0.0.0.0         255.255.0.0     U     0      0        0 br-211476fe73de
192.168.0.0     0.0.0.0         255.255.255.0   U     202    0        0 enp8s0";

/// Output of ip command
pub(crate) static IP_IFACE: &str = r#"[{"ifindex":2,"ifname":"enp8s0","flags":["BROADCAST","MULTICAST","UP","LOWER_UP"],"mtu":1500,"qdisc":"fq_codel","operstate":"UP","group":"default","txqlen":1000,"link_type":"ether","address":"70:85:c2:f9:9b:2a","broadcast":"ff:ff:ff:ff:ff:ff","addr_info":[{"family":"inet","local":"192.168.0.6","prefixlen":24,"broadcast":"192.168.0.255","scope":"global","dynamic":true,"noprefixroute":true,"label":"enp8s0","valid_life_time":598731,"preferred_life_time":523131},{"family":"inet6","local":"fd00:a84e:3f17:bf12:3d36:34b6:ccc3:1e56","prefixlen":64,"scope":"global","dynamic":true,"mngtmpaddr":true,"noprefixroute":true,"valid_life_time":535426,"preferred_life_time":401569},{"family":"inet6","local":"fe80::3d6e:4143:7f56:3bf9","prefixlen":64,"scope":"link","valid_life_time":4294967295,"preferred_life_time":4294967295}]}]"#;

/// Output of ip command
pub(crate) static IP: &str = r#"[{"ifindex":1,"ifname":"lo","flags":["LOOPBACK","UP","LOWER_UP"],"mtu":65536,"qdisc":"noqueue","operstate":"UNKNOWN","group":"default","txqlen":1000,"link_type":"loopback","address":"00:00:00:00:00:00","broadcast":"00:00:00:00:00:00","addr_info":[{"family":"inet","local":"127.0.0.1","prefixlen":8,"scope":"host","label":"lo","valid_life_time":4294967295,"preferred_life_time":4294967295},{"family":"inet6","local":"::1","prefixlen":128,"scope":"host","valid_life_time":4294967295,"preferred_life_time":4294967295}]},{"ifindex":2,"ifname":"enp8s0","flags":["BROADCAST","MULTICAST","UP","LOWER_UP"],"mtu":1500,"qdisc":"fq_codel","operstate":"UP","group":"default","txqlen":1000,"link_type":"ether","address":"70:85:c2:f9:9b:2a","broadcast":"ff:ff:ff:ff:ff:ff","addr_info":[{"family":"inet","local":"192.168.0.6","prefixlen":24,"broadcast":"192.168.0.255","scope":"global","dynamic":true,"noprefixroute":true,"label":"enp8s0","valid_life_time":598439,"preferred_life_time":522839},{"family":"inet6","local":"fd00:a84e:3f17:bf12:3d36:34b6:ccc3:1e56","prefixlen":64,"scope":"global","dynamic":true,"mngtmpaddr":true,"noprefixroute":true,"valid_life_time":535427,"preferred_life_time":401570},{"family":"inet6","local":"fe80::3d6e:4143:7f56:3bf9","prefixlen":64,"scope":"link","valid_life_time":4294967295,"preferred_life_time":4294967295}]},{"ifindex":3,"ifname":"br-211476fe73de","flags":["NO-CARRIER","BROADCAST","MULTICAST","UP"],"mtu":1500,"qdisc":"noqueue","operstate":"DOWN","group":"default","link_type":"ether","address":"02:42:ca:00:b8:1a","broadcast":"ff:ff:ff:ff:ff:ff","addr_info":[{"family":"inet","local":"172.18.0.1","prefixlen":16,"broadcast":"172.18.255.255","scope":"global","label":"br-211476fe73de","valid_life_time":4294967295,"preferred_life_time":4294967295}]},{"ifindex":4,"ifname":"docker0","flags":["NO-CARRIER","BROADCAST","MULTICAST","UP"],"mtu":1500,"qdisc":"noqueue","operstate":"DOWN","group":"default","link_type":"ether","address":"02:42:29:bc:04:aa","broadcast":"ff:ff:ff:ff:ff:ff","addr_info":[{"family":"inet","local":"172.17.0.1","prefixlen":16,"broadcast":"172.17.255.255","scope":"global","label":"docker0","valid_life_time":4294967295,"preferred_life_time":4294967295}]}]"#;

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
