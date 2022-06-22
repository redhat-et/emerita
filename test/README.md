# cnibench

Benchmark emerita vs bridge plugin.

## Prerequisites

1. Install CNI plugins (including emerita) to `/opt/cni/bin`
2. `curl -sSL -o exec-plugins.sh https://raw.githubusercontent.com/containernetworking/cni/main/scripts/exec-plugins.sh`
3. `https://raw.githubusercontent.com/sharkdp/hyperfine/master/scripts/plot_whisker.py`

## Running the test

```sh
sudo -v
hyperfine --show-output --runs 100 \
    "source $(pwd)/test.sh; export NETCONFPATH=$(pwd)/testdata/emerita;eval cni_test" \
    "source $(pwd)/test.sh; export NETCONFPATH=$(pwd)/testdata/bridge;eval cni_test" \
    --export-json result.json
```

## Results

```sh
python plot_whisker.py result.json
```
