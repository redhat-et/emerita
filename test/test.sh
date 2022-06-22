#!/bin/sh

cni_test() {
    export CNI_PATH=/opt/cni/bin
    sudo ip netns add "cnibench"
    sudo -E ./exec-plugins.sh add "cnibench" "/var/run/netns/cnibench"
    sudo -E ./exec-plugins.sh del "cnibench" "/var/run/netns/cnibench"
    sudo ip netns del "cnibench"
}
